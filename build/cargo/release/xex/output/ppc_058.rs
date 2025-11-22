pub fn sub_8262D258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262D258 size=16
    let mut pc: u32 = 0x8262D258;
    'dispatch: loop {
        match pc {
            0x8262D258 => {
    //   block [0x8262D258..0x8262D268)
	// 8262D258: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8262D25C: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8262D260: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8262D264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8262D268 size=440
    let mut pc: u32 = 0x8262D268;
    'dispatch: loop {
        match pc {
            0x8262D268 => {
    //   block [0x8262D268..0x8262D420)
	// 8262D268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D26C: 4867C191  bl 0x82ca93fc
	ctx.lr = 0x8262D270;
	sub_82CA93D0(ctx, base);
	// 8262D270: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8262D274: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D278: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262D27C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8262D280: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 8262D284: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 8262D288: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8262D28C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262D290: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8262D294: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8262D298: 419A0078  beq cr6, 0x8262d310
	if ctx.cr[6].eq {
	pc = 0x8262D310; continue 'dispatch;
	}
	// 8262D29C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8262D2A0: C3EB9484  lfs f31, -0x6b7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8262D2A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262D2A8: 4BFFF589  bl 0x8262c830
	ctx.lr = 0x8262D2AC;
	sub_8262C830(ctx, base);
	// 8262D2AC: C01F0010  lfs f0, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262D2B0: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8262D2B4: FF0DF800  fcmpu cr6, f13, f31
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[31].f64);
	// 8262D2B8: 409A0044  bne cr6, 0x8262d2fc
	if !ctx.cr[6].eq {
	pc = 0x8262D2FC; continue 'dispatch;
	}
	// 8262D2BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8262D2C0: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262D2C4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8262D2C8: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 8262D2CC: 48001F05  bl 0x8262f1d0
	ctx.lr = 0x8262D2D0;
	sub_8262F1D0(ctx, base);
	// 8262D2D0: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262D2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262D2D8: 3885FFC0  addi r4, r5, -0x40
	ctx.r[4].s64 = ctx.r[5].s64 + -64;
	// 8262D2DC: 4BFFFDC5  bl 0x8262d0a0
	ctx.lr = 0x8262D2E0;
	sub_8262D0A0(ctx, base);
	// 8262D2E0: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8262D2E4: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262D2E8: 392BFFC0  addi r9, r11, -0x40
	ctx.r[9].s64 = ctx.r[11].s64 + -64;
	// 8262D2EC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8262D2F0: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8262D2F4: 913E001C  stw r9, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 8262D2F8: 4800000C  b 0x8262d304
	pc = 0x8262D304; continue 'dispatch;
	// 8262D2FC: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262D300: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8262D304: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262D308: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8262D30C: 409AFF98  bne cr6, 0x8262d2a4
	if !ctx.cr[6].eq {
	pc = 0x8262D2A4; continue 'dispatch;
	}
	// 8262D310: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8262D314: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262D318: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 8262D31C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8262D320: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8262D324: 419A00EC  beq cr6, 0x8262d410
	if ctx.cr[6].eq {
	pc = 0x8262D410; continue 'dispatch;
	}
	// 8262D328: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 8262D32C: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8262D330: 3B9F002C  addi r28, r31, 0x2c
	ctx.r[28].s64 = ctx.r[31].s64 + 44;
	// 8262D334: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8262D338: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 8262D33C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8262D340: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8262D344: 419A00B8  beq cr6, 0x8262d3fc
	if ctx.cr[6].eq {
	pc = 0x8262D3FC; continue 'dispatch;
	}
	// 8262D348: EBA10060  ld r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8262D34C: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 8262D350: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8262D354: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262D358: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262D35C: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8262D360: FBA10068  std r29, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u64 ) };
	// 8262D364: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8262D368: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D36C: 419A007C  beq cr6, 0x8262d3e8
	if ctx.cr[6].eq {
	pc = 0x8262D3E8; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D420 size=216
    let mut pc: u32 = 0x8262D420;
    'dispatch: loop {
        match pc {
            0x8262D420 => {
    //   block [0x8262D420..0x8262D4F8)
	// 8262D420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D424: 4867BFE5  bl 0x82ca9408
	ctx.lr = 0x8262D428;
	sub_82CA93D0(ctx, base);
	// 8262D428: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D42C: 3D6003FF  lis r11, 0x3ff
	ctx.r[11].s64 = 67043328;
	// 8262D430: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8262D434: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 8262D438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262D43C: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8262D440: 40990010  ble cr6, 0x8262d450
	if !ctx.cr[6].gt {
	pc = 0x8262D450; continue 'dispatch;
	}
	// 8262D444: 4841A085  bl 0x82a474c8
	ctx.lr = 0x8262D448;
	sub_82A474C8(ctx, base);
	// 8262D448: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8262D44C: 4867C00C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 8262D450: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D454: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8262D458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262D45C: 409A000C  bne cr6, 0x8262d468
	if !ctx.cr[6].eq {
	pc = 0x8262D468; continue 'dispatch;
	}
	// 8262D460: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8262D464: 48000010  b 0x8262d474
	pc = 0x8262D474; continue 'dispatch;
	// 8262D468: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8262D46C: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262D470: 7D2B3670  srawi r11, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 8262D474: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262D478: 40980078  bge cr6, 0x8262d4f0
	if !ctx.cr[6].lt {
	pc = 0x8262D4F0; continue 'dispatch;
	}
	// 8262D47C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8262D480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262D484: 483A17AD  bl 0x829cec30
	ctx.lr = 0x8262D488;
	sub_829CEC30(ctx, base);
	// 8262D488: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D48C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D490: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262D494: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8262D498: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8262D49C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8262D4A0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8262D4A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D4A8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8262D4AC: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8262D4B0: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8262D4B4: 48001EA5  bl 0x8262f358
	ctx.lr = 0x8262D4B8;
	sub_8262F358(ctx, base);
	// 8262D4B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D4BC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D4C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8262D4C4: 7D034850  subf r8, r3, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	// 8262D4C8: 7D1D3670  srawi r29, r8, 6
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[8].s32 >> 6) as i64;
	// 8262D4CC: 419A0008  beq cr6, 0x8262d4d4
	if ctx.cr[6].eq {
	pc = 0x8262D4D4; continue 'dispatch;
	}
	// 8262D4D0: 4BBEE869  bl 0x8221bd38
	ctx.lr = 0x8262D4D4;
	sub_8221BD38(ctx, base);
	// 8262D4D4: 578A3032  slwi r10, r28, 6
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262D4D8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8262D4DC: 57AB3032  slwi r11, r29, 6
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262D4E0: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8262D4E4: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8262D4E8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8262D4EC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8262D4F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8262D4F4: 4867BF64  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D4F8 size=252
    let mut pc: u32 = 0x8262D4F8;
    'dispatch: loop {
        match pc {
            0x8262D4F8 => {
    //   block [0x8262D4F8..0x8262D5F4)
	// 8262D4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D4FC: 4867BF05  bl 0x82ca9400
	ctx.lr = 0x8262D500;
	sub_82CA93D0(ctx, base);
	// 8262D500: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D504: 3D6003FF  lis r11, 0x3ff
	ctx.r[11].s64 = 67043328;
	// 8262D508: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8262D50C: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 8262D510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262D514: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8262D518: 40990010  ble cr6, 0x8262d528
	if !ctx.cr[6].gt {
	pc = 0x8262D528; continue 'dispatch;
	}
	// 8262D51C: 48419FAD  bl 0x82a474c8
	ctx.lr = 0x8262D520;
	sub_82A474C8(ctx, base);
	// 8262D520: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8262D524: 4867BF2C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 8262D528: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D52C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262D530: 419A0010  beq cr6, 0x8262d540
	if ctx.cr[6].eq {
	pc = 0x8262D540; continue 'dispatch;
	}
	// 8262D534: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8262D538: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262D53C: 7D2B3670  srawi r11, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 8262D540: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8262D544: 409800A8  bge cr6, 0x8262d5ec
	if !ctx.cr[6].lt {
	pc = 0x8262D5EC; continue 'dispatch;
	}
	// 8262D548: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8262D54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262D550: 483A16E1  bl 0x829cec30
	ctx.lr = 0x8262D554;
	sub_829CEC30(ctx, base);
	// 8262D554: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D55C: 839F0008  lwz r28, 8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D560: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8262D564: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8262D568: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262D56C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8262D570: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D574: E9210050  ld r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8262D578: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8262D57C: 419A002C  beq cr6, 0x8262d5a8
	if ctx.cr[6].eq {
	pc = 0x8262D5A8; continue 'dispatch;
	}
	// 8262D580: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8262D584: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8262D588: 419A0010  beq cr6, 0x8262d598
	if ctx.cr[6].eq {
	pc = 0x8262D598; continue 'dispatch;
	}
	// 8262D58C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262D590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262D594: 48000975  bl 0x8262df08
	ctx.lr = 0x8262D598;
	sub_8262DF08(ctx, base);
	// 8262D598: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 8262D59C: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262D5A0: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262D5A4: 409AFFE0  bne cr6, 0x8262d584
	if !ctx.cr[6].eq {
	pc = 0x8262D584; continue 'dispatch;
	}
	// 8262D5A8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D5AC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D5B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8262D5B4: 7D642850  subf r11, r4, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 8262D5B8: 7D7E3670  srawi r30, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262D5BC: 419A0014  beq cr6, 0x8262d5d0
	if ctx.cr[6].eq {
	pc = 0x8262D5D0; continue 'dispatch;
	}
	// 8262D5C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262D5C4: 4BFFFADD  bl 0x8262d0a0
	ctx.lr = 0x8262D5C8;
	sub_8262D0A0(ctx, base);
	// 8262D5C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D5CC: 4BBEE76D  bl 0x8221bd38
	ctx.lr = 0x8262D5D0;
	sub_8221BD38(ctx, base);
	// 8262D5D0: 574A3032  slwi r10, r26, 6
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262D5D4: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8262D5D8: 57CB3032  slwi r11, r30, 6
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262D5DC: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 8262D5E0: 7D2BDA14  add r9, r11, r27
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8262D5E4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8262D5E8: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8262D5EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8262D5F0: 4867BE60  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D5F8 size=204
    let mut pc: u32 = 0x8262D5F8;
    'dispatch: loop {
        match pc {
            0x8262D5F8 => {
    //   block [0x8262D5F8..0x8262D6C4)
	// 8262D5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D5FC: 4867BE11  bl 0x82ca940c
	ctx.lr = 0x8262D600;
	sub_82CA93D0(ctx, base);
	// 8262D600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D604: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8262D608: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8262D60C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8262D610: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D614: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D618: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8262D61C: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8262D620: 93DD0008  stw r30, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8262D624: 93DD000C  stw r30, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8262D628: 7D243671  srawi. r4, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[9].s32 >> 6) as i64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8262D62C: 4082000C  bne 0x8262d638
	if !ctx.cr[0].eq {
	pc = 0x8262D638; continue 'dispatch;
	}
	// 8262D630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262D634: 4800000C  b 0x8262d640
	pc = 0x8262D640; continue 'dispatch;
	// 8262D638: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8262D63C: 4BFFF555  bl 0x8262cb90
	ctx.lr = 0x8262D640;
	sub_8262CB90(ctx, base);
	// 8262D640: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262D644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262D648: 419A0070  beq cr6, 0x8262d6b8
	if ctx.cr[6].eq {
	pc = 0x8262D6B8; continue 'dispatch;
	}
	// 8262D64C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D650: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D654: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8262D658: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8262D65C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D660: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8262D664: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8262D668: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8262D66C: 419A0048  beq cr6, 0x8262d6b4
	if ctx.cr[6].eq {
	pc = 0x8262D6B4; continue 'dispatch;
	}
	// 8262D670: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8262D674: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8262D678: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8262D67C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262D680: 419A0024  beq cr6, 0x8262d6a4
	if ctx.cr[6].eq {
	pc = 0x8262D6A4; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D6C8 size=592
    let mut pc: u32 = 0x8262D6C8;
    'dispatch: loop {
        match pc {
            0x8262D6C8 => {
    //   block [0x8262D6C8..0x8262D918)
	// 8262D6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D6CC: 4867BD41  bl 0x82ca940c
	ctx.lr = 0x8262D6D0;
	sub_82CA93D0(ctx, base);
	// 8262D6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D6D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8262D6D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8262D6DC: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262D6E0: 419A022C  beq cr6, 0x8262d90c
	if ctx.cr[6].eq {
	pc = 0x8262D90C; continue 'dispatch;
	}
	// 8262D6E4: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D6E8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D6EC: 7D693850  subf r11, r9, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 8262D6F0: 7D683671  srawi. r8, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 6) as i64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8262D6F4: 40820014  bne 0x8262d708
	if !ctx.cr[0].eq {
	pc = 0x8262D708; continue 'dispatch;
	}
	// 8262D6F8: 4BFFF431  bl 0x8262cb28
	ctx.lr = 0x8262D6FC;
	sub_8262CB28(ctx, base);
	// 8262D6FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8262D700: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262D704: 4867BD58  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8262D708: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D70C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D710: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8262D714: 7D4A3670  srawi r10, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 8262D718: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8262D71C: 4199006C  bgt cr6, 0x8262d788
	if ctx.cr[6].gt {
	pc = 0x8262D788; continue 'dispatch;
	}
	// 8262D720: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8262D724: 419A0038  beq cr6, 0x8262d75c
	if ctx.cr[6].eq {
	pc = 0x8262D75C; continue 'dispatch;
	}
	// 8262D728: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8262D72C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8262D730: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8262D734: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8262D738: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8262D73C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8262D740: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8262D744: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8262D748: 4200FFF0  bdnz 0x8262d738
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262D738; continue 'dispatch;
	}
	// 8262D74C: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 8262D750: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 8262D754: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8262D758: 409AFFD0  bne cr6, 0x8262d728
	if !ctx.cr[6].eq {
	pc = 0x8262D728; continue 'dispatch;
	}
	// 8262D75C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D760: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8262D764: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D768: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D76C: 7D095850  subf r8, r9, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8262D770: 7D073670  srawi r7, r8, 6
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[8].s32 >> 6) as i64;
	// 8262D774: 54EB3032  slwi r11, r7, 6
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262D778: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8262D77C: 90DD0008  stw r6, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8262D780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262D784: 4867BCD8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8262D788: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8262D78C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8262D790: 409A000C  bne cr6, 0x8262d79c
	if !ctx.cr[6].eq {
	pc = 0x8262D79C; continue 'dispatch;
	}
	// 8262D794: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8262D798: 48000010  b 0x8262d7a8
	pc = 0x8262D7A8; continue 'dispatch;
	// 8262D79C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8262D7A0: 7D235850  subf r9, r3, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8262D7A4: 7D2B3670  srawi r11, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 8262D7A8: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8262D7AC: 419900C0  bgt cr6, 0x8262d86c
	if ctx.cr[6].gt {
	pc = 0x8262D86C; continue 'dispatch;
	}
	// 8262D7B0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D7B4: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262D7B8: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8262D7BC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8262D7C0: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8262D7C4: 419A0038  beq cr6, 0x8262d7fc
	if ctx.cr[6].eq {
	pc = 0x8262D7FC; continue 'dispatch;
	}
	// 8262D7C8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8262D7CC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8262D7D0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8262D7D4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8262D7D8: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8262D7DC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8262D7E0: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8262D7E4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8262D7E8: 4200FFF0  bdnz 0x8262d7d8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262D7D8; continue 'dispatch;
	}
	// 8262D7EC: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 8262D7F0: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 8262D7F4: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8262D7F8: 409AFFD0  bne cr6, 0x8262d7c8
	if !ctx.cr[6].eq {
	pc = 0x8262D7C8; continue 'dispatch;
	}
	// 8262D7FC: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D800: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D804: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8262D808: 419A0054  beq cr6, 0x8262d85c
	if ctx.cr[6].eq {
	pc = 0x8262D85C; continue 'dispatch;
	}
	// 8262D80C: 39470020  addi r10, r7, 0x20
	ctx.r[10].s64 = ctx.r[7].s64 + 32;
	// 8262D810: 38C0FFE0  li r6, -0x20
	ctx.r[6].s64 = -32;
	// 8262D814: 38E0FFF0  li r7, -0x10
	ctx.r[7].s64 = -16;
	// 8262D818: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8262D81C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8262D820: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262D824: 419A0024  beq cr6, 0x8262d848
	if ctx.cr[6].eq {
	pc = 0x8262D848; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D918 size=176
    let mut pc: u32 = 0x8262D918;
    'dispatch: loop {
        match pc {
            0x8262D918 => {
    //   block [0x8262D918..0x8262D9C8)
	// 8262D918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D91C: 4867BAED  bl 0x82ca9408
	ctx.lr = 0x8262D920;
	sub_82CA93D0(ctx, base);
	// 8262D920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D924: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8262D928: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8262D92C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8262D930: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D934: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D938: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8262D93C: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8262D940: 93DD0008  stw r30, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8262D944: 93DD000C  stw r30, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8262D948: 7D243671  srawi. r4, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[9].s32 >> 6) as i64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8262D94C: 4082000C  bne 0x8262d958
	if !ctx.cr[0].eq {
	pc = 0x8262D958; continue 'dispatch;
	}
	// 8262D950: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262D954: 4800000C  b 0x8262d960
	pc = 0x8262D960; continue 'dispatch;
	// 8262D958: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8262D95C: 4BFFF235  bl 0x8262cb90
	ctx.lr = 0x8262D960;
	sub_8262CB90(ctx, base);
	// 8262D960: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262D964: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262D968: 419A0054  beq cr6, 0x8262d9bc
	if ctx.cr[6].eq {
	pc = 0x8262D9BC; continue 'dispatch;
	}
	// 8262D96C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D970: 839F0008  lwz r28, 8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D974: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8262D978: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262D97C: 83FD0004  lwz r31, 4(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D980: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D984: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8262D988: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8262D98C: 419A002C  beq cr6, 0x8262d9b8
	if ctx.cr[6].eq {
	pc = 0x8262D9B8; continue 'dispatch;
	}
	// 8262D990: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8262D994: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262D998: 419A0010  beq cr6, 0x8262d9a8
	if ctx.cr[6].eq {
	pc = 0x8262D9A8; continue 'dispatch;
	}
	// 8262D99C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8262D9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262D9A4: 48000565  bl 0x8262df08
	ctx.lr = 0x8262D9A8;
	sub_8262DF08(ctx, base);
	// 8262D9A8: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262D9AC: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262D9B0: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262D9B4: 409AFFE0  bne cr6, 0x8262d994
	if !ctx.cr[6].eq {
	pc = 0x8262D994; continue 'dispatch;
	}
	// 8262D9B8: 93FD0008  stw r31, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8262D9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8262D9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D9C4: 4867BA94  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D9C8 size=476
    let mut pc: u32 = 0x8262D9C8;
    'dispatch: loop {
        match pc {
            0x8262D9C8 => {
    //   block [0x8262D9C8..0x8262DBA4)
	// 8262D9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D9CC: 4867BA3D  bl 0x82ca9408
	ctx.lr = 0x8262D9D0;
	sub_82CA93D0(ctx, base);
	// 8262D9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D9D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262D9D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8262D9DC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262D9E0: 419A01B8  beq cr6, 0x8262db98
	if ctx.cr[6].eq {
	pc = 0x8262DB98; continue 'dispatch;
	}
	// 8262D9E4: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262D9E8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262D9EC: 7D632050  subf r11, r3, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 8262D9F0: 7D693671  srawi. r9, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 6) as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262D9F4: 40820018  bne 0x8262da0c
	if !ctx.cr[0].eq {
	pc = 0x8262DA0C; continue 'dispatch;
	}
	// 8262D9F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262D9FC: 4BFFF62D  bl 0x8262d028
	ctx.lr = 0x8262DA00;
	sub_8262D028(ctx, base);
	// 8262DA00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DA08: 4867BA50  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 8262DA0C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DA10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DA14: 7D4B2850  subf r10, r11, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 8262DA18: 7D4A3670  srawi r10, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 8262DA1C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8262DA20: 41990060  bgt cr6, 0x8262da80
	if ctx.cr[6].gt {
	pc = 0x8262DA80; continue 'dispatch;
	}
	// 8262DA24: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8262DA28: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8262DA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DA30: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8262DA34: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8262DA38: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8262DA3C: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8262DA40: 48001791  bl 0x8262f1d0
	ctx.lr = 0x8262DA44;
	sub_8262F1D0(ctx, base);
	// 8262DA44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8262DA48: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DA4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DA50: 4BFFF651  bl 0x8262d0a0
	ctx.lr = 0x8262DA54;
	sub_8262D0A0(ctx, base);
	// 8262DA54: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DA58: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DA5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DA60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DA64: 7CC74050  subf r6, r7, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 8262DA68: 7CC53670  srawi r5, r6, 6
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[6].s32 >> 6) as i64;
	// 8262DA6C: 54AA3032  slwi r10, r5, 6
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262DA70: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8262DA74: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8262DA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DA7C: 4867B9DC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 8262DA80: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8262DA84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262DA88: 409A000C  bne cr6, 0x8262da94
	if !ctx.cr[6].eq {
	pc = 0x8262DA94; continue 'dispatch;
	}
	// 8262DA8C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8262DA90: 48000010  b 0x8262daa0
	pc = 0x8262DAA0; continue 'dispatch;
	// 8262DA94: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8262DA98: 7CEB4050  subf r7, r11, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 8262DA9C: 7CE83670  srawi r8, r7, 6
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[7].s32 >> 6) as i64;
	// 8262DAA0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262DAA4: 41990060  bgt cr6, 0x8262db04
	if ctx.cr[6].gt {
	pc = 0x8262DB04; continue 'dispatch;
	}
	// 8262DAA8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DAAC: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262DAB0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8262DAB4: 7FAA1A14  add r29, r10, r3
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8262DAB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262DABC: 48001715  bl 0x8262f1d0
	ctx.lr = 0x8262DAC0;
	sub_8262F1D0(ctx, base);
	// 8262DAC0: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DAC4: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DAC8: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262DACC: 419A0028  beq cr6, 0x8262daf4
	if ctx.cr[6].eq {
	pc = 0x8262DAF4; continue 'dispatch;
	}
	// 8262DAD0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8262DAD4: 419A0010  beq cr6, 0x8262dae4
	if ctx.cr[6].eq {
	pc = 0x8262DAE4; continue 'dispatch;
	}
	// 8262DAD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262DADC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262DAE0: 48000429  bl 0x8262df08
	ctx.lr = 0x8262DAE4;
	sub_8262DF08(ctx, base);
	// 8262DAE4: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 8262DAE8: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262DAEC: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262DAF0: 409AFFE0  bne cr6, 0x8262dad0
	if !ctx.cr[6].eq {
	pc = 0x8262DAD0; continue 'dispatch;
	}
	// 8262DAF4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8262DAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DAFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DB00: 4867B958  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 8262DB04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262DB08: 419A0018  beq cr6, 0x8262db20
	if ctx.cr[6].eq {
	pc = 0x8262DB20; continue 'dispatch;
	}
	// 8262DB0C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8262DB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DB14: 4BFFF58D  bl 0x8262d0a0
	ctx.lr = 0x8262DB18;
	sub_8262D0A0(ctx, base);
	// 8262DB18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DB1C: 4BBEE21D  bl 0x8221bd38
	ctx.lr = 0x8262DB20;
	sub_8221BD38(ctx, base);
	// 8262DB20: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DB24: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DB28: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8262DB2C: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8262DB30: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8262DB34: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8262DB38: 7D243671  srawi. r4, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[9].s32 >> 6) as i64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8262DB3C: 4082000C  bne 0x8262db48
	if !ctx.cr[0].eq {
	pc = 0x8262DB48; continue 'dispatch;
	}
	// 8262DB40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8262DB44: 4800000C  b 0x8262db50
	pc = 0x8262DB50; continue 'dispatch;
	// 8262DB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DB4C: 4BFFF045  bl 0x8262cb90
	ctx.lr = 0x8262DB50;
	sub_8262CB90(ctx, base);
	// 8262DB50: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262DB54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262DB58: 419A0040  beq cr6, 0x8262db98
	if ctx.cr[6].eq {
	pc = 0x8262DB98; continue 'dispatch;
	}
	// 8262DB5C: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DB60: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DB64: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DB68: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262DB6C: 419A0028  beq cr6, 0x8262db94
	if ctx.cr[6].eq {
	pc = 0x8262DB94; continue 'dispatch;
	}
	// 8262DB70: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8262DB74: 419A0010  beq cr6, 0x8262db84
	if ctx.cr[6].eq {
	pc = 0x8262DB84; continue 'dispatch;
	}
	// 8262DB78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8262DB7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8262DB80: 48000389  bl 0x8262df08
	ctx.lr = 0x8262DB84;
	sub_8262DF08(ctx, base);
	// 8262DB84: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262DB88: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 8262DB8C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262DB90: 409AFFE0  bne cr6, 0x8262db70
	if !ctx.cr[6].eq {
	pc = 0x8262DB70; continue 'dispatch;
	}
	// 8262DB94: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8262DB98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DB9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DBA0: 4867B8B8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DBA8 size=664
    let mut pc: u32 = 0x8262DBA8;
    'dispatch: loop {
        match pc {
            0x8262DBA8 => {
    //   block [0x8262DBA8..0x8262DE40)
	// 8262DBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DBAC: 4867B851  bl 0x82ca93fc
	ctx.lr = 0x8262DBB0;
	sub_82CA93D0(ctx, base);
	// 8262DBB0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DBB4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8262DBB8: F88100E8  std r4, 0xe8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[4].u64 ) };
	// 8262DBBC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8262DBC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8262DBC4: 48000345  bl 0x8262df08
	ctx.lr = 0x8262DBC8;
	sub_8262DF08(ctx, base);
	// 8262DBC8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DBCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262DBD0: 409A000C  bne cr6, 0x8262dbdc
	if !ctx.cr[6].eq {
	pc = 0x8262DBDC; continue 'dispatch;
	}
	// 8262DBD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DBD8: 48000010  b 0x8262dbe8
	pc = 0x8262DBE8; continue 'dispatch;
	// 8262DBDC: 815A000C  lwz r10, 0xc(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 8262DBE0: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262DBE4: 7D293670  srawi r9, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 8262DBE8: 83DA0008  lwz r30, 8(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DBEC: 3D4003FF  lis r10, 0x3ff
	ctx.r[10].s64 = 67043328;
	// 8262DBF0: 7D0BF050  subf r8, r11, r30
	ctx.r[8].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8262DBF4: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8262DBF8: 7D0B3670  srawi r11, r8, 6
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 6) as i64;
	// 8262DBFC: 7CEB5050  subf r7, r11, r10
	ctx.r[7].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262DC00: 2B070001  cmplwi cr6, r7, 1
	ctx.cr[6].compare_u32(ctx.r[7].u32, 1 as u32, &mut ctx.xer);
	// 8262DC04: 40980018  bge cr6, 0x8262dc1c
	if !ctx.cr[6].lt {
	pc = 0x8262DC1C; continue 'dispatch;
	}
	// 8262DC08: 484198C1  bl 0x82a474c8
	ctx.lr = 0x8262DC0C;
	sub_82A474C8(ctx, base);
	// 8262DC0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8262DC10: 4BFF8571  bl 0x82626180
	ctx.lr = 0x8262DC14;
	sub_82626180(ctx, base);
	// 8262DC14: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8262DC18: 4867B834  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 8262DC1C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 8262DC20: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262DC24: 4098011C  bge cr6, 0x8262dd40
	if !ctx.cr[6].lt {
	pc = 0x8262DD40; continue 'dispatch;
	}
	// 8262DC28: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262DC2C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8262DC30: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262DC34: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8262DC38: 41980008  blt cr6, 0x8262dc40
	if ctx.cr[6].lt {
	pc = 0x8262DC40; continue 'dispatch;
	}
	// 8262DC3C: 7F2B4A14  add r25, r11, r9
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8262DC40: 7F194040  cmplw cr6, r25, r8
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262DC44: 40980008  bge cr6, 0x8262dc4c
	if !ctx.cr[6].lt {
	pc = 0x8262DC4C; continue 'dispatch;
	}
	// 8262DC48: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8262DC4C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8262DC50: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8262DC54: 483A0FDD  bl 0x829cec30
	ctx.lr = 0x8262DC58;
	sub_829CEC30(ctx, base);
	// 8262DC58: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DC5C: 83A100EC  lwz r29, 0xec(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(236 as u32) ) } as u64;
	// 8262DC60: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8262DC64: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262DC68: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 8262DC6C: 419A0028  beq cr6, 0x8262dc94
	if ctx.cr[6].eq {
	pc = 0x8262DC94; continue 'dispatch;
	}
	// 8262DC70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262DC74: 419A0010  beq cr6, 0x8262dc84
	if ctx.cr[6].eq {
	pc = 0x8262DC84; continue 'dispatch;
	}
	// 8262DC78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8262DC7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DC80: 48000289  bl 0x8262df08
	ctx.lr = 0x8262DC84;
	sub_8262DF08(ctx, base);
	// 8262DC84: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262DC88: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262DC8C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262DC90: 409AFFE0  bne cr6, 0x8262dc70
	if !ctx.cr[6].eq {
	pc = 0x8262DC70; continue 'dispatch;
	}
	// 8262DC94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262DC98: 419A0010  beq cr6, 0x8262dca8
	if ctx.cr[6].eq {
	pc = 0x8262DCA8; continue 'dispatch;
	}
	// 8262DC9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8262DCA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DCA4: 48000265  bl 0x8262df08
	ctx.lr = 0x8262DCA8;
	sub_8262DF08(ctx, base);
	// 8262DCA8: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DCAC: 3BDF0040  addi r30, r31, 0x40
	ctx.r[30].s64 = ctx.r[31].s64 + 64;
	// 8262DCB0: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262DCB4: 419A0034  beq cr6, 0x8262dce8
	if ctx.cr[6].eq {
	pc = 0x8262DCE8; continue 'dispatch;
	}
	// 8262DCB8: 7D7FF050  subf r11, r31, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 8262DCBC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8262DCC0: 3BEBFFC0  addi r31, r11, -0x40
	ctx.r[31].s64 = ctx.r[11].s64 + -64;
	// 8262DCC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8262DCC8: 419A0010  beq cr6, 0x8262dcd8
	if ctx.cr[6].eq {
	pc = 0x8262DCD8; continue 'dispatch;
	}
	// 8262DCCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8262DCD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262DCD4: 48000235  bl 0x8262df08
	ctx.lr = 0x8262DCD8;
	sub_8262DF08(ctx, base);
	// 8262DCD8: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262DCDC: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262DCE0: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262DCE4: 409AFFE0  bne cr6, 0x8262dcc4
	if !ctx.cr[6].eq {
	pc = 0x8262DCC4; continue 'dispatch;
	}
	// 8262DCE8: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DCEC: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DCF0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8262DCF4: 7D642850  subf r11, r4, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 8262DCF8: 7D6B3670  srawi r11, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262DCFC: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 8262DD00: 419A0014  beq cr6, 0x8262dd14
	if ctx.cr[6].eq {
	pc = 0x8262DD14; continue 'dispatch;
	}
	// 8262DD04: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8262DD08: 4BFFF399  bl 0x8262d0a0
	ctx.lr = 0x8262DD0C;
	sub_8262D0A0(ctx, base);
	// 8262DD0C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DD10: 4BBEE029  bl 0x8221bd38
	ctx.lr = 0x8262DD14;
	sub_8221BD38(ctx, base);
	// 8262DD14: 57EB3032  slwi r11, r31, 6
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262DD18: 937A0004  stw r27, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8262DD1C: 572A3032  slwi r10, r25, 6
	ctx.r[10].u32 = ctx.r[25].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262DD20: 7D2BDA14  add r9, r11, r27
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8262DD24: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 8262DD28: 913A0008  stw r9, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8262DD2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8262DD30: 915A000C  stw r10, 0xc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8262DD34: 4BFF844D  bl 0x82626180
	ctx.lr = 0x8262DD38;
	sub_82626180(ctx, base);
	// 8262DD38: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8262DD3C: 4867B710  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 8262DD40: 836100EC  lwz r27, 0xec(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(236 as u32) ) } as u64;
	// 8262DD44: 7D7BF050  subf r11, r27, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[27].s64;
	// 8262DD48: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262DD4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8262DD50: 40980084  bge cr6, 0x8262ddd4
	if !ctx.cr[6].lt {
	pc = 0x8262DDD4; continue 'dispatch;
	}
	// 8262DD54: 3BFB0040  addi r31, r27, 0x40
	ctx.r[31].s64 = ctx.r[27].s64 + 64;
	// 8262DD58: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262DD5C: 419A002C  beq cr6, 0x8262dd88
	if ctx.cr[6].eq {
	pc = 0x8262DD88; continue 'dispatch;
	}
	// 8262DD60: 3BBFFFC0  addi r29, r31, -0x40
	ctx.r[29].s64 = ctx.r[31].s64 + -64;
	// 8262DD64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262DD68: 419A0010  beq cr6, 0x8262dd78
	if ctx.cr[6].eq {
	pc = 0x8262DD78; continue 'dispatch;
	}
	// 8262DD6C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262DD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DD74: 48000195  bl 0x8262df08
	ctx.lr = 0x8262DD78;
	sub_8262DF08(ctx, base);
	// 8262DD78: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 8262DD7C: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262DD80: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262DD84: 409AFFE0  bne cr6, 0x8262dd64
	if !ctx.cr[6].eq {
	pc = 0x8262DD64; continue 'dispatch;
	}
	// 8262DD88: 83DA0008  lwz r30, 8(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DD8C: 7D7BF050  subf r11, r27, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[27].s64;
	// 8262DD90: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262DD94: 23EA0001  subfic r31, r10, 1
	ctx.xer.ca = ctx.r[10].u32 <= 1 as u32;
	ctx.r[31].s64 = (1 as i64) - ctx.r[10].s64;
	// 8262DD98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262DD9C: 419A0024  beq cr6, 0x8262ddc0
	if ctx.cr[6].eq {
	pc = 0x8262DDC0; continue 'dispatch;
	}
	// 8262DDA0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8262DDA4: 419A0010  beq cr6, 0x8262ddb4
	if ctx.cr[6].eq {
	pc = 0x8262DDB4; continue 'dispatch;
	}
	// 8262DDA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8262DDAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262DDB0: 48000159  bl 0x8262df08
	ctx.lr = 0x8262DDB4;
	sub_8262DF08(ctx, base);
	// 8262DDB4: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8262DDB8: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262DDBC: 4082FFE4  bne 0x8262dda0
	if !ctx.cr[0].eq {
	pc = 0x8262DDA0; continue 'dispatch;
	}
	// 8262DDC0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DDC4: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 8262DDC8: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262DDCC: 388BFFC0  addi r4, r11, -0x40
	ctx.r[4].s64 = ctx.r[11].s64 + -64;
	// 8262DDD0: 48000054  b 0x8262de24
	pc = 0x8262DE24; continue 'dispatch;
	// 8262DDD4: 3B9EFFC0  addi r28, r30, -0x40
	ctx.r[28].s64 = ctx.r[30].s64 + -64;
	// 8262DDD8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8262DDDC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8262DDE0: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262DDE4: 419A0028  beq cr6, 0x8262de0c
	if ctx.cr[6].eq {
	pc = 0x8262DE0C; continue 'dispatch;
	}
	// 8262DDE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262DDEC: 419A0010  beq cr6, 0x8262ddfc
	if ctx.cr[6].eq {
	pc = 0x8262DDFC; continue 'dispatch;
	}
	// 8262DDF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262DDF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DDF8: 48000111  bl 0x8262df08
	ctx.lr = 0x8262DDFC;
	sub_8262DF08(ctx, base);
	// 8262DDFC: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 8262DE00: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262DE04: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262DE08: 409AFFE0  bne cr6, 0x8262dde8
	if !ctx.cr[6].eq {
	pc = 0x8262DDE8; continue 'dispatch;
	}
	// 8262DE0C: 93FA0008  stw r31, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8262DE10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8262DE14: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8262DE18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8262DE1C: 480005DD  bl 0x8262e3f8
	ctx.lr = 0x8262DE20;
	sub_8262E3F8(ctx, base);
	// 8262DE20: 389B0040  addi r4, r27, 0x40
	ctx.r[4].s64 = ctx.r[27].s64 + 64;
	// 8262DE24: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8262DE28: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8262DE2C: 48000555  bl 0x8262e380
	ctx.lr = 0x8262DE30;
	sub_8262E380(ctx, base);
	// 8262DE30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8262DE34: 4BFF834D  bl 0x82626180
	ctx.lr = 0x8262DE38;
	sub_82626180(ctx, base);
	// 8262DE38: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8262DE3C: 4867B610  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262DE40 size=52
    let mut pc: u32 = 0x8262DE40;
    'dispatch: loop {
        match pc {
            0x8262DE40 => {
    //   block [0x8262DE40..0x8262DE74)
	// 8262DE40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262DE44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262DE48: 409A0008  bne cr6, 0x8262de50
	if !ctx.cr[6].eq {
	pc = 0x8262DE50; continue 'dispatch;
	}
	// 8262DE4C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 8262DE50: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DE54: 894B0061  lbz r10, 0x61(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DE58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262DE5C: 419A0020  beq cr6, 0x8262de7c
	if ctx.cr[6].eq {
		sub_8262DE7C(ctx, base);
		return;
	}
	// 8262DE60: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DE64: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8262DE68: 894B0061  lbz r10, 0x61(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DE6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262DE70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DE74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262DE74 size=8
    let mut pc: u32 = 0x8262DE74;
    'dispatch: loop {
        match pc {
            0x8262DE74 => {
    //   block [0x8262DE74..0x8262DE7C)
	// 8262DE74: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 8262DE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DE7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262DE7C size=60
    let mut pc: u32 = 0x8262DE7C;
    'dispatch: loop {
        match pc {
            0x8262DE7C => {
    //   block [0x8262DE7C..0x8262DEB8)
	// 8262DE7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262DE80: 892A0061  lbz r9, 0x61(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DE84: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262DE88: 409A0030  bne cr6, 0x8262deb8
	if !ctx.cr[6].eq {
		sub_8262DEB8(ctx, base);
		return;
	}
	// 8262DE8C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DE90: 892B0061  lbz r9, 0x61(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DE94: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262DE98: 409A0018  bne cr6, 0x8262deb0
	if !ctx.cr[6].eq {
	pc = 0x8262DEB0; continue 'dispatch;
	}
	// 8262DE9C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8262DEA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262DEA4: 892B0061  lbz r9, 0x61(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DEA8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262DEAC: 419AFFF0  beq cr6, 0x8262de9c
	if ctx.cr[6].eq {
	pc = 0x8262DE9C; continue 'dispatch;
	}
	// 8262DEB0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8262DEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262DEB8 size=80
    let mut pc: u32 = 0x8262DEB8;
    'dispatch: loop {
        match pc {
            0x8262DEB8 => {
    //   block [0x8262DEB8..0x8262DF08)
	// 8262DEB8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DEBC: 894B0061  lbz r10, 0x61(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DEC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262DEC4: 409A002C  bne cr6, 0x8262def0
	if !ctx.cr[6].eq {
	pc = 0x8262DEF0; continue 'dispatch;
	}
	// 8262DEC8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DECC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262DED0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8262DED4: 409A001C  bne cr6, 0x8262def0
	if !ctx.cr[6].eq {
	pc = 0x8262DEF0; continue 'dispatch;
	}
	// 8262DED8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8262DEDC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262DEE0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DEE4: 894B0061  lbz r10, 0x61(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DEE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262DEEC: 419AFFDC  beq cr6, 0x8262dec8
	if ctx.cr[6].eq {
	pc = 0x8262DEC8; continue 'dispatch;
	}
	// 8262DEF0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262DEF4: 892A0061  lbz r9, 0x61(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(97 as u32) ) } as u64;
	// 8262DEF8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262DEFC: 409AFF78  bne cr6, 0x8262de74
	if !ctx.cr[6].eq {
		sub_8262DE74(ctx, base);
		return;
	}
	// 8262DF00: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8262DF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DF08 size=104
    let mut pc: u32 = 0x8262DF08;
    'dispatch: loop {
        match pc {
            0x8262DF08 => {
    //   block [0x8262DF08..0x8262DF70)
	// 8262DF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DF10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262DF14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262DF18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DF1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8262DF20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262DF24: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 8262DF28: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DF70 size=136
    let mut pc: u32 = 0x8262DF70;
    'dispatch: loop {
        match pc {
            0x8262DF70 => {
    //   block [0x8262DF70..0x8262DFF8)
	// 8262DF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DF78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262DF7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DF80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262DF84: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8262DF88: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8262DF8C: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8262DF90: 39099128  addi r8, r9, -0x6ed8
	ctx.r[8].s64 = ctx.r[9].s64 + -28376;
	// 8262DF94: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262DF98: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8262DF9C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262DFA0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262DFA4: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8262DFA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262DFAC: 409AFFF0  bne cr6, 0x8262df9c
	if !ctx.cr[6].eq {
	pc = 0x8262DF9C; continue 'dispatch;
	}
	// 8262DFB0: 90BF0104  stw r5, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[5].u32 ) };
	// 8262DFB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DFB8: 480007C9  bl 0x8262e780
	ctx.lr = 0x8262DFBC;
	sub_8262E780(ctx, base);
	// 8262DFBC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262DFC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262DFC4: 409A000C  bne cr6, 0x8262dfd0
	if !ctx.cr[6].eq {
	pc = 0x8262DFD0; continue 'dispatch;
	}
	// 8262DFC8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8262DFCC: 48000010  b 0x8262dfdc
	pc = 0x8262DFDC; continue 'dispatch;
	// 8262DFD0: 480004B9  bl 0x8262e488
	ctx.lr = 0x8262DFD4;
	sub_8262E488(ctx, base);
	// 8262DFD4: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262DFD8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8262DFDC: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 8262DFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262DFE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262DFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DFF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262DFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DFF8 size=136
    let mut pc: u32 = 0x8262DFF8;
    'dispatch: loop {
        match pc {
            0x8262DFF8 => {
    //   block [0x8262DFF8..0x8262E080)
	// 8262DFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E004: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262E00C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8262E010: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8262E014: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8262E018: 39099128  addi r8, r9, -0x6ed8
	ctx.r[8].s64 = ctx.r[9].s64 + -28376;
	// 8262E01C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262E020: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8262E024: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E028: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262E02C: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8262E030: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262E034: 409AFFF0  bne cr6, 0x8262e024
	if !ctx.cr[6].eq {
	pc = 0x8262E024; continue 'dispatch;
	}
	// 8262E038: 90BF0104  stw r5, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[5].u32 ) };
	// 8262E03C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E040: 48000819  bl 0x8262e858
	ctx.lr = 0x8262E044;
	sub_8262E858(ctx, base);
	// 8262E044: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262E048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E04C: 409A000C  bne cr6, 0x8262e058
	if !ctx.cr[6].eq {
	pc = 0x8262E058; continue 'dispatch;
	}
	// 8262E050: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8262E054: 48000010  b 0x8262e064
	pc = 0x8262E064; continue 'dispatch;
	// 8262E058: 480004C9  bl 0x8262e520
	ctx.lr = 0x8262E05C;
	sub_8262E520(ctx, base);
	// 8262E05C: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E060: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8262E064: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 8262E068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E06C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E080 size=136
    let mut pc: u32 = 0x8262E080;
    'dispatch: loop {
        match pc {
            0x8262E080 => {
    //   block [0x8262E080..0x8262E108)
	// 8262E080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E08C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E090: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262E094: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8262E098: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8262E09C: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8262E0A0: 39099128  addi r8, r9, -0x6ed8
	ctx.r[8].s64 = ctx.r[9].s64 + -28376;
	// 8262E0A4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262E0A8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8262E0AC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E0B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262E0B4: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8262E0B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262E0BC: 409AFFF0  bne cr6, 0x8262e0ac
	if !ctx.cr[6].eq {
	pc = 0x8262E0AC; continue 'dispatch;
	}
	// 8262E0C0: 90BF0104  stw r5, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[5].u32 ) };
	// 8262E0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E0C8: 48000869  bl 0x8262e930
	ctx.lr = 0x8262E0CC;
	sub_8262E930(ctx, base);
	// 8262E0CC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262E0D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E0D4: 409A000C  bne cr6, 0x8262e0e0
	if !ctx.cr[6].eq {
	pc = 0x8262E0E0; continue 'dispatch;
	}
	// 8262E0D8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8262E0DC: 48000010  b 0x8262e0ec
	pc = 0x8262E0EC; continue 'dispatch;
	// 8262E0E0: 480004D9  bl 0x8262e5b8
	ctx.lr = 0x8262E0E4;
	sub_8262E5B8(ctx, base);
	// 8262E0E4: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E0E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8262E0EC: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 8262E0F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E0F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E0F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E0FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E108 size=136
    let mut pc: u32 = 0x8262E108;
    'dispatch: loop {
        match pc {
            0x8262E108 => {
    //   block [0x8262E108..0x8262E190)
	// 8262E108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262E11C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8262E120: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8262E124: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8262E128: 39099128  addi r8, r9, -0x6ed8
	ctx.r[8].s64 = ctx.r[9].s64 + -28376;
	// 8262E12C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262E130: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8262E134: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E138: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262E13C: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8262E140: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262E144: 409AFFF0  bne cr6, 0x8262e134
	if !ctx.cr[6].eq {
	pc = 0x8262E134; continue 'dispatch;
	}
	// 8262E148: 90BF0104  stw r5, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[5].u32 ) };
	// 8262E14C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E150: 480008B9  bl 0x8262ea08
	ctx.lr = 0x8262E154;
	sub_8262EA08(ctx, base);
	// 8262E154: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262E158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E15C: 409A000C  bne cr6, 0x8262e168
	if !ctx.cr[6].eq {
	pc = 0x8262E168; continue 'dispatch;
	}
	// 8262E160: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8262E164: 48000010  b 0x8262e174
	pc = 0x8262E174; continue 'dispatch;
	// 8262E168: 480004E9  bl 0x8262e650
	ctx.lr = 0x8262E16C;
	sub_8262E650(ctx, base);
	// 8262E16C: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E170: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8262E174: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 8262E178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E17C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E188: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E190 size=136
    let mut pc: u32 = 0x8262E190;
    'dispatch: loop {
        match pc {
            0x8262E190 => {
    //   block [0x8262E190..0x8262E218)
	// 8262E190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E198: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E19C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E1A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262E1A4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8262E1A8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8262E1AC: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8262E1B0: 39099128  addi r8, r9, -0x6ed8
	ctx.r[8].s64 = ctx.r[9].s64 + -28376;
	// 8262E1B4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262E1B8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8262E1BC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E1C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262E1C4: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8262E1C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262E1CC: 409AFFF0  bne cr6, 0x8262e1bc
	if !ctx.cr[6].eq {
	pc = 0x8262E1BC; continue 'dispatch;
	}
	// 8262E1D0: 90BF0104  stw r5, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[5].u32 ) };
	// 8262E1D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E1D8: 48000909  bl 0x8262eae0
	ctx.lr = 0x8262E1DC;
	sub_8262EAE0(ctx, base);
	// 8262E1DC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262E1E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E1E4: 409A000C  bne cr6, 0x8262e1f0
	if !ctx.cr[6].eq {
	pc = 0x8262E1F0; continue 'dispatch;
	}
	// 8262E1E8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8262E1EC: 48000010  b 0x8262e1fc
	pc = 0x8262E1FC; continue 'dispatch;
	// 8262E1F0: 480004F9  bl 0x8262e6e8
	ctx.lr = 0x8262E1F4;
	sub_8262E6E8(ctx, base);
	// 8262E1F4: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E1F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8262E1FC: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 8262E200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E210: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E218 size=260
    let mut pc: u32 = 0x8262E218;
    'dispatch: loop {
        match pc {
            0x8262E218 => {
    //   block [0x8262E218..0x8262E31C)
	// 8262E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E21C: 4867B1E9  bl 0x82ca9404
	ctx.lr = 0x8262E220;
	sub_82CA93D0(ctx, base);
	// 8262E220: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262E228: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8262E22C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8262E230: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8262E234: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E23C: 419A00D8  beq cr6, 0x8262e314
	if ctx.cr[6].eq {
	pc = 0x8262E314; continue 'dispatch;
	}
	// 8262E240: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262E244: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8262E248: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E24C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E250: 388BEE10  addi r4, r11, -0x11f0
	ctx.r[4].s64 = ctx.r[11].s64 + -4592;
	// 8262E254: 4BB6C82D  bl 0x8219aa80
	ctx.lr = 0x8262E258;
	sub_8219AA80(ctx, base);
	// 8262E258: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E25C: 3880D8F0  li r4, -0x2710
	ctx.r[4].s64 = -10000;
	// 8262E260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E264: 80AA0008  lwz r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262E268: 4BBFAEA1  bl 0x82229108
	ctx.lr = 0x8262E26C;
	sub_82229108(ctx, base);
	// 8262E26C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E274: 419A0014  beq cr6, 0x8262e288
	if ctx.cr[6].eq {
	pc = 0x8262E288; continue 'dispatch;
	}
	// 8262E278: 3880D8F0  li r4, -0x2710
	ctx.r[4].s64 = -10000;
	// 8262E27C: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262E280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E284: 4BBFAE85  bl 0x82229108
	ctx.lr = 0x8262E288;
	sub_82229108(ctx, base);
	// 8262E288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262E320 size=12
    let mut pc: u32 = 0x8262E320;
    'dispatch: loop {
        match pc {
            0x8262E320 => {
    //   block [0x8262E320..0x8262E32C)
	// 8262E320: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8262E324: 7F042840  cmplw cr6, r4, r5
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8262E328: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E32C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262E32C size=84
    let mut pc: u32 = 0x8262E32C;
    'dispatch: loop {
        match pc {
            0x8262E32C => {
    //   block [0x8262E32C..0x8262E380)
	// 8262E32C: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 8262E330: 7D041850  subf r8, r4, r3
	ctx.r[8].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 8262E334: 3920FFE0  li r9, -0x20
	ctx.r[9].s64 = -32;
	// 8262E338: 38E0FFF0  li r7, -0x10
	ctx.r[7].s64 = -16;
	// 8262E33C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8262E340: 419A0028  beq cr6, 0x8262e368
	if ctx.cr[6].eq {
	pc = 0x8262E368; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E380 size=116
    let mut pc: u32 = 0x8262E380;
    'dispatch: loop {
        match pc {
            0x8262E380 => {
    //   block [0x8262E380..0x8262E3F4)
	// 8262E380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E384: 4867B07D  bl 0x82ca9400
	ctx.lr = 0x8262E388;
	sub_82CA93D0(ctx, base);
	// 8262E388: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E38C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8262E390: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8262E394: 7F03D840  cmplw cr6, r3, r27
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8262E398: 419A0054  beq cr6, 0x8262e3ec
	if ctx.cr[6].eq {
	pc = 0x8262E3EC; continue 'dispatch;
	}
	// 8262E39C: 3BBE0014  addi r29, r30, 0x14
	ctx.r[29].s64 = ctx.r[30].s64 + 20;
	// 8262E3A0: 3B9E0024  addi r28, r30, 0x24
	ctx.r[28].s64 = ctx.r[30].s64 + 36;
	// 8262E3A4: 3BE30014  addi r31, r3, 0x14
	ctx.r[31].s64 = ctx.r[3].s64 + 20;
	// 8262E3A8: 3B40FFEC  li r26, -0x14
	ctx.r[26].s64 = -20;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E3F8 size=140
    let mut pc: u32 = 0x8262E3F8;
    'dispatch: loop {
        match pc {
            0x8262E3F8 => {
    //   block [0x8262E3F8..0x8262E484)
	// 8262E3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E3FC: 4867B001  bl 0x82ca93fc
	ctx.lr = 0x8262E400;
	sub_82CA93D0(ctx, base);
	// 8262E400: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E404: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8262E408: 7D7D2050  subf r11, r29, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[29].s64;
	// 8262E40C: 7F1D2040  cmplw cr6, r29, r4
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8262E410: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262E414: 55493032  slwi r9, r10, 6
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8262E418: 7F292850  subf r25, r9, r5
	ctx.r[25].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 8262E41C: 419A005C  beq cr6, 0x8262e478
	if ctx.cr[6].eq {
	pc = 0x8262E478; continue 'dispatch;
	}
	// 8262E420: 3BC40024  addi r30, r4, 0x24
	ctx.r[30].s64 = ctx.r[4].s64 + 36;
	// 8262E424: 3BE50014  addi r31, r5, 0x14
	ctx.r[31].s64 = ctx.r[5].s64 + 20;
	// 8262E428: 7F852050  subf r28, r5, r4
	ctx.r[28].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 8262E42C: 3B40FFDC  li r26, -0x24
	ctx.r[26].s64 = -36;
	// 8262E430: 3B60FFEC  li r27, -0x14
	ctx.r[27].s64 = -20;
	// 8262E434: 3BDEFFC0  addi r30, r30, -0x40
	ctx.r[30].s64 = ctx.r[30].s64 + -64;
	// 8262E438: 3BFFFFC0  addi r31, r31, -0x40
	ctx.r[31].s64 = ctx.r[31].s64 + -64;
	// 8262E43C: 7C9CFA14  add r4, r28, r31
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 8262E440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E488 size=148
    let mut pc: u32 = 0x8262E488;
    'dispatch: loop {
        match pc {
            0x8262E488 => {
    //   block [0x8262E488..0x8262E51C)
	// 8262E488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E498: 3D00834C  lis r8, -0x7cb4
	ctx.r[8].s64 = -2092171264;
	// 8262E49C: 81481C68  lwz r10, 0x1c68(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(7272 as u32) ) } as u64;
	// 8262E4A0: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8262E4A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E4A8: 409A0058  bne cr6, 0x8262e500
	if !ctx.cr[6].eq {
	pc = 0x8262E500; continue 'dispatch;
	}
	// 8262E4AC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8262E4B0: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 8262E4B4: 3D20834C  lis r9, -0x7cb4
	ctx.r[9].s64 = -2092171264;
	// 8262E4B8: 91481C68  stw r10, 0x1c68(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(7272 as u32), ctx.r[10].u32 ) };
	// 8262E4BC: 3BE91C20  addi r31, r9, 0x1c20
	ctx.r[31].s64 = ctx.r[9].s64 + 7200;
	// 8262E4C0: 894B0BB0  lbz r10, 0xbb0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2992 as u32) ) } as u64;
	// 8262E4C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262E4C8: 419A000C  beq cr6, 0x8262e4d4
	if ctx.cr[6].eq {
	pc = 0x8262E4D4; continue 'dispatch;
	}
	// 8262E4CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E4D0: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E4D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E4D8: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8262E4DC: 994B0BB0  stb r10, 0xbb0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2992 as u32), ctx.r[10].u8 ) };
	// 8262E4E0: 3869BA20  addi r3, r9, -0x45e0
	ctx.r[3].s64 = ctx.r[9].s64 + -17888;
	// 8262E4E4: 4867BA3D  bl 0x82ca9f20
	ctx.lr = 0x8262E4E8;
	sub_82CA9F20(ctx, base);
	// 8262E4E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E4EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E4F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E4FC: 4E800020  blr
	return;
	// 8262E500: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 8262E504: 386B1C20  addi r3, r11, 0x1c20
	ctx.r[3].s64 = ctx.r[11].s64 + 7200;
	// 8262E508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E520 size=148
    let mut pc: u32 = 0x8262E520;
    'dispatch: loop {
        match pc {
            0x8262E520 => {
    //   block [0x8262E520..0x8262E5B4)
	// 8262E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E52C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E530: 3D00834C  lis r8, -0x7cb4
	ctx.r[8].s64 = -2092171264;
	// 8262E534: 81481C18  lwz r10, 0x1c18(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(7192 as u32) ) } as u64;
	// 8262E538: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8262E53C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E540: 409A0058  bne cr6, 0x8262e598
	if !ctx.cr[6].eq {
	pc = 0x8262E598; continue 'dispatch;
	}
	// 8262E544: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8262E548: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 8262E54C: 3D20834C  lis r9, -0x7cb4
	ctx.r[9].s64 = -2092171264;
	// 8262E550: 91481C18  stw r10, 0x1c18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(7192 as u32), ctx.r[10].u32 ) };
	// 8262E554: 3BE91BD0  addi r31, r9, 0x1bd0
	ctx.r[31].s64 = ctx.r[9].s64 + 7120;
	// 8262E558: 894B0BB1  lbz r10, 0xbb1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2993 as u32) ) } as u64;
	// 8262E55C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262E560: 419A000C  beq cr6, 0x8262e56c
	if ctx.cr[6].eq {
	pc = 0x8262E56C; continue 'dispatch;
	}
	// 8262E564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E568: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E56C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E570: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8262E574: 994B0BB1  stb r10, 0xbb1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2993 as u32), ctx.r[10].u8 ) };
	// 8262E578: 3869BA00  addi r3, r9, -0x4600
	ctx.r[3].s64 = ctx.r[9].s64 + -17920;
	// 8262E57C: 4867B9A5  bl 0x82ca9f20
	ctx.lr = 0x8262E580;
	sub_82CA9F20(ctx, base);
	// 8262E580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E594: 4E800020  blr
	return;
	// 8262E598: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 8262E59C: 386B1BD0  addi r3, r11, 0x1bd0
	ctx.r[3].s64 = ctx.r[11].s64 + 7120;
	// 8262E5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E5AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E5B8 size=148
    let mut pc: u32 = 0x8262E5B8;
    'dispatch: loop {
        match pc {
            0x8262E5B8 => {
    //   block [0x8262E5B8..0x8262E64C)
	// 8262E5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E5C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E5C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E5C8: 3D00834C  lis r8, -0x7cb4
	ctx.r[8].s64 = -2092171264;
	// 8262E5CC: 81481BC8  lwz r10, 0x1bc8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(7112 as u32) ) } as u64;
	// 8262E5D0: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8262E5D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E5D8: 409A0058  bne cr6, 0x8262e630
	if !ctx.cr[6].eq {
	pc = 0x8262E630; continue 'dispatch;
	}
	// 8262E5DC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8262E5E0: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 8262E5E4: 3D20834C  lis r9, -0x7cb4
	ctx.r[9].s64 = -2092171264;
	// 8262E5E8: 91481BC8  stw r10, 0x1bc8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(7112 as u32), ctx.r[10].u32 ) };
	// 8262E5EC: 3BE91B80  addi r31, r9, 0x1b80
	ctx.r[31].s64 = ctx.r[9].s64 + 7040;
	// 8262E5F0: 894B0BB2  lbz r10, 0xbb2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2994 as u32) ) } as u64;
	// 8262E5F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262E5F8: 419A000C  beq cr6, 0x8262e604
	if ctx.cr[6].eq {
	pc = 0x8262E604; continue 'dispatch;
	}
	// 8262E5FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E600: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E604: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E608: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8262E60C: 994B0BB2  stb r10, 0xbb2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2994 as u32), ctx.r[10].u8 ) };
	// 8262E610: 3869B9E0  addi r3, r9, -0x4620
	ctx.r[3].s64 = ctx.r[9].s64 + -17952;
	// 8262E614: 4867B90D  bl 0x82ca9f20
	ctx.lr = 0x8262E618;
	sub_82CA9F20(ctx, base);
	// 8262E618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E61C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E628: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E62C: 4E800020  blr
	return;
	// 8262E630: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 8262E634: 386B1B80  addi r3, r11, 0x1b80
	ctx.r[3].s64 = ctx.r[11].s64 + 7040;
	// 8262E638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E650 size=148
    let mut pc: u32 = 0x8262E650;
    'dispatch: loop {
        match pc {
            0x8262E650 => {
    //   block [0x8262E650..0x8262E6E4)
	// 8262E650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E65C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E660: 3D00834C  lis r8, -0x7cb4
	ctx.r[8].s64 = -2092171264;
	// 8262E664: 81481B78  lwz r10, 0x1b78(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(7032 as u32) ) } as u64;
	// 8262E668: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8262E66C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E670: 409A0058  bne cr6, 0x8262e6c8
	if !ctx.cr[6].eq {
	pc = 0x8262E6C8; continue 'dispatch;
	}
	// 8262E674: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8262E678: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 8262E67C: 3D20834C  lis r9, -0x7cb4
	ctx.r[9].s64 = -2092171264;
	// 8262E680: 91481B78  stw r10, 0x1b78(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(7032 as u32), ctx.r[10].u32 ) };
	// 8262E684: 3BE91B30  addi r31, r9, 0x1b30
	ctx.r[31].s64 = ctx.r[9].s64 + 6960;
	// 8262E688: 894B0BB3  lbz r10, 0xbb3(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2995 as u32) ) } as u64;
	// 8262E68C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262E690: 419A000C  beq cr6, 0x8262e69c
	if ctx.cr[6].eq {
	pc = 0x8262E69C; continue 'dispatch;
	}
	// 8262E694: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E698: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E69C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E6A0: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8262E6A4: 994B0BB3  stb r10, 0xbb3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2995 as u32), ctx.r[10].u8 ) };
	// 8262E6A8: 3869B9C0  addi r3, r9, -0x4640
	ctx.r[3].s64 = ctx.r[9].s64 + -17984;
	// 8262E6AC: 4867B875  bl 0x82ca9f20
	ctx.lr = 0x8262E6B0;
	sub_82CA9F20(ctx, base);
	// 8262E6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E6B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E6C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E6C4: 4E800020  blr
	return;
	// 8262E6C8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 8262E6CC: 386B1B30  addi r3, r11, 0x1b30
	ctx.r[3].s64 = ctx.r[11].s64 + 6960;
	// 8262E6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E6DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E6E8 size=148
    let mut pc: u32 = 0x8262E6E8;
    'dispatch: loop {
        match pc {
            0x8262E6E8 => {
    //   block [0x8262E6E8..0x8262E77C)
	// 8262E6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E6F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E6F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E6F8: 3D00834C  lis r8, -0x7cb4
	ctx.r[8].s64 = -2092171264;
	// 8262E6FC: 81481B2C  lwz r10, 0x1b2c(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(6956 as u32) ) } as u64;
	// 8262E700: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 8262E704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262E708: 409A0058  bne cr6, 0x8262e760
	if !ctx.cr[6].eq {
	pc = 0x8262E760; continue 'dispatch;
	}
	// 8262E70C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8262E710: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 8262E714: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 8262E718: 91481B2C  stw r10, 0x1b2c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(6956 as u32), ctx.r[10].u32 ) };
	// 8262E71C: 3BE91C70  addi r31, r9, 0x1c70
	ctx.r[31].s64 = ctx.r[9].s64 + 7280;
	// 8262E720: 894B0BB4  lbz r10, 0xbb4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2996 as u32) ) } as u64;
	// 8262E724: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262E728: 419A000C  beq cr6, 0x8262e734
	if ctx.cr[6].eq {
	pc = 0x8262E734; continue 'dispatch;
	}
	// 8262E72C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E730: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E738: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8262E73C: 994B0BB4  stb r10, 0xbb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(2996 as u32), ctx.r[10].u8 ) };
	// 8262E740: 3869B9A0  addi r3, r9, -0x4660
	ctx.r[3].s64 = ctx.r[9].s64 + -18016;
	// 8262E744: 4867B7DD  bl 0x82ca9f20
	ctx.lr = 0x8262E748;
	sub_82CA9F20(ctx, base);
	// 8262E748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262E74C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E758: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E75C: 4E800020  blr
	return;
	// 8262E760: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8262E764: 386B1C70  addi r3, r11, 0x1c70
	ctx.r[3].s64 = ctx.r[11].s64 + 7280;
	// 8262E768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262E76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E780 size=216
    let mut pc: u32 = 0x8262E780;
    'dispatch: loop {
        match pc {
            0x8262E780 => {
    //   block [0x8262E780..0x8262E858)
	// 8262E780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262E78C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E794: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262E798: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 8262E79C: 4BFFFCED  bl 0x8262e488
	ctx.lr = 0x8262E7A0;
	sub_8262E488(ctx, base);
	// 8262E7A0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8262E7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E7A8: 80A70040  lwz r5, 0x40(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E7AC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8262E7B0: 40990054  ble cr6, 0x8262e804
	if !ctx.cr[6].gt {
	pc = 0x8262E804; continue 'dispatch;
	}
	// 8262E7B4: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8262E7B8: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E7BC: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8262E7C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262E7C4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E7C8: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E7CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E7D0: 7D244850  subf r9, r4, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 8262E7D4: 419A0014  beq cr6, 0x8262e7e8
	if ctx.cr[6].eq {
	pc = 0x8262E7E8; continue 'dispatch;
	}
	// 8262E7D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262E7DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8262E7E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E7E4: 419AFFE0  beq cr6, 0x8262e7c4
	if ctx.cr[6].eq {
	pc = 0x8262E7C4; continue 'dispatch;
	}
	// 8262E7E8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E7EC: 419A0030  beq cr6, 0x8262e81c
	if ctx.cr[6].eq {
	pc = 0x8262E81C; continue 'dispatch;
	}
	// 8262E7F0: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E7F4: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8262E7F8: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8262E7FC: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8262E800: 4198FFB8  blt cr6, 0x8262e7b8
	if ctx.cr[6].lt {
	pc = 0x8262E7B8; continue 'dispatch;
	}
	// 8262E804: 2B050010  cmplwi cr6, r5, 0x10
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16 as u32, &mut ctx.xer);
	// 8262E808: 409A0020  bne cr6, 0x8262e828
	if !ctx.cr[6].eq {
	pc = 0x8262E828; continue 'dispatch;
	}
	// 8262E80C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262E810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262E814: 99670044  stb r11, 0x44(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 8262E818: 48000028  b 0x8262e840
	pc = 0x8262E840; continue 'dispatch;
	// 8262E81C: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262E820: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262E824: 48000018  b 0x8262e83c
	pc = 0x8262E83C; continue 'dispatch;
	// 8262E828: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262E82C: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262E830: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E834: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8262E838: 91470040  stw r10, 0x40(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E83C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8262E840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262E844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E84C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262E850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E858 size=216
    let mut pc: u32 = 0x8262E858;
    'dispatch: loop {
        match pc {
            0x8262E858 => {
    //   block [0x8262E858..0x8262E930)
	// 8262E858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E860: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262E864: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E868: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E86C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262E870: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 8262E874: 4BFFFCAD  bl 0x8262e520
	ctx.lr = 0x8262E878;
	sub_8262E520(ctx, base);
	// 8262E878: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8262E87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E880: 80A70040  lwz r5, 0x40(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E884: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8262E888: 40990054  ble cr6, 0x8262e8dc
	if !ctx.cr[6].gt {
	pc = 0x8262E8DC; continue 'dispatch;
	}
	// 8262E88C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8262E890: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E894: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8262E898: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262E89C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E8A0: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E8A4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E8A8: 7D244850  subf r9, r4, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 8262E8AC: 419A0014  beq cr6, 0x8262e8c0
	if ctx.cr[6].eq {
	pc = 0x8262E8C0; continue 'dispatch;
	}
	// 8262E8B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262E8B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8262E8B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E8BC: 419AFFE0  beq cr6, 0x8262e89c
	if ctx.cr[6].eq {
	pc = 0x8262E89C; continue 'dispatch;
	}
	// 8262E8C0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E8C4: 419A0030  beq cr6, 0x8262e8f4
	if ctx.cr[6].eq {
	pc = 0x8262E8F4; continue 'dispatch;
	}
	// 8262E8C8: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E8CC: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8262E8D0: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8262E8D4: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8262E8D8: 4198FFB8  blt cr6, 0x8262e890
	if ctx.cr[6].lt {
	pc = 0x8262E890; continue 'dispatch;
	}
	// 8262E8DC: 2B050010  cmplwi cr6, r5, 0x10
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16 as u32, &mut ctx.xer);
	// 8262E8E0: 409A0020  bne cr6, 0x8262e900
	if !ctx.cr[6].eq {
	pc = 0x8262E900; continue 'dispatch;
	}
	// 8262E8E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262E8E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262E8EC: 99670044  stb r11, 0x44(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 8262E8F0: 48000028  b 0x8262e918
	pc = 0x8262E918; continue 'dispatch;
	// 8262E8F4: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262E8F8: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262E8FC: 48000018  b 0x8262e914
	pc = 0x8262E914; continue 'dispatch;
	// 8262E900: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262E904: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262E908: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E90C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8262E910: 91470040  stw r10, 0x40(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E914: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8262E918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262E91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262E928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262E92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E930 size=216
    let mut pc: u32 = 0x8262E930;
    'dispatch: loop {
        match pc {
            0x8262E930 => {
    //   block [0x8262E930..0x8262EA08)
	// 8262E930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262E93C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262E940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E944: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262E948: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 8262E94C: 4BFFFC6D  bl 0x8262e5b8
	ctx.lr = 0x8262E950;
	sub_8262E5B8(ctx, base);
	// 8262E950: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8262E954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E958: 80A70040  lwz r5, 0x40(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E95C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8262E960: 40990054  ble cr6, 0x8262e9b4
	if !ctx.cr[6].gt {
	pc = 0x8262E9B4; continue 'dispatch;
	}
	// 8262E964: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8262E968: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E96C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8262E970: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262E974: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E978: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262E97C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E980: 7D244850  subf r9, r4, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 8262E984: 419A0014  beq cr6, 0x8262e998
	if ctx.cr[6].eq {
	pc = 0x8262E998; continue 'dispatch;
	}
	// 8262E988: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262E98C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8262E990: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E994: 419AFFE0  beq cr6, 0x8262e974
	if ctx.cr[6].eq {
	pc = 0x8262E974; continue 'dispatch;
	}
	// 8262E998: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262E99C: 419A0030  beq cr6, 0x8262e9cc
	if ctx.cr[6].eq {
	pc = 0x8262E9CC; continue 'dispatch;
	}
	// 8262E9A0: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E9A4: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8262E9A8: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8262E9AC: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8262E9B0: 4198FFB8  blt cr6, 0x8262e968
	if ctx.cr[6].lt {
	pc = 0x8262E968; continue 'dispatch;
	}
	// 8262E9B4: 2B050010  cmplwi cr6, r5, 0x10
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16 as u32, &mut ctx.xer);
	// 8262E9B8: 409A0020  bne cr6, 0x8262e9d8
	if !ctx.cr[6].eq {
	pc = 0x8262E9D8; continue 'dispatch;
	}
	// 8262E9BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262E9C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262E9C4: 99670044  stb r11, 0x44(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 8262E9C8: 48000028  b 0x8262e9f0
	pc = 0x8262E9F0; continue 'dispatch;
	// 8262E9CC: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262E9D0: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262E9D4: 48000018  b 0x8262e9ec
	pc = 0x8262E9EC; continue 'dispatch;
	// 8262E9D8: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262E9DC: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262E9E0: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262E9E4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8262E9E8: 91470040  stw r10, 0x40(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262E9EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8262E9F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262E9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E9FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262EA00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262EA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EA08 size=216
    let mut pc: u32 = 0x8262EA08;
    'dispatch: loop {
        match pc {
            0x8262EA08 => {
    //   block [0x8262EA08..0x8262EAE0)
	// 8262EA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EA10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262EA14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262EA18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EA1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262EA20: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 8262EA24: 4BFFFC2D  bl 0x8262e650
	ctx.lr = 0x8262EA28;
	sub_8262E650(ctx, base);
	// 8262EA28: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8262EA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EA30: 80A70040  lwz r5, 0x40(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262EA34: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8262EA38: 40990054  ble cr6, 0x8262ea8c
	if !ctx.cr[6].gt {
	pc = 0x8262EA8C; continue 'dispatch;
	}
	// 8262EA3C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8262EA40: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262EA44: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8262EA48: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262EA4C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262EA50: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262EA54: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262EA58: 7D244850  subf r9, r4, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 8262EA5C: 419A0014  beq cr6, 0x8262ea70
	if ctx.cr[6].eq {
	pc = 0x8262EA70; continue 'dispatch;
	}
	// 8262EA60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262EA64: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8262EA68: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262EA6C: 419AFFE0  beq cr6, 0x8262ea4c
	if ctx.cr[6].eq {
	pc = 0x8262EA4C; continue 'dispatch;
	}
	// 8262EA70: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262EA74: 419A0030  beq cr6, 0x8262eaa4
	if ctx.cr[6].eq {
	pc = 0x8262EAA4; continue 'dispatch;
	}
	// 8262EA78: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262EA7C: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8262EA80: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8262EA84: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8262EA88: 4198FFB8  blt cr6, 0x8262ea40
	if ctx.cr[6].lt {
	pc = 0x8262EA40; continue 'dispatch;
	}
	// 8262EA8C: 2B050010  cmplwi cr6, r5, 0x10
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16 as u32, &mut ctx.xer);
	// 8262EA90: 409A0020  bne cr6, 0x8262eab0
	if !ctx.cr[6].eq {
	pc = 0x8262EAB0; continue 'dispatch;
	}
	// 8262EA94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262EA98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262EA9C: 99670044  stb r11, 0x44(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 8262EAA0: 48000028  b 0x8262eac8
	pc = 0x8262EAC8; continue 'dispatch;
	// 8262EAA4: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262EAA8: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262EAAC: 48000018  b 0x8262eac4
	pc = 0x8262EAC4; continue 'dispatch;
	// 8262EAB0: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262EAB4: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262EAB8: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262EABC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8262EAC0: 91470040  stw r10, 0x40(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262EAC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8262EAC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262EACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EAD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262EAD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262EADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EAE0 size=216
    let mut pc: u32 = 0x8262EAE0;
    'dispatch: loop {
        match pc {
            0x8262EAE0 => {
    //   block [0x8262EAE0..0x8262EBB8)
	// 8262EAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EAE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262EAEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262EAF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EAF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262EAF8: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 8262EAFC: 4BFFFBED  bl 0x8262e6e8
	ctx.lr = 0x8262EB00;
	sub_8262E6E8(ctx, base);
	// 8262EB00: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8262EB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EB08: 80A70040  lwz r5, 0x40(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262EB0C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8262EB10: 40990054  ble cr6, 0x8262eb64
	if !ctx.cr[6].gt {
	pc = 0x8262EB64; continue 'dispatch;
	}
	// 8262EB14: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8262EB18: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262EB1C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8262EB20: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262EB24: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262EB28: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262EB2C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262EB30: 7D244850  subf r9, r4, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 8262EB34: 419A0014  beq cr6, 0x8262eb48
	if ctx.cr[6].eq {
	pc = 0x8262EB48; continue 'dispatch;
	}
	// 8262EB38: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8262EB3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8262EB40: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262EB44: 419AFFE0  beq cr6, 0x8262eb24
	if ctx.cr[6].eq {
	pc = 0x8262EB24; continue 'dispatch;
	}
	// 8262EB48: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8262EB4C: 419A0030  beq cr6, 0x8262eb7c
	if ctx.cr[6].eq {
	pc = 0x8262EB7C; continue 'dispatch;
	}
	// 8262EB50: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262EB54: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8262EB58: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8262EB5C: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8262EB60: 4198FFB8  blt cr6, 0x8262eb18
	if ctx.cr[6].lt {
	pc = 0x8262EB18; continue 'dispatch;
	}
	// 8262EB64: 2B050010  cmplwi cr6, r5, 0x10
	ctx.cr[6].compare_u32(ctx.r[5].u32, 16 as u32, &mut ctx.xer);
	// 8262EB68: 409A0020  bne cr6, 0x8262eb88
	if !ctx.cr[6].eq {
	pc = 0x8262EB88; continue 'dispatch;
	}
	// 8262EB6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262EB70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262EB74: 99670044  stb r11, 0x44(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 8262EB78: 48000028  b 0x8262eba0
	pc = 0x8262EBA0; continue 'dispatch;
	// 8262EB7C: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262EB80: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262EB84: 48000018  b 0x8262eb9c
	pc = 0x8262EB9C; continue 'dispatch;
	// 8262EB88: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262EB8C: 7FCB392E  stwx r30, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[30].u32) };
	// 8262EB90: 81670040  lwz r11, 0x40(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262EB94: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8262EB98: 91470040  stw r10, 0x40(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8262EB9C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8262EBA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262EBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EBAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262EBB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262EBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EBB8 size=232
    let mut pc: u32 = 0x8262EBB8;
    'dispatch: loop {
        match pc {
            0x8262EBB8 => {
    //   block [0x8262EBB8..0x8262ECA0)
	// 8262EBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EBBC: 4867A845  bl 0x82ca9400
	ctx.lr = 0x8262EBC0;
	sub_82CA93D0(ctx, base);
	// 8262EBC0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EBC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262EBC8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8262EBCC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8262EBD0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8262EBD4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8262EBD8: 83FE0028  lwz r31, 0x28(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8262EBDC: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8262EBE0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8262EBE4: 419A0030  beq cr6, 0x8262ec14
	if ctx.cr[6].eq {
	pc = 0x8262EC14; continue 'dispatch;
	}
	// 8262EBE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262EBEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262EBF0: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 8262EBF4: 4E800421  bctrl
	ctx.lr = 0x8262EBF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8262EBF8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262EBFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262EC00: 409A0094  bne cr6, 0x8262ec94
	if !ctx.cr[6].eq {
	pc = 0x8262EC94; continue 'dispatch;
	}
	// 8262EC04: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8262EC08: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262EC0C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8262EC10: 409AFFD8  bne cr6, 0x8262ebe8
	if !ctx.cr[6].eq {
	pc = 0x8262EBE8; continue 'dispatch;
	}
	// 8262EC14: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ECA0 size=572
    let mut pc: u32 = 0x8262ECA0;
    'dispatch: loop {
        match pc {
            0x8262ECA0 => {
    //   block [0x8262ECA0..0x8262EEDC)
	// 8262ECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ECA4: 4867A759  bl 0x82ca93fc
	ctx.lr = 0x8262ECA8;
	sub_82CA93D0(ctx, base);
	// 8262ECA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ECAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262ECB0: F88100B8  std r4, 0xb8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[4].u64 ) };
	// 8262ECB4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8262ECB8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8262ECBC: FB4100C0  std r26, 0xc0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[26].u64 ) };
	// 8262ECC0: FB2100C8  std r25, 0xc8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[25].u64 ) };
	// 8262ECC4: 814100CC  lwz r10, 0xcc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 8262ECC8: 838100C4  lwz r28, 0xc4(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 8262ECCC: 7D3C5050  subf r9, r28, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[28].s64;
	// 8262ECD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262ECD4: 7D3D3670  srawi r29, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 8262ECD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262ECDC: 409A000C  bne cr6, 0x8262ece8
	if !ctx.cr[6].eq {
	pc = 0x8262ECE8; continue 'dispatch;
	}
	// 8262ECE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262ECE4: 48000010  b 0x8262ecf4
	pc = 0x8262ECF4; continue 'dispatch;
	// 8262ECE8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8262ECEC: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262ECF0: 7D293670  srawi r9, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 8262ECF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8262ECF8: 419A01DC  beq cr6, 0x8262eed4
	if ctx.cr[6].eq {
	pc = 0x8262EED4; continue 'dispatch;
	}
	// 8262ECFC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262ED00: 3D4003FF  lis r10, 0x3ff
	ctx.r[10].s64 = 67043328;
	// 8262ED04: 7D0B2850  subf r8, r11, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 8262ED08: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8262ED0C: 7D0B3670  srawi r11, r8, 6
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 6) as i64;
	// 8262ED10: 7CEB5050  subf r7, r11, r10
	ctx.r[7].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262ED14: 7F07E840  cmplw cr6, r7, r29
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262ED18: 40980010  bge cr6, 0x8262ed28
	if !ctx.cr[6].lt {
	pc = 0x8262ED28; continue 'dispatch;
	}
	// 8262ED1C: 484187AD  bl 0x82a474c8
	ctx.lr = 0x8262ED20;
	sub_82A474C8(ctx, base);
	// 8262ED20: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8262ED24: 4867A728  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 8262ED28: 7D0BEA14  add r8, r11, r29
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8262ED2C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262ED30: 409800BC  bge cr6, 0x8262edec
	if !ctx.cr[6].lt {
	pc = 0x8262EDEC; continue 'dispatch;
	}
	// 8262ED34: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262ED38: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8262ED3C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262ED40: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8262ED44: 41980008  blt cr6, 0x8262ed4c
	if ctx.cr[6].lt {
	pc = 0x8262ED4C; continue 'dispatch;
	}
	// 8262ED48: 7F8B4A14  add r28, r11, r9
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8262ED4C: 7F1C4040  cmplw cr6, r28, r8
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262ED50: 40980008  bge cr6, 0x8262ed58
	if !ctx.cr[6].lt {
	pc = 0x8262ED58; continue 'dispatch;
	}
	// 8262ED54: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 8262ED58: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8262ED5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262ED60: 4839FED1  bl 0x829cec30
	ctx.lr = 0x8262ED64;
	sub_829CEC30(ctx, base);
	// 8262ED64: 836100BC  lwz r27, 0xbc(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 8262ED68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262ED6C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262ED70: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8262ED74: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8262ED78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262ED7C: 4BFFF5A5  bl 0x8262e320
	ctx.lr = 0x8262ED80;
	sub_8262E320(ctx, base);
	// 8262ED80: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8262ED84: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8262ED88: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8262ED8C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8262ED90: 480005C9  bl 0x8262f358
	ctx.lr = 0x8262ED94;
	sub_8262F358(ctx, base);
	// 8262ED94: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8262ED98: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8262ED9C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262EDA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262EDA4: 4BFFF57D  bl 0x8262e320
	ctx.lr = 0x8262EDA8;
	sub_8262E320(ctx, base);
	// 8262EDA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262EDAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262EDB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8262EDB4: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8262EDB8: 7D4B3670  srawi r11, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 8262EDBC: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8262EDC0: 419A0008  beq cr6, 0x8262edc8
	if ctx.cr[6].eq {
	pc = 0x8262EDC8; continue 'dispatch;
	}
	// 8262EDC4: 4BBECF75  bl 0x8221bd38
	ctx.lr = 0x8262EDC8;
	sub_8221BD38(ctx, base);
	// 8262EDC8: 578B3032  slwi r11, r28, 6
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262EDCC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8262EDD0: 57AA3032  slwi r10, r29, 6
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262EDD4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8262EDD8: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8262EDDC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8262EDE0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8262EDE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8262EDE8: 4867A664  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 8262EDEC: 83C100BC  lwz r30, 0xbc(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 8262EDF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262EDF4: 7D7E2850  subf r11, r30, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[30].s64;
	// 8262EDF8: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262EDFC: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262EE00: 40980068  bge cr6, 0x8262ee68
	if !ctx.cr[6].lt {
	pc = 0x8262EE68; continue 'dispatch;
	}
	// 8262EE04: 57BD3032  slwi r29, r29, 6
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8262EE08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8262EE0C: 7CDDF214  add r6, r29, r30
	ctx.r[6].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 8262EE10: 4BFFF511  bl 0x8262e320
	ctx.lr = 0x8262EE14;
	sub_8262E320(ctx, base);
	// 8262EE14: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262EE18: FB410050  std r26, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u64 ) };
	// 8262EE1C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8262EE20: 7D7E2850  subf r11, r30, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[30].s64;
	// 8262EE24: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8262EE28: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262EE2C: 554B3032  slwi r11, r10, 6
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262EE30: 7D2BE214  add r9, r11, r28
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8262EE34: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8262EE38: EB810050  ld r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8262EE3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8262EE40: 48000519  bl 0x8262f358
	ctx.lr = 0x8262EE44;
	sub_8262F358(ctx, base);
	// 8262EE44: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262EE48: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8262EE4C: 7D1D5A14  add r8, r29, r11
	ctx.r[8].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 8262EE50: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8262EE54: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8262EE58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8262EE5C: 48000405  bl 0x8262f260
	ctx.lr = 0x8262EE60;
	sub_8262F260(ctx, base);
	// 8262EE60: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8262EE64: 4867A5E8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 8262EE68: 57BC3032  slwi r28, r29, 6
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8262EE6C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8262EE70: 7FBC2850  subf r29, r28, r5
	ctx.r[29].s64 = ctx.r[5].s64 - ctx.r[28].s64;
	// 8262EE74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262EE78: 4BFFF4A9  bl 0x8262e320
	ctx.lr = 0x8262EE7C;
	sub_8262E320(ctx, base);
	// 8262EE7C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8262EE80: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8262EE84: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262EE88: 419A003C  beq cr6, 0x8262eec4
	if ctx.cr[6].eq {
	pc = 0x8262EEC4; continue 'dispatch;
	}
	// 8262EE8C: 7D1C5A14  add r8, r28, r11
	ctx.r[8].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 8262EE90: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 8262EE94: 3908FFC0  addi r8, r8, -0x40
	ctx.r[8].s64 = ctx.r[8].s64 + -64;
	// 8262EE98: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8262EE9C: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8262EEA0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8262EEA4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8262EEA8: E8EA0000  ld r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8262EEAC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8262EEB0: F8E90000  std r7, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8262EEB4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8262EEB8: 4200FFF0  bdnz 0x8262eea8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262EEA8; continue 'dispatch;
	}
	// 8262EEBC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262EEC0: 409AFFD0  bne cr6, 0x8262ee90
	if !ctx.cr[6].eq {
	pc = 0x8262EE90; continue 'dispatch;
	}
	// 8262EEC4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8262EEC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8262EECC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8262EED0: 48000391  bl 0x8262f260
	ctx.lr = 0x8262EED4;
	sub_8262F260(ctx, base);
	// 8262EED4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8262EED8: 4867A574  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EEE0 size=748
    let mut pc: u32 = 0x8262EEE0;
    'dispatch: loop {
        match pc {
            0x8262EEE0 => {
    //   block [0x8262EEE0..0x8262F1CC)
	// 8262EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EEE4: 4867A509  bl 0x82ca93ec
	ctx.lr = 0x8262EEE8;
	sub_82CA93D0(ctx, base);
	// 8262EEE8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EEEC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8262EEF0: F88100D8  std r4, 0xd8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[4].u64 ) };
	// 8262EEF4: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 8262EEF8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8262EEFC: FAA100E0  std r21, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[21].u64 ) };
	// 8262EF00: FB2100E8  std r25, 0xe8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[25].u64 ) };
	// 8262EF04: 82E100E4  lwz r23, 0xe4(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 8262EF08: 82C100EC  lwz r22, 0xec(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(236 as u32) ) } as u64;
	// 8262EF0C: 7D57B050  subf r10, r23, r22
	ctx.r[10].s64 = ctx.r[22].s64 - ctx.r[23].s64;
	// 8262EF10: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262EF14: 7D583670  srawi r24, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 8262EF18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262EF1C: 409A000C  bne cr6, 0x8262ef28
	if !ctx.cr[6].eq {
	pc = 0x8262EF28; continue 'dispatch;
	}
	// 8262EF20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EF24: 48000010  b 0x8262ef34
	pc = 0x8262EF34; continue 'dispatch;
	// 8262EF28: 815A000C  lwz r10, 0xc(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 8262EF2C: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262EF30: 7D293670  srawi r9, r9, 6
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 6) as i64;
	// 8262EF34: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8262EF38: 419A028C  beq cr6, 0x8262f1c4
	if ctx.cr[6].eq {
	pc = 0x8262F1C4; continue 'dispatch;
	}
	// 8262EF3C: 83FA0008  lwz r31, 8(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262EF40: 3D4003FF  lis r10, 0x3ff
	ctx.r[10].s64 = 67043328;
	// 8262EF44: 7D0BF850  subf r8, r11, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8262EF48: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8262EF4C: 7D0B3670  srawi r11, r8, 6
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 6) as i64;
	// 8262EF50: 7CEB5050  subf r7, r11, r10
	ctx.r[7].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262EF54: 7F07C040  cmplw cr6, r7, r24
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8262EF58: 40980010  bge cr6, 0x8262ef68
	if !ctx.cr[6].lt {
	pc = 0x8262EF68; continue 'dispatch;
	}
	// 8262EF5C: 4841856D  bl 0x82a474c8
	ctx.lr = 0x8262EF60;
	sub_82A474C8(ctx, base);
	// 8262EF60: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8262EF64: 4867A4D8  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 8262EF68: 7D0BC214  add r8, r11, r24
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 8262EF6C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262EF70: 4098012C  bge cr6, 0x8262f09c
	if !ctx.cr[6].lt {
	pc = 0x8262F09C; continue 'dispatch;
	}
	// 8262EF74: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262EF78: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8262EF7C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8262EF80: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8262EF84: 41980008  blt cr6, 0x8262ef8c
	if ctx.cr[6].lt {
	pc = 0x8262EF8C; continue 'dispatch;
	}
	// 8262EF88: 7F2B4A14  add r25, r11, r9
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8262EF8C: 7F194040  cmplw cr6, r25, r8
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262EF90: 40980008  bge cr6, 0x8262ef98
	if !ctx.cr[6].lt {
	pc = 0x8262EF98; continue 'dispatch;
	}
	// 8262EF94: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8262EF98: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8262EF9C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8262EFA0: 4839FC91  bl 0x829cec30
	ctx.lr = 0x8262EFA4;
	sub_829CEC30(ctx, base);
	// 8262EFA4: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262EFA8: 838100DC  lwz r28, 0xdc(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) } as u64;
	// 8262EFAC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8262EFB0: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262EFB4: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8262EFB8: 419A0028  beq cr6, 0x8262efe0
	if ctx.cr[6].eq {
	pc = 0x8262EFE0; continue 'dispatch;
	}
	// 8262EFBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8262EFC0: 419A0010  beq cr6, 0x8262efd0
	if ctx.cr[6].eq {
	pc = 0x8262EFD0; continue 'dispatch;
	}
	// 8262EFC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8262EFC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262EFCC: 4BFFEF3D  bl 0x8262df08
	ctx.lr = 0x8262EFD0;
	sub_8262DF08(ctx, base);
	// 8262EFD0: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262EFD4: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262EFD8: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8262EFDC: 409AFFE0  bne cr6, 0x8262efbc
	if !ctx.cr[6].eq {
	pc = 0x8262EFBC; continue 'dispatch;
	}
	// 8262EFE0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8262EFE4: FAA10050  std r21, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[21].u64 ) };
	// 8262EFE8: 7F17B040  cmplw cr6, r23, r22
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[22].u32, &mut ctx.xer);
	// 8262EFEC: 419A002C  beq cr6, 0x8262f018
	if ctx.cr[6].eq {
	pc = 0x8262F018; continue 'dispatch;
	}
	// 8262EFF0: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8262EFF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262EFF8: 419A0010  beq cr6, 0x8262f008
	if ctx.cr[6].eq {
	pc = 0x8262F008; continue 'dispatch;
	}
	// 8262EFFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8262F000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F004: 4BFFEF05  bl 0x8262df08
	ctx.lr = 0x8262F008;
	sub_8262DF08(ctx, base);
	// 8262F008: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262F00C: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262F010: 7F1EB040  cmplw cr6, r30, r22
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[22].u32, &mut ctx.xer);
	// 8262F014: 409AFFE0  bne cr6, 0x8262eff4
	if !ctx.cr[6].eq {
	pc = 0x8262EFF4; continue 'dispatch;
	}
	// 8262F018: 83BA0008  lwz r29, 8(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262F01C: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262F020: 419A002C  beq cr6, 0x8262f04c
	if ctx.cr[6].eq {
	pc = 0x8262F04C; continue 'dispatch;
	}
	// 8262F024: 7FDFE050  subf r30, r31, r28
	ctx.r[30].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 8262F028: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262F02C: 419A0010  beq cr6, 0x8262f03c
	if ctx.cr[6].eq {
	pc = 0x8262F03C; continue 'dispatch;
	}
	// 8262F030: 7C9EFA14  add r4, r30, r31
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8262F034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F038: 4BFFEED1  bl 0x8262df08
	ctx.lr = 0x8262F03C;
	sub_8262DF08(ctx, base);
	// 8262F03C: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262F040: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8262F044: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262F048: 409AFFE0  bne cr6, 0x8262f028
	if !ctx.cr[6].eq {
	pc = 0x8262F028; continue 'dispatch;
	}
	// 8262F04C: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262F050: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262F054: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8262F058: 7D642850  subf r11, r4, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 8262F05C: 7D6B3670  srawi r11, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262F060: 7FEBC214  add r31, r11, r24
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 8262F064: 419A0014  beq cr6, 0x8262f078
	if ctx.cr[6].eq {
	pc = 0x8262F078; continue 'dispatch;
	}
	// 8262F068: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8262F06C: 4BFFE035  bl 0x8262d0a0
	ctx.lr = 0x8262F070;
	sub_8262D0A0(ctx, base);
	// 8262F070: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262F074: 4BBECCC5  bl 0x8221bd38
	ctx.lr = 0x8262F078;
	sub_8221BD38(ctx, base);
	// 8262F078: 572A3032  slwi r10, r25, 6
	ctx.r[10].u32 = ctx.r[25].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8262F07C: 937A0004  stw r27, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8262F080: 57EB3032  slwi r11, r31, 6
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262F084: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 8262F088: 7D2BDA14  add r9, r11, r27
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8262F08C: 915A000C  stw r10, 0xc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8262F090: 913A0008  stw r9, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8262F094: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8262F098: 4867A3A4  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 8262F09C: 836100DC  lwz r27, 0xdc(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) } as u64;
	// 8262F0A0: 7D7BF850  subf r11, r27, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[27].s64;
	// 8262F0A4: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262F0A8: 7F0AC040  cmplw cr6, r10, r24
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8262F0AC: 409800B8  bge cr6, 0x8262f164
	if !ctx.cr[6].lt {
	pc = 0x8262F164; continue 'dispatch;
	}
	// 8262F0B0: 571C3032  slwi r28, r24, 6
	ctx.r[28].u32 = ctx.r[24].u32.wrapping_shl(6);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8262F0B4: 7F1BF840  cmplw cr6, r27, r31
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8262F0B8: 7FDCDA14  add r30, r28, r27
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 8262F0BC: 419A002C  beq cr6, 0x8262f0e8
	if ctx.cr[6].eq {
	pc = 0x8262F0E8; continue 'dispatch;
	}
	// 8262F0C0: 7FBCF050  subf r29, r28, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 8262F0C4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8262F0C8: 419A0010  beq cr6, 0x8262f0d8
	if ctx.cr[6].eq {
	pc = 0x8262F0D8; continue 'dispatch;
	}
	// 8262F0CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262F0D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262F0D4: 4BFFEE35  bl 0x8262df08
	ctx.lr = 0x8262F0D8;
	sub_8262DF08(ctx, base);
	// 8262F0D8: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 8262F0DC: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262F0E0: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8262F0E4: 409AFFE0  bne cr6, 0x8262f0c4
	if !ctx.cr[6].eq {
	pc = 0x8262F0C4; continue 'dispatch;
	}
	// 8262F0E8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262F0EC: FAA10050  std r21, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[21].u64 ) };
	// 8262F0F0: 7D5B5850  subf r10, r27, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 8262F0F4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8262F0F8: 7D493670  srawi r9, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 8262F0FC: 552B3032  slwi r11, r9, 6
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262F100: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 8262F104: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262F108: 7F0BB040  cmplw cr6, r11, r22
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[22].u32, &mut ctx.xer);
	// 8262F10C: EBA10050  ld r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8262F110: FBA10050  std r29, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u64 ) };
	// 8262F114: 419A002C  beq cr6, 0x8262f140
	if ctx.cr[6].eq {
	pc = 0x8262F140; continue 'dispatch;
	}
	// 8262F118: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8262F11C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262F120: 419A0010  beq cr6, 0x8262f130
	if ctx.cr[6].eq {
	pc = 0x8262F130; continue 'dispatch;
	}
	// 8262F124: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8262F128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F12C: 4BFFEDDD  bl 0x8262df08
	ctx.lr = 0x8262F130;
	sub_8262DF08(ctx, base);
	// 8262F130: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262F134: 3BFF0040  addi r31, r31, 0x40
	ctx.r[31].s64 = ctx.r[31].s64 + 64;
	// 8262F138: 7F1EB040  cmplw cr6, r30, r22
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[22].u32, &mut ctx.xer);
	// 8262F13C: 409AFFE0  bne cr6, 0x8262f11c
	if !ctx.cr[6].eq {
	pc = 0x8262F11C; continue 'dispatch;
	}
	// 8262F140: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8262F144: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262F148: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8262F14C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8262F150: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8262F154: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262F158: 48000171  bl 0x8262f2c8
	ctx.lr = 0x8262F15C;
	sub_8262F2C8(ctx, base);
	// 8262F15C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8262F160: 4867A2DC  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 8262F164: 570B3032  slwi r11, r24, 6
	ctx.r[11].u32 = ctx.r[24].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262F168: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8262F16C: 7F8BF850  subf r28, r11, r31
	ctx.r[28].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8262F170: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8262F174: 7F1CF840  cmplw cr6, r28, r31
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8262F178: 419A0028  beq cr6, 0x8262f1a0
	if ctx.cr[6].eq {
	pc = 0x8262F1A0; continue 'dispatch;
	}
	// 8262F17C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8262F180: 419A0010  beq cr6, 0x8262f190
	if ctx.cr[6].eq {
	pc = 0x8262F190; continue 'dispatch;
	}
	// 8262F184: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262F188: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8262F18C: 4BFFED7D  bl 0x8262df08
	ctx.lr = 0x8262F190;
	sub_8262DF08(ctx, base);
	// 8262F190: 3BBD0040  addi r29, r29, 0x40
	ctx.r[29].s64 = ctx.r[29].s64 + 64;
	// 8262F194: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 8262F198: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8262F19C: 409AFFE0  bne cr6, 0x8262f17c
	if !ctx.cr[6].eq {
	pc = 0x8262F17C; continue 'dispatch;
	}
	// 8262F1A0: 93DA0008  stw r30, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8262F1A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8262F1A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8262F1AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8262F1B0: 4BFFF249  bl 0x8262e3f8
	ctx.lr = 0x8262F1B4;
	sub_8262E3F8(ctx, base);
	// 8262F1B4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8262F1B8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8262F1BC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8262F1C0: 48000109  bl 0x8262f2c8
	ctx.lr = 0x8262F1C4;
	sub_8262F2C8(ctx, base);
	// 8262F1C4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8262F1C8: 4867A274  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F1D0 size=140
    let mut pc: u32 = 0x8262F1D0;
    'dispatch: loop {
        match pc {
            0x8262F1D0 => {
    //   block [0x8262F1D0..0x8262F25C)
	// 8262F1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F1D4: 4867A229  bl 0x82ca93fc
	ctx.lr = 0x8262F1D8;
	sub_82CA93D0(ctx, base);
	// 8262F1D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F1DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8262F1E0: 7D63E850  subf r11, r3, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 8262F1E4: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262F1E8: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8262F1EC: 554B3032  slwi r11, r10, 6
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262F1F0: 7F2B2A14  add r25, r11, r5
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8262F1F4: 419A005C  beq cr6, 0x8262f250
	if ctx.cr[6].eq {
	pc = 0x8262F250; continue 'dispatch;
	}
	// 8262F1F8: 3BC30024  addi r30, r3, 0x24
	ctx.r[30].s64 = ctx.r[3].s64 + 36;
	// 8262F1FC: 3BE50014  addi r31, r5, 0x14
	ctx.r[31].s64 = ctx.r[5].s64 + 20;
	// 8262F200: 7F851850  subf r28, r5, r3
	ctx.r[28].s64 = ctx.r[3].s64 - ctx.r[5].s64;
	// 8262F204: 3B40FFDC  li r26, -0x24
	ctx.r[26].s64 = -36;
	// 8262F208: 3B60FFEC  li r27, -0x14
	ctx.r[27].s64 = -20;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F260 size=44
    let mut pc: u32 = 0x8262F260;
    'dispatch: loop {
        match pc {
            0x8262F260 => {
    //   block [0x8262F260..0x8262F28C)
	// 8262F260: F8610010  std r3, 0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.r[3].u64 ) };
	// 8262F264: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 8262F268: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 8262F26C: 80E1001C  lwz r7, 0x1c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262F270: 7D4B3850  subf r10, r11, r7
	ctx.r[10].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 8262F274: 7D493670  srawi r9, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 8262F278: F861FFF0  std r3, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[3].u64 ) };
	// 8262F27C: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8262F280: 552B3032  slwi r11, r9, 6
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262F284: 7C6B2A14  add r3, r11, r5
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8262F288: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F28C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F28C size=60
    let mut pc: u32 = 0x8262F28C;
    'dispatch: loop {
        match pc {
            0x8262F28C => {
    //   block [0x8262F28C..0x8262F2C8)
	// 8262F28C: 8101FFF4  lwz r8, -0xc(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8262F290: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8262F294: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8262F298: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8262F29C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8262F2A0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8262F2A4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8262F2A8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8262F2AC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8262F2B0: 4200FFF0  bdnz 0x8262f2a0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262F2A0; continue 'dispatch;
	}
	// 8262F2B4: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 8262F2B8: 38A50040  addi r5, r5, 0x40
	ctx.r[5].s64 = ctx.r[5].s64 + 64;
	// 8262F2BC: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8262F2C0: 409AFFD0  bne cr6, 0x8262f290
	if !ctx.cr[6].eq {
	pc = 0x8262F290; continue 'dispatch;
	}
	// 8262F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F2C8 size=144
    let mut pc: u32 = 0x8262F2C8;
    'dispatch: loop {
        match pc {
            0x8262F2C8 => {
    //   block [0x8262F2C8..0x8262F358)
	// 8262F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F2CC: 4867A139  bl 0x82ca9404
	ctx.lr = 0x8262F2D0;
	sub_82CA93D0(ctx, base);
	// 8262F2D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F2D4: F86100A0  std r3, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[3].u64 ) };
	// 8262F2D8: F88100A8  std r4, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[4].u64 ) };
	// 8262F2DC: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 8262F2E0: 83A100AC  lwz r29, 0xac(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 8262F2E4: 7D4BE850  subf r10, r11, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8262F2E8: 7D493670  srawi r9, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 8262F2EC: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8262F2F0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8262F2F4: 552B3032  slwi r11, r9, 6
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262F2F8: 7F6B2A14  add r27, r11, r5
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8262F2FC: 419A0050  beq cr6, 0x8262f34c
	if ctx.cr[6].eq {
	pc = 0x8262F34C; continue 'dispatch;
	}
	// 8262F300: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8262F304: 3BC50014  addi r30, r5, 0x14
	ctx.r[30].s64 = ctx.r[5].s64 + 20;
	// 8262F308: 3B80FFEC  li r28, -0x14
	ctx.r[28].s64 = -20;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F358 size=36
    let mut pc: u32 = 0x8262F358;
    'dispatch: loop {
        match pc {
            0x8262F358 => {
    //   block [0x8262F358..0x8262F37C)
	// 8262F358: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8262F35C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8262F360: F9610010  std r11, 0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 8262F364: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 8262F368: 81410014  lwz r10, 0x14(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 8262F36C: 8101001C  lwz r8, 0x1c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) } as u64;
	// 8262F370: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8262F374: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8262F378: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F37C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F37C size=72
    let mut pc: u32 = 0x8262F37C;
    'dispatch: loop {
        match pc {
            0x8262F37C => {
    //   block [0x8262F37C..0x8262F3C4)
	// 8262F37C: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8262F380: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8262F384: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8262F388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8262F38C: 419A0024  beq cr6, 0x8262f3b0
	if ctx.cr[6].eq {
	pc = 0x8262F3B0; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8262F3C8 size=328
    let mut pc: u32 = 0x8262F3C8;
    'dispatch: loop {
        match pc {
            0x8262F3C8 => {
    //   block [0x8262F3C8..0x8262F510)
	// 8262F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F3D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262F3D4: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F3D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8262F3DC: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F3E0: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8262F3E4: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8262F3E8: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 8262F3EC: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8262F3F0: D0010114  stfs f0, 0x114(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 8262F3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F3F8: D1A10118  stfs f13, 0x118(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 8262F3FC: 39410120  addi r10, r1, 0x120
	ctx.r[10].s64 = ctx.r[1].s64 + 288;
	// 8262F400: D181011C  stfs f12, 0x11c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 8262F404: 91610110  stw r11, 0x110(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8262F408: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8262F40C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8262F410: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8262F414: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8262F418: 4200FFF8  bdnz 0x8262f410
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262F410; continue 'dispatch;
	}
	// 8262F41C: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F420: 98C10168  stb r6, 0x168(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(360 as u32), ctx.r[6].u8 ) };
	// 8262F424: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8262F428: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8262F42C: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8262F430: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8262F434: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8262F438: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8262F43C: D1A100B8  stfs f13, 0xb8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8262F440: D18100BC  stfs f12, 0xbc(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8262F444: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 8262F448: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8262F44C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8262F450: 4200FFF8  bdnz 0x8262f448
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262F448; continue 'dispatch;
	}
	// 8262F454: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262F458: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8262F45C: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8262F460: 98C10108  stb r6, 0x108(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[6].u8 ) };
	// 8262F464: D181005C  stfs f12, 0x5c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8262F468: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8262F46C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8262F470: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8262F474: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8262F478: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8262F47C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262F480: 4200FFF8  bdnz 0x8262f478
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262F478; continue 'dispatch;
	}
	// 8262F484: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8262F488: 98C100A8  stb r6, 0xa8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[6].u8 ) };
	// 8262F48C: 396BDAC0  addi r11, r11, -0x2540
	ctx.r[11].s64 = ctx.r[11].s64 + -9536;
	// 8262F490: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8262F494: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262F498: 409A000C  bne cr6, 0x8262f4a4
	if !ctx.cr[6].eq {
	pc = 0x8262F4A4; continue 'dispatch;
	}
	// 8262F49C: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8262F4A0: 4800001C  b 0x8262f4bc
	pc = 0x8262F4BC; continue 'dispatch;
	// 8262F4A4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8262F4A8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8262F4AC: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 8262F4B0: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 8262F4B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8262F4B8: 4E800421  bctrl
	ctx.lr = 0x8262F4BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8262F4BC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262F4C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F4C4: 419A0034  beq cr6, 0x8262f4f8
	if ctx.cr[6].eq {
	pc = 0x8262F4F8; continue 'dispatch;
	}
	// 8262F4C8: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F4CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8262F4D0: C1A10058  lfs f13, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8262F4D4: C181005C  lfs f12, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8262F4D8: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8262F4DC: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8262F4E0: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8262F4E4: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8262F4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F4F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F4F4: 4E800020  blr
	return;
	// 8262F4F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262F4FC: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8262F500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8262F510 size=216
    let mut pc: u32 = 0x8262F510;
    'dispatch: loop {
        match pc {
            0x8262F510 => {
    //   block [0x8262F510..0x8262F5E8)
	// 8262F510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F518: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F51C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8262F520: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F524: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8262F528: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F52C: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 8262F530: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8262F534: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8262F538: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8262F53C: D1A100B8  stfs f13, 0xb8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8262F540: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8262F544: D18100BC  stfs f12, 0xbc(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8262F548: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8262F54C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8262F550: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8262F554: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8262F558: 4200FFF8  bdnz 0x8262f550
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262F550; continue 'dispatch;
	}
	// 8262F55C: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F560: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262F564: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8262F568: 99010108  stb r8, 0x108(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[8].u8 ) };
	// 8262F56C: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8262F570: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8262F574: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8262F578: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8262F57C: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8262F580: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8262F584: D181005C  stfs f12, 0x5c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8262F588: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8262F58C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8262F590: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262F594: 4200FFF8  bdnz 0x8262f58c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8262F58C; continue 'dispatch;
	}
	// 8262F598: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8262F59C: 990100A8  stb r8, 0xa8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[8].u8 ) };
	// 8262F5A0: 396BDAC0  addi r11, r11, -0x2540
	ctx.r[11].s64 = ctx.r[11].s64 + -9536;
	// 8262F5A4: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8262F5A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262F5AC: 409A0018  bne cr6, 0x8262f5c4
	if !ctx.cr[6].eq {
	pc = 0x8262F5C4; continue 'dispatch;
	}
	// 8262F5B0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8262F5B4: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8262F5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F5C0: 4E800020  blr
	return;
	// 8262F5C4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8262F5C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8262F5CC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8262F5D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8262F5D4: 4E800421  bctrl
	ctx.lr = 0x8262F5D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8262F5D8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8262F5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F5E8 size=12
    let mut pc: u32 = 0x8262F5E8;
    'dispatch: loop {
        match pc {
            0x8262F5E8 => {
    //   block [0x8262F5E8..0x8262F5F4)
	// 8262F5E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8262F5EC: 386BA508  addi r3, r11, -0x5af8
	ctx.r[3].s64 = ctx.r[11].s64 + -23288;
	// 8262F5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F5F8 size=100
    let mut pc: u32 = 0x8262F5F8;
    'dispatch: loop {
        match pc {
            0x8262F5F8 => {
    //   block [0x8262F5F8..0x8262F65C)
	// 8262F5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262F604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262F608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F60C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8262F610: 386001D0  li r3, 0x1d0
	ctx.r[3].s64 = 464;
	// 8262F614: 4BFE7315  bl 0x82616928
	ctx.lr = 0x8262F618;
	sub_82616928(ctx, base);
	// 8262F618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262F61C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8262F620: 419A0020  beq cr6, 0x8262f640
	if ctx.cr[6].eq {
	pc = 0x8262F640; continue 'dispatch;
	}
	// 8262F624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8262F628: 488809F1  bl 0x82eb0018
	ctx.lr = 0x8262F62C;
	sub_82EB0018(ctx, base);
	// 8262F62C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8262F630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F634: 394B91D8  addi r10, r11, -0x6e28
	ctx.r[10].s64 = ctx.r[11].s64 + -28200;
	// 8262F638: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8262F63C: 48000008  b 0x8262f644
	pc = 0x8262F644; continue 'dispatch;
	// 8262F640: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262F644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262F648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F650: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262F654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F660 size=92
    let mut pc: u32 = 0x8262F660;
    'dispatch: loop {
        match pc {
            0x8262F660 => {
    //   block [0x8262F660..0x8262F6BC)
	// 8262F660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262F66C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262F670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262F678: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8262F67C: 48881F55  bl 0x82eb15d0
	ctx.lr = 0x8262F680;
	sub_82EB15D0(ctx, base);
	// 8262F680: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8262F684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F688: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F68C: 419A0018  beq cr6, 0x8262f6a4
	if ctx.cr[6].eq {
	pc = 0x8262F6A4; continue 'dispatch;
	}
	// 8262F690: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8262F694: 814BDAB4  lwz r10, -0x254c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9548 as u32) ) } as u64;
	// 8262F698: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8262F69C: 4E800421  bctrl
	ctx.lr = 0x8262F6A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8262F6A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F6A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262F6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F6B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262F6B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F6C0 size=132
    let mut pc: u32 = 0x8262F6C0;
    'dispatch: loop {
        match pc {
            0x8262F6C0 => {
    //   block [0x8262F6C0..0x8262F744)
	// 8262F6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F6C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262F6CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262F6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F6D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262F6D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8262F6DC: 48881A7D  bl 0x82eb1158
	ctx.lr = 0x8262F6E0;
	sub_82EB1158(ctx, base);
	// 8262F6E0: 897E00E0  lbz r11, 0xe0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(224 as u32) ) } as u64;
	// 8262F6E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F6E8: 419A0044  beq cr6, 0x8262f72c
	if ctx.cr[6].eq {
	pc = 0x8262F72C; continue 'dispatch;
	}
	// 8262F6EC: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8262F6F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F6F4: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8262F6F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F6FC: 40990030  ble cr6, 0x8262f72c
	if !ctx.cr[6].gt {
	pc = 0x8262F72C; continue 'dispatch;
	}
	// 8262F700: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8262F704: 815F008C  lwz r10, 0x8c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8262F708: 7CEA582E  lwzx r7, r10, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8262F70C: 7F07F040  cmplw cr6, r7, r30
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8262F710: 409A0008  bne cr6, 0x8262f718
	if !ctx.cr[6].eq {
	pc = 0x8262F718; continue 'dispatch;
	}
	// 8262F714: 7D0A592E  stwx r8, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 8262F718: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8262F71C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8262F720: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8262F724: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8262F728: 4198FFDC  blt cr6, 0x8262f704
	if ctx.cr[6].lt {
	pc = 0x8262F704; continue 'dispatch;
	}
	// 8262F72C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8262F730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F738: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8262F73C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F748 size=92
    let mut pc: u32 = 0x8262F748;
    'dispatch: loop {
        match pc {
            0x8262F748 => {
    //   block [0x8262F748..0x8262F7A4)
	// 8262F748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262F754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262F75C: 48882AED  bl 0x82eb2248
	ctx.lr = 0x8262F760;
	sub_82EB2248(ctx, base);
	// 8262F760: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8262F764: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8262F768: 409A001C  bne cr6, 0x8262f784
	if !ctx.cr[6].eq {
	pc = 0x8262F784; continue 'dispatch;
	}
	// 8262F76C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8262F770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262F774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F77C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F780: 4E800020  blr
	return;
	// 8262F784: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8262F788: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8262F78C: 917F013C  stw r11, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 8262F790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262F794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F79C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F7A8 size=132
    let mut pc: u32 = 0x8262F7A8;
    'dispatch: loop {
        match pc {
            0x8262F7A8 => {
    //   block [0x8262F7A8..0x8262F82C)
	// 8262F7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F7AC: 48679C61  bl 0x82ca940c
	ctx.lr = 0x8262F7B0;
	sub_82CA93D0(ctx, base);
	// 8262F7B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F7B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262F7B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8262F7BC: 48860C1D  bl 0x82e903d8
	ctx.lr = 0x8262F7C0;
	sub_82E903D8(ctx, base);
	// 8262F7C0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8262F7C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8262F7C8: 392B9270  addi r9, r11, -0x6d90
	ctx.r[9].s64 = ctx.r[11].s64 + -28048;
	// 8262F7CC: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 8262F7D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8262F7D4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8262F7D8: 911F0088  stw r8, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[8].u32 ) };
	// 8262F7DC: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 8262F7E0: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 8262F7E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8262F7E8: 4BBE50B1  bl 0x82214898
	ctx.lr = 0x8262F7EC;
	sub_82214898(ctx, base);
	// 8262F7EC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8262F7F0: 38E00090  li r7, 0x90
	ctx.r[7].s64 = 144;
	// 8262F7F4: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8262F7F8: 38A000B0  li r5, 0xb0
	ctx.r[5].s64 = 176;
	// 8262F7FC: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 8262F800: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F830 size=128
    let mut pc: u32 = 0x8262F830;
    'dispatch: loop {
        match pc {
            0x8262F830 => {
    //   block [0x8262F830..0x8262F8B0)
	// 8262F830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262F83C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8262F844: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8262F848: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F84C: 392B9270  addi r9, r11, -0x6d90
	ctx.r[9].s64 = ctx.r[11].s64 + -28048;
	// 8262F850: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8262F854: 995F00E0  stb r10, 0xe0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u8 ) };
	// 8262F858: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8262F85C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8262F860: 419A0010  beq cr6, 0x8262f870
	if ctx.cr[6].eq {
	pc = 0x8262F870; continue 'dispatch;
	}
	// 8262F864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F868: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8262F86C: 48868CED  bl 0x82e98558
	ctx.lr = 0x8262F870;
	sub_82E98558(ctx, base);
	// 8262F870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F874: 48860CCD  bl 0x82e90540
	ctx.lr = 0x8262F878;
	sub_82E90540(ctx, base);
	// 8262F878: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8262F87C: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 8262F880: 394B0B7C  addi r10, r11, 0xb7c
	ctx.r[10].s64 = ctx.r[11].s64 + 2940;
	// 8262F884: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 8262F888: 4BB645B1  bl 0x82193e38
	ctx.lr = 0x8262F88C;
	sub_82193E38(ctx, base);
	// 8262F88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F894: 913F008C  stw r9, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 8262F898: 48860B99  bl 0x82e90430
	ctx.lr = 0x8262F89C;
	sub_82E90430(ctx, base);
	// 8262F89C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262F8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F8A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8262F8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8262F8B0 size=404
    let mut pc: u32 = 0x8262F8B0;
    'dispatch: loop {
        match pc {
            0x8262F8B0 => {
    //   block [0x8262F8B0..0x8262FA44)
	// 8262F8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F8B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8262F8BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8262F8C0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8262F8C4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F8C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8262F8CC: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F8D0: 3BC30034  addi r30, r3, 0x34
	ctx.r[30].s64 = ctx.r[3].s64 + 52;
	// 8262F8D4: C3EB9484  lfs f31, -0x6b7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8262F8D8: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8262F8DC: 409A0020  bne cr6, 0x8262f8fc
	if !ctx.cr[6].eq {
	pc = 0x8262F8FC; continue 'dispatch;
	}
	// 8262F8E0: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F8E4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8262F8E8: 409A0014  bne cr6, 0x8262f8fc
	if !ctx.cr[6].eq {
	pc = 0x8262F8FC; continue 'dispatch;
	}
	// 8262F8EC: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F8F4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8262F8F8: 419A0008  beq cr6, 0x8262f900
	if ctx.cr[6].eq {
	pc = 0x8262F900; continue 'dispatch;
	}
	// 8262F8FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262F900: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8262F904: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F908: 409A0110  bne cr6, 0x8262fa18
	if !ctx.cr[6].eq {
	pc = 0x8262FA18; continue 'dispatch;
	}
	// 8262F90C: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 8262F910: 3BE30088  addi r31, r3, 0x88
	ctx.r[31].s64 = ctx.r[3].s64 + 136;
	// 8262F914: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F918: 419A0038  beq cr6, 0x8262f950
	if ctx.cr[6].eq {
	pc = 0x8262F950; continue 'dispatch;
	}
	// 8262F91C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262F920: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262F924: 419A0114  beq cr6, 0x8262fa38
	if ctx.cr[6].eq {
	pc = 0x8262FA38; continue 'dispatch;
	}
	// 8262F928: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262F92C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F930: 419A0020  beq cr6, 0x8262f950
	if ctx.cr[6].eq {
	pc = 0x8262F950; continue 'dispatch;
	}
	// 8262F934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F938: 4BB44501  bl 0x82173e38
	ctx.lr = 0x8262F93C;
	sub_82173E38(ctx, base);
	// 8262F93C: 89630090  lbz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8262F940: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8262F944: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262F948: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262F94C: 409A0008  bne cr6, 0x8262f954
	if !ctx.cr[6].eq {
	pc = 0x8262F954; continue 'dispatch;
	}
	// 8262F950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F954: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8262F958: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262F95C: 419A00BC  beq cr6, 0x8262fa18
	if ctx.cr[6].eq {
	pc = 0x8262FA18; continue 'dispatch;
	}
	// 8262F960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F964: 4BB444D5  bl 0x82173e38
	ctx.lr = 0x8262F968;
	sub_82173E38(ctx, base);
	// 8262F968: 8163007C  lwz r11, 0x7c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 8262F96C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8262F970: 419A00A8  beq cr6, 0x8262fa18
	if ctx.cr[6].eq {
	pc = 0x8262FA18; continue 'dispatch;
	}
	// 8262F974: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8262F978: D3E10058  stfs f31, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8262F97C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8262F980: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8262F984: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8262F988: C00BCDBC  lfs f0, -0x3244(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12868 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262F98C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8262F990: 4BB444A9  bl 0x82173e38
	ctx.lr = 0x8262F994;
	sub_82173E38(ctx, base);
	// 8262F994: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8262F998: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8262F99C: 808A007C  lwz r4, 0x7c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(124 as u32) ) } as u64;
	// 8262F9A0: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262F9A4: 81090040  lwz r8, 0x40(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(64 as u32) ) } as u64;
	// 8262F9A8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8262F9AC: 4E800421  bctrl
	ctx.lr = 0x8262F9B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8262F9B0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8262F9B4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8262F9B8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8262F9BC: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 8262F9C0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8262FA48 size=1684
    let mut pc: u32 = 0x8262FA48;
    'dispatch: loop {
        match pc {
            0x8262FA48 => {
    //   block [0x8262FA48..0x826300DC)
	// 8262FA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FA4C: 4867999D  bl 0x82ca93e8
	ctx.lr = 0x8262FA50;
	sub_82CA93D0(ctx, base);
	// 8262FA50: 9421FAA0  stwu r1, -0x560(r1)
	ea = ctx.r[1].u32.wrapping_add(-1376 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FA54: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8262FA58: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8262FA5C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8262FA60: 3B3A0088  addi r25, r26, 0x88
	ctx.r[25].s64 = ctx.r[26].s64 + 136;
	// 8262FA64: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 8262FA68: 817A008C  lwz r11, 0x8c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(140 as u32) ) } as u64;
	// 8262FA6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262FA70: 419A0020  beq cr6, 0x8262fa90
	if ctx.cr[6].eq {
	pc = 0x8262FA90; continue 'dispatch;
	}
	// 8262FA74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262FA78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262FA7C: 419A000C  beq cr6, 0x8262fa88
	if ctx.cr[6].eq {
	pc = 0x8262FA88; continue 'dispatch;
	}
	// 8262FA80: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8262FA84: 48000010  b 0x8262fa94
	pc = 0x8262FA94; continue 'dispatch;
	// 8262FA88: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8262FA8C: 4BB643AD  bl 0x82193e38
	ctx.lr = 0x8262FA90;
	sub_82193E38(ctx, base);
	// 8262FA90: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8262FA94: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8262FA98: 5549EFFE  rlwinm r9, r10, 0x1d, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 8262FA9C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8262FAA0: 419A00F4  beq cr6, 0x8262fb94
	if ctx.cr[6].eq {
	pc = 0x8262FB94; continue 'dispatch;
	}
	// 8262FAA4: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8262FAA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262FAAC: 419A0024  beq cr6, 0x8262fad0
	if ctx.cr[6].eq {
	pc = 0x8262FAD0; continue 'dispatch;
	}
	// 8262FAB0: 894A0003  lbz r10, 3(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 8262FAB4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8262FAB8: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 8262FABC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8262FAC0: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262FAC4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8262FAC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262FACC: 480000CC  b 0x8262fb98
	pc = 0x8262FB98; continue 'dispatch;
	// 8262FAD0: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8262FAD4: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8262FAD8: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 8262FADC: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 8262FAE0: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8262FAE4: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8262FAE8: 40810054  ble 0x8262fb3c
	if !ctx.cr[0].gt {
	pc = 0x8262FB3C; continue 'dispatch;
	}
	// 8262FAEC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8262FAF0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8262FAF4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 8262FAF8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262FAFC: 2F070003  cmpwi cr6, r7, 3
	ctx.cr[6].compare_i32(ctx.r[7].s32, 3, &mut ctx.xer);
	// 8262FB00: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262FB04: 41980008  blt cr6, 0x8262fb0c
	if ctx.cr[6].lt {
	pc = 0x8262FB0C; continue 'dispatch;
	}
	// 8262FB08: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 8262FB0C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 8262FB10: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8262FB14: 419A0014  beq cr6, 0x8262fb28
	if ctx.cr[6].eq {
	pc = 0x8262FB28; continue 'dispatch;
	}
	// 8262FB18: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8262FB1C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8262FB20: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8262FB24: 4800000C  b 0x8262fb30
	pc = 0x8262FB30; continue 'dispatch;
	// 8262FB28: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 8262FB2C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8262FB30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8262FB34: 4199FFB8  bgt cr6, 0x8262faec
	if ctx.cr[6].gt {
	pc = 0x8262FAEC; continue 'dispatch;
	}
	// 8262FB38: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8262FB3C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8262FB40: 419A0040  beq cr6, 0x8262fb80
	if ctx.cr[6].eq {
	pc = 0x8262FB80; continue 'dispatch;
	}
	// 8262FB44: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8262FB48: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8262FB4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262FB50: 41990008  bgt cr6, 0x8262fb58
	if ctx.cr[6].gt {
	pc = 0x8262FB58; continue 'dispatch;
	}
	// 8262FB54: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8262FB58: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8262FB5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8262FB60: 409A0020  bne cr6, 0x8262fb80
	if !ctx.cr[6].eq {
	pc = 0x8262FB80; continue 'dispatch;
	}
	// 8262FB64: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8262FB68: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8262FB6C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8262FB70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262FB74: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8262FB78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262FB7C: 4800001C  b 0x8262fb98
	pc = 0x8262FB98; continue 'dispatch;
	// 8262FB80: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8262FB84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8262FB88: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8262FB8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8262FB90: 48000008  b 0x8262fb98
	pc = 0x8262FB98; continue 'dispatch;
	// 8262FB94: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8262FB98: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8262FB9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8262FBA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8262FBA4: 3B8BB730  addi r28, r11, -0x48d0
	ctx.r[28].s64 = ctx.r[11].s64 + -18640;
	// 8262FBA8: 419A0138  beq cr6, 0x8262fce0
	if ctx.cr[6].eq {
	pc = 0x8262FCE0; continue 'dispatch;
	}
	// 8262FBAC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8262FBB0: 930100A0  stw r24, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[24].u32 ) };
	// 8262FBB4: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 8262FBB8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8262FBBC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8262FBC0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8262FBC4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8262FBC8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8262FBCC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8262FBD0: 4082FFE8  bne 0x8262fbb8
	if !ctx.cr[0].eq {
	pc = 0x8262FBB8; continue 'dispatch;
	}
	// 8262FBD4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8262FBD8: C01CDD60  lfs f0, -0x22a0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8262FBDC: 3CC0820A  lis r6, -0x7df6
	ctx.r[6].s64 = -2113273856;
	// 8262FBE0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8262FBE4: 11A0038C  vspltisw v13, 0
	for i in 0..4 {
		ctx.v[13].u32[i] = 0;
	}
	// 8262FBE8: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 8262FBEC: 38869160  addi r4, r6, -0x6ea0
	ctx.r[4].s64 = ctx.r[6].s64 + -28320;
	// 8262FBF0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826300E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826300E0 size=80
    let mut pc: u32 = 0x826300E0;
    'dispatch: loop {
        match pc {
            0x826300E0 => {
    //   block [0x826300E0..0x82630130)
	// 826300E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826300E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826300E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826300EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826300F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826300F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826300F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 826300FC: 48000035  bl 0x82630130
	ctx.lr = 0x82630100;
	sub_82630130(ctx, base);
	// 82630100: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82630104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263010C: 419A000C  beq cr6, 0x82630118
	if ctx.cr[6].eq {
	pc = 0x82630118; continue 'dispatch;
	}
	// 82630110: 4BBEBC29  bl 0x8221bd38
	ctx.lr = 0x82630114;
	sub_8221BD38(ctx, base);
	// 82630114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263011C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630124: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82630128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263012C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630130 size=232
    let mut pc: u32 = 0x82630130;
    'dispatch: loop {
        match pc {
            0x82630130 => {
    //   block [0x82630130..0x82630218)
	// 82630130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630134: 486792CD  bl 0x82ca9400
	ctx.lr = 0x82630138;
	sub_82CA93D0(ctx, base);
	// 82630138: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263013C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82630140: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82630144: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82630148: 394B92E4  addi r10, r11, -0x6d1c
	ctx.r[10].s64 = ctx.r[11].s64 + -27932;
	// 8263014C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82630150: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82630154: 3BFC0004  addi r31, r28, 4
	ctx.r[31].s64 = ctx.r[28].s64 + 4;
	// 82630158: 813C000C  lwz r9, 0xc(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8263015C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82630160: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82630164: 7CE84850  subf r7, r8, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82630168: 7CC7DBD7  divw. r6, r7, r27
	ctx.r[6].s32 = ctx.r[7].s32 / ctx.r[27].s32;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8263016C: 4182003C  beq 0x826301a8
	if ctx.cr[0].eq {
	pc = 0x826301A8; continue 'dispatch;
	}
	// 82630170: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82630174: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82630178: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8263017C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82630180: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82630184: 4E800421  bctrl
	ctx.lr = 0x82630188;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82630188: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263018C: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82630190: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82630194: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 82630198: 7CE84850  subf r7, r8, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8263019C: 7CC7DBD6  divw r6, r7, r27
	ctx.r[6].s32 = ctx.r[7].s32 / ctx.r[27].s32;
	// 826301A0: 7F1D3040  cmplw cr6, r29, r6
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[6].u32, &mut ctx.xer);
	// 826301A4: 4198FFD0  blt cr6, 0x82630174
	if ctx.cr[6].lt {
	pc = 0x82630174; continue 'dispatch;
	}
	// 826301A8: 809C001C  lwz r4, 0x1c(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 826301AC: 3BDC0018  addi r30, r28, 0x18
	ctx.r[30].s64 = ctx.r[28].s64 + 24;
	// 826301B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 826301B4: 419A0018  beq cr6, 0x826301cc
	if ctx.cr[6].eq {
	pc = 0x826301CC; continue 'dispatch;
	}
	// 826301B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 826301BC: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 826301C0: 4BF3B7A1  bl 0x8256b960
	ctx.lr = 0x826301C4;
	sub_8256B960(ctx, base);
	// 826301C4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 826301C8: 4BBEBB71  bl 0x8221bd38
	ctx.lr = 0x826301CC;
	sub_8221BD38(ctx, base);
	// 826301CC: 935E0004  stw r26, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 826301D0: 935E0008  stw r26, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 826301D4: 935E000C  stw r26, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 826301D8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 826301DC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 826301E0: 419A0018  beq cr6, 0x826301f8
	if ctx.cr[6].eq {
	pc = 0x826301F8; continue 'dispatch;
	}
	// 826301E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826301E8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 826301EC: 48000175  bl 0x82630360
	ctx.lr = 0x826301F0;
	sub_82630360(ctx, base);
	// 826301F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 826301F4: 4BBEBB45  bl 0x8221bd38
	ctx.lr = 0x826301F8;
	sub_8221BD38(ctx, base);
	// 826301F8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 826301FC: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82630200: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82630204: 394B2B90  addi r10, r11, 0x2b90
	ctx.r[10].s64 = ctx.r[11].s64 + 11152;
	// 82630208: 935F000C  stw r26, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 8263020C: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82630210: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82630214: 4867923C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630218 size=84
    let mut pc: u32 = 0x82630218;
    'dispatch: loop {
        match pc {
            0x82630218 => {
    //   block [0x82630218..0x8263026C)
	// 82630218: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263021C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630220: 80E3000C  lwz r7, 0xc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82630224: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82630228: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 8263022C: 7D093850  subf r8, r9, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[9].s64;
	// 82630230: 9121FFF4  stw r9, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[9].u32 ) };
	// 82630234: 7D485BD7  divw. r10, r8, r11
	ctx.r[10].s32 = ctx.r[8].s32 / ctx.r[11].s32;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82630238: 40810048  ble 0x82630280
	if !ctx.cr[0].gt {
		sub_8263026C(ctx, base);
		return;
	}
	// 8263023C: 7D4B0E70  srawi r11, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82630240: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82630244: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82630248: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8263024C: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82630250: 80C90000  lwz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82630254: 7F062040  cmplw cr6, r6, r4
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82630258: 40980014  bge cr6, 0x8263026c
	if !ctx.cr[6].lt {
		sub_8263026C(ctx, base);
		return;
	}
	// 8263025C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82630260: 39290018  addi r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 + 24;
	// 82630264: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82630268: 4800000C  b 0x82630274
	sub_8263026C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263026C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263026C size=56
    let mut pc: u32 = 0x8263026C;
    'dispatch: loop {
        match pc {
            0x8263026C => {
    //   block [0x8263026C..0x826302A4)
	// 8263026C: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82630270: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82630274: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82630278: 4199FFC4  bgt cr6, 0x8263023c
	if ctx.cr[6].gt {
		sub_82630218(ctx, base);
		return;
	}
	// 8263027C: 9121FFF4  stw r9, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[9].u32 ) };
	// 82630280: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82630284: 419A0020  beq cr6, 0x826302a4
	if ctx.cr[6].eq {
		sub_826302A4(ctx, base);
		return;
	}
	// 82630288: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263028C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82630290: 41980014  blt cr6, 0x826302a4
	if ctx.cr[6].lt {
		sub_826302A4(ctx, base);
		return;
	}
	// 82630294: E961FFF0  ld r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630298: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8263029C: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 826302A0: 48000008  b 0x826302a8
	sub_826302A4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826302A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826302A4 size=20
    let mut pc: u32 = 0x826302A4;
    'dispatch: loop {
        match pc {
            0x826302A4 => {
    //   block [0x826302A4..0x826302B8)
	// 826302A4: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 826302A8: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 826302AC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 826302B0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 826302B4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826302B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826302B8 size=8
    let mut pc: u32 = 0x826302B8;
    'dispatch: loop {
        match pc {
            0x826302B8 => {
    //   block [0x826302B8..0x826302C0)
	// 826302B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826302BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826302C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826302C0 size=156
    let mut pc: u32 = 0x826302C0;
    'dispatch: loop {
        match pc {
            0x826302C0 => {
    //   block [0x826302C0..0x8263035C)
	// 826302C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826302C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826302C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826302CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826302D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826302D4: 38840003  addi r4, r4, 3
	ctx.r[4].s64 = ctx.r[4].s64 + 3;
	// 826302D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 826302DC: 4BBFCBF5  bl 0x8222ced0
	ctx.lr = 0x826302E0;
	sub_8222CED0(ctx, base);
	// 826302E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 826302E4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 826302E8: 4BBBFF59  bl 0x821f0240
	ctx.lr = 0x826302EC;
	sub_821F0240(ctx, base);
	// 826302EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 826302F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 826302F4: 388B92D4  addi r4, r11, -0x6d2c
	ctx.r[4].s64 = ctx.r[11].s64 + -27948;
	// 826302F8: 4BBAA6C9  bl 0x821da9c0
	ctx.lr = 0x826302FC;
	sub_821DA9C0(ctx, base);
	// 826302FC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82630300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82630304: 4BC34E9D  bl 0x822651a0
	ctx.lr = 0x82630308;
	sub_822651A0(ctx, base);
	// 82630308: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8263030C: 4BBE4ACD  bl 0x82214dd8
	ctx.lr = 0x82630310;
	sub_82214DD8(ctx, base);
	// 82630310: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82630314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82630318: 409A0010  bne cr6, 0x82630328
	if !ctx.cr[6].eq {
	pc = 0x82630328; continue 'dispatch;
	}
	// 8263031C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82630320: 386BFFDF  addi r3, r11, -0x21
	ctx.r[3].s64 = ctx.r[11].s64 + -33;
	// 82630324: 48000008  b 0x8263032c
	pc = 0x8263032C; continue 'dispatch;
	// 82630328: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8263032C: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 82630330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 82630334: 4BBC3A25  bl 0x821f3d58
	ctx.lr = 0x82630338;
	sub_821F3D58(ctx, base);
	// 82630338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263033C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82630340: 4BBE4A99  bl 0x82214dd8
	ctx.lr = 0x82630344;
	sub_82214DD8(ctx, base);
	// 82630344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630348: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263034C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630360 size=108
    let mut pc: u32 = 0x82630360;
    'dispatch: loop {
        match pc {
            0x82630360 => {
    //   block [0x82630360..0x826303CC)
	// 82630360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630364: 486790A5  bl 0x82ca9408
	ctx.lr = 0x82630368;
	sub_82CA93D0(ctx, base);
	// 82630368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263036C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82630370: 7F04E040  cmplw cr6, r4, r28
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82630374: 419A0050  beq cr6, 0x826303c4
	if ctx.cr[6].eq {
	pc = 0x826303C4; continue 'dispatch;
	}
	// 82630378: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8263037C: 3BE40010  addi r31, r4, 0x10
	ctx.r[31].s64 = ctx.r[4].s64 + 16;
	// 82630380: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82630384: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	// 82630388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263038C: 4BB963DD  bl 0x821c6768
	ctx.lr = 0x82630390;
	sub_821C6768(ctx, base);
	// 82630390: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82630394: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 82630398: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8263039C: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 826303A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 826303A4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 826303A8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 826303AC: 4082FFE8  bne 0x82630394
	if !ctx.cr[0].eq {
	pc = 0x82630394; continue 'dispatch;
	}
	// 826303B0: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 826303B4: 3BFF0018  addi r31, r31, 0x18
	ctx.r[31].s64 = ctx.r[31].s64 + 24;
	// 826303B8: 391FFFF0  addi r8, r31, -0x10
	ctx.r[8].s64 = ctx.r[31].s64 + -16;
	// 826303BC: 7F08E040  cmplw cr6, r8, r28
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[28].u32, &mut ctx.xer);
	// 826303C0: 409AFFC8  bne cr6, 0x82630388
	if !ctx.cr[6].eq {
	pc = 0x82630388; continue 'dispatch;
	}
	// 826303C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826303C8: 48679090  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826303D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826303D0 size=208
    let mut pc: u32 = 0x826303D0;
    'dispatch: loop {
        match pc {
            0x826303D0 => {
    //   block [0x826303D0..0x826304A0)
	// 826303D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826303D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826303D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826303DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826303E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826303E4: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 826303E8: 4BBEEE71  bl 0x8221f258
	ctx.lr = 0x826303EC;
	sub_8221F258(ctx, base);
	// 826303EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826303F0: 419A0098  beq cr6, 0x82630488
	if ctx.cr[6].eq {
	pc = 0x82630488; continue 'dispatch;
	}
	// 826303F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826303F8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826303FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630400: 392A38B8  addi r9, r10, 0x38b8
	ctx.r[9].s64 = ctx.r[10].s64 + 14520;
	// 82630404: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82630408: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8263040C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82630410: 3943001C  addi r10, r3, 0x1c
	ctx.r[10].s64 = ctx.r[3].s64 + 28;
	// 82630414: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82630418: 3943002C  addi r10, r3, 0x2c
	ctx.r[10].s64 = ctx.r[3].s64 + 44;
	// 8263041C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82630420: 3943003C  addi r10, r3, 0x3c
	ctx.r[10].s64 = ctx.r[3].s64 + 60;
	// 82630424: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82630428: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8263042C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82630430: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82630434: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82630438: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8263043C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82630440: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82630444: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82630448: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8263044C: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82630450: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82630454: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82630458: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8263045C: 9063000C  stw r3, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82630460: 9063003C  stw r3, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82630464: 9063001C  stw r3, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82630468: 9063002C  stw r3, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8263046C: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82630470: 99630050  stb r11, 0x50(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82630474: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263047C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630480: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630484: 4E800020  blr
	return;
	// 82630488: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263048C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263049C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826304A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826304A0 size=144
    let mut pc: u32 = 0x826304A0;
    'dispatch: loop {
        match pc {
            0x826304A0 => {
    //   block [0x826304A0..0x82630530)
	// 826304A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826304A4: 48678F69  bl 0x82ca940c
	ctx.lr = 0x826304A8;
	sub_82CA93D0(ctx, base);
	// 826304A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826304AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826304B0: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 826304B4: 4BBEEDA5  bl 0x8221f258
	ctx.lr = 0x826304B8;
	sub_8221F258(ctx, base);
	// 826304B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826304BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826304C0: 419A0064  beq cr6, 0x82630524
	if ctx.cr[6].eq {
	pc = 0x82630524; continue 'dispatch;
	}
	// 826304C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 826304C8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 826304CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826304D0: 394B48C0  addi r10, r11, 0x48c0
	ctx.r[10].s64 = ctx.r[11].s64 + 18624;
	// 826304D4: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826304D8: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 826304DC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826304E0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 826304E4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826304E8: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 826304EC: 4BCF069D  bl 0x82320b88
	ctx.lr = 0x826304F0;
	sub_82320B88(ctx, base);
	// 826304F0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 826304F4: 4BCF0695  bl 0x82320b88
	ctx.lr = 0x826304F8;
	sub_82320B88(ctx, base);
	// 826304F8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 826304FC: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82630500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630504: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 82630508: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8263050C: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82630510: 9BDF0044  stb r30, 0x44(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u8 ) };
	// 82630514: 9BDF0045  stb r30, 0x45(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(69 as u32), ctx.r[30].u8 ) };
	// 82630518: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 8263051C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82630520: 48678F3C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82630524: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630528: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263052C: 48678F30  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630530 size=88
    let mut pc: u32 = 0x82630530;
    'dispatch: loop {
        match pc {
            0x82630530 => {
    //   block [0x82630530..0x82630588)
	// 82630530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263053C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630544: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82630548: 4BBEED11  bl 0x8221f258
	ctx.lr = 0x8263054C;
	sub_8221F258(ctx, base);
	// 8263054C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630550: 419A0020  beq cr6, 0x82630570
	if ctx.cr[6].eq {
	pc = 0x82630570; continue 'dispatch;
	}
	// 82630554: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82630558: 48000071  bl 0x826305c8
	ctx.lr = 0x8263055C;
	sub_826305C8(ctx, base);
	// 8263055C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630568: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263056C: 4E800020  blr
	return;
	// 82630570: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263057C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630588 size=60
    let mut pc: u32 = 0x82630588;
    'dispatch: loop {
        match pc {
            0x82630588 => {
    //   block [0x82630588..0x826305C4)
	// 82630588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263058C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630598: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263059C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826305A0: 388B9300  addi r4, r11, -0x6d00
	ctx.r[4].s64 = ctx.r[11].s64 + -27904;
	// 826305A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826305A8: 4BBFC929  bl 0x8222ced0
	ctx.lr = 0x826305AC;
	sub_8222CED0(ctx, base);
	// 826305AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826305B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826305B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826305B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826305BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826305C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826305C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x826305C8 size=168
    let mut pc: u32 = 0x826305C8;
    'dispatch: loop {
        match pc {
            0x826305C8 => {
    //   block [0x826305C8..0x82630670)
	// 826305C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 826305CC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 826305D0: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 826305D4: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 826305D8: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 826305DC: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 826305E0: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826305E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826305E8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 826305EC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826305F0: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826305F4: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 826305F8: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 826305FC: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630670 size=80
    let mut pc: u32 = 0x82630670;
    'dispatch: loop {
        match pc {
            0x82630670 => {
    //   block [0x82630670..0x826306C0)
	// 82630670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630678: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263067C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630680: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630688: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8263068C: 4BF65C95  bl 0x82596320
	ctx.lr = 0x82630690;
	sub_82596320(ctx, base);
	// 82630690: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82630694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630698: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8263069C: 419A000C  beq cr6, 0x826306a8
	if ctx.cr[6].eq {
	pc = 0x826306A8; continue 'dispatch;
	}
	// 826306A0: 4BBEB699  bl 0x8221bd38
	ctx.lr = 0x826306A4;
	sub_8221BD38(ctx, base);
	// 826306A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826306A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826306AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826306B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826306B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826306B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826306BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826306C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826306C0 size=196
    let mut pc: u32 = 0x826306C0;
    'dispatch: loop {
        match pc {
            0x826306C0 => {
    //   block [0x826306C0..0x82630784)
	// 826306C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826306C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826306C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826306CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826306D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826306D4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 826306D8: 4BBEEB81  bl 0x8221f258
	ctx.lr = 0x826306DC;
	sub_8221F258(ctx, base);
	// 826306DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826306E0: 419A008C  beq cr6, 0x8263076c
	if ctx.cr[6].eq {
	pc = 0x8263076C; continue 'dispatch;
	}
	// 826306E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 826306E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826306EC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 826306F0: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 826306F4: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 826306F8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 826306FC: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82630700: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82630704: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82630708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630788 size=60
    let mut pc: u32 = 0x82630788;
    'dispatch: loop {
        match pc {
            0x82630788 => {
    //   block [0x82630788..0x826307C4)
	// 82630788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263078C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630798: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8263079C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826307A0: 388B9314  addi r4, r11, -0x6cec
	ctx.r[4].s64 = ctx.r[11].s64 + -27884;
	// 826307A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826307A8: 4BBFC729  bl 0x8222ced0
	ctx.lr = 0x826307AC;
	sub_8222CED0(ctx, base);
	// 826307AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826307B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826307B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826307B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826307BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826307C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826307C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826307C8 size=312
    let mut pc: u32 = 0x826307C8;
    'dispatch: loop {
        match pc {
            0x826307C8 => {
    //   block [0x826307C8..0x82630900)
	// 826307C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826307CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826307D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826307D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826307D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826307DC: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 826307E0: 4BBEEA79  bl 0x8221f258
	ctx.lr = 0x826307E4;
	sub_8221F258(ctx, base);
	// 826307E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826307E8: 419A0100  beq cr6, 0x826308e8
	if ctx.cr[6].eq {
	pc = 0x826308E8; continue 'dispatch;
	}
	// 826307EC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 826307F0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826307F4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 826307F8: 390A9490  addi r8, r10, -0x6b70
	ctx.r[8].s64 = ctx.r[10].s64 + -27504;
	// 826307FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82630800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630804: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82630808: C1AA9490  lfs f13, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8263080C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82630810: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82630814: C008FFF4  lfs f0, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82630818: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8263081C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82630820: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630900 size=108
    let mut pc: u32 = 0x82630900;
    'dispatch: loop {
        match pc {
            0x82630900 => {
    //   block [0x82630900..0x8263096C)
	// 82630900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263090C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630914: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82630918: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8263091C: 4BBEE93D  bl 0x8221f258
	ctx.lr = 0x82630920;
	sub_8221F258(ctx, base);
	// 82630920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630924: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82630928: 419A0028  beq cr6, 0x82630950
	if ctx.cr[6].eq {
	pc = 0x82630950; continue 'dispatch;
	}
	// 8263092C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82630930: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82630934: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82630938: 48045969  bl 0x826762a0
	ctx.lr = 0x8263093C;
	sub_826762A0(ctx, base);
	// 8263093C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82630940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630944: 394B7308  addi r10, r11, 0x7308
	ctx.r[10].s64 = ctx.r[11].s64 + 29448;
	// 82630948: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8263094C: 48000008  b 0x82630954
	pc = 0x82630954; continue 'dispatch;
	// 82630950: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82630958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263095C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630960: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82630964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630970 size=108
    let mut pc: u32 = 0x82630970;
    'dispatch: loop {
        match pc {
            0x82630970 => {
    //   block [0x82630970..0x826309DC)
	// 82630970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263097C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630984: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82630988: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8263098C: 4BBEE8CD  bl 0x8221f258
	ctx.lr = 0x82630990;
	sub_8221F258(ctx, base);
	// 82630990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630994: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82630998: 419A0028  beq cr6, 0x826309c0
	if ctx.cr[6].eq {
	pc = 0x826309C0; continue 'dispatch;
	}
	// 8263099C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 826309A0: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 826309A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 826309A8: 480458F9  bl 0x826762a0
	ctx.lr = 0x826309AC;
	sub_826762A0(ctx, base);
	// 826309AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 826309B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826309B4: 394B6E58  addi r10, r11, 0x6e58
	ctx.r[10].s64 = ctx.r[11].s64 + 28248;
	// 826309B8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826309BC: 48000008  b 0x826309c4
	pc = 0x826309C4; continue 'dispatch;
	// 826309C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826309C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826309C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826309CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826309D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826309D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826309D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826309E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826309E0 size=188
    let mut pc: u32 = 0x826309E0;
    'dispatch: loop {
        match pc {
            0x826309E0 => {
    //   block [0x826309E0..0x82630A9C)
	// 826309E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826309E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826309E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826309EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826309F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826309F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826309F8: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 826309FC: 4BBEE85D  bl 0x8221f258
	ctx.lr = 0x82630A00;
	sub_8221F258(ctx, base);
	// 82630A00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630A04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82630A08: 419A0078  beq cr6, 0x82630a80
	if ctx.cr[6].eq {
	pc = 0x82630A80; continue 'dispatch;
	}
	// 82630A0C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82630A10: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82630A14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82630A18: 48045889  bl 0x826762a0
	ctx.lr = 0x82630A1C;
	sub_826762A0(ctx, base);
	// 82630A1C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82630A20: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82630A24: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 82630A28: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 82630A2C: 38CBBD44  addi r6, r11, -0x42bc
	ctx.r[6].s64 = ctx.r[11].s64 + -17084;
	// 82630A30: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82630A34: 388973B8  addi r4, r9, 0x73b8
	ctx.r[4].s64 = ctx.r[9].s64 + 29624;
	// 82630A38: 90DF0054  stw r6, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82630A3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630A40: 386891B8  addi r3, r8, -0x6e48
	ctx.r[3].s64 = ctx.r[8].s64 + -28232;
	// 82630A44: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82630A48: 396791AC  addi r11, r7, -0x6e54
	ctx.r[11].s64 = ctx.r[7].s64 + -28244;
	// 82630A4C: 995F0058  stb r10, 0x58(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u8 ) };
	// 82630A50: 39250B7C  addi r9, r5, 0xb7c
	ctx.r[9].s64 = ctx.r[5].s64 + 2940;
	// 82630A54: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82630A58: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82630A5C: 397F005C  addi r11, r31, 0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + 92;
	// 82630A60: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82630A64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82630A68: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82630A6C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82630A70: 397F0054  addi r11, r31, 0x54
	ctx.r[11].s64 = ctx.r[31].s64 + 84;
	// 82630A74: 4BBE3E25  bl 0x82214898
	ctx.lr = 0x82630A78;
	sub_82214898(ctx, base);
	// 82630A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630A7C: 48000008  b 0x82630a84
	pc = 0x82630A84; continue 'dispatch;
	// 82630A80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630A84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82630A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630A90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82630A94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630AA0 size=140
    let mut pc: u32 = 0x82630AA0;
    'dispatch: loop {
        match pc {
            0x82630AA0 => {
    //   block [0x82630AA0..0x82630B2C)
	// 82630AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630AA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82630AAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630AB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82630AB8: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82630ABC: 4BBEE79D  bl 0x8221f258
	ctx.lr = 0x82630AC0;
	sub_8221F258(ctx, base);
	// 82630AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630AC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82630AC8: 419A0048  beq cr6, 0x82630b10
	if ctx.cr[6].eq {
	pc = 0x82630B10; continue 'dispatch;
	}
	// 82630ACC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82630AD0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82630AD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82630AD8: 480457C9  bl 0x826762a0
	ctx.lr = 0x82630ADC;
	sub_826762A0(ctx, base);
	// 82630ADC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82630AE0: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82630AE4: 394B70F0  addi r10, r11, 0x70f0
	ctx.r[10].s64 = ctx.r[11].s64 + 28912;
	// 82630AE8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82630AEC: 482174C5  bl 0x82847fb0
	ctx.lr = 0x82630AF0;
	sub_82847FB0(ctx, base);
	// 82630AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630AF4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82630AF8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82630AFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630B00: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630B04: 913F0068  stw r9, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82630B08: 997F006C  stb r11, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u8 ) };
	// 82630B0C: 48000008  b 0x82630b14
	pc = 0x82630B14; continue 'dispatch;
	// 82630B10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630B14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82630B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630B20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82630B24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630B30 size=124
    let mut pc: u32 = 0x82630B30;
    'dispatch: loop {
        match pc {
            0x82630B30 => {
    //   block [0x82630B30..0x82630BAC)
	// 82630B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630B38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82630B3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630B44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82630B48: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 82630B4C: 4BBEE70D  bl 0x8221f258
	ctx.lr = 0x82630B50;
	sub_8221F258(ctx, base);
	// 82630B50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630B54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82630B58: 419A0038  beq cr6, 0x82630b90
	if ctx.cr[6].eq {
	pc = 0x82630B90; continue 'dispatch;
	}
	// 82630B5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82630B60: 4BF65C59  bl 0x825967b8
	ctx.lr = 0x82630B64;
	sub_825967B8(ctx, base);
	// 82630B64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82630B68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82630B6C: 392B71A0  addi r9, r11, 0x71a0
	ctx.r[9].s64 = ctx.r[11].s64 + 29088;
	// 82630B70: 390A0DB8  addi r8, r10, 0xdb8
	ctx.r[8].s64 = ctx.r[10].s64 + 3512;
	// 82630B74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630B78: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82630B7C: 911F0054  stw r8, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630B84: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 82630B88: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82630B8C: 48000008  b 0x82630b94
	pc = 0x82630B94; continue 'dispatch;
	// 82630B90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630B94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82630B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630BA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82630BA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630BB0 size=124
    let mut pc: u32 = 0x82630BB0;
    'dispatch: loop {
        match pc {
            0x82630BB0 => {
    //   block [0x82630BB0..0x82630C2C)
	// 82630BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82630BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630BC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630BC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82630BC8: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 82630BCC: 4BBEE68D  bl 0x8221f258
	ctx.lr = 0x82630BD0;
	sub_8221F258(ctx, base);
	// 82630BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630BD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82630BD8: 419A0038  beq cr6, 0x82630c10
	if ctx.cr[6].eq {
	pc = 0x82630C10; continue 'dispatch;
	}
	// 82630BDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82630BE0: 4BF65BD9  bl 0x825967b8
	ctx.lr = 0x82630BE4;
	sub_825967B8(ctx, base);
	// 82630BE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82630BE8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82630BEC: 392B7038  addi r9, r11, 0x7038
	ctx.r[9].s64 = ctx.r[11].s64 + 28728;
	// 82630BF0: 390A0DB8  addi r8, r10, 0xdb8
	ctx.r[8].s64 = ctx.r[10].s64 + 3512;
	// 82630BF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630BF8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82630BFC: 911F0054  stw r8, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630C04: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 82630C08: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82630C0C: 48000008  b 0x82630c14
	pc = 0x82630C14; continue 'dispatch;
	// 82630C10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630C14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82630C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630C20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82630C24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630C30 size=60
    let mut pc: u32 = 0x82630C30;
    'dispatch: loop {
        match pc {
            0x82630C30 => {
    //   block [0x82630C30..0x82630C6C)
	// 82630C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630C3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630C40: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82630C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82630C48: 388B936C  addi r4, r11, -0x6c94
	ctx.r[4].s64 = ctx.r[11].s64 + -27796;
	// 82630C4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630C50: 4BBFC281  bl 0x8222ced0
	ctx.lr = 0x82630C54;
	sub_8222CED0(ctx, base);
	// 82630C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630C64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630C70 size=92
    let mut pc: u32 = 0x82630C70;
    'dispatch: loop {
        match pc {
            0x82630C70 => {
    //   block [0x82630C70..0x82630CCC)
	// 82630C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630C78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82630C7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630C80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630C88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82630C8C: 387F0130  addi r3, r31, 0x130
	ctx.r[3].s64 = ctx.r[31].s64 + 304;
	// 82630C90: 4BB86E89  bl 0x821b7b18
	ctx.lr = 0x82630C94;
	sub_821B7B18(ctx, base);
	// 82630C94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630C98: 4BF65DC1  bl 0x82596a58
	ctx.lr = 0x82630C9C;
	sub_82596A58(ctx, base);
	// 82630C9C: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82630CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630CA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82630CA8: 419A000C  beq cr6, 0x82630cb4
	if ctx.cr[6].eq {
	pc = 0x82630CB4; continue 'dispatch;
	}
	// 82630CAC: 4BBEB08D  bl 0x8221bd38
	ctx.lr = 0x82630CB0;
	sub_8221BD38(ctx, base);
	// 82630CB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82630CB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82630CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630CC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82630CC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630CD0 size=88
    let mut pc: u32 = 0x82630CD0;
    'dispatch: loop {
        match pc {
            0x82630CD0 => {
    //   block [0x82630CD0..0x82630D28)
	// 82630CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630CD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630CDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630CE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630CE4: 386002A0  li r3, 0x2a0
	ctx.r[3].s64 = 672;
	// 82630CE8: 4BBEE571  bl 0x8221f258
	ctx.lr = 0x82630CEC;
	sub_8221F258(ctx, base);
	// 82630CEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630CF0: 419A0020  beq cr6, 0x82630d10
	if ctx.cr[6].eq {
	pc = 0x82630D10; continue 'dispatch;
	}
	// 82630CF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82630CF8: 480319B1  bl 0x826626a8
	ctx.lr = 0x82630CFC;
	sub_826626A8(ctx, base);
	// 82630CFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630D08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630D0C: 4E800020  blr
	return;
	// 82630D10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630D14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630D20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82630D28 size=128
    let mut pc: u32 = 0x82630D28;
    'dispatch: loop {
        match pc {
            0x82630D28 => {
    //   block [0x82630D28..0x82630DA8)
	// 82630D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630D30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630D34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630D38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630D3C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82630D40: 4BBEE519  bl 0x8221f258
	ctx.lr = 0x82630D44;
	sub_8221F258(ctx, base);
	// 82630D44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630D48: 419A0048  beq cr6, 0x82630d90
	if ctx.cr[6].eq {
	pc = 0x82630D90; continue 'dispatch;
	}
	// 82630D4C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82630D50: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82630D54: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82630D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630D5C: 39096C08  addi r8, r9, 0x6c08
	ctx.r[8].s64 = ctx.r[9].s64 + 27656;
	// 82630D60: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82630D64: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82630D68: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82630D6C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82630D70: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82630D74: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82630D78: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82630D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630D8C: 4E800020  blr
	return;
	// 82630D90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630D94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630DA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630DA8 size=88
    let mut pc: u32 = 0x82630DA8;
    'dispatch: loop {
        match pc {
            0x82630DA8 => {
    //   block [0x82630DA8..0x82630E00)
	// 82630DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630DB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630DB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630DB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630DBC: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 82630DC0: 4BBEE499  bl 0x8221f258
	ctx.lr = 0x82630DC4;
	sub_8221F258(ctx, base);
	// 82630DC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630DC8: 419A0020  beq cr6, 0x82630de8
	if ctx.cr[6].eq {
	pc = 0x82630DE8; continue 'dispatch;
	}
	// 82630DCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82630DD0: 4BF41029  bl 0x82571df8
	ctx.lr = 0x82630DD4;
	sub_82571DF8(ctx, base);
	// 82630DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630DE4: 4E800020  blr
	return;
	// 82630DE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630DEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630DF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630E00 size=152
    let mut pc: u32 = 0x82630E00;
    'dispatch: loop {
        match pc {
            0x82630E00 => {
    //   block [0x82630E00..0x82630E98)
	// 82630E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630E08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630E10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630E14: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82630E18: 4BBEE441  bl 0x8221f258
	ctx.lr = 0x82630E1C;
	sub_8221F258(ctx, base);
	// 82630E1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630E20: 419A0060  beq cr6, 0x82630e80
	if ctx.cr[6].eq {
	pc = 0x82630E80; continue 'dispatch;
	}
	// 82630E24: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82630E28: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82630E2C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82630E30: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82630E34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630E38: 38EA388C  addi r7, r10, 0x388c
	ctx.r[7].s64 = ctx.r[10].s64 + 14476;
	// 82630E3C: 38C93778  addi r6, r9, 0x3778
	ctx.r[6].s64 = ctx.r[9].s64 + 14200;
	// 82630E40: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82630E44: 38A82ABC  addi r5, r8, 0x2abc
	ctx.r[5].s64 = ctx.r[8].s64 + 10940;
	// 82630E48: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82630E4C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82630E50: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82630E54: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82630E58: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82630E5C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82630E60: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82630E64: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82630E68: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82630E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630E7C: 4E800020  blr
	return;
	// 82630E80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630E90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630E98 size=140
    let mut pc: u32 = 0x82630E98;
    'dispatch: loop {
        match pc {
            0x82630E98 => {
    //   block [0x82630E98..0x82630F24)
	// 82630E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630EA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630EA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630EAC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82630EB0: 4BBEE3A9  bl 0x8221f258
	ctx.lr = 0x82630EB4;
	sub_8221F258(ctx, base);
	// 82630EB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630EB8: 419A0054  beq cr6, 0x82630f0c
	if ctx.cr[6].eq {
	pc = 0x82630F0C; continue 'dispatch;
	}
	// 82630EBC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82630EC0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82630EC4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82630EC8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 82630ECC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630ED0: 38EA388C  addi r7, r10, 0x388c
	ctx.r[7].s64 = ctx.r[10].s64 + 14476;
	// 82630ED4: 38C94CF0  addi r6, r9, 0x4cf0
	ctx.r[6].s64 = ctx.r[9].s64 + 19696;
	// 82630ED8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82630EDC: 38A84D30  addi r5, r8, 0x4d30
	ctx.r[5].s64 = ctx.r[8].s64 + 19760;
	// 82630EE0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82630EE4: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82630EE8: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82630EEC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82630EF0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82630EF4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82630EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630F04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630F08: 4E800020  blr
	return;
	// 82630F0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630F28 size=88
    let mut pc: u32 = 0x82630F28;
    'dispatch: loop {
        match pc {
            0x82630F28 => {
    //   block [0x82630F28..0x82630F80)
	// 82630F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630F30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630F34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630F38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630F3C: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82630F40: 4BBEE319  bl 0x8221f258
	ctx.lr = 0x82630F44;
	sub_8221F258(ctx, base);
	// 82630F44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630F48: 419A0020  beq cr6, 0x82630f68
	if ctx.cr[6].eq {
	pc = 0x82630F68; continue 'dispatch;
	}
	// 82630F4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82630F50: 4BF4B849  bl 0x8257c798
	ctx.lr = 0x82630F54;
	sub_8257C798(ctx, base);
	// 82630F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630F60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630F64: 4E800020  blr
	return;
	// 82630F68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630F6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630F78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630F80 size=88
    let mut pc: u32 = 0x82630F80;
    'dispatch: loop {
        match pc {
            0x82630F80 => {
    //   block [0x82630F80..0x82630FD8)
	// 82630F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630F88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630F8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630F94: 386000C4  li r3, 0xc4
	ctx.r[3].s64 = 196;
	// 82630F98: 4BBEE2C1  bl 0x8221f258
	ctx.lr = 0x82630F9C;
	sub_8221F258(ctx, base);
	// 82630F9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630FA0: 419A0020  beq cr6, 0x82630fc0
	if ctx.cr[6].eq {
	pc = 0x82630FC0; continue 'dispatch;
	}
	// 82630FA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82630FA8: 4BF83071  bl 0x825b4018
	ctx.lr = 0x82630FAC;
	sub_825B4018(ctx, base);
	// 82630FAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630FB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630FBC: 4E800020  blr
	return;
	// 82630FC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82630FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82630FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630FD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82630FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82630FD8 size=244
    let mut pc: u32 = 0x82630FD8;
    'dispatch: loop {
        match pc {
            0x82630FD8 => {
    //   block [0x82630FD8..0x826310CC)
	// 82630FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82630FE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630FE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82630FEC: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82630FF0: 4BBEE269  bl 0x8221f258
	ctx.lr = 0x82630FF4;
	sub_8221F258(ctx, base);
	// 82630FF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82630FF8: 419A00BC  beq cr6, 0x826310b4
	if ctx.cr[6].eq {
	pc = 0x826310B4; continue 'dispatch;
	}
	// 82630FFC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82631000: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631004: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82631008: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8263100C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631010: 38EB388C  addi r7, r11, 0x388c
	ctx.r[7].s64 = ctx.r[11].s64 + 14476;
	// 82631014: 38C94178  addi r6, r9, 0x4178
	ctx.r[6].s64 = ctx.r[9].s64 + 16760;
	// 82631018: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 8263101C: 38A82D54  addi r5, r8, 0x2d54
	ctx.r[5].s64 = ctx.r[8].s64 + 11604;
	// 82631020: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82631024: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82631028: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8263102C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82631030: 3C80820A  lis r4, -0x7df6
	ctx.r[4].s64 = -2113273856;
	// 82631034: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631038: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 8263103C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82631040: 38E49160  addi r7, r4, -0x6ea0
	ctx.r[7].s64 = ctx.r[4].s64 + -28320;
	// 82631044: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82631048: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8263104C: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82631050: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82631054: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82631058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263105C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82631060: C0089490  lfs f0, -0x6b70(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631064: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82631068: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8263106C: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826310D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826310D0 size=308
    let mut pc: u32 = 0x826310D0;
    'dispatch: loop {
        match pc {
            0x826310D0 => {
    //   block [0x826310D0..0x82631204)
	// 826310D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826310D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826310D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826310DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826310E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826310E4: 3860008C  li r3, 0x8c
	ctx.r[3].s64 = 140;
	// 826310E8: 4BBEE171  bl 0x8221f258
	ctx.lr = 0x826310EC;
	sub_8221F258(ctx, base);
	// 826310EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826310F0: 419A00FC  beq cr6, 0x826311ec
	if ctx.cr[6].eq {
	pc = 0x826311EC; continue 'dispatch;
	}
	// 826310F4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 826310F8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826310FC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 82631100: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82631104: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82631108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263110C: 38CACB94  addi r6, r10, -0x346c
	ctx.r[6].s64 = ctx.r[10].s64 + -13420;
	// 82631110: C18ACB94  lfs f12, -0x346c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13420 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82631114: 38A9388C  addi r5, r9, 0x388c
	ctx.r[5].s64 = ctx.r[9].s64 + 14476;
	// 82631118: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263111C: 38884980  addi r4, r8, 0x4980
	ctx.r[4].s64 = ctx.r[8].s64 + 18816;
	// 82631120: 39272E30  addi r9, r7, 0x2e30
	ctx.r[9].s64 = ctx.r[7].s64 + 11824;
	// 82631124: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82631128: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 8263112C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82631130: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82631134: C006C8F0  lfs f0, -0x3710(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-14096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631138: 99430012  stb r10, 0x12(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[10].u8 ) };
	// 8263113C: C1A6C87C  lfs f13, -0x3784(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-14212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82631140: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 82631144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631148: 99430010  stb r10, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 8263114C: 99430013  stb r10, 0x13(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(19 as u32), ctx.r[10].u8 ) };
	// 82631150: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82631154: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82631158: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8263115C: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82631160: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82631164: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82631168: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8263116C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82631170: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82631174: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82631178: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8263117C: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82631180: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82631184: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82631188: D003004C  stfs f0, 0x4c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 8263118C: D0030050  stfs f0, 0x50(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82631190: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82631194: D0030058  stfs f0, 0x58(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82631198: D003005C  stfs f0, 0x5c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8263119C: D0030060  stfs f0, 0x60(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 826311A0: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826311A4: D0030068  stfs f0, 0x68(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 826311A8: 91630078  stw r11, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 826311AC: D003006C  stfs f0, 0x6c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 826311B0: 9163007C  stw r11, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 826311B4: D1830070  stfs f12, 0x70(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 826311B8: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826311BC: 99630084  stb r11, 0x84(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 826311C0: 99630085  stb r11, 0x85(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(133 as u32), ctx.r[11].u8 ) };
	// 826311C4: 99630086  stb r11, 0x86(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(134 as u32), ctx.r[11].u8 ) };
	// 826311C8: 99630087  stb r11, 0x87(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(135 as u32), ctx.r[11].u8 ) };
	// 826311CC: 99030088  stb r8, 0x88(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[8].u8 ) };
	// 826311D0: 99630089  stb r11, 0x89(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(137 as u32), ctx.r[11].u8 ) };
	// 826311D4: 9963008A  stb r11, 0x8a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(138 as u32), ctx.r[11].u8 ) };
	// 826311D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826311DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826311E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826311E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826311E8: 4E800020  blr
	return;
	// 826311EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826311F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826311F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826311F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826311FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631208 size=140
    let mut pc: u32 = 0x82631208;
    'dispatch: loop {
        match pc {
            0x82631208 => {
    //   block [0x82631208..0x82631294)
	// 82631208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263120C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631214: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263121C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82631220: 4BBEE039  bl 0x8221f258
	ctx.lr = 0x82631224;
	sub_8221F258(ctx, base);
	// 82631224: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631228: 419A0054  beq cr6, 0x8263127c
	if ctx.cr[6].eq {
	pc = 0x8263127C; continue 'dispatch;
	}
	// 8263122C: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82631230: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631234: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82631238: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8263123C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631240: 38EA388C  addi r7, r10, 0x388c
	ctx.r[7].s64 = ctx.r[10].s64 + 14476;
	// 82631244: 38C96EC8  addi r6, r9, 0x6ec8
	ctx.r[6].s64 = ctx.r[9].s64 + 28360;
	// 82631248: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263124C: 38A86F08  addi r5, r8, 0x6f08
	ctx.r[5].s64 = ctx.r[8].s64 + 28424;
	// 82631250: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82631254: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82631258: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8263125C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82631260: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82631264: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82631268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263126C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631278: 4E800020  blr
	return;
	// 8263127C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263128C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631298 size=156
    let mut pc: u32 = 0x82631298;
    'dispatch: loop {
        match pc {
            0x82631298 => {
    //   block [0x82631298..0x82631334)
	// 82631298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263129C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826312A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826312A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826312A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826312AC: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 826312B0: 4BBEDFA9  bl 0x8221f258
	ctx.lr = 0x826312B4;
	sub_8221F258(ctx, base);
	// 826312B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826312B8: 419A0064  beq cr6, 0x8263131c
	if ctx.cr[6].eq {
	pc = 0x8263131C; continue 'dispatch;
	}
	// 826312BC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 826312C0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826312C4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 826312C8: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 826312CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826312D0: 38EA388C  addi r7, r10, 0x388c
	ctx.r[7].s64 = ctx.r[10].s64 + 14476;
	// 826312D4: 38C97060  addi r6, r9, 0x7060
	ctx.r[6].s64 = ctx.r[9].s64 + 28768;
	// 826312D8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826312DC: 38A870A0  addi r5, r8, 0x70a0
	ctx.r[5].s64 = ctx.r[8].s64 + 28832;
	// 826312E0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 826312E4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 826312E8: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 826312EC: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 826312F0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 826312F4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826312F8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826312FC: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82631300: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82631304: 91430024  stw r10, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82631308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263130C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631318: 4E800020  blr
	return;
	// 8263131C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263132C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631338 size=88
    let mut pc: u32 = 0x82631338;
    'dispatch: loop {
        match pc {
            0x82631338 => {
    //   block [0x82631338..0x82631390)
	// 82631338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263133C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263134C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82631350: 4BBEDF09  bl 0x8221f258
	ctx.lr = 0x82631354;
	sub_8221F258(ctx, base);
	// 82631354: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631358: 419A0020  beq cr6, 0x82631378
	if ctx.cr[6].eq {
	pc = 0x82631378; continue 'dispatch;
	}
	// 8263135C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82631360: 481F8151  bl 0x828294b0
	ctx.lr = 0x82631364;
	sub_828294B0(ctx, base);
	// 82631364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263136C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631374: 4E800020  blr
	return;
	// 82631378: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263137C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263138C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631390 size=88
    let mut pc: u32 = 0x82631390;
    'dispatch: loop {
        match pc {
            0x82631390 => {
    //   block [0x82631390..0x826313E8)
	// 82631390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263139C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826313A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826313A4: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 826313A8: 4BBEDEB1  bl 0x8221f258
	ctx.lr = 0x826313AC;
	sub_8221F258(ctx, base);
	// 826313AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826313B0: 419A0020  beq cr6, 0x826313d0
	if ctx.cr[6].eq {
	pc = 0x826313D0; continue 'dispatch;
	}
	// 826313B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826313B8: 4BD71D91  bl 0x823a3148
	ctx.lr = 0x826313BC;
	sub_823A3148(ctx, base);
	// 826313BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826313C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826313C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826313C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826313CC: 4E800020  blr
	return;
	// 826313D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826313D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826313D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826313DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826313E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826313E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826313E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826313E8 size=120
    let mut pc: u32 = 0x826313E8;
    'dispatch: loop {
        match pc {
            0x826313E8 => {
    //   block [0x826313E8..0x82631460)
	// 826313E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826313EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826313F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826313F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826313F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826313FC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82631400: 4BBEDE59  bl 0x8221f258
	ctx.lr = 0x82631404;
	sub_8221F258(ctx, base);
	// 82631404: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631408: 419A0040  beq cr6, 0x82631448
	if ctx.cr[6].eq {
	pc = 0x82631448; continue 'dispatch;
	}
	// 8263140C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631410: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631414: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631418: 392A5198  addi r9, r10, 0x5198
	ctx.r[9].s64 = ctx.r[10].s64 + 20888;
	// 8263141C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631420: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82631424: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631428: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8263142C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82631430: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82631434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263143C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631444: 4E800020  blr
	return;
	// 82631448: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263144C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263145C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631460 size=120
    let mut pc: u32 = 0x82631460;
    'dispatch: loop {
        match pc {
            0x82631460 => {
    //   block [0x82631460..0x826314D8)
	// 82631460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263146C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631470: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631474: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82631478: 4BBEDDE1  bl 0x8221f258
	ctx.lr = 0x8263147C;
	sub_8221F258(ctx, base);
	// 8263147C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631480: 419A0040  beq cr6, 0x826314c0
	if ctx.cr[6].eq {
	pc = 0x826314C0; continue 'dispatch;
	}
	// 82631484: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631488: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263148C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631490: 392A50C8  addi r9, r10, 0x50c8
	ctx.r[9].s64 = ctx.r[10].s64 + 20680;
	// 82631494: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631498: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8263149C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826314A0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826314A4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826314A8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826314AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826314B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826314B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826314B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826314BC: 4E800020  blr
	return;
	// 826314C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826314C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826314C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826314CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826314D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826314D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826314D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826314D8 size=324
    let mut pc: u32 = 0x826314D8;
    'dispatch: loop {
        match pc {
            0x826314D8 => {
    //   block [0x826314D8..0x8263161C)
	// 826314D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826314DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826314E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826314E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826314E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826314EC: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 826314F0: 4BBEDD69  bl 0x8221f258
	ctx.lr = 0x826314F4;
	sub_8221F258(ctx, base);
	// 826314F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826314F8: 419A010C  beq cr6, 0x82631604
	if ctx.cr[6].eq {
	pc = 0x82631604; continue 'dispatch;
	}
	// 826314FC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82631500: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631504: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631508: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 8263150C: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 82631510: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631514: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82631518: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263151C: 388A04D4  addi r4, r10, 0x4d4
	ctx.r[4].s64 = ctx.r[10].s64 + 1236;
	// 82631520: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82631524: 39463D78  addi r10, r6, 0x3d78
	ctx.r[10].s64 = ctx.r[6].s64 + 15736;
	// 82631528: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8263152C: 38C53BE4  addi r6, r5, 0x3be4
	ctx.r[6].s64 = ctx.r[5].s64 + 15332;
	// 82631530: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82631534: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82631538: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8263153C: 3C808349  lis r4, -0x7cb7
	ctx.r[4].s64 = -2092367872;
	// 82631540: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631544: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82631548: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 8263154C: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82631550: 39447088  addi r10, r4, 0x7088
	ctx.r[10].s64 = ctx.r[4].s64 + 28808;
	// 82631554: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82631558: 99630022  stb r11, 0x22(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(34 as u32), ctx.r[11].u8 ) };
	// 8263155C: 99630023  stb r11, 0x23(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(35 as u32), ctx.r[11].u8 ) };
	// 82631560: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82631564: 99630025  stb r11, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82631568: 99630026  stb r11, 0x26(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(38 as u32), ctx.r[11].u8 ) };
	// 8263156C: 99630027  stb r11, 0x27(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(39 as u32), ctx.r[11].u8 ) };
	// 82631570: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82631574: 98A30029  stb r5, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[5].u8 ) };
	// 82631578: 9963002A  stb r11, 0x2a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(42 as u32), ctx.r[11].u8 ) };
	// 8263157C: 9963002B  stb r11, 0x2b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(43 as u32), ctx.r[11].u8 ) };
	// 82631580: 9963002C  stb r11, 0x2c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 82631584: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82631588: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8263158C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82631590: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82631594: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82631598: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8263159C: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 826315A0: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 826315A4: 4082FFE8  bne 0x8263158c
	if !ctx.cr[0].eq {
	pc = 0x8263158C; continue 'dispatch;
	}
	// 826315A8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 826315AC: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 826315B0: 9123003C  stw r9, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 826315B4: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 826315B8: 38E80E0C  addi r7, r8, 0xe0c
	ctx.r[7].s64 = ctx.r[8].s64 + 3596;
	// 826315BC: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 826315C0: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 826315C4: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826315C8: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826315CC: 91630058  stw r11, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 826315D0: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 826315D4: 90E3005C  stw r7, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826315D8: 91630068  stw r11, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826315DC: 9163006C  stw r11, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826315E0: 80C30004  lwz r6, 4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 826315E4: 88A60090  lbz r5, 0x90(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(144 as u32) ) } as u64;
	// 826315E8: 60A40001  ori r4, r5, 1
	ctx.r[4].u64 = ctx.r[5].u64 | 1;
	// 826315EC: 98860090  stb r4, 0x90(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(144 as u32), ctx.r[4].u8 ) };
	// 826315F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826315F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826315F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826315FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631600: 4E800020  blr
	return;
	// 82631604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263160C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631620 size=88
    let mut pc: u32 = 0x82631620;
    'dispatch: loop {
        match pc {
            0x82631620 => {
    //   block [0x82631620..0x82631678)
	// 82631620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263162C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631634: 3860018C  li r3, 0x18c
	ctx.r[3].s64 = 396;
	// 82631638: 4BBEDC21  bl 0x8221f258
	ctx.lr = 0x8263163C;
	sub_8221F258(ctx, base);
	// 8263163C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631640: 419A0020  beq cr6, 0x82631660
	if ctx.cr[6].eq {
	pc = 0x82631660; continue 'dispatch;
	}
	// 82631644: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82631648: 4BDAEAA1  bl 0x823e00e8
	ctx.lr = 0x8263164C;
	sub_823E00E8(ctx, base);
	// 8263164C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263165C: 4E800020  blr
	return;
	// 82631660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263166C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631678 size=88
    let mut pc: u32 = 0x82631678;
    'dispatch: loop {
        match pc {
            0x82631678 => {
    //   block [0x82631678..0x826316D0)
	// 82631678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263167C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263168C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82631690: 4BBEDBC9  bl 0x8221f258
	ctx.lr = 0x82631694;
	sub_8221F258(ctx, base);
	// 82631694: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631698: 419A0020  beq cr6, 0x826316b8
	if ctx.cr[6].eq {
	pc = 0x826316B8; continue 'dispatch;
	}
	// 8263169C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826316A0: 48325119  bl 0x829567b8
	ctx.lr = 0x826316A4;
	sub_829567B8(ctx, base);
	// 826316A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826316A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826316AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826316B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826316B4: 4E800020  blr
	return;
	// 826316B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826316BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826316C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826316C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826316C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826316CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826316D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826316D0 size=88
    let mut pc: u32 = 0x826316D0;
    'dispatch: loop {
        match pc {
            0x826316D0 => {
    //   block [0x826316D0..0x82631728)
	// 826316D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826316D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826316D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826316DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826316E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826316E4: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 826316E8: 4BBEDB71  bl 0x8221f258
	ctx.lr = 0x826316EC;
	sub_8221F258(ctx, base);
	// 826316EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826316F0: 419A0020  beq cr6, 0x82631710
	if ctx.cr[6].eq {
	pc = 0x82631710; continue 'dispatch;
	}
	// 826316F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826316F8: 48326C31  bl 0x82958328
	ctx.lr = 0x826316FC;
	sub_82958328(ctx, base);
	// 826316FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263170C: 4E800020  blr
	return;
	// 82631710: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263171C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631728 size=88
    let mut pc: u32 = 0x82631728;
    'dispatch: loop {
        match pc {
            0x82631728 => {
    //   block [0x82631728..0x82631780)
	// 82631728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263172C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631730: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631734: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263173C: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82631740: 4BBEDB19  bl 0x8221f258
	ctx.lr = 0x82631744;
	sub_8221F258(ctx, base);
	// 82631744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631748: 419A0020  beq cr6, 0x82631768
	if ctx.cr[6].eq {
	pc = 0x82631768; continue 'dispatch;
	}
	// 8263174C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82631750: 482111D9  bl 0x82842928
	ctx.lr = 0x82631754;
	sub_82842928(ctx, base);
	// 82631754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263175C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631764: 4E800020  blr
	return;
	// 82631768: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263176C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631780 size=88
    let mut pc: u32 = 0x82631780;
    'dispatch: loop {
        match pc {
            0x82631780 => {
    //   block [0x82631780..0x826317D8)
	// 82631780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263178C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631794: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 82631798: 4BBEDAC1  bl 0x8221f258
	ctx.lr = 0x8263179C;
	sub_8221F258(ctx, base);
	// 8263179C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826317A0: 419A0020  beq cr6, 0x826317c0
	if ctx.cr[6].eq {
	pc = 0x826317C0; continue 'dispatch;
	}
	// 826317A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826317A8: 4BDF60D1  bl 0x82427878
	ctx.lr = 0x826317AC;
	sub_82427878(ctx, base);
	// 826317AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826317B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826317B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826317B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826317BC: 4E800020  blr
	return;
	// 826317C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826317C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826317C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826317CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826317D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826317D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826317D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826317D8 size=216
    let mut pc: u32 = 0x826317D8;
    'dispatch: loop {
        match pc {
            0x826317D8 => {
    //   block [0x826317D8..0x826318B0)
	// 826317D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826317DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826317E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826317E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826317E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826317EC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 826317F0: 4BBEDA69  bl 0x8221f258
	ctx.lr = 0x826317F4;
	sub_8221F258(ctx, base);
	// 826317F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826317F8: 419A00A0  beq cr6, 0x82631898
	if ctx.cr[6].eq {
	pc = 0x82631898; continue 'dispatch;
	}
	// 826317FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631800: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631804: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 82631808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263180C: 38CA4D28  addi r6, r10, 0x4d28
	ctx.r[6].s64 = ctx.r[10].s64 + 19752;
	// 82631810: 38A87088  addi r5, r8, 0x7088
	ctx.r[5].s64 = ctx.r[8].s64 + 28808;
	// 82631814: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631818: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8263181C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82631820: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82631824: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82631828: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8263182C: 7D202028  lwarx r9, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82631830: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82631834: 7D20212D  stwcx. r9, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82631838: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8263183C: 4082FFE8  bne 0x82631824
	if !ctx.cr[0].eq {
	pc = 0x82631824; continue 'dispatch;
	}
	// 82631840: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631844: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82631848: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8263184C: 7D402828  lwarx r10, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82631850: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82631854: 7D40292D  stwcx. r10, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82631858: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8263185C: 4082FFE8  bne 0x82631844
	if !ctx.cr[0].eq {
	pc = 0x82631844; continue 'dispatch;
	}
	// 82631860: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82631864: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82631868: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263186C: 9963001A  stb r11, 0x1a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[11].u8 ) };
	// 82631870: 9963001B  stb r11, 0x1b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(27 as u32), ctx.r[11].u8 ) };
	// 82631874: 98E30019  stb r7, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[7].u8 ) };
	// 82631878: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8263187C: C0089490  lfs f0, -0x6b70(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631880: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82631884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263188C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631894: 4E800020  blr
	return;
	// 82631898: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263189C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826318A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826318A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826318A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826318AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826318B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826318B0 size=88
    let mut pc: u32 = 0x826318B0;
    'dispatch: loop {
        match pc {
            0x826318B0 => {
    //   block [0x826318B0..0x82631908)
	// 826318B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826318B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826318B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826318BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826318C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826318C4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 826318C8: 4BBED991  bl 0x8221f258
	ctx.lr = 0x826318CC;
	sub_8221F258(ctx, base);
	// 826318CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826318D0: 419A0020  beq cr6, 0x826318f0
	if ctx.cr[6].eq {
	pc = 0x826318F0; continue 'dispatch;
	}
	// 826318D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826318D8: 48209259  bl 0x8283ab30
	ctx.lr = 0x826318DC;
	sub_8283AB30(ctx, base);
	// 826318DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826318E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826318E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826318E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826318EC: 4E800020  blr
	return;
	// 826318F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826318F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826318F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826318FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631908 size=136
    let mut pc: u32 = 0x82631908;
    'dispatch: loop {
        match pc {
            0x82631908 => {
    //   block [0x82631908..0x82631990)
	// 82631908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263191C: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82631920: 4BBED939  bl 0x8221f258
	ctx.lr = 0x82631924;
	sub_8221F258(ctx, base);
	// 82631924: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631928: 419A0050  beq cr6, 0x82631978
	if ctx.cr[6].eq {
	pc = 0x82631978; continue 'dispatch;
	}
	// 8263192C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631930: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631934: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82631938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263193C: 390A3C38  addi r8, r10, 0x3c38
	ctx.r[8].s64 = ctx.r[10].s64 + 15416;
	// 82631940: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 82631944: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631948: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263194C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631950: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82631954: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82631958: 90E3001C  stw r7, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 8263195C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82631960: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82631964: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263196C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631970: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631974: 4E800020  blr
	return;
	// 82631978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263197C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263198C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631990 size=156
    let mut pc: u32 = 0x82631990;
    'dispatch: loop {
        match pc {
            0x82631990 => {
    //   block [0x82631990..0x82631A2C)
	// 82631990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631994: 48677A79  bl 0x82ca940c
	ctx.lr = 0x82631998;
	sub_82CA93D0(ctx, base);
	// 82631998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263199C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826319A0: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 826319A4: 4BBED8B5  bl 0x8221f258
	ctx.lr = 0x826319A8;
	sub_8221F258(ctx, base);
	// 826319A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826319AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826319B0: 419A0070  beq cr6, 0x82631a20
	if ctx.cr[6].eq {
	pc = 0x82631A20; continue 'dispatch;
	}
	// 826319B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 826319B8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 826319BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826319C0: 394B40F8  addi r10, r11, 0x40f8
	ctx.r[10].s64 = ctx.r[11].s64 + 16632;
	// 826319C4: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826319C8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 826319CC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826319D0: 4BBED889  bl 0x8221f258
	ctx.lr = 0x826319D4;
	sub_8221F258(ctx, base);
	// 826319D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826319D8: 419A0008  beq cr6, 0x826319e0
	if ctx.cr[6].eq {
	pc = 0x826319E0; continue 'dispatch;
	}
	// 826319DC: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826319E0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 826319E4: 41820008  beq 0x826319ec
	if ctx.cr[0].eq {
	pc = 0x826319EC; continue 'dispatch;
	}
	// 826319E8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826319EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 826319F0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 826319F4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826319F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826319FC: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82631A00: 9BDF0019  stb r30, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[30].u8 ) };
	// 82631A04: 9BDF001A  stb r30, 0x1a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(26 as u32), ctx.r[30].u8 ) };
	// 82631A08: 997F001B  stb r11, 0x1b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(27 as u32), ctx.r[11].u8 ) };
	// 82631A0C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82631A10: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82631A14: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82631A18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82631A1C: 48677A40  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82631A20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631A24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82631A28: 48677A34  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631A30 size=132
    let mut pc: u32 = 0x82631A30;
    'dispatch: loop {
        match pc {
            0x82631A30 => {
    //   block [0x82631A30..0x82631AB4)
	// 82631A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631A38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631A3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631A40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631A44: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82631A48: 4BBED811  bl 0x8221f258
	ctx.lr = 0x82631A4C;
	sub_8221F258(ctx, base);
	// 82631A4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631A50: 419A004C  beq cr6, 0x82631a9c
	if ctx.cr[6].eq {
	pc = 0x82631A9C; continue 'dispatch;
	}
	// 82631A54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631A58: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631A5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631A60: 392A3A38  addi r9, r10, 0x3a38
	ctx.r[9].s64 = ctx.r[10].s64 + 14904;
	// 82631A64: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631A68: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82631A6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82631A70: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82631A74: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631A78: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82631A7C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82631A80: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82631A84: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82631A88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631A94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631A98: 4E800020  blr
	return;
	// 82631A9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82631AB8 size=128
    let mut pc: u32 = 0x82631AB8;
    'dispatch: loop {
        match pc {
            0x82631AB8 => {
    //   block [0x82631AB8..0x82631B38)
	// 82631AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631AC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631AC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631AC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631ACC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82631AD0: 4BBED789  bl 0x8221f258
	ctx.lr = 0x82631AD4;
	sub_8221F258(ctx, base);
	// 82631AD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631AD8: 419A0048  beq cr6, 0x82631b20
	if ctx.cr[6].eq {
	pc = 0x82631B20; continue 'dispatch;
	}
	// 82631ADC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82631AE0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631AE4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82631AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631AEC: 39093128  addi r8, r9, 0x3128
	ctx.r[8].s64 = ctx.r[9].s64 + 12584;
	// 82631AF0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82631AF4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631AF8: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631AFC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82631B00: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82631B04: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82631B08: 98E30014  stb r7, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u8 ) };
	// 82631B0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631B1C: 4E800020  blr
	return;
	// 82631B20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631B24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631B30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82631B38 size=176
    let mut pc: u32 = 0x82631B38;
    'dispatch: loop {
        match pc {
            0x82631B38 => {
    //   block [0x82631B38..0x82631BE8)
	// 82631B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631B40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82631B44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631B48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631B4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82631B50: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 82631B54: 4BBED705  bl 0x8221f258
	ctx.lr = 0x82631B58;
	sub_8221F258(ctx, base);
	// 82631B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631B5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82631B60: 419A006C  beq cr6, 0x82631bcc
	if ctx.cr[6].eq {
	pc = 0x82631BCC; continue 'dispatch;
	}
	// 82631B64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631B68: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82631B6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631B70: 392A4300  addi r9, r10, 0x4300
	ctx.r[9].s64 = ctx.r[10].s64 + 17152;
	// 82631B74: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82631B78: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631B7C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82631B80: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82631B84: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631B88: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82631B8C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82631B90: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 82631B94: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631B98: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82631B9C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82631BA0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82631BA4: 997F0031  stb r11, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[11].u8 ) };
	// 82631BA8: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82631BAC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82631BB0: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82631BB4: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82631BB8: 997F003C  stb r11, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 82631BBC: 997F003D  stb r11, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 82631BC0: 4BE4E679  bl 0x82480238
	ctx.lr = 0x82631BC4;
	sub_82480238(ctx, base);
	// 82631BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82631BC8: 48000008  b 0x82631bd0
	pc = 0x82631BD0; continue 'dispatch;
	// 82631BCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631BD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82631BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631BDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82631BE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82631BE8 size=228
    let mut pc: u32 = 0x82631BE8;
    'dispatch: loop {
        match pc {
            0x82631BE8 => {
    //   block [0x82631BE8..0x82631CCC)
	// 82631BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631BF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631BF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631BF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631BFC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82631C00: 4BBED659  bl 0x8221f258
	ctx.lr = 0x82631C04;
	sub_8221F258(ctx, base);
	// 82631C04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631C08: 419A00AC  beq cr6, 0x82631cb4
	if ctx.cr[6].eq {
	pc = 0x82631CB4; continue 'dispatch;
	}
	// 82631C0C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631C10: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631C14: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82631C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631C1C: 38CA3CF8  addi r6, r10, 0x3cf8
	ctx.r[6].s64 = ctx.r[10].s64 + 15608;
	// 82631C20: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
	// 82631C24: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631C28: 38880B7C  addi r4, r8, 0xb7c
	ctx.r[4].s64 = ctx.r[8].s64 + 2940;
	// 82631C2C: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82631C30: 39457088  addi r10, r5, 0x7088
	ctx.r[10].s64 = ctx.r[5].s64 + 28808;
	// 82631C34: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631C38: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82631C3C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82631C40: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82631C44: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82631C48: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82631C4C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82631C50: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82631C54: 7D204028  lwarx r9, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82631C58: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82631C5C: 7D20412D  stwcx. r9, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82631C60: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82631C64: 4082FFE8  bne 0x82631c4c
	if !ctx.cr[0].eq {
	pc = 0x82631C4C; continue 'dispatch;
	}
	// 82631C68: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82631C6C: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 82631C70: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82631C74: 7CE05028  lwarx r7, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 82631C78: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82631C7C: 7CE0512D  stwcx. r7, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82631C80: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82631C84: 4082FFE8  bne 0x82631c6c
	if !ctx.cr[0].eq {
	pc = 0x82631C6C; continue 'dispatch;
	}
	// 82631C88: 3CA0820A  lis r5, -0x7df6
	ctx.r[5].s64 = -2113273856;
	// 82631C8C: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82631C90: 99630025  stb r11, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82631C94: C0059484  lfs f0, -0x6b7c(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631C98: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82631C9C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82631CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631CAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631CB0: 4E800020  blr
	return;
	// 82631CB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631CC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631CD0 size=124
    let mut pc: u32 = 0x82631CD0;
    'dispatch: loop {
        match pc {
            0x82631CD0 => {
    //   block [0x82631CD0..0x82631D4C)
	// 82631CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631CD4: 48677739  bl 0x82ca940c
	ctx.lr = 0x82631CD8;
	sub_82CA93D0(ctx, base);
	// 82631CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631CDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82631CE0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82631CE4: 4BBED575  bl 0x8221f258
	ctx.lr = 0x82631CE8;
	sub_8221F258(ctx, base);
	// 82631CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631CEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82631CF0: 419A0050  beq cr6, 0x82631d40
	if ctx.cr[6].eq {
	pc = 0x82631D40; continue 'dispatch;
	}
	// 82631CF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82631CF8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82631CFC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82631D00: 394B36B0  addi r10, r11, 0x36b0
	ctx.r[10].s64 = ctx.r[11].s64 + 14000;
	// 82631D04: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82631D08: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82631D0C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82631D10: 4BBED549  bl 0x8221f258
	ctx.lr = 0x82631D14;
	sub_8221F258(ctx, base);
	// 82631D14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631D18: 419A0008  beq cr6, 0x82631d20
	if ctx.cr[6].eq {
	pc = 0x82631D20; continue 'dispatch;
	}
	// 82631D1C: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82631D20: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82631D24: 41820008  beq 0x82631d2c
	if ctx.cr[0].eq {
	pc = 0x82631D2C; continue 'dispatch;
	}
	// 82631D28: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82631D2C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82631D30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82631D34: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82631D38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82631D3C: 48677720  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82631D40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631D44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82631D48: 48677714  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631D50 size=88
    let mut pc: u32 = 0x82631D50;
    'dispatch: loop {
        match pc {
            0x82631D50 => {
    //   block [0x82631D50..0x82631DA8)
	// 82631D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631D5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631D64: 386002A0  li r3, 0x2a0
	ctx.r[3].s64 = 672;
	// 82631D68: 4BBED4F1  bl 0x8221f258
	ctx.lr = 0x82631D6C;
	sub_8221F258(ctx, base);
	// 82631D6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631D70: 419A0020  beq cr6, 0x82631d90
	if ctx.cr[6].eq {
	pc = 0x82631D90; continue 'dispatch;
	}
	// 82631D74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82631D78: 4BF30F51  bl 0x82562cc8
	ctx.lr = 0x82631D7C;
	sub_82562CC8(ctx, base);
	// 82631D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631D8C: 4E800020  blr
	return;
	// 82631D90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631D94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631DA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631DA8 size=88
    let mut pc: u32 = 0x82631DA8;
    'dispatch: loop {
        match pc {
            0x82631DA8 => {
    //   block [0x82631DA8..0x82631E00)
	// 82631DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631DB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631DB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631DB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631DBC: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 82631DC0: 4BBED499  bl 0x8221f258
	ctx.lr = 0x82631DC4;
	sub_8221F258(ctx, base);
	// 82631DC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631DC8: 419A0020  beq cr6, 0x82631de8
	if ctx.cr[6].eq {
	pc = 0x82631DE8; continue 'dispatch;
	}
	// 82631DCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82631DD0: 4BDECB41  bl 0x8241e910
	ctx.lr = 0x82631DD4;
	sub_8241E910(ctx, base);
	// 82631DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631DE4: 4E800020  blr
	return;
	// 82631DE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631DEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631DF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82631E00 size=176
    let mut pc: u32 = 0x82631E00;
    'dispatch: loop {
        match pc {
            0x82631E00 => {
    //   block [0x82631E00..0x82631EB0)
	// 82631E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631E08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631E10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631E14: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82631E18: 4BBED441  bl 0x8221f258
	ctx.lr = 0x82631E1C;
	sub_8221F258(ctx, base);
	// 82631E1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631E20: 419A0078  beq cr6, 0x82631e98
	if ctx.cr[6].eq {
	pc = 0x82631E98; continue 'dispatch;
	}
	// 82631E24: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82631E28: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631E2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631E30: 392A6DD8  addi r9, r10, 0x6dd8
	ctx.r[9].s64 = ctx.r[10].s64 + 28120;
	// 82631E34: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82631E38: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631E3C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82631E40: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82631E44: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82631E48: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82631E4C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82631E50: 38A70B7C  addi r5, r7, 0xb7c
	ctx.r[5].s64 = ctx.r[7].s64 + 2940;
	// 82631E54: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82631E58: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82631E5C: C0089490  lfs f0, -0x6b70(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631E60: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82631E64: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82631E68: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82631E6C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82631E70: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82631E74: 99630029  stb r11, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 82631E78: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82631E7C: 90A30030  stw r5, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 82631E80: 90C30038  stw r6, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[6].u32 ) };
	// 82631E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631E90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631E94: 4E800020  blr
	return;
	// 82631E98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631EA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631EB0 size=88
    let mut pc: u32 = 0x82631EB0;
    'dispatch: loop {
        match pc {
            0x82631EB0 => {
    //   block [0x82631EB0..0x82631F08)
	// 82631EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631EB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631EBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631EC4: 38600150  li r3, 0x150
	ctx.r[3].s64 = 336;
	// 82631EC8: 4BBED391  bl 0x8221f258
	ctx.lr = 0x82631ECC;
	sub_8221F258(ctx, base);
	// 82631ECC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631ED0: 419A0020  beq cr6, 0x82631ef0
	if ctx.cr[6].eq {
	pc = 0x82631EF0; continue 'dispatch;
	}
	// 82631ED4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82631ED8: 4BE83301  bl 0x824b51d8
	ctx.lr = 0x82631EDC;
	sub_824B51D8(ctx, base);
	// 82631EDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631EE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631EEC: 4E800020  blr
	return;
	// 82631EF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631EF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631F00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82631F08 size=252
    let mut pc: u32 = 0x82631F08;
    'dispatch: loop {
        match pc {
            0x82631F08 => {
    //   block [0x82631F08..0x82632004)
	// 82631F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82631F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82631F1C: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 82631F20: 4BBED339  bl 0x8221f258
	ctx.lr = 0x82631F24;
	sub_8221F258(ctx, base);
	// 82631F24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82631F28: 419A00C4  beq cr6, 0x82631fec
	if ctx.cr[6].eq {
	pc = 0x82631FEC; continue 'dispatch;
	}
	// 82631F2C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82631F30: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82631F34: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 82631F38: 38CAD5C8  addi r6, r10, -0x2a38
	ctx.r[6].s64 = ctx.r[10].s64 + -10808;
	// 82631F3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631F40: 38A88C40  addi r5, r8, -0x73c0
	ctx.r[5].s64 = ctx.r[8].s64 + -29632;
	// 82631F44: 3C808349  lis r4, -0x7cb7
	ctx.r[4].s64 = -2092367872;
	// 82631F48: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82631F4C: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82631F50: C006BD0C  lfs f0, -0x42f4(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-17140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631F54: 39047088  addi r8, r4, 0x7088
	ctx.r[8].s64 = ctx.r[4].s64 + 28808;
	// 82631F58: C1A6E168  lfs f13, -0x1e98(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-7832 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82631F5C: C186DEA4  lfs f12, -0x215c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8540 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82631F60: C166E0F8  lfs f11, -0x1f08(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-7944 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82631F64: C146F6CC  lfs f10, -0x934(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-2356 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82631F68: C126FFE4  lfs f9, -0x1c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-28 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82631F6C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82631F70: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82631F74: D1830014  stfs f12, 0x14(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82631F78: D1630018  stfs f11, 0x18(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82631F7C: D143001C  stfs f10, 0x1c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82631F80: D1230020  stfs f9, 0x20(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82631F84: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82631F88: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82631F8C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82631F90: 7D204028  lwarx r9, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82631F94: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82631F98: 7D20412D  stwcx. r9, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82631F9C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82631FA0: 4082FFE8  bne 0x82631f88
	if !ctx.cr[0].eq {
	pc = 0x82631F88; continue 'dispatch;
	}
	// 82631FA4: C006BEBC  lfs f0, -0x4144(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-16708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82631FA8: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 82631FAC: C1A6BEC8  lfs f13, -0x4138(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-16696 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82631FB0: C18AD5C8  lfs f12, -0x2a38(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10808 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82631FB4: C1660B1C  lfs f11, 0xb1c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(2844 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82631FB8: D1A30028  stfs f13, 0x28(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82631FBC: D163002C  stfs f11, 0x2c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82631FC0: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82631FC4: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82631FC8: D1A3003C  stfs f13, 0x3c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82631FCC: D1830040  stfs f12, 0x40(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82631FD0: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82631FD4: D1830048  stfs f12, 0x48(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82631FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82631FE8: 4E800020  blr
	return;
	// 82631FEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82631FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82631FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631FFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82632008 size=124
    let mut pc: u32 = 0x82632008;
    'dispatch: loop {
        match pc {
            0x82632008 => {
    //   block [0x82632008..0x82632084)
	// 82632008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263200C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263201C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82632020: 4BBED239  bl 0x8221f258
	ctx.lr = 0x82632024;
	sub_8221F258(ctx, base);
	// 82632024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632028: 419A0044  beq cr6, 0x8263206c
	if ctx.cr[6].eq {
	pc = 0x8263206C; continue 'dispatch;
	}
	// 8263202C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82632030: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82632034: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82632038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263203C: 39094640  addi r8, r9, 0x4640
	ctx.r[8].s64 = ctx.r[9].s64 + 17984;
	// 82632040: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82632044: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82632048: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263204C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82632050: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82632054: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82632058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632068: 4E800020  blr
	return;
	// 8263206C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263207C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632088 size=104
    let mut pc: u32 = 0x82632088;
    'dispatch: loop {
        match pc {
            0x82632088 => {
    //   block [0x82632088..0x826320F0)
	// 82632088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263208C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632090: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263209C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 826320A0: 4BBED1B9  bl 0x8221f258
	ctx.lr = 0x826320A4;
	sub_8221F258(ctx, base);
	// 826320A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826320A8: 419A0030  beq cr6, 0x826320d8
	if ctx.cr[6].eq {
	pc = 0x826320D8; continue 'dispatch;
	}
	// 826320AC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 826320B0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826320B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826320B8: 392A8CE8  addi r9, r10, -0x7318
	ctx.r[9].s64 = ctx.r[10].s64 + -29464;
	// 826320BC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826320C0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826320C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826320C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826320CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826320D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826320D4: 4E800020  blr
	return;
	// 826320D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826320DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826320E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826320E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826320E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826320EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826320F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826320F0 size=88
    let mut pc: u32 = 0x826320F0;
    'dispatch: loop {
        match pc {
            0x826320F0 => {
    //   block [0x826320F0..0x82632148)
	// 826320F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826320F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826320F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826320FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632104: 38600320  li r3, 0x320
	ctx.r[3].s64 = 800;
	// 82632108: 4BBED151  bl 0x8221f258
	ctx.lr = 0x8263210C;
	sub_8221F258(ctx, base);
	// 8263210C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632110: 419A0020  beq cr6, 0x82632130
	if ctx.cr[6].eq {
	pc = 0x82632130; continue 'dispatch;
	}
	// 82632114: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632118: 4BD5F9B1  bl 0x82391ac8
	ctx.lr = 0x8263211C;
	sub_82391AC8(ctx, base);
	// 8263211C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263212C: 4E800020  blr
	return;
	// 82632130: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632134: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263213C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632140: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632148 size=88
    let mut pc: u32 = 0x82632148;
    'dispatch: loop {
        match pc {
            0x82632148 => {
    //   block [0x82632148..0x826321A0)
	// 82632148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263214C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632154: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632158: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263215C: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82632160: 4BBED0F9  bl 0x8221f258
	ctx.lr = 0x82632164;
	sub_8221F258(ctx, base);
	// 82632164: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632168: 419A0020  beq cr6, 0x82632188
	if ctx.cr[6].eq {
	pc = 0x82632188; continue 'dispatch;
	}
	// 8263216C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632170: 4BF92991  bl 0x825c4b00
	ctx.lr = 0x82632174;
	sub_825C4B00(ctx, base);
	// 82632174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263217C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632184: 4E800020  blr
	return;
	// 82632188: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263218C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263219C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826321A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826321A0 size=88
    let mut pc: u32 = 0x826321A0;
    'dispatch: loop {
        match pc {
            0x826321A0 => {
    //   block [0x826321A0..0x826321F8)
	// 826321A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826321A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826321A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826321AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826321B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826321B4: 38600150  li r3, 0x150
	ctx.r[3].s64 = 336;
	// 826321B8: 4BBED0A1  bl 0x8221f258
	ctx.lr = 0x826321BC;
	sub_8221F258(ctx, base);
	// 826321BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826321C0: 419A0020  beq cr6, 0x826321e0
	if ctx.cr[6].eq {
	pc = 0x826321E0; continue 'dispatch;
	}
	// 826321C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826321C8: 4BE985B9  bl 0x824ca780
	ctx.lr = 0x826321CC;
	sub_824CA780(ctx, base);
	// 826321CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826321D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826321D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826321D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826321DC: 4E800020  blr
	return;
	// 826321E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826321E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826321E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826321EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826321F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826321F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826321F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826321F8 size=88
    let mut pc: u32 = 0x826321F8;
    'dispatch: loop {
        match pc {
            0x826321F8 => {
    //   block [0x826321F8..0x82632250)
	// 826321F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826321FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632208: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263220C: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82632210: 4BBED049  bl 0x8221f258
	ctx.lr = 0x82632214;
	sub_8221F258(ctx, base);
	// 82632214: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632218: 419A0020  beq cr6, 0x82632238
	if ctx.cr[6].eq {
	pc = 0x82632238; continue 'dispatch;
	}
	// 8263221C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632220: 4BF376E1  bl 0x82569900
	ctx.lr = 0x82632224;
	sub_82569900(ctx, base);
	// 82632224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263222C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632230: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632234: 4E800020  blr
	return;
	// 82632238: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263223C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263224C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632250 size=156
    let mut pc: u32 = 0x82632250;
    'dispatch: loop {
        match pc {
            0x82632250 => {
    //   block [0x82632250..0x826322EC)
	// 82632250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632254: 486771B9  bl 0x82ca940c
	ctx.lr = 0x82632258;
	sub_82CA93D0(ctx, base);
	// 82632258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263225C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82632260: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82632264: 4BBECFF5  bl 0x8221f258
	ctx.lr = 0x82632268;
	sub_8221F258(ctx, base);
	// 82632268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263226C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632270: 419A0070  beq cr6, 0x826322e0
	if ctx.cr[6].eq {
	pc = 0x826322E0; continue 'dispatch;
	}
	// 82632274: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632278: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263227C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82632280: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82632284: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82632288: 390B1848  addi r8, r11, 0x1848
	ctx.r[8].s64 = ctx.r[11].s64 + 6216;
	// 8263228C: 38EA3AF8  addi r7, r10, 0x3af8
	ctx.r[7].s64 = ctx.r[10].s64 + 15096;
	// 82632290: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82632294: 38C9173C  addi r6, r9, 0x173c
	ctx.r[6].s64 = ctx.r[9].s64 + 5948;
	// 82632298: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8263229C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 826322A0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 826322A4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 826322A8: 48215D09  bl 0x82847fb0
	ctx.lr = 0x826322AC;
	sub_82847FB0(ctx, base);
	// 826322AC: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 826322B0: 48215D01  bl 0x82847fb0
	ctx.lr = 0x826322B4;
	sub_82847FB0(ctx, base);
	// 826322B4: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 826322B8: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 826322BC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 826322C0: 39650E0C  addi r11, r5, 0xe0c
	ctx.r[11].s64 = ctx.r[5].s64 + 3596;
	// 826322C4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 826322C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826322CC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 826322D0: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 826322D4: 909F003C  stw r4, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 826322D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826322DC: 48677180  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 826322E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826322E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826322E8: 48677174  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826322F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826322F0 size=132
    let mut pc: u32 = 0x826322F0;
    'dispatch: loop {
        match pc {
            0x826322F0 => {
    //   block [0x826322F0..0x82632374)
	// 826322F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826322F4: 48677119  bl 0x82ca940c
	ctx.lr = 0x826322F8;
	sub_82CA93D0(ctx, base);
	// 826322F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826322FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82632300: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82632304: 4BBECF55  bl 0x8221f258
	ctx.lr = 0x82632308;
	sub_8221F258(ctx, base);
	// 82632308: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263230C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632310: 419A0058  beq cr6, 0x82632368
	if ctx.cr[6].eq {
	pc = 0x82632368; continue 'dispatch;
	}
	// 82632314: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82632318: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263231C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82632320: 394B33A8  addi r10, r11, 0x33a8
	ctx.r[10].s64 = ctx.r[11].s64 + 13224;
	// 82632324: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82632328: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8263232C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82632330: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82632334: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82632338: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8263233C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82632340: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82632344: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82632348: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 8263234C: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82632350: 48215C61  bl 0x82847fb0
	ctx.lr = 0x82632354;
	sub_82847FB0(ctx, base);
	// 82632354: 9BDF0040  stb r30, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82632358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263235C: 9BDF0041  stb r30, 0x41(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(65 as u32), ctx.r[30].u8 ) };
	// 82632360: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632364: 486770F8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82632368: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263236C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632370: 486770EC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632378 size=88
    let mut pc: u32 = 0x82632378;
    'dispatch: loop {
        match pc {
            0x82632378 => {
    //   block [0x82632378..0x826323D0)
	// 82632378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263237C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632388: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263238C: 386008A0  li r3, 0x8a0
	ctx.r[3].s64 = 2208;
	// 82632390: 4BBECEC9  bl 0x8221f258
	ctx.lr = 0x82632394;
	sub_8221F258(ctx, base);
	// 82632394: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632398: 419A0020  beq cr6, 0x826323b8
	if ctx.cr[6].eq {
	pc = 0x826323B8; continue 'dispatch;
	}
	// 8263239C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826323A0: 4BEA3149  bl 0x824d54e8
	ctx.lr = 0x826323A4;
	sub_824D54E8(ctx, base);
	// 826323A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826323A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826323AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826323B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826323B4: 4E800020  blr
	return;
	// 826323B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826323BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826323C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826323C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826323C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826323CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826323D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826323D0 size=128
    let mut pc: u32 = 0x826323D0;
    'dispatch: loop {
        match pc {
            0x826323D0 => {
    //   block [0x826323D0..0x82632450)
	// 826323D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826323D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826323D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826323DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826323E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826323E4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 826323E8: 4BBECE71  bl 0x8221f258
	ctx.lr = 0x826323EC;
	sub_8221F258(ctx, base);
	// 826323EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826323F0: 419A0048  beq cr6, 0x82632438
	if ctx.cr[6].eq {
	pc = 0x82632438; continue 'dispatch;
	}
	// 826323F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826323F8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826323FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632400: 392A2EA8  addi r9, r10, 0x2ea8
	ctx.r[9].s64 = ctx.r[10].s64 + 11944;
	// 82632404: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632408: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263240C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632410: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82632414: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82632418: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8263241C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82632420: 9103001C  stw r8, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82632424: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263242C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632430: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632434: 4E800020  blr
	return;
	// 82632438: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263243C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263244C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82632450 size=172
    let mut pc: u32 = 0x82632450;
    'dispatch: loop {
        match pc {
            0x82632450 => {
    //   block [0x82632450..0x826324FC)
	// 82632450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632454: 48676FB9  bl 0x82ca940c
	ctx.lr = 0x82632458;
	sub_82CA93D0(ctx, base);
	// 82632458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263245C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82632460: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82632464: 4BBECDF5  bl 0x8221f258
	ctx.lr = 0x82632468;
	sub_8221F258(ctx, base);
	// 82632468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263246C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632470: 419A0080  beq cr6, 0x826324f0
	if ctx.cr[6].eq {
	pc = 0x826324F0; continue 'dispatch;
	}
	// 82632474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82632478: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263247C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82632480: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82632484: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82632488: 390A4880  addi r8, r10, 0x4880
	ctx.r[8].s64 = ctx.r[10].s64 + 18560;
	// 8263248C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82632490: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82632494: 38C90B7C  addi r6, r9, 0xb7c
	ctx.r[6].s64 = ctx.r[9].s64 + 2940;
	// 82632498: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8263249C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826324A0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 826324A4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826324A8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 826324AC: 9BDF0010  stb r30, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u8 ) };
	// 826324B0: 9BDF0011  stb r30, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[30].u8 ) };
	// 826324B4: 9BDF0012  stb r30, 0x12(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[30].u8 ) };
	// 826324B8: 9BDF0013  stb r30, 0x13(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(19 as u32), ctx.r[30].u8 ) };
	// 826324BC: 98FF0014  stb r7, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u8 ) };
	// 826324C0: 9BDF0015  stb r30, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[30].u8 ) };
	// 826324C4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 826324C8: 90DF001C  stw r6, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 826324CC: 4BB6196D  bl 0x82193e38
	ctx.lr = 0x826324D0;
	sub_82193E38(ctx, base);
	// 826324D0: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 826324D4: 48215ADD  bl 0x82847fb0
	ctx.lr = 0x826324D8;
	sub_82847FB0(ctx, base);
	// 826324D8: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 826324DC: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 826324E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826324E4: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 826324E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826324EC: 48676F70  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 826324F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826324F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826324F8: 48676F64  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632500 size=128
    let mut pc: u32 = 0x82632500;
    'dispatch: loop {
        match pc {
            0x82632500 => {
    //   block [0x82632500..0x82632580)
	// 82632500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263250C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632514: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82632518: 4BBECD41  bl 0x8221f258
	ctx.lr = 0x8263251C;
	sub_8221F258(ctx, base);
	// 8263251C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632520: 419A0048  beq cr6, 0x82632568
	if ctx.cr[6].eq {
	pc = 0x82632568; continue 'dispatch;
	}
	// 82632524: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82632528: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263252C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632530: 392A3FF8  addi r9, r10, 0x3ff8
	ctx.r[9].s64 = ctx.r[10].s64 + 16376;
	// 82632534: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82632538: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263253C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82632540: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82632544: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82632548: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8263254C: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82632550: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82632554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263255C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632564: 4E800020  blr
	return;
	// 82632568: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263256C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632578: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263257C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82632580 size=144
    let mut pc: u32 = 0x82632580;
    'dispatch: loop {
        match pc {
            0x82632580 => {
    //   block [0x82632580..0x82632610)
	// 82632580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263258C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82632598: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8263259C: 4BBECCBD  bl 0x8221f258
	ctx.lr = 0x826325A0;
	sub_8221F258(ctx, base);
	// 826325A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826325A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826325A8: 419A004C  beq cr6, 0x826325f4
	if ctx.cr[6].eq {
	pc = 0x826325F4; continue 'dispatch;
	}
	// 826325AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826325B0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 826325B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826325B8: 392A4138  addi r9, r10, 0x4138
	ctx.r[9].s64 = ctx.r[10].s64 + 16696;
	// 826325BC: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826325C0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 826325C4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826325C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826325CC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 826325D0: 38880CA0  addi r4, r8, 0xca0
	ctx.r[4].s64 = ctx.r[8].s64 + 3232;
	// 826325D4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 826325D8: 4BBFA8F9  bl 0x8222ced0
	ctx.lr = 0x826325DC;
	sub_8222CED0(ctx, base);
	// 826325DC: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 826325E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826325E4: C0079484  lfs f0, -0x6b7c(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826325E8: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 826325EC: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 826325F0: 48000008  b 0x826325f8
	pc = 0x826325F8; continue 'dispatch;
	// 826325F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826325F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826325FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632604: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82632608: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263260C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632610 size=112
    let mut pc: u32 = 0x82632610;
    'dispatch: loop {
        match pc {
            0x82632610 => {
    //   block [0x82632610..0x82632680)
	// 82632610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632618: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263261C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632624: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82632628: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8263262C: 4BBECC2D  bl 0x8221f258
	ctx.lr = 0x82632630;
	sub_8221F258(ctx, base);
	// 82632630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632634: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632638: 419A002C  beq cr6, 0x82632664
	if ctx.cr[6].eq {
	pc = 0x82632664; continue 'dispatch;
	}
	// 8263263C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82632640: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82632644: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632648: 392B4278  addi r9, r11, 0x4278
	ctx.r[9].s64 = ctx.r[11].s64 + 17016;
	// 8263264C: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82632650: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82632654: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632658: 48215959  bl 0x82847fb0
	ctx.lr = 0x8263265C;
	sub_82847FB0(ctx, base);
	// 8263265C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632660: 48000008  b 0x82632668
	pc = 0x82632668; continue 'dispatch;
	// 82632664: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632668: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263266C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632674: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82632678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263267C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632680 size=124
    let mut pc: u32 = 0x82632680;
    'dispatch: loop {
        match pc {
            0x82632680 => {
    //   block [0x82632680..0x826326FC)
	// 82632680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263268C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632694: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82632698: 4BBECBC1  bl 0x8221f258
	ctx.lr = 0x8263269C;
	sub_8221F258(ctx, base);
	// 8263269C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826326A0: 419A0044  beq cr6, 0x826326e4
	if ctx.cr[6].eq {
	pc = 0x826326E4; continue 'dispatch;
	}
	// 826326A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826326A8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826326AC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 826326B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826326B4: 390A44C0  addi r8, r10, 0x44c0
	ctx.r[8].s64 = ctx.r[10].s64 + 17600;
	// 826326B8: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 826326BC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826326C0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826326C4: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 826326C8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826326CC: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 826326D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826326D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826326D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826326DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826326E0: 4E800020  blr
	return;
	// 826326E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826326E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826326EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826326F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826326F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826326F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632700 size=132
    let mut pc: u32 = 0x82632700;
    'dispatch: loop {
        match pc {
            0x82632700 => {
    //   block [0x82632700..0x82632784)
	// 82632700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263270C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632714: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82632718: 4BBECB41  bl 0x8221f258
	ctx.lr = 0x8263271C;
	sub_8221F258(ctx, base);
	// 8263271C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632720: 419A004C  beq cr6, 0x8263276c
	if ctx.cr[6].eq {
	pc = 0x8263276C; continue 'dispatch;
	}
	// 82632724: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82632728: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263272C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632730: 392ADEB0  addi r9, r10, -0x2150
	ctx.r[9].s64 = ctx.r[10].s64 + -8528;
	// 82632734: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632738: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263273C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632740: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82632744: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82632748: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8263274C: 99030012  stb r8, 0x12(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[8].u8 ) };
	// 82632750: 99630013  stb r11, 0x13(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(19 as u32), ctx.r[11].u8 ) };
	// 82632754: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82632758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263275C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632768: 4E800020  blr
	return;
	// 8263276C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263277C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632788 size=132
    let mut pc: u32 = 0x82632788;
    'dispatch: loop {
        match pc {
            0x82632788 => {
    //   block [0x82632788..0x8263280C)
	// 82632788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263278C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263279C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 826327A0: 4BBECAB9  bl 0x8221f258
	ctx.lr = 0x826327A4;
	sub_8221F258(ctx, base);
	// 826327A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826327A8: 419A004C  beq cr6, 0x826327f4
	if ctx.cr[6].eq {
	pc = 0x826327F4; continue 'dispatch;
	}
	// 826327AC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826327B0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826327B4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 826327B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826327BC: 390A5C88  addi r8, r10, 0x5c88
	ctx.r[8].s64 = ctx.r[10].s64 + 23688;
	// 826327C0: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 826327C4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826327C8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826327CC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826327D0: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826327D4: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 826327D8: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 826327DC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826327E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826327E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826327E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826327EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826327F0: 4E800020  blr
	return;
	// 826327F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826327F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826327FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632810 size=156
    let mut pc: u32 = 0x82632810;
    'dispatch: loop {
        match pc {
            0x82632810 => {
    //   block [0x82632810..0x826328AC)
	// 82632810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263281C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82632828: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8263282C: 4BBECA2D  bl 0x8221f258
	ctx.lr = 0x82632830;
	sub_8221F258(ctx, base);
	// 82632830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632834: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632838: 419A0058  beq cr6, 0x82632890
	if ctx.cr[6].eq {
	pc = 0x82632890; continue 'dispatch;
	}
	// 8263283C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82632840: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82632844: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632848: 392A3268  addi r9, r10, 0x3268
	ctx.r[9].s64 = ctx.r[10].s64 + 12904;
	// 8263284C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632850: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82632854: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8263285C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82632860: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82632864: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82632868: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8263286C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82632870: 991F0024  stb r8, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[8].u8 ) };
	// 82632874: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82632878: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8263287C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82632880: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82632884: 4BD5AC35  bl 0x8238d4b8
	ctx.lr = 0x82632888;
	sub_8238D4B8(ctx, base);
	// 82632888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263288C: 48000008  b 0x82632894
	pc = 0x82632894; continue 'dispatch;
	// 82632890: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263289C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826328A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826328A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826328A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826328B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826328B0 size=88
    let mut pc: u32 = 0x826328B0;
    'dispatch: loop {
        match pc {
            0x826328B0 => {
    //   block [0x826328B0..0x82632908)
	// 826328B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826328B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826328B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826328BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826328C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826328C4: 38600810  li r3, 0x810
	ctx.r[3].s64 = 2064;
	// 826328C8: 4BBEC991  bl 0x8221f258
	ctx.lr = 0x826328CC;
	sub_8221F258(ctx, base);
	// 826328CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826328D0: 419A0020  beq cr6, 0x826328f0
	if ctx.cr[6].eq {
	pc = 0x826328F0; continue 'dispatch;
	}
	// 826328D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826328D8: 481D5721  bl 0x82807ff8
	ctx.lr = 0x826328DC;
	sub_82807FF8(ctx, base);
	// 826328DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826328E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826328E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826328E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826328EC: 4E800020  blr
	return;
	// 826328F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826328F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826328F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826328FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632908 size=88
    let mut pc: u32 = 0x82632908;
    'dispatch: loop {
        match pc {
            0x82632908 => {
    //   block [0x82632908..0x82632960)
	// 82632908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263291C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82632920: 4BBEC939  bl 0x8221f258
	ctx.lr = 0x82632924;
	sub_8221F258(ctx, base);
	// 82632924: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632928: 419A0020  beq cr6, 0x82632948
	if ctx.cr[6].eq {
	pc = 0x82632948; continue 'dispatch;
	}
	// 8263292C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632930: 48000071  bl 0x826329a0
	ctx.lr = 0x82632934;
	sub_826329A0(ctx, base);
	// 82632934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263293C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632944: 4E800020  blr
	return;
	// 82632948: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263294C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632958: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263295C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632960 size=60
    let mut pc: u32 = 0x82632960;
    'dispatch: loop {
        match pc {
            0x82632960 => {
    //   block [0x82632960..0x8263299C)
	// 82632960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263296C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632970: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632974: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82632978: 388B8680  addi r4, r11, -0x7980
	ctx.r[4].s64 = ctx.r[11].s64 + -31104;
	// 8263297C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632980: 4BBFA551  bl 0x8222ced0
	ctx.lr = 0x82632984;
	sub_8222CED0(ctx, base);
	// 82632984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263298C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826329A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826329A0 size=224
    let mut pc: u32 = 0x826329A0;
    'dispatch: loop {
        match pc {
            0x826329A0 => {
    //   block [0x826329A0..0x82632A80)
	// 826329A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826329A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826329A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826329AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826329B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826329B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826329B8: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 826329BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826329C0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 826329C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826329C8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 826329CC: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 826329D0: 38EB1848  addi r7, r11, 0x1848
	ctx.r[7].s64 = ctx.r[11].s64 + 6216;
	// 826329D4: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826329D8: 38CA4A80  addi r6, r10, 0x4a80
	ctx.r[6].s64 = ctx.r[10].s64 + 19072;
	// 826329DC: 38A91760  addi r5, r9, 0x1760
	ctx.r[5].s64 = ctx.r[9].s64 + 5984;
	// 826329E0: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 826329E4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 826329E8: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 826329EC: 39680E0C  addi r11, r8, 0xe0c
	ctx.r[11].s64 = ctx.r[8].s64 + 3596;
	// 826329F0: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 826329F4: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 826329F8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 826329FC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82632A00: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82632A04: 4BBEC855  bl 0x8221f258
	ctx.lr = 0x82632A08;
	sub_8221F258(ctx, base);
	// 82632A08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632A0C: 419A0008  beq cr6, 0x82632a14
	if ctx.cr[6].eq {
	pc = 0x82632A14; continue 'dispatch;
	}
	// 82632A10: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632A14: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82632A18: 41820008  beq 0x82632a20
	if ctx.cr[0].eq {
	pc = 0x82632A20; continue 'dispatch;
	}
	// 82632A1C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632A20: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82632A24: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82632A28: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82632A2C: 4BBEC82D  bl 0x8221f258
	ctx.lr = 0x82632A30;
	sub_8221F258(ctx, base);
	// 82632A30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632A34: 419A0008  beq cr6, 0x82632a3c
	if ctx.cr[6].eq {
	pc = 0x82632A3C; continue 'dispatch;
	}
	// 82632A38: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632A3C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82632A40: 41820008  beq 0x82632a48
	if ctx.cr[0].eq {
	pc = 0x82632A48; continue 'dispatch;
	}
	// 82632A44: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632A48: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82632A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632A50: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82632A54: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82632A58: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82632A5C: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82632A60: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82632A64: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82632A68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632A74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82632A78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632A80 size=156
    let mut pc: u32 = 0x82632A80;
    'dispatch: loop {
        match pc {
            0x82632A80 => {
    //   block [0x82632A80..0x82632B1C)
	// 82632A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632A84: 48676989  bl 0x82ca940c
	ctx.lr = 0x82632A88;
	sub_82CA93D0(ctx, base);
	// 82632A88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632A90: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82632A94: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82632A98: 4BB85081  bl 0x821b7b18
	ctx.lr = 0x82632A9C;
	sub_821B7B18(ctx, base);
	// 82632A9C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82632AA0: 4BDA9D89  bl 0x823dc828
	ctx.lr = 0x82632AA4;
	sub_823DC828(ctx, base);
	// 82632AA4: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82632AA8: 480FA891  bl 0x8272d338
	ctx.lr = 0x82632AAC;
	sub_8272D338(ctx, base);
	// 82632AAC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82632AB0: 4BBE9289  bl 0x8221bd38
	ctx.lr = 0x82632AB4;
	sub_8221BD38(ctx, base);
	// 82632AB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82632AB8: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82632ABC: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82632AC0: 480FA879  bl 0x8272d338
	ctx.lr = 0x82632AC4;
	sub_8272D338(ctx, base);
	// 82632AC4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82632AC8: 4BBE9271  bl 0x8221bd38
	ctx.lr = 0x82632ACC;
	sub_8221BD38(ctx, base);
	// 82632ACC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82632AD0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82632AD4: 4BC67D7D  bl 0x8229a850
	ctx.lr = 0x82632AD8;
	sub_8229A850(ctx, base);
	// 82632AD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632ADC: 419A0008  beq cr6, 0x82632ae4
	if ctx.cr[6].eq {
	pc = 0x82632AE4; continue 'dispatch;
	}
	// 82632AE0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82632AE4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632AE8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82632AEC: 392B1848  addi r9, r11, 0x1848
	ctx.r[9].s64 = ctx.r[11].s64 + 6216;
	// 82632AF0: 390A2850  addi r8, r10, 0x2850
	ctx.r[8].s64 = ctx.r[10].s64 + 10320;
	// 82632AF4: 57A707FE  clrlwi r7, r29, 0x1f
	ctx.r[7].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82632AF8: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632AFC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82632B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632B04: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82632B08: 419A000C  beq cr6, 0x82632b14
	if ctx.cr[6].eq {
	pc = 0x82632B14; continue 'dispatch;
	}
	// 82632B0C: 4BBE922D  bl 0x8221bd38
	ctx.lr = 0x82632B10;
	sub_8221BD38(ctx, base);
	// 82632B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632B14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632B18: 48676944  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632B20 size=188
    let mut pc: u32 = 0x82632B20;
    'dispatch: loop {
        match pc {
            0x82632B20 => {
    //   block [0x82632B20..0x82632BDC)
	// 82632B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632B24: 486768E9  bl 0x82ca940c
	ctx.lr = 0x82632B28;
	sub_82CA93D0(ctx, base);
	// 82632B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632B2C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82632B30: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82632B34: 4BBEC725  bl 0x8221f258
	ctx.lr = 0x82632B38;
	sub_8221F258(ctx, base);
	// 82632B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632B3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82632B40: 419A0090  beq cr6, 0x82632bd0
	if ctx.cr[6].eq {
	pc = 0x82632BD0; continue 'dispatch;
	}
	// 82632B44: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82632B48: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82632B4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82632B50: 394B3428  addi r10, r11, 0x3428
	ctx.r[10].s64 = ctx.r[11].s64 + 13352;
	// 82632B54: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82632B58: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82632B5C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82632B60: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82632B64: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82632B68: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82632B6C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82632B70: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82632B74: 9BDF0025  stb r30, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[30].u8 ) };
	// 82632B78: 4BBEC6E1  bl 0x8221f258
	ctx.lr = 0x82632B7C;
	sub_8221F258(ctx, base);
	// 82632B7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632B80: 419A0008  beq cr6, 0x82632b88
	if ctx.cr[6].eq {
	pc = 0x82632B88; continue 'dispatch;
	}
	// 82632B84: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632B88: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82632B8C: 41820008  beq 0x82632b94
	if ctx.cr[0].eq {
	pc = 0x82632B94; continue 'dispatch;
	}
	// 82632B90: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632B94: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82632B98: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82632B9C: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82632BA0: 4BBEC6B9  bl 0x8221f258
	ctx.lr = 0x82632BA4;
	sub_8221F258(ctx, base);
	// 82632BA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632BA8: 419A0008  beq cr6, 0x82632bb0
	if ctx.cr[6].eq {
	pc = 0x82632BB0; continue 'dispatch;
	}
	// 82632BAC: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632BB0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82632BB4: 41820008  beq 0x82632bbc
	if ctx.cr[0].eq {
	pc = 0x82632BBC; continue 'dispatch;
	}
	// 82632BB8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82632BBC: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82632BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632BC4: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82632BC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632BCC: 48676890  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82632BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632BD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632BD8: 48676884  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632BE0 size=60
    let mut pc: u32 = 0x82632BE0;
    'dispatch: loop {
        match pc {
            0x82632BE0 => {
    //   block [0x82632BE0..0x82632C1C)
	// 82632BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632BEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632BF0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82632BF8: 388B872C  addi r4, r11, -0x78d4
	ctx.r[4].s64 = ctx.r[11].s64 + -30932;
	// 82632BFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632C00: 4BBFA2D1  bl 0x8222ced0
	ctx.lr = 0x82632C04;
	sub_8222CED0(ctx, base);
	// 82632C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632C14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632C20 size=112
    let mut pc: u32 = 0x82632C20;
    'dispatch: loop {
        match pc {
            0x82632C20 => {
    //   block [0x82632C20..0x82632C90)
	// 82632C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82632C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632C38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82632C3C: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 82632C40: 48007A49  bl 0x8263a688
	ctx.lr = 0x82632C44;
	sub_8263A688(ctx, base);
	// 82632C44: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82632C48: 48007A41  bl 0x8263a688
	ctx.lr = 0x82632C4C;
	sub_8263A688(ctx, base);
	// 82632C4C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82632C50: 4BCE51F9  bl 0x82317e48
	ctx.lr = 0x82632C54;
	sub_82317E48(ctx, base);
	// 82632C54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82632C58: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82632C5C: 392B2850  addi r9, r11, 0x2850
	ctx.r[9].s64 = ctx.r[11].s64 + 10320;
	// 82632C60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82632C64: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632C6C: 419A000C  beq cr6, 0x82632c78
	if ctx.cr[6].eq {
	pc = 0x82632C78; continue 'dispatch;
	}
	// 82632C70: 4BBE90C9  bl 0x8221bd38
	ctx.lr = 0x82632C74;
	sub_8221BD38(ctx, base);
	// 82632C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82632C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632C84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82632C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632C90 size=256
    let mut pc: u32 = 0x82632C90;
    'dispatch: loop {
        match pc {
            0x82632C90 => {
    //   block [0x82632C90..0x82632D90)
	// 82632C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632CA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632CA4: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82632CA8: 4BBEC5B1  bl 0x8221f258
	ctx.lr = 0x82632CAC;
	sub_8221F258(ctx, base);
	// 82632CAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632CB0: 419A00C8  beq cr6, 0x82632d78
	if ctx.cr[6].eq {
	pc = 0x82632D78; continue 'dispatch;
	}
	// 82632CB4: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82632CB8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82632CBC: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82632CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632CC4: 38AA1848  addi r5, r10, 0x1848
	ctx.r[5].s64 = ctx.r[10].s64 + 6216;
	// 82632CC8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82632CCC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82632CD0: 394612C0  addi r10, r6, 0x12c0
	ctx.r[10].s64 = ctx.r[6].s64 + 4800;
	// 82632CD4: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82632CD8: 388839B8  addi r4, r8, 0x39b8
	ctx.r[4].s64 = ctx.r[8].s64 + 14776;
	// 82632CDC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82632CE0: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82632CE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82632CE8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82632CEC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82632CF0: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 82632CF4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82632CF8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82632CFC: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
	// 82632D00: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82632D04: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82632D08: 38857088  addi r4, r5, 0x7088
	ctx.r[4].s64 = ctx.r[5].s64 + 28808;
	// 82632D0C: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82632D10: 99030026  stb r8, 0x26(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(38 as u32), ctx.r[8].u8 ) };
	// 82632D14: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82632D18: 90C3002C  stw r6, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82632D1C: 99430032  stb r10, 0x32(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(50 as u32), ctx.r[10].u8 ) };
	// 82632D20: 99430031  stb r10, 0x31(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(49 as u32), ctx.r[10].u8 ) };
	// 82632D24: 99430030  stb r10, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u8 ) };
	// 82632D28: 99430033  stb r10, 0x33(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(51 as u32), ctx.r[10].u8 ) };
	// 82632D2C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82632D30: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82632D34: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82632D38: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82632D3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82632D40: 7D202028  lwarx r9, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82632D44: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82632D48: 7D20212D  stwcx. r9, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82632D4C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82632D50: 4082FFE8  bne 0x82632d38
	if !ctx.cr[0].eq {
	pc = 0x82632D38; continue 'dispatch;
	}
	// 82632D54: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82632D58: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82632D5C: 392A0E0C  addi r9, r10, 0xe0c
	ctx.r[9].s64 = ctx.r[10].s64 + 3596;
	// 82632D60: 91230044  stw r9, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82632D64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632D70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632D74: 4E800020  blr
	return;
	// 82632D78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632D90 size=88
    let mut pc: u32 = 0x82632D90;
    'dispatch: loop {
        match pc {
            0x82632D90 => {
    //   block [0x82632D90..0x82632DE8)
	// 82632D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632DA4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82632DA8: 4BBEC4B1  bl 0x8221f258
	ctx.lr = 0x82632DAC;
	sub_8221F258(ctx, base);
	// 82632DAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632DB0: 419A0020  beq cr6, 0x82632dd0
	if ctx.cr[6].eq {
	pc = 0x82632DD0; continue 'dispatch;
	}
	// 82632DB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632DB8: 48264FA1  bl 0x82897d58
	ctx.lr = 0x82632DBC;
	sub_82897D58(ctx, base);
	// 82632DBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632DC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632DCC: 4E800020  blr
	return;
	// 82632DD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632DE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632DE8 size=88
    let mut pc: u32 = 0x82632DE8;
    'dispatch: loop {
        match pc {
            0x82632DE8 => {
    //   block [0x82632DE8..0x82632E40)
	// 82632DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632DFC: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82632E00: 4BBEC459  bl 0x8221f258
	ctx.lr = 0x82632E04;
	sub_8221F258(ctx, base);
	// 82632E04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632E08: 419A0020  beq cr6, 0x82632e28
	if ctx.cr[6].eq {
	pc = 0x82632E28; continue 'dispatch;
	}
	// 82632E0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82632E10: 48000071  bl 0x82632e80
	ctx.lr = 0x82632E14;
	sub_82632E80(ctx, base);
	// 82632E14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632E24: 4E800020  blr
	return;
	// 82632E28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632E40 size=60
    let mut pc: u32 = 0x82632E40;
    'dispatch: loop {
        match pc {
            0x82632E40 => {
    //   block [0x82632E40..0x82632E7C)
	// 82632E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632E4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632E50: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82632E54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82632E58: 388B9554  addi r4, r11, -0x6aac
	ctx.r[4].s64 = ctx.r[11].s64 + -27308;
	// 82632E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632E60: 4BBFA071  bl 0x8222ced0
	ctx.lr = 0x82632E64;
	sub_8222CED0(ctx, base);
	// 82632E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82632E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82632E80 size=256
    let mut pc: u32 = 0x82632E80;
    'dispatch: loop {
        match pc {
            0x82632E80 => {
    //   block [0x82632E80..0x82632F80)
	// 82632E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632E84: 48676589  bl 0x82ca940c
	ctx.lr = 0x82632E88;
	sub_82CA93D0(ctx, base);
	// 82632E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632E8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632E90: 4BFFD739  bl 0x826305c8
	ctx.lr = 0x82632E94;
	sub_826305C8(ctx, base);
	// 82632E94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82632E98: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82632E9C: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82632EA0: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 82632EA4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82632EA8: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82632EAC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82632EB0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82632EB4: 3961005C  addi r11, r1, 0x5c
	ctx.r[11].s64 = ctx.r[1].s64 + 92;
	// 82632EB8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82632EBC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632F80 size=136
    let mut pc: u32 = 0x82632F80;
    'dispatch: loop {
        match pc {
            0x82632F80 => {
    //   block [0x82632F80..0x82633008)
	// 82632F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632F88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82632F8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82632F94: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82632F98: 4BBEC2C1  bl 0x8221f258
	ctx.lr = 0x82632F9C;
	sub_8221F258(ctx, base);
	// 82632F9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82632FA0: 419A0050  beq cr6, 0x82632ff0
	if ctx.cr[6].eq {
	pc = 0x82632FF0; continue 'dispatch;
	}
	// 82632FA4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82632FA8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82632FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632FB0: 392A1A18  addi r9, r10, 0x1a18
	ctx.r[9].s64 = ctx.r[10].s64 + 6680;
	// 82632FB4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82632FB8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82632FBC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82632FC0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82632FC4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82632FC8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82632FCC: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82632FD0: 88E80090  lbz r7, 0x90(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 82632FD4: 60E60001  ori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 | 1;
	// 82632FD8: 98C80090  stb r6, 0x90(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(144 as u32), ctx.r[6].u8 ) };
	// 82632FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632FE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82632FEC: 4E800020  blr
	return;
	// 82632FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82632FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82632FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633008 size=88
    let mut pc: u32 = 0x82633008;
    'dispatch: loop {
        match pc {
            0x82633008 => {
    //   block [0x82633008..0x82633060)
	// 82633008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263300C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263301C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82633020: 4BBEC239  bl 0x8221f258
	ctx.lr = 0x82633024;
	sub_8221F258(ctx, base);
	// 82633024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633028: 419A0020  beq cr6, 0x82633048
	if ctx.cr[6].eq {
	pc = 0x82633048; continue 'dispatch;
	}
	// 8263302C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633030: 480FB239  bl 0x8272e268
	ctx.lr = 0x82633034;
	sub_8272E268(ctx, base);
	// 82633034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263303C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633044: 4E800020  blr
	return;
	// 82633048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263304C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633060 size=148
    let mut pc: u32 = 0x82633060;
    'dispatch: loop {
        match pc {
            0x82633060 => {
    //   block [0x82633060..0x826330F4)
	// 82633060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263306C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633074: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82633078: 4BBEC1E1  bl 0x8221f258
	ctx.lr = 0x8263307C;
	sub_8221F258(ctx, base);
	// 8263307C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633080: 419A005C  beq cr6, 0x826330dc
	if ctx.cr[6].eq {
	pc = 0x826330DC; continue 'dispatch;
	}
	// 82633084: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633088: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263308C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633090: 392A6808  addi r9, r10, 0x6808
	ctx.r[9].s64 = ctx.r[10].s64 + 26632;
	// 82633094: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633098: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263309C: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826330A0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826330A4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826330A8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 826330AC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826330B0: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 826330B4: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 826330B8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 826330BC: 88E80090  lbz r7, 0x90(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 826330C0: 60E60001  ori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 | 1;
	// 826330C4: 98C80090  stb r6, 0x90(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(144 as u32), ctx.r[6].u8 ) };
	// 826330C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826330CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826330D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826330D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826330D8: 4E800020  blr
	return;
	// 826330DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826330E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826330E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826330E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826330EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826330F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826330F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826330F8 size=120
    let mut pc: u32 = 0x826330F8;
    'dispatch: loop {
        match pc {
            0x826330F8 => {
    //   block [0x826330F8..0x82633170)
	// 826330F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826330FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263310C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82633110: 4BBEC149  bl 0x8221f258
	ctx.lr = 0x82633114;
	sub_8221F258(ctx, base);
	// 82633114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633118: 419A0040  beq cr6, 0x82633158
	if ctx.cr[6].eq {
	pc = 0x82633158; continue 'dispatch;
	}
	// 8263311C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633120: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633128: 392A69D0  addi r9, r10, 0x69d0
	ctx.r[9].s64 = ctx.r[10].s64 + 27088;
	// 8263312C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633130: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633134: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633138: 891F0090  lbz r8, 0x90(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8263313C: 61070001  ori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 | 1;
	// 82633140: 98FF0090  stb r7, 0x90(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[7].u8 ) };
	// 82633144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263314C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633154: 4E800020  blr
	return;
	// 82633158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263315C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263316C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633170 size=152
    let mut pc: u32 = 0x82633170;
    'dispatch: loop {
        match pc {
            0x82633170 => {
    //   block [0x82633170..0x82633208)
	// 82633170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263317C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633184: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82633188: 4BBEC0D1  bl 0x8221f258
	ctx.lr = 0x8263318C;
	sub_8221F258(ctx, base);
	// 8263318C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633190: 419A0060  beq cr6, 0x826331f0
	if ctx.cr[6].eq {
	pc = 0x826331F0; continue 'dispatch;
	}
	// 82633194: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82633198: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263319C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826331A0: 392A37B8  addi r9, r10, 0x37b8
	ctx.r[9].s64 = ctx.r[10].s64 + 14264;
	// 826331A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826331A8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826331AC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826331B0: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826331B4: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826331B8: 9903000E  stb r8, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[8].u8 ) };
	// 826331BC: 9963000F  stb r11, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[11].u8 ) };
	// 826331C0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826331C4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826331C8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826331CC: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 826331D0: 88C70090  lbz r6, 0x90(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(144 as u32) ) } as u64;
	// 826331D4: 60C50002  ori r5, r6, 2
	ctx.r[5].u64 = ctx.r[6].u64 | 2;
	// 826331D8: 98A70090  stb r5, 0x90(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(144 as u32), ctx.r[5].u8 ) };
	// 826331DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826331E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826331E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826331E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826331EC: 4E800020  blr
	return;
	// 826331F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826331F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826331F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826331FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633208 size=88
    let mut pc: u32 = 0x82633208;
    'dispatch: loop {
        match pc {
            0x82633208 => {
    //   block [0x82633208..0x82633260)
	// 82633208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263320C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633210: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633214: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263321C: 386003E0  li r3, 0x3e0
	ctx.r[3].s64 = 992;
	// 82633220: 4BBEC039  bl 0x8221f258
	ctx.lr = 0x82633224;
	sub_8221F258(ctx, base);
	// 82633224: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633228: 419A0020  beq cr6, 0x82633248
	if ctx.cr[6].eq {
	pc = 0x82633248; continue 'dispatch;
	}
	// 8263322C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633230: 48150789  bl 0x827839b8
	ctx.lr = 0x82633234;
	sub_827839B8(ctx, base);
	// 82633234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263323C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633244: 4E800020  blr
	return;
	// 82633248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263324C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633260 size=104
    let mut pc: u32 = 0x82633260;
    'dispatch: loop {
        match pc {
            0x82633260 => {
    //   block [0x82633260..0x826332C8)
	// 82633260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263326C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633274: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82633278: 4BBEBFE1  bl 0x8221f258
	ctx.lr = 0x8263327C;
	sub_8221F258(ctx, base);
	// 8263327C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633280: 419A0030  beq cr6, 0x826332b0
	if ctx.cr[6].eq {
	pc = 0x826332B0; continue 'dispatch;
	}
	// 82633284: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82633288: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263328C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633290: 392A4680  addi r9, r10, 0x4680
	ctx.r[9].s64 = ctx.r[10].s64 + 18048;
	// 82633294: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633298: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263329C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826332A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826332A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826332AC: 4E800020  blr
	return;
	// 826332B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826332B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826332B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826332C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826332C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826332C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826332C8 size=88
    let mut pc: u32 = 0x826332C8;
    'dispatch: loop {
        match pc {
            0x826332C8 => {
    //   block [0x826332C8..0x82633320)
	// 826332C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826332CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826332D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826332D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826332D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826332DC: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 826332E0: 4BBEBF79  bl 0x8221f258
	ctx.lr = 0x826332E4;
	sub_8221F258(ctx, base);
	// 826332E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826332E8: 419A0020  beq cr6, 0x82633308
	if ctx.cr[6].eq {
	pc = 0x82633308; continue 'dispatch;
	}
	// 826332EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826332F0: 480E1121  bl 0x82714410
	ctx.lr = 0x826332F4;
	sub_82714410(ctx, base);
	// 826332F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826332F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826332FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633304: 4E800020  blr
	return;
	// 82633308: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263330C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633318: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263331C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633320 size=160
    let mut pc: u32 = 0x82633320;
    'dispatch: loop {
        match pc {
            0x82633320 => {
    //   block [0x82633320..0x826333C0)
	// 82633320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263332C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633334: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82633338: 4BBEBF21  bl 0x8221f258
	ctx.lr = 0x8263333C;
	sub_8221F258(ctx, base);
	// 8263333C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633340: 419A0068  beq cr6, 0x826333a8
	if ctx.cr[6].eq {
	pc = 0x826333A8; continue 'dispatch;
	}
	// 82633344: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82633348: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263334C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633350: 392AB5F0  addi r9, r10, -0x4a10
	ctx.r[9].s64 = ctx.r[10].s64 + -18960;
	// 82633354: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633358: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263335C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633360: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82633364: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633368: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263336C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82633370: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82633374: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82633378: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8263337C: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82633380: 99630034  stb r11, 0x34(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u8 ) };
	// 82633384: 99630035  stb r11, 0x35(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(53 as u32), ctx.r[11].u8 ) };
	// 82633388: 88E80090  lbz r7, 0x90(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(144 as u32) ) } as u64;
	// 8263338C: 60E60001  ori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 | 1;
	// 82633390: 98C80090  stb r6, 0x90(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(144 as u32), ctx.r[6].u8 ) };
	// 82633394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263339C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826333A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826333A4: 4E800020  blr
	return;
	// 826333A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826333AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826333B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826333B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826333B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826333BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826333C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826333C0 size=136
    let mut pc: u32 = 0x826333C0;
    'dispatch: loop {
        match pc {
            0x826333C0 => {
    //   block [0x826333C0..0x82633448)
	// 826333C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826333C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826333C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826333CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826333D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826333D4: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 826333D8: 4BBEBE81  bl 0x8221f258
	ctx.lr = 0x826333DC;
	sub_8221F258(ctx, base);
	// 826333DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826333E0: 419A0050  beq cr6, 0x82633430
	if ctx.cr[6].eq {
	pc = 0x82633430; continue 'dispatch;
	}
	// 826333E4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826333E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826333EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826333F0: 392A2930  addi r9, r10, 0x2930
	ctx.r[9].s64 = ctx.r[10].s64 + 10544;
	// 826333F4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826333F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826333FC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633400: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82633404: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633408: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263340C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82633410: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82633414: 9963002D  stb r11, 0x2d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(45 as u32), ctx.r[11].u8 ) };
	// 82633418: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 8263341C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263342C: 4E800020  blr
	return;
	// 82633430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263343C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633448 size=108
    let mut pc: u32 = 0x82633448;
    'dispatch: loop {
        match pc {
            0x82633448 => {
    //   block [0x82633448..0x826334B4)
	// 82633448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263344C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263345C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82633460: 4BBEBDF9  bl 0x8221f258
	ctx.lr = 0x82633464;
	sub_8221F258(ctx, base);
	// 82633464: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633468: 419A0034  beq cr6, 0x8263349c
	if ctx.cr[6].eq {
	pc = 0x8263349C; continue 'dispatch;
	}
	// 8263346C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633470: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633474: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633478: 392A5BC8  addi r9, r10, 0x5bc8
	ctx.r[9].s64 = ctx.r[10].s64 + 23496;
	// 8263347C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633480: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633484: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263348C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633494: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633498: 4E800020  blr
	return;
	// 8263349C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826334A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826334A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826334A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826334AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826334B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826334B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826334B8 size=88
    let mut pc: u32 = 0x826334B8;
    'dispatch: loop {
        match pc {
            0x826334B8 => {
    //   block [0x826334B8..0x82633510)
	// 826334B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826334BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826334C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826334C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826334C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826334CC: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 826334D0: 4BBEBD89  bl 0x8221f258
	ctx.lr = 0x826334D4;
	sub_8221F258(ctx, base);
	// 826334D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826334D8: 419A0020  beq cr6, 0x826334f8
	if ctx.cr[6].eq {
	pc = 0x826334F8; continue 'dispatch;
	}
	// 826334DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826334E0: 4820ACE1  bl 0x8283e1c0
	ctx.lr = 0x826334E4;
	sub_8283E1C0(ctx, base);
	// 826334E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826334E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826334EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826334F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826334F4: 4E800020  blr
	return;
	// 826334F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826334FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263350C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633510 size=164
    let mut pc: u32 = 0x82633510;
    'dispatch: loop {
        match pc {
            0x82633510 => {
    //   block [0x82633510..0x826335B4)
	// 82633510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263351C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633520: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633524: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82633528: 4BBEBD31  bl 0x8221f258
	ctx.lr = 0x8263352C;
	sub_8221F258(ctx, base);
	// 8263352C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633530: 419A006C  beq cr6, 0x8263359c
	if ctx.cr[6].eq {
	pc = 0x8263359C; continue 'dispatch;
	}
	// 82633534: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82633538: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263353C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633540: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82633544: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82633548: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263354C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82633550: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633554: 38C859B8  addi r6, r8, 0x59b8
	ctx.r[6].s64 = ctx.r[8].s64 + 22968;
	// 82633558: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8263355C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82633560: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82633564: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82633568: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 8263356C: C009A4E4  lfs f0, -0x5b1c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-23324 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633570: 99630025  stb r11, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82633574: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82633578: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263357C: 90E30058  stw r7, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82633580: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82633584: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82633588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263358C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633598: 4E800020  blr
	return;
	// 8263359C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826335A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826335A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826335A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826335AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826335B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826335B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826335B8 size=200
    let mut pc: u32 = 0x826335B8;
    'dispatch: loop {
        match pc {
            0x826335B8 => {
    //   block [0x826335B8..0x82633680)
	// 826335B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826335BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826335C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826335C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826335C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826335CC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826335D0: 4BBEBC89  bl 0x8221f258
	ctx.lr = 0x826335D4;
	sub_8221F258(ctx, base);
	// 826335D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826335D8: 419A0090  beq cr6, 0x82633668
	if ctx.cr[6].eq {
	pc = 0x82633668; continue 'dispatch;
	}
	// 826335DC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 826335E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826335E4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 826335E8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826335EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826335F0: 390A3228  addi r8, r10, 0x3228
	ctx.r[8].s64 = ctx.r[10].s64 + 12840;
	// 826335F4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 826335F8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826335FC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82633600: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633604: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82633608: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8263360C: 38870B7C  addi r4, r7, 0xb7c
	ctx.r[4].s64 = ctx.r[7].s64 + 2940;
	// 82633610: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633680 size=108
    let mut pc: u32 = 0x82633680;
    'dispatch: loop {
        match pc {
            0x82633680 => {
    //   block [0x82633680..0x826336EC)
	// 82633680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263368C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633694: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82633698: 4BBEBBC1  bl 0x8221f258
	ctx.lr = 0x8263369C;
	sub_8221F258(ctx, base);
	// 8263369C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826336A0: 419A0034  beq cr6, 0x826336d4
	if ctx.cr[6].eq {
	pc = 0x826336D4; continue 'dispatch;
	}
	// 826336A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826336A8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826336AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826336B0: 392A45C0  addi r9, r10, 0x45c0
	ctx.r[9].s64 = ctx.r[10].s64 + 17856;
	// 826336B4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826336B8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826336BC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826336C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826336C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826336C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826336CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826336D0: 4E800020  blr
	return;
	// 826336D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826336D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826336DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826336E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826336E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826336E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826336F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826336F0 size=128
    let mut pc: u32 = 0x826336F0;
    'dispatch: loop {
        match pc {
            0x826336F0 => {
    //   block [0x826336F0..0x82633770)
	// 826336F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826336F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826336F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826336FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633704: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82633708: 4BBEBB51  bl 0x8221f258
	ctx.lr = 0x8263370C;
	sub_8221F258(ctx, base);
	// 8263370C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633710: 419A0048  beq cr6, 0x82633758
	if ctx.cr[6].eq {
	pc = 0x82633758; continue 'dispatch;
	}
	// 82633714: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633718: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263371C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633720: 392A6270  addi r9, r10, 0x6270
	ctx.r[9].s64 = ctx.r[10].s64 + 25200;
	// 82633724: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82633728: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263372C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633730: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82633734: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633738: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8263373C: 9103001C  stw r8, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82633740: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82633744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263374C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633754: 4E800020  blr
	return;
	// 82633758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263375C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263376C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633770 size=184
    let mut pc: u32 = 0x82633770;
    'dispatch: loop {
        match pc {
            0x82633770 => {
    //   block [0x82633770..0x82633828)
	// 82633770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263377C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633784: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 82633788: 4BBEBAD1  bl 0x8221f258
	ctx.lr = 0x8263378C;
	sub_8221F258(ctx, base);
	// 8263378C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633790: 419A0080  beq cr6, 0x82633810
	if ctx.cr[6].eq {
	pc = 0x82633810; continue 'dispatch;
	}
	// 82633794: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633798: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263379C: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 826337A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826337A4: 390AD1F8  addi r8, r10, -0x2e08
	ctx.r[8].s64 = ctx.r[10].s64 + -11784;
	// 826337A8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826337AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826337B0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 826337B4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826337B8: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826337BC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826337C0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826337C4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 826337C8: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826337CC: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 826337D0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 826337D4: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 826337D8: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 826337DC: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 826337E0: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 826337E4: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 826337E8: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 826337EC: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 826337F0: 98E30050  stb r7, 0x50(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 826337F4: 99630051  stb r11, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 826337F8: 99630052  stb r11, 0x52(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 826337FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263380C: 4E800020  blr
	return;
	// 82633810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263381C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633828 size=180
    let mut pc: u32 = 0x82633828;
    'dispatch: loop {
        match pc {
            0x82633828 => {
    //   block [0x82633828..0x826338DC)
	// 82633828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263383C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82633840: 4BBEBA19  bl 0x8221f258
	ctx.lr = 0x82633844;
	sub_8221F258(ctx, base);
	// 82633844: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633848: 419A007C  beq cr6, 0x826338c4
	if ctx.cr[6].eq {
	pc = 0x826338C4; continue 'dispatch;
	}
	// 8263384C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82633850: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82633854: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633858: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263385C: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82633860: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82633864: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633868: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8263386C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633870: 38C94540  addi r6, r9, 0x4540
	ctx.r[6].s64 = ctx.r[9].s64 + 17728;
	// 82633874: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82633878: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8263387C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633880: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82633884: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82633888: 98A3001C  stb r5, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u8 ) };
	// 8263388C: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826338E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826338E0 size=156
    let mut pc: u32 = 0x826338E0;
    'dispatch: loop {
        match pc {
            0x826338E0 => {
    //   block [0x826338E0..0x8263397C)
	// 826338E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826338E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826338E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826338EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826338F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826338F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826338F8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 826338FC: 4BBEB95D  bl 0x8221f258
	ctx.lr = 0x82633900;
	sub_8221F258(ctx, base);
	// 82633900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633904: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82633908: 419A0058  beq cr6, 0x82633960
	if ctx.cr[6].eq {
	pc = 0x82633960; continue 'dispatch;
	}
	// 8263390C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82633910: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82633914: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82633918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263391C: 39094340  addi r8, r9, 0x4340
	ctx.r[8].s64 = ctx.r[9].s64 + 17216;
	// 82633920: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633924: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82633928: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8263392C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633930: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82633934: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82633938: 997F0011  stb r11, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8263393C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82633940: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82633944: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82633948: 48214669  bl 0x82847fb0
	ctx.lr = 0x8263394C;
	sub_82847FB0(ctx, base);
	// 8263394C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633950: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82633954: 4BD59B65  bl 0x8238d4b8
	ctx.lr = 0x82633958;
	sub_8238D4B8(ctx, base);
	// 82633958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263395C: 48000008  b 0x82633964
	pc = 0x82633964; continue 'dispatch;
	// 82633960: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82633968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263396C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82633974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633980 size=104
    let mut pc: u32 = 0x82633980;
    'dispatch: loop {
        match pc {
            0x82633980 => {
    //   block [0x82633980..0x826339E8)
	// 82633980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633988: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263398C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633994: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82633998: 4BBEB8C1  bl 0x8221f258
	ctx.lr = 0x8263399C;
	sub_8221F258(ctx, base);
	// 8263399C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826339A0: 419A0030  beq cr6, 0x826339d0
	if ctx.cr[6].eq {
	pc = 0x826339D0; continue 'dispatch;
	}
	// 826339A4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826339A8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826339AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826339B0: 392A6680  addi r9, r10, 0x6680
	ctx.r[9].s64 = ctx.r[10].s64 + 26240;
	// 826339B4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826339B8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826339BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826339C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826339C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826339C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826339CC: 4E800020  blr
	return;
	// 826339D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826339D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826339D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826339DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826339E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826339E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826339E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826339E8 size=88
    let mut pc: u32 = 0x826339E8;
    'dispatch: loop {
        match pc {
            0x826339E8 => {
    //   block [0x826339E8..0x82633A40)
	// 826339E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826339EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826339F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826339F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826339F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826339FC: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82633A00: 4BBEB859  bl 0x8221f258
	ctx.lr = 0x82633A04;
	sub_8221F258(ctx, base);
	// 82633A04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633A08: 419A0020  beq cr6, 0x82633a28
	if ctx.cr[6].eq {
	pc = 0x82633A28; continue 'dispatch;
	}
	// 82633A0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633A10: 4BE754D9  bl 0x824a8ee8
	ctx.lr = 0x82633A14;
	sub_824A8EE8(ctx, base);
	// 82633A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A24: 4E800020  blr
	return;
	// 82633A28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633A40 size=88
    let mut pc: u32 = 0x82633A40;
    'dispatch: loop {
        match pc {
            0x82633A40 => {
    //   block [0x82633A40..0x82633A98)
	// 82633A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633A48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633A4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633A50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633A54: 38600310  li r3, 0x310
	ctx.r[3].s64 = 784;
	// 82633A58: 4BBEB801  bl 0x8221f258
	ctx.lr = 0x82633A5C;
	sub_8221F258(ctx, base);
	// 82633A5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633A60: 419A0020  beq cr6, 0x82633a80
	if ctx.cr[6].eq {
	pc = 0x82633A80; continue 'dispatch;
	}
	// 82633A64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633A68: 4BF091B9  bl 0x8253cc20
	ctx.lr = 0x82633A6C;
	sub_8253CC20(ctx, base);
	// 82633A6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A7C: 4E800020  blr
	return;
	// 82633A80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633A90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633A98 size=88
    let mut pc: u32 = 0x82633A98;
    'dispatch: loop {
        match pc {
            0x82633A98 => {
    //   block [0x82633A98..0x82633AF0)
	// 82633A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633AA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633AAC: 386002E0  li r3, 0x2e0
	ctx.r[3].s64 = 736;
	// 82633AB0: 4BBEB7A9  bl 0x8221f258
	ctx.lr = 0x82633AB4;
	sub_8221F258(ctx, base);
	// 82633AB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633AB8: 419A0020  beq cr6, 0x82633ad8
	if ctx.cr[6].eq {
	pc = 0x82633AD8; continue 'dispatch;
	}
	// 82633ABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633AC0: 4BFD5B89  bl 0x82609648
	ctx.lr = 0x82633AC4;
	sub_82609648(ctx, base);
	// 82633AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633AD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633AD4: 4E800020  blr
	return;
	// 82633AD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633ADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633AF0 size=88
    let mut pc: u32 = 0x82633AF0;
    'dispatch: loop {
        match pc {
            0x82633AF0 => {
    //   block [0x82633AF0..0x82633B48)
	// 82633AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633AFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633B04: 386000B4  li r3, 0xb4
	ctx.r[3].s64 = 180;
	// 82633B08: 4BBEB751  bl 0x8221f258
	ctx.lr = 0x82633B0C;
	sub_8221F258(ctx, base);
	// 82633B0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633B10: 419A0020  beq cr6, 0x82633b30
	if ctx.cr[6].eq {
	pc = 0x82633B30; continue 'dispatch;
	}
	// 82633B14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633B18: 48000071  bl 0x82633b88
	ctx.lr = 0x82633B1C;
	sub_82633B88(ctx, base);
	// 82633B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633B2C: 4E800020  blr
	return;
	// 82633B30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633B48 size=60
    let mut pc: u32 = 0x82633B48;
    'dispatch: loop {
        match pc {
            0x82633B48 => {
    //   block [0x82633B48..0x82633B84)
	// 82633B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B58: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82633B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82633B60: 388B9648  addi r4, r11, -0x69b8
	ctx.r[4].s64 = ctx.r[11].s64 + -27064;
	// 82633B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633B68: 4BBF9369  bl 0x8222ced0
	ctx.lr = 0x82633B6C;
	sub_8222CED0(ctx, base);
	// 82633B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633B88 size=332
    let mut pc: u32 = 0x82633B88;
    'dispatch: loop {
        match pc {
            0x82633B88 => {
    //   block [0x82633B88..0x82633CD4)
	// 82633B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633B8C: 48675881  bl 0x82ca940c
	ctx.lr = 0x82633B90;
	sub_82CA93D0(ctx, base);
	// 82633B90: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82633B94: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633B9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82633BA0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633BA4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82633BA8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82633BAC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82633BB0: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82633BB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82633BB8: 38CB04D4  addi r6, r11, 0x4d4
	ctx.r[6].s64 = ctx.r[11].s64 + 1236;
	// 82633BBC: 38AAE738  addi r5, r10, -0x18c8
	ctx.r[5].s64 = ctx.r[10].s64 + -6344;
	// 82633BC0: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82633BC4: 3868A244  addi r3, r8, -0x5dbc
	ctx.r[3].s64 = ctx.r[8].s64 + -23996;
	// 82633BC8: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82633BCC: 38894A00  addi r4, r9, 0x4a00
	ctx.r[4].s64 = ctx.r[9].s64 + 18944;
	// 82633BD0: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82633BD4: 39670BAC  addi r11, r7, 0xbac
	ctx.r[11].s64 = ctx.r[7].s64 + 2988;
	// 82633BD8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82633BDC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82633BE0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82633BE4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633BE8: 4BBEB671  bl 0x8221f258
	ctx.lr = 0x82633BEC;
	sub_8221F258(ctx, base);
	// 82633BEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633BF0: 419A0008  beq cr6, 0x82633bf8
	if ctx.cr[6].eq {
	pc = 0x82633BF8; continue 'dispatch;
	}
	// 82633BF4: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633BF8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82633BFC: 41820008  beq 0x82633c04
	if ctx.cr[0].eq {
	pc = 0x82633C04; continue 'dispatch;
	}
	// 82633C00: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633C04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82633C08: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82633C0C: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82633C10: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82633C14: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82633C18: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82633C1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82633C20: 392A0E0C  addi r9, r10, 0xe0c
	ctx.r[9].s64 = ctx.r[10].s64 + 3596;
	// 82633C24: C3EB9484  lfs f31, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82633C28: D3FF0024  stfs f31, 0x24(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82633C2C: D3FF0028  stfs f31, 0x28(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82633C30: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82633C34: D3FF0030  stfs f31, 0x30(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82633C38: D3FF0034  stfs f31, 0x34(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82633C3C: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82633C40: D3FF003C  stfs f31, 0x3c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82633C44: D3FF0040  stfs f31, 0x40(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82633C48: D3FF0044  stfs f31, 0x44(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82633C4C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82633C50: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82633C54: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82633C58: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82633C5C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82633C60: D3FF0064  stfs f31, 0x64(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82633C64: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82633C68: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82633C6C: 4BBEB5ED  bl 0x8221f258
	ctx.lr = 0x82633C70;
	sub_8221F258(ctx, base);
	// 82633C70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633C74: 419A0008  beq cr6, 0x82633c7c
	if ctx.cr[6].eq {
	pc = 0x82633C7C; continue 'dispatch;
	}
	// 82633C78: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633C7C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82633C80: 41820008  beq 0x82633c88
	if ctx.cr[0].eq {
	pc = 0x82633C88; continue 'dispatch;
	}
	// 82633C84: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82633C88: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82633C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633C90: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82633C94: D3FF0078  stfs f31, 0x78(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82633C98: 93BF0074  stw r29, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82633C9C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82633CA0: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 82633CA4: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82633CA8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82633CAC: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 82633CB0: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 82633CB4: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 82633CB8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82633CBC: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82633CC0: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82633CC4: 9BDF00B0  stb r30, 0xb0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u8 ) };
	// 82633CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82633CCC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82633CD0: 4867578C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633CD8 size=164
    let mut pc: u32 = 0x82633CD8;
    'dispatch: loop {
        match pc {
            0x82633CD8 => {
    //   block [0x82633CD8..0x82633D7C)
	// 82633CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633CDC: 48675731  bl 0x82ca940c
	ctx.lr = 0x82633CE0;
	sub_82CA93D0(ctx, base);
	// 82633CE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633CE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633CE8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82633CEC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82633CF0: 4BCE6E69  bl 0x8231ab58
	ctx.lr = 0x82633CF4;
	sub_8231AB58(ctx, base);
	// 82633CF4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82633CF8: 4BCE6E61  bl 0x8231ab58
	ctx.lr = 0x82633CFC;
	sub_8231AB58(ctx, base);
	// 82633CFC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82633D00: 483E1091  bl 0x82a14d90
	ctx.lr = 0x82633D04;
	sub_82A14D90(ctx, base);
	// 82633D04: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82633D08: 4BBE8031  bl 0x8221bd38
	ctx.lr = 0x82633D0C;
	sub_8221BD38(ctx, base);
	// 82633D0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82633D10: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82633D14: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82633D18: 4BC66B39  bl 0x8229a850
	ctx.lr = 0x82633D1C;
	sub_8229A850(ctx, base);
	// 82633D1C: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82633D20: 4BB83DF9  bl 0x821b7b18
	ctx.lr = 0x82633D24;
	sub_821B7B18(ctx, base);
	// 82633D24: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82633D28: 480069C9  bl 0x8263a6f0
	ctx.lr = 0x82633D2C;
	sub_8263A6F0(ctx, base);
	// 82633D2C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82633D30: 4BBE8009  bl 0x8221bd38
	ctx.lr = 0x82633D34;
	sub_8221BD38(ctx, base);
	// 82633D34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82633D38: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82633D3C: 419A0008  beq cr6, 0x82633d44
	if ctx.cr[6].eq {
	pc = 0x82633D44; continue 'dispatch;
	}
	// 82633D40: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82633D44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82633D48: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82633D4C: 392B04D4  addi r9, r11, 0x4d4
	ctx.r[9].s64 = ctx.r[11].s64 + 1236;
	// 82633D50: 390A2850  addi r8, r10, 0x2850
	ctx.r[8].s64 = ctx.r[10].s64 + 10320;
	// 82633D54: 57A707FE  clrlwi r7, r29, 0x1f
	ctx.r[7].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82633D58: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633D5C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633D64: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82633D68: 419A000C  beq cr6, 0x82633d74
	if ctx.cr[6].eq {
	pc = 0x82633D74; continue 'dispatch;
	}
	// 82633D6C: 4BBE7FCD  bl 0x8221bd38
	ctx.lr = 0x82633D70;
	sub_8221BD38(ctx, base);
	// 82633D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82633D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82633D78: 486756E4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633D80 size=200
    let mut pc: u32 = 0x82633D80;
    'dispatch: loop {
        match pc {
            0x82633D80 => {
    //   block [0x82633D80..0x82633E48)
	// 82633D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633D94: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82633D98: 4BBEB4C1  bl 0x8221f258
	ctx.lr = 0x82633D9C;
	sub_8221F258(ctx, base);
	// 82633D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633DA0: 419A0090  beq cr6, 0x82633e30
	if ctx.cr[6].eq {
	pc = 0x82633E30; continue 'dispatch;
	}
	// 82633DA4: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82633DA8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633DAC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 82633DB0: 390ACBBC  addi r8, r10, -0x3444
	ctx.r[8].s64 = ctx.r[10].s64 + -13380;
	// 82633DB4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82633DB8: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 82633DBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633DC0: C18ACBBC  lfs f12, -0x3444(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13380 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82633DC4: 38A904D4  addi r5, r9, 0x4d4
	ctx.r[5].s64 = ctx.r[9].s64 + 1236;
	// 82633DC8: C0080A0C  lfs f0, 0xa0c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633DCC: 388732E8  addi r4, r7, 0x32e8
	ctx.r[4].s64 = ctx.r[7].s64 + 13032;
	// 82633DD0: C1A8F248  lfs f13, -0xdb8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-3512 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82633DD4: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82633DD8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82633DDC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633DE0: 39264300  addi r9, r6, 0x4300
	ctx.r[9].s64 = ctx.r[6].s64 + 17152;
	// 82633DE4: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82633DE8: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82633DEC: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82633DF0: 38C80E0C  addi r6, r8, 0xe0c
	ctx.r[6].s64 = ctx.r[8].s64 + 3596;
	// 82633DF4: D1A30028  stfs f13, 0x28(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82633DF8: D183002C  stfs f12, 0x2c(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82633DFC: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82633E00: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82633E04: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82633E08: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82633E0C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82633E10: 90E30024  stw r7, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82633E14: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82633E18: 90C30030  stw r6, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82633E1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E2C: 4E800020  blr
	return;
	// 82633E30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633E34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633E48 size=88
    let mut pc: u32 = 0x82633E48;
    'dispatch: loop {
        match pc {
            0x82633E48 => {
    //   block [0x82633E48..0x82633EA0)
	// 82633E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633E58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633E5C: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 82633E60: 4BBEB3F9  bl 0x8221f258
	ctx.lr = 0x82633E64;
	sub_8221F258(ctx, base);
	// 82633E64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633E68: 419A0020  beq cr6, 0x82633e88
	if ctx.cr[6].eq {
	pc = 0x82633E88; continue 'dispatch;
	}
	// 82633E6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82633E70: 4BE89F49  bl 0x824bddb8
	ctx.lr = 0x82633E74;
	sub_824BDDB8(ctx, base);
	// 82633E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E84: 4E800020  blr
	return;
	// 82633E88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633E8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82633EA0 size=156
    let mut pc: u32 = 0x82633EA0;
    'dispatch: loop {
        match pc {
            0x82633EA0 => {
    //   block [0x82633EA0..0x82633F3C)
	// 82633EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633EB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633EB4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82633EB8: 4BBEB3A1  bl 0x8221f258
	ctx.lr = 0x82633EBC;
	sub_8221F258(ctx, base);
	// 82633EBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633EC0: 419A0064  beq cr6, 0x82633f24
	if ctx.cr[6].eq {
	pc = 0x82633F24; continue 'dispatch;
	}
	// 82633EC4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633EC8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633ECC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82633ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633ED4: 390A47A8  addi r8, r10, 0x47a8
	ctx.r[8].s64 = ctx.r[10].s64 + 18344;
	// 82633ED8: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 82633EDC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633EE0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82633EE4: 9963000E  stb r11, 0xe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u8 ) };
	// 82633EE8: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82633EEC: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 82633EF0: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82633EF4: 9943000F  stb r10, 0xf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(15 as u32), ctx.r[10].u8 ) };
	// 82633EF8: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82633EFC: 99630016  stb r11, 0x16(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 82633F00: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82633F04: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82633F08: 99430017  stb r10, 0x17(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(23 as u32), ctx.r[10].u8 ) };
	// 82633F0C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82633F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633F20: 4E800020  blr
	return;
	// 82633F24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633F40 size=112
    let mut pc: u32 = 0x82633F40;
    'dispatch: loop {
        match pc {
            0x82633F40 => {
    //   block [0x82633F40..0x82633FB0)
	// 82633F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633F48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633F4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633F50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633F54: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82633F58: 4BBEB301  bl 0x8221f258
	ctx.lr = 0x82633F5C;
	sub_8221F258(ctx, base);
	// 82633F5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633F60: 419A0038  beq cr6, 0x82633f98
	if ctx.cr[6].eq {
	pc = 0x82633F98; continue 'dispatch;
	}
	// 82633F64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82633F68: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633F6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633F70: 392A41B8  addi r9, r10, 0x41b8
	ctx.r[9].s64 = ctx.r[10].s64 + 16824;
	// 82633F74: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633F78: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633F7C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633F80: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82633F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633F90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633F94: 4E800020  blr
	return;
	// 82633F98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82633F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82633FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82633FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82633FA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82633FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82633FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82633FB0 size=120
    let mut pc: u32 = 0x82633FB0;
    'dispatch: loop {
        match pc {
            0x82633FB0 => {
    //   block [0x82633FB0..0x82634028)
	// 82633FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82633FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82633FB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82633FBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82633FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82633FC4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82633FC8: 4BBEB291  bl 0x8221f258
	ctx.lr = 0x82633FCC;
	sub_8221F258(ctx, base);
	// 82633FCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82633FD0: 419A0040  beq cr6, 0x82634010
	if ctx.cr[6].eq {
	pc = 0x82634010; continue 'dispatch;
	}
	// 82633FD4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82633FD8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82633FDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82633FE0: 392A8990  addi r9, r10, -0x7670
	ctx.r[9].s64 = ctx.r[10].s64 + -30320;
	// 82633FE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82633FE8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82633FEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82633FF0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82633FF4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82633FF8: 99030014  stb r8, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82633FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263400C: 4E800020  blr
	return;
	// 82634010: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634014: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263401C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634028 size=124
    let mut pc: u32 = 0x82634028;
    'dispatch: loop {
        match pc {
            0x82634028 => {
    //   block [0x82634028..0x826340A4)
	// 82634028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263402C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263403C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82634040: 4BBEB219  bl 0x8221f258
	ctx.lr = 0x82634044;
	sub_8221F258(ctx, base);
	// 82634044: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634048: 419A0044  beq cr6, 0x8263408c
	if ctx.cr[6].eq {
	pc = 0x8263408C; continue 'dispatch;
	}
	// 8263404C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634050: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634054: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634058: 392A89D0  addi r9, r10, -0x7630
	ctx.r[9].s64 = ctx.r[10].s64 + -30256;
	// 8263405C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634060: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634064: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634068: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263406C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634070: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82634074: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263407C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634088: 4E800020  blr
	return;
	// 8263408C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263409C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826340A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826340A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826340A8 size=88
    let mut pc: u32 = 0x826340A8;
    'dispatch: loop {
        match pc {
            0x826340A8 => {
    //   block [0x826340A8..0x82634100)
	// 826340A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826340AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826340B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826340B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826340B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826340BC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826340C0: 4BBEB199  bl 0x8221f258
	ctx.lr = 0x826340C4;
	sub_8221F258(ctx, base);
	// 826340C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826340C8: 419A0020  beq cr6, 0x826340e8
	if ctx.cr[6].eq {
	pc = 0x826340E8; continue 'dispatch;
	}
	// 826340CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826340D0: 4818A2D1  bl 0x827be3a0
	ctx.lr = 0x826340D4;
	sub_827BE3A0(ctx, base);
	// 826340D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826340D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826340DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826340E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826340E4: 4E800020  blr
	return;
	// 826340E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826340EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826340F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826340F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826340F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826340FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634100 size=88
    let mut pc: u32 = 0x82634100;
    'dispatch: loop {
        match pc {
            0x82634100 => {
    //   block [0x82634100..0x82634158)
	// 82634100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263410C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634114: 3860015C  li r3, 0x15c
	ctx.r[3].s64 = 348;
	// 82634118: 4BBEB141  bl 0x8221f258
	ctx.lr = 0x8263411C;
	sub_8221F258(ctx, base);
	// 8263411C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634120: 419A0020  beq cr6, 0x82634140
	if ctx.cr[6].eq {
	pc = 0x82634140; continue 'dispatch;
	}
	// 82634124: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634128: 4BE5ECD1  bl 0x82492df8
	ctx.lr = 0x8263412C;
	sub_82492DF8(ctx, base);
	// 8263412C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263413C: 4E800020  blr
	return;
	// 82634140: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263414C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634158 size=120
    let mut pc: u32 = 0x82634158;
    'dispatch: loop {
        match pc {
            0x82634158 => {
    //   block [0x82634158..0x826341D0)
	// 82634158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634160: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634164: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263416C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82634170: 4BBEB0E9  bl 0x8221f258
	ctx.lr = 0x82634174;
	sub_8221F258(ctx, base);
	// 82634174: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634178: 419A0040  beq cr6, 0x826341b8
	if ctx.cr[6].eq {
	pc = 0x826341B8; continue 'dispatch;
	}
	// 8263417C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634180: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634184: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263418C: 390A77A0  addi r8, r10, 0x77a0
	ctx.r[8].s64 = ctx.r[10].s64 + 30624;
	// 82634190: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 82634194: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634198: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263419C: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 826341A0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826341A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826341A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826341AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826341B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826341B4: 4E800020  blr
	return;
	// 826341B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826341BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826341C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826341C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826341C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826341CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826341D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826341D0 size=164
    let mut pc: u32 = 0x826341D0;
    'dispatch: loop {
        match pc {
            0x826341D0 => {
    //   block [0x826341D0..0x82634274)
	// 826341D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826341D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826341D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826341DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826341E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826341E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826341E8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 826341EC: 4BBEB06D  bl 0x8221f258
	ctx.lr = 0x826341F0;
	sub_8221F258(ctx, base);
	// 826341F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826341F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826341F8: 419A0060  beq cr6, 0x82634258
	if ctx.cr[6].eq {
	pc = 0x82634258; continue 'dispatch;
	}
	// 826341FC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 82634200: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82634204: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82634208: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8263420C: 3BCAABC8  addi r30, r10, -0x5438
	ctx.r[30].s64 = ctx.r[10].s64 + -21560;
	// 82634210: 39094600  addi r8, r9, 0x4600
	ctx.r[8].s64 = ctx.r[9].s64 + 17920;
	// 82634214: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 82634218: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263421C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82634220: C02AABC8  lfs f1, -0x5438(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21560 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82634224: 98FF0008  stb r7, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 82634228: C01E351C  lfs f0, 0x351c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13596 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8263422C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82634230: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82634234: 80A60058  lwz r5, 0x58(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) } as u64;
	// 82634238: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263423C: 388B0078  addi r4, r11, 0x78
	ctx.r[4].s64 = ctx.r[11].s64 + 120;
	// 82634240: 4BBBC789  bl 0x821f09c8
	ctx.lr = 0x82634244;
	sub_821F09C8(ctx, base);
	// 82634244: C01E3514  lfs f0, 0x3514(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(13588 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634248: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8263424C: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82634250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634254: 48000008  b 0x8263425c
	pc = 0x8263425C; continue 'dispatch;
	// 82634258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263425C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634268: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8263426C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634278 size=136
    let mut pc: u32 = 0x82634278;
    'dispatch: loop {
        match pc {
            0x82634278 => {
    //   block [0x82634278..0x82634300)
	// 82634278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263427C: 48675191  bl 0x82ca940c
	ctx.lr = 0x82634280;
	sub_82CA93D0(ctx, base);
	// 82634280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634284: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634288: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8263428C: 4BBEAFCD  bl 0x8221f258
	ctx.lr = 0x82634290;
	sub_8221F258(ctx, base);
	// 82634290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634294: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634298: 419A005C  beq cr6, 0x826342f4
	if ctx.cr[6].eq {
	pc = 0x826342F4; continue 'dispatch;
	}
	// 8263429C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 826342A0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 826342A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 826342A8: 394B79F0  addi r10, r11, 0x79f0
	ctx.r[10].s64 = ctx.r[11].s64 + 31216;
	// 826342AC: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 826342B0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 826342B4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826342B8: 4BBEAFA1  bl 0x8221f258
	ctx.lr = 0x826342BC;
	sub_8221F258(ctx, base);
	// 826342BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826342C0: 419A0008  beq cr6, 0x826342c8
	if ctx.cr[6].eq {
	pc = 0x826342C8; continue 'dispatch;
	}
	// 826342C4: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826342C8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 826342CC: 41820008  beq 0x826342d4
	if ctx.cr[0].eq {
	pc = 0x826342D4; continue 'dispatch;
	}
	// 826342D0: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826342D4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 826342D8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 826342DC: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 826342E0: 48213CD1  bl 0x82847fb0
	ctx.lr = 0x826342E4;
	sub_82847FB0(ctx, base);
	// 826342E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826342E8: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 826342EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826342F0: 4867516C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 826342F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826342F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826342FC: 48675160  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634300 size=148
    let mut pc: u32 = 0x82634300;
    'dispatch: loop {
        match pc {
            0x82634300 => {
    //   block [0x82634300..0x82634394)
	// 82634300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263430C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634314: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82634318: 4BBEAF41  bl 0x8221f258
	ctx.lr = 0x8263431C;
	sub_8221F258(ctx, base);
	// 8263431C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634320: 419A005C  beq cr6, 0x8263437c
	if ctx.cr[6].eq {
	pc = 0x8263437C; continue 'dispatch;
	}
	// 82634324: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634328: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263432C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634330: 38EA7C08  addi r7, r10, 0x7c08
	ctx.r[7].s64 = ctx.r[10].s64 + 31752;
	// 82634334: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 82634338: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263433C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82634340: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82634344: 38A67088  addi r5, r6, 0x7088
	ctx.r[5].s64 = ctx.r[6].s64 + 28808;
	// 82634348: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263434C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82634350: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82634354: 7D202828  lwarx r9, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82634358: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8263435C: 7D20292D  stwcx. r9, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82634360: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82634364: 4082FFE8  bne 0x8263434c
	if !ctx.cr[0].eq {
	pc = 0x8263434C; continue 'dispatch;
	}
	// 82634368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263436C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634374: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634378: 4E800020  blr
	return;
	// 8263437C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263438C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634398 size=88
    let mut pc: u32 = 0x82634398;
    'dispatch: loop {
        match pc {
            0x82634398 => {
    //   block [0x82634398..0x826343F0)
	// 82634398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826343A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826343A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826343A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826343AC: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 826343B0: 4BBEAEA9  bl 0x8221f258
	ctx.lr = 0x826343B4;
	sub_8221F258(ctx, base);
	// 826343B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826343B8: 419A0020  beq cr6, 0x826343d8
	if ctx.cr[6].eq {
	pc = 0x826343D8; continue 'dispatch;
	}
	// 826343BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826343C0: 4BF522C9  bl 0x82586688
	ctx.lr = 0x826343C4;
	sub_82586688(ctx, base);
	// 826343C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826343C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826343CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826343D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826343D4: 4E800020  blr
	return;
	// 826343D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826343DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826343E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826343E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826343E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826343EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826343F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826343F0 size=88
    let mut pc: u32 = 0x826343F0;
    'dispatch: loop {
        match pc {
            0x826343F0 => {
    //   block [0x826343F0..0x82634448)
	// 826343F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826343F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826343F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826343FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634404: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 82634408: 4BBEAE51  bl 0x8221f258
	ctx.lr = 0x8263440C;
	sub_8221F258(ctx, base);
	// 8263440C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634410: 419A0020  beq cr6, 0x82634430
	if ctx.cr[6].eq {
	pc = 0x82634430; continue 'dispatch;
	}
	// 82634414: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634418: 48335051  bl 0x82969468
	ctx.lr = 0x8263441C;
	sub_82969468(ctx, base);
	// 8263441C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263442C: 4E800020  blr
	return;
	// 82634430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263443C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634448 size=140
    let mut pc: u32 = 0x82634448;
    'dispatch: loop {
        match pc {
            0x82634448 => {
    //   block [0x82634448..0x826344D4)
	// 82634448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82634454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263445C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634460: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82634464: 4BBEADF5  bl 0x8221f258
	ctx.lr = 0x82634468;
	sub_8221F258(ctx, base);
	// 82634468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263446C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634470: 419A0048  beq cr6, 0x826344b8
	if ctx.cr[6].eq {
	pc = 0x826344B8; continue 'dispatch;
	}
	// 82634474: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82634478: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263447C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634480: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82634484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634488: 38E92F28  addi r7, r9, 0x2f28
	ctx.r[7].s64 = ctx.r[9].s64 + 12072;
	// 8263448C: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 82634490: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634494: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82634498: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263449C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 826344A0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 826344A4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826344A8: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 826344AC: 4BB5F98D  bl 0x82193e38
	ctx.lr = 0x826344B0;
	sub_82193E38(ctx, base);
	// 826344B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826344B4: 48000008  b 0x826344bc
	pc = 0x826344BC; continue 'dispatch;
	// 826344B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826344BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826344C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826344C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826344C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826344CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826344D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826344D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826344D8 size=124
    let mut pc: u32 = 0x826344D8;
    'dispatch: loop {
        match pc {
            0x826344D8 => {
    //   block [0x826344D8..0x82634554)
	// 826344D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826344DC: 48674F31  bl 0x82ca940c
	ctx.lr = 0x826344E0;
	sub_82CA93D0(ctx, base);
	// 826344E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826344E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826344E8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 826344EC: 4BBEAD6D  bl 0x8221f258
	ctx.lr = 0x826344F0;
	sub_8221F258(ctx, base);
	// 826344F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826344F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826344F8: 419A0050  beq cr6, 0x82634548
	if ctx.cr[6].eq {
	pc = 0x82634548; continue 'dispatch;
	}
	// 826344FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82634500: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82634504: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82634508: 394B7E78  addi r10, r11, 0x7e78
	ctx.r[10].s64 = ctx.r[11].s64 + 32376;
	// 8263450C: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82634510: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82634514: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82634518: 4BBEAD41  bl 0x8221f258
	ctx.lr = 0x8263451C;
	sub_8221F258(ctx, base);
	// 8263451C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634520: 419A0008  beq cr6, 0x82634528
	if ctx.cr[6].eq {
	pc = 0x82634528; continue 'dispatch;
	}
	// 82634524: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82634528: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8263452C: 41820008  beq 0x82634534
	if ctx.cr[0].eq {
	pc = 0x82634534; continue 'dispatch;
	}
	// 82634530: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82634534: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82634538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8263453C: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82634540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634544: 48674F18  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82634548: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263454C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634550: 48674F0C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634558 size=128
    let mut pc: u32 = 0x82634558;
    'dispatch: loop {
        match pc {
            0x82634558 => {
    //   block [0x82634558..0x826345D8)
	// 82634558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263455C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263456C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82634570: 4BBEACE9  bl 0x8221f258
	ctx.lr = 0x82634574;
	sub_8221F258(ctx, base);
	// 82634574: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634578: 419A0048  beq cr6, 0x826345c0
	if ctx.cr[6].eq {
	pc = 0x826345C0; continue 'dispatch;
	}
	// 8263457C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82634580: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634584: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634588: 392A3CB8  addi r9, r10, 0x3cb8
	ctx.r[9].s64 = ctx.r[10].s64 + 15544;
	// 8263458C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82634590: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634594: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634598: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8263459C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826345A0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826345A4: 99030018  stb r8, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u8 ) };
	// 826345A8: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 826345AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826345B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826345B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826345B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826345BC: 4E800020  blr
	return;
	// 826345C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826345C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826345C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826345CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826345D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826345D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826345D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826345D8 size=280
    let mut pc: u32 = 0x826345D8;
    'dispatch: loop {
        match pc {
            0x826345D8 => {
    //   block [0x826345D8..0x826346F0)
	// 826345D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826345DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826345E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826345E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826345E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826345EC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826345F0: 4BBEAC69  bl 0x8221f258
	ctx.lr = 0x826345F4;
	sub_8221F258(ctx, base);
	// 826345F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826345F8: 419A00E0  beq cr6, 0x826346d8
	if ctx.cr[6].eq {
	pc = 0x826346D8; continue 'dispatch;
	}
	// 826345FC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82634600: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82634604: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 82634608: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263460C: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82634610: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 82634614: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634618: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 8263461C: 38AABD44  addi r5, r10, -0x42bc
	ctx.r[5].s64 = ctx.r[10].s64 + -17084;
	// 82634620: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634624: 388855A8  addi r4, r8, 0x55a8
	ctx.r[4].s64 = ctx.r[8].s64 + 21928;
	// 82634628: C1A99490  lfs f13, -0x6b70(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8263462C: 394755E8  addi r10, r7, 0x55e8
	ctx.r[10].s64 = ctx.r[7].s64 + 21992;
	// 82634630: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82634634: 39099490  addi r8, r9, -0x6b70
	ctx.r[8].s64 = ctx.r[9].s64 + -27504;
	// 82634638: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8263463C: 38E60B7C  addi r7, r6, 0xb7c
	ctx.r[7].s64 = ctx.r[6].s64 + 2940;
	// 82634640: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82634644: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82634648: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8263464C: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 82634650: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82634654: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634658: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263465C: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82634660: C008FFF4  lfs f0, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634664: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82634668: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 8263466C: 90E30020  stw r7, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[7].u32 ) };
	// 82634670: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82634674: D1A30040  stfs f13, 0x40(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634678: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8263467C: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82634680: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82634684: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82634688: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8263468C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82634690: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826346F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826346F0 size=128
    let mut pc: u32 = 0x826346F0;
    'dispatch: loop {
        match pc {
            0x826346F0 => {
    //   block [0x826346F0..0x82634770)
	// 826346F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826346F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826346F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826346FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634704: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82634708: 4BBEAB51  bl 0x8221f258
	ctx.lr = 0x8263470C;
	sub_8221F258(ctx, base);
	// 8263470C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634710: 419A0048  beq cr6, 0x82634758
	if ctx.cr[6].eq {
	pc = 0x82634758; continue 'dispatch;
	}
	// 82634714: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82634718: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263471C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634724: 390A36F0  addi r8, r10, 0x36f0
	ctx.r[8].s64 = ctx.r[10].s64 + 14064;
	// 82634728: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 8263472C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634730: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82634734: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82634738: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8263473C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634740: 90C30014  stw r6, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82634744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263474C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634750: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634754: 4E800020  blr
	return;
	// 82634758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263475C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263476C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634770 size=112
    let mut pc: u32 = 0x82634770;
    'dispatch: loop {
        match pc {
            0x82634770 => {
    //   block [0x82634770..0x826347E0)
	// 82634770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263477C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634784: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82634788: 4BBEAAD1  bl 0x8221f258
	ctx.lr = 0x8263478C;
	sub_8221F258(ctx, base);
	// 8263478C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634790: 419A0038  beq cr6, 0x826347c8
	if ctx.cr[6].eq {
	pc = 0x826347C8; continue 'dispatch;
	}
	// 82634794: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634798: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263479C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826347A0: 392A5A20  addi r9, r10, 0x5a20
	ctx.r[9].s64 = ctx.r[10].s64 + 23072;
	// 826347A4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826347A8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826347AC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 826347B0: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 826347B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826347B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826347BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826347C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826347C4: 4E800020  blr
	return;
	// 826347C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826347CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826347D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826347D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826347D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826347DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826347E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826347E0 size=128
    let mut pc: u32 = 0x826347E0;
    'dispatch: loop {
        match pc {
            0x826347E0 => {
    //   block [0x826347E0..0x82634860)
	// 826347E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826347E4: 48674C29  bl 0x82ca940c
	ctx.lr = 0x826347E8;
	sub_82CA93D0(ctx, base);
	// 826347E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826347EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826347F0: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 826347F4: 4BBEAA65  bl 0x8221f258
	ctx.lr = 0x826347F8;
	sub_8221F258(ctx, base);
	// 826347F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826347FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634800: 419A0054  beq cr6, 0x82634854
	if ctx.cr[6].eq {
	pc = 0x82634854; continue 'dispatch;
	}
	// 82634804: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82634808: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263480C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82634810: 394B3938  addi r10, r11, 0x3938
	ctx.r[10].s64 = ctx.r[11].s64 + 14648;
	// 82634814: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82634818: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8263481C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82634820: 9BDF000C  stb r30, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u8 ) };
	// 82634824: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82634828: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8263482C: 48213785  bl 0x82847fb0
	ctx.lr = 0x82634830;
	sub_82847FB0(ctx, base);
	// 82634830: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82634834: 4821377D  bl 0x82847fb0
	ctx.lr = 0x82634838;
	sub_82847FB0(ctx, base);
	// 82634838: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8263483C: 48213775  bl 0x82847fb0
	ctx.lr = 0x82634840;
	sub_82847FB0(ctx, base);
	// 82634840: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82634844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634848: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 8263484C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634850: 48674C0C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82634854: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263485C: 48674C00  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82634860 size=8
    let mut pc: u32 = 0x82634860;
    'dispatch: loop {
        match pc {
            0x82634860 => {
    //   block [0x82634860..0x82634868)
	// 82634860: 38600082  li r3, 0x82
	ctx.r[3].s64 = 130;
	// 82634864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634868 size=116
    let mut pc: u32 = 0x82634868;
    'dispatch: loop {
        match pc {
            0x82634868 => {
    //   block [0x82634868..0x826348DC)
	// 82634868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263486C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263487C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82634880: 4BBEA9D9  bl 0x8221f258
	ctx.lr = 0x82634884;
	sub_8221F258(ctx, base);
	// 82634884: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634888: 419A003C  beq cr6, 0x826348c4
	if ctx.cr[6].eq {
	pc = 0x826348C4; continue 'dispatch;
	}
	// 8263488C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82634890: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634898: 392AA250  addi r9, r10, -0x5db0
	ctx.r[9].s64 = ctx.r[10].s64 + -23984;
	// 8263489C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826348A0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826348A4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826348A8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 826348AC: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826348B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826348B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826348B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826348BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826348C0: 4E800020  blr
	return;
	// 826348C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826348C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826348CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826348D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826348D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826348D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826348E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826348E0 size=60
    let mut pc: u32 = 0x826348E0;
    'dispatch: loop {
        match pc {
            0x826348E0 => {
    //   block [0x826348E0..0x8263491C)
	// 826348E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826348E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826348E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826348EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826348F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 826348F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 826348F8: 388B9770  addi r4, r11, -0x6890
	ctx.r[4].s64 = ctx.r[11].s64 + -26768;
	// 826348FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634900: 4BBF85D1  bl 0x8222ced0
	ctx.lr = 0x82634904;
	sub_8222CED0(ctx, base);
	// 82634904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263490C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634920 size=116
    let mut pc: u32 = 0x82634920;
    'dispatch: loop {
        match pc {
            0x82634920 => {
    //   block [0x82634920..0x82634994)
	// 82634920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634924: 48674AE9  bl 0x82ca940c
	ctx.lr = 0x82634928;
	sub_82CA93D0(ctx, base);
	// 82634928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263492C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634930: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82634934: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82634938: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8263493C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82634940: 419A0018  beq cr6, 0x82634958
	if ctx.cr[6].eq {
	pc = 0x82634958; continue 'dispatch;
	}
	// 82634944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634948: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8263494C: 48005E9D  bl 0x8263a7e8
	ctx.lr = 0x82634950;
	sub_8263A7E8(ctx, base);
	// 82634950: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82634954: 4BBE73E5  bl 0x8221bd38
	ctx.lr = 0x82634958;
	sub_8221BD38(ctx, base);
	// 82634958: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8263495C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634960: 392A2850  addi r9, r10, 0x2850
	ctx.r[9].s64 = ctx.r[10].s64 + 10320;
	// 82634964: 57A807FE  clrlwi r8, r29, 0x1f
	ctx.r[8].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	// 82634968: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8263496C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82634970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82634974: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634978: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8263497C: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634980: 419A000C  beq cr6, 0x8263498c
	if ctx.cr[6].eq {
	pc = 0x8263498C; continue 'dispatch;
	}
	// 82634984: 4BBE73B5  bl 0x8221bd38
	ctx.lr = 0x82634988;
	sub_8221BD38(ctx, base);
	// 82634988: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8263498C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634990: 48674ACC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634998 size=88
    let mut pc: u32 = 0x82634998;
    'dispatch: loop {
        match pc {
            0x82634998 => {
    //   block [0x82634998..0x826349F0)
	// 82634998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263499C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826349A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826349A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826349A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826349AC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 826349B0: 4BBEA8A9  bl 0x8221f258
	ctx.lr = 0x826349B4;
	sub_8221F258(ctx, base);
	// 826349B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826349B8: 419A0020  beq cr6, 0x826349d8
	if ctx.cr[6].eq {
	pc = 0x826349D8; continue 'dispatch;
	}
	// 826349BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826349C0: 4BFA7E69  bl 0x825dc828
	ctx.lr = 0x826349C4;
	sub_825DC828(ctx, base);
	// 826349C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826349C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826349CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826349D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826349D4: 4E800020  blr
	return;
	// 826349D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826349DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826349E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826349E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826349E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826349EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826349F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826349F0 size=136
    let mut pc: u32 = 0x826349F0;
    'dispatch: loop {
        match pc {
            0x826349F0 => {
    //   block [0x826349F0..0x82634A78)
	// 826349F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826349F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826349F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826349FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634A00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634A04: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82634A08: 4BBEA851  bl 0x8221f258
	ctx.lr = 0x82634A0C;
	sub_8221F258(ctx, base);
	// 82634A0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634A10: 419A0050  beq cr6, 0x82634a60
	if ctx.cr[6].eq {
	pc = 0x82634A60; continue 'dispatch;
	}
	// 82634A14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82634A18: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634A1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634A20: 392A4780  addi r9, r10, 0x4780
	ctx.r[9].s64 = ctx.r[10].s64 + 18304;
	// 82634A24: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634A28: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634A2C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634A30: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634A34: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82634A38: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82634A3C: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 82634A40: 9963001E  stb r11, 0x1e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u8 ) };
	// 82634A44: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82634A48: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82634A4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634A5C: 4E800020  blr
	return;
	// 82634A60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634A64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634A70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634A78 size=204
    let mut pc: u32 = 0x82634A78;
    'dispatch: loop {
        match pc {
            0x82634A78 => {
    //   block [0x82634A78..0x82634B44)
	// 82634A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634A84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634A88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634A8C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82634A90: 4BBEA7C9  bl 0x8221f258
	ctx.lr = 0x82634A94;
	sub_8221F258(ctx, base);
	// 82634A94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634A98: 419A0094  beq cr6, 0x82634b2c
	if ctx.cr[6].eq {
	pc = 0x82634B2C; continue 'dispatch;
	}
	// 82634A9C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 82634AA0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634AA4: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82634AA8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 82634AAC: 38E99484  addi r7, r9, -0x6b7c
	ctx.r[7].s64 = ctx.r[9].s64 + -27516;
	// 82634AB0: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82634AB4: 814B8824  lwz r10, -0x77dc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30684 as u32) ) } as u64;
	// 82634AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634ABC: 38A82D10  addi r5, r8, 0x2d10
	ctx.r[5].s64 = ctx.r[8].s64 + 11536;
	// 82634AC0: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634AC4: 38860B7C  addi r4, r6, 0xb7c
	ctx.r[4].s64 = ctx.r[6].s64 + 2940;
	// 82634AC8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634ACC: C1A7000C  lfs f13, 0xc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82634AD0: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82634AD4: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82634AD8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634ADC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82634AE0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634AE4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634AE8: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82634AEC: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82634AF0: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82634AF4: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82634AF8: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82634AFC: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82634B00: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82634B04: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82634B08: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82634B0C: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82634B10: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82634B14: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82634B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634B24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634B28: 4E800020  blr
	return;
	// 82634B2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634B48 size=176
    let mut pc: u32 = 0x82634B48;
    'dispatch: loop {
        match pc {
            0x82634B48 => {
    //   block [0x82634B48..0x82634BF8)
	// 82634B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634B4C: 486748BD  bl 0x82ca9408
	ctx.lr = 0x82634B50;
	sub_82CA93D0(ctx, base);
	// 82634B50: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82634B54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634B58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82634B5C: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82634B60: 4BBEA6F9  bl 0x8221f258
	ctx.lr = 0x82634B64;
	sub_8221F258(ctx, base);
	// 82634B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634B68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634B6C: 419A007C  beq cr6, 0x82634be8
	if ctx.cr[6].eq {
	pc = 0x82634BE8; continue 'dispatch;
	}
	// 82634B70: 3F80820A  lis r28, -0x7df6
	ctx.r[28].s64 = -2113273856;
	// 82634B74: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82634B78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82634B7C: 395C9490  addi r10, r28, -0x6b70
	ctx.r[10].s64 = ctx.r[28].s64 + -27504;
	// 82634B80: 392B32A8  addi r9, r11, 0x32a8
	ctx.r[9].s64 = ctx.r[11].s64 + 12968;
	// 82634B84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82634B88: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82634B8C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82634B90: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82634B94: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82634B98: C3EAFFF4  lfs f31, -0xc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82634B9C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82634BA0: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82634BA4: 4BE15225  bl 0x82449dc8
	ctx.lr = 0x82634BA8;
	sub_82449DC8(ctx, base);
	// 82634BA8: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82634BAC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82634BB0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82634BB4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82634BB8: 4BE15211  bl 0x82449dc8
	ctx.lr = 0x82634BBC;
	sub_82449DC8(ctx, base);
	// 82634BBC: C01C9490  lfs f0, -0x6b70(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634BC0: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82634BC4: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82634BC8: D3FF0040  stfs f31, 0x40(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634BD0: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82634BD4: 9BDF0038  stb r30, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 82634BD8: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82634BDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82634BE0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82634BE4: 48674874  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82634BE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634BEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82634BF0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82634BF4: 48674864  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634BF8 size=280
    let mut pc: u32 = 0x82634BF8;
    'dispatch: loop {
        match pc {
            0x82634BF8 => {
    //   block [0x82634BF8..0x82634D10)
	// 82634BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634C04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634C0C: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82634C10: 4BBEA649  bl 0x8221f258
	ctx.lr = 0x82634C14;
	sub_8221F258(ctx, base);
	// 82634C14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634C18: 419A00E0  beq cr6, 0x82634cf8
	if ctx.cr[6].eq {
	pc = 0x82634CF8; continue 'dispatch;
	}
	// 82634C1C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82634C20: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82634C24: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 82634C28: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634C2C: 390A9490  addi r8, r10, -0x6b70
	ctx.r[8].s64 = ctx.r[10].s64 + -27504;
	// 82634C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634C34: 388904D4  addi r4, r9, 0x4d4
	ctx.r[4].s64 = ctx.r[9].s64 + 1236;
	// 82634C38: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82634C3C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634C40: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 82634C44: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634C48: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82634C4C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82634C50: 39200060  li r9, 0x60
	ctx.r[9].s64 = 96;
	// 82634C54: C1A8FFF4  lfs f13, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82634C58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82634C5C: C188FE44  lfs f12, -0x1bc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-444 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82634C60: 38E73F78  addi r7, r7, 0x3f78
	ctx.r[7].s64 = ctx.r[7].s64 + 16248;
	// 82634C64: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82634C68: 38C64F50  addi r6, r6, 0x4f50
	ctx.r[6].s64 = ctx.r[6].s64 + 20304;
	// 82634C6C: 91430024  stw r10, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82634C70: 38A50B7C  addi r5, r5, 0xb7c
	ctx.r[5].s64 = ctx.r[5].s64 + 2940;
	// 82634C74: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82634C78: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82634C7C: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82634C80: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82634C84: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82634C88: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82634C8C: D1830038  stfs f12, 0x38(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82634C90: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634C94: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82634C98: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82634C9C: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82634CA0: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82634CA4: 38E80E0C  addi r7, r8, 0xe0c
	ctx.r[7].s64 = ctx.r[8].s64 + 3596;
	// 82634CA8: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82634CAC: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82634CB0: 90A30050  stw r5, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D10 size=88
    let mut pc: u32 = 0x82634D10;
    'dispatch: loop {
        match pc {
            0x82634D10 => {
    //   block [0x82634D10..0x82634D68)
	// 82634D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634D1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634D24: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82634D28: 4BBEA531  bl 0x8221f258
	ctx.lr = 0x82634D2C;
	sub_8221F258(ctx, base);
	// 82634D2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634D30: 419A0020  beq cr6, 0x82634d50
	if ctx.cr[6].eq {
	pc = 0x82634D50; continue 'dispatch;
	}
	// 82634D34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634D38: 4BDFC341  bl 0x82431078
	ctx.lr = 0x82634D3C;
	sub_82431078(ctx, base);
	// 82634D3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634D4C: 4E800020  blr
	return;
	// 82634D50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634D60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634D68 size=88
    let mut pc: u32 = 0x82634D68;
    'dispatch: loop {
        match pc {
            0x82634D68 => {
    //   block [0x82634D68..0x82634DC0)
	// 82634D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634D70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634D74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634D78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634D7C: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82634D80: 4BBEA4D9  bl 0x8221f258
	ctx.lr = 0x82634D84;
	sub_8221F258(ctx, base);
	// 82634D84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634D88: 419A0020  beq cr6, 0x82634da8
	if ctx.cr[6].eq {
	pc = 0x82634DA8; continue 'dispatch;
	}
	// 82634D8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82634D90: 48175051  bl 0x827a9de0
	ctx.lr = 0x82634D94;
	sub_827A9DE0(ctx, base);
	// 82634D94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634DA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634DA4: 4E800020  blr
	return;
	// 82634DA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82634DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634DC0 size=168
    let mut pc: u32 = 0x82634DC0;
    'dispatch: loop {
        match pc {
            0x82634DC0 => {
    //   block [0x82634DC0..0x82634E68)
	// 82634DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82634DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634DD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82634DD8: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 82634DDC: 4BBEA47D  bl 0x8221f258
	ctx.lr = 0x82634DE0;
	sub_8221F258(ctx, base);
	// 82634DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634DE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82634DE8: 419A0064  beq cr6, 0x82634e4c
	if ctx.cr[6].eq {
	pc = 0x82634E4C; continue 'dispatch;
	}
	// 82634DEC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634DF0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82634DF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82634DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82634DFC: 39092E68  addi r8, r9, 0x2e68
	ctx.r[8].s64 = ctx.r[9].s64 + 11880;
	// 82634E00: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634E04: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82634E08: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 82634E0C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82634E10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 82634E14: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634E18: 386727D8  addi r3, r7, 0x27d8
	ctx.r[3].s64 = ctx.r[7].s64 + 10200;
	// 82634E1C: 997F001E  stb r11, 0x1e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[11].u8 ) };
	// 82634E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 82634E24: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82634E28: 4BBBEF31  bl 0x821f3d58
	ctx.lr = 0x82634E2C;
	sub_821F3D58(ctx, base);
	// 82634E2C: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 82634E30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82634E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82634E38: 80666DA0  lwz r3, 0x6da0(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28064 as u32) ) } as u64;
	// 82634E3C: 4BC9868D  bl 0x822cd4c8
	ctx.lr = 0x82634E40;
	sub_822CD4C8(ctx, base);
	// 82634E40: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82634E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82634E48: 48000008  b 0x82634e50
	pc = 0x82634E50; continue 'dispatch;
	// 82634E4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82634E50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82634E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82634E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82634E5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82634E60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82634E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82634E68 size=244
    let mut pc: u32 = 0x82634E68;
    'dispatch: loop {
        match pc {
            0x82634E68 => {
    //   block [0x82634E68..0x82634F5C)
	// 82634E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634E70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634E74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634E78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634E7C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82634E80: 4BBEA3D9  bl 0x8221f258
	ctx.lr = 0x82634E84;
	sub_8221F258(ctx, base);
	// 82634E84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634E88: 419A00BC  beq cr6, 0x82634f44
	if ctx.cr[6].eq {
	pc = 0x82634F44; continue 'dispatch;
	}
	// 82634E8C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82634E90: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82634E94: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82634E98: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634E9C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82634EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634EA4: 38EA61E0  addi r7, r10, 0x61e0
	ctx.r[7].s64 = ctx.r[10].s64 + 25056;
	// 82634EA8: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 82634EAC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634EB0: 38A99484  addi r5, r9, -0x6b7c
	ctx.r[5].s64 = ctx.r[9].s64 + -27516;
	// 82634EB4: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82634EB8: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82634EBC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82634EC0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82634EC4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82634EC8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82634ECC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82634ED0: 90C30014  stw r6, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82634ED4: C1899484  lfs f12, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82634ED8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634EDC: C005000C  lfs f0, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82634EE0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82634EE4: C1A54144  lfs f13, 0x4144(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82634EE8: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82634EEC: 90C30024  stw r6, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 82634EF0: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82634EF4: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82634EF8: 91430044  stw r10, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82634F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82634F60 size=216
    let mut pc: u32 = 0x82634F60;
    'dispatch: loop {
        match pc {
            0x82634F60 => {
    //   block [0x82634F60..0x82635038)
	// 82634F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82634F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82634F68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82634F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82634F70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82634F74: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 82634F78: 4BBEA2E1  bl 0x8221f258
	ctx.lr = 0x82634F7C;
	sub_8221F258(ctx, base);
	// 82634F7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82634F80: 419A00A0  beq cr6, 0x82635020
	if ctx.cr[6].eq {
	pc = 0x82635020; continue 'dispatch;
	}
	// 82634F84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82634F88: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82634F8C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82634F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82634F94: 390A3EB8  addi r8, r10, 0x3eb8
	ctx.r[8].s64 = ctx.r[10].s64 + 16056;
	// 82634F98: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 82634F9C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82634FA0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82634FA4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82634FA8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82634FAC: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82634FB0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82634FB4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82634FB8: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82634FBC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82634FC0: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82634FC4: 90E30024  stw r7, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82634FC8: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82634FCC: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82634FD0: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82634FD4: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82634FD8: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82634FDC: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82634FE0: 99630044  stb r11, 0x44(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 82634FE4: 99630045  stb r11, 0x45(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(69 as u32), ctx.r[11].u8 ) };
	// 82634FE8: 99630046  stb r11, 0x46(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(70 as u32), ctx.r[11].u8 ) };
	// 82634FEC: 99630047  stb r11, 0x47(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(71 as u32), ctx.r[11].u8 ) };
	// 82634FF0: 99630048  stb r11, 0x48(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 82634FF4: 99630049  stb r11, 0x49(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(73 as u32), ctx.r[11].u8 ) };
	// 82634FF8: 9963004A  stb r11, 0x4a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 82634FFC: 9963004B  stb r11, 0x4b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(75 as u32), ctx.r[11].u8 ) };
	// 82635000: 98C3004C  stb r6, 0x4c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[6].u8 ) };
	// 82635004: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82635008: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263500C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263501C: 4E800020  blr
	return;
	// 82635020: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263502C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635038 size=104
    let mut pc: u32 = 0x82635038;
    'dispatch: loop {
        match pc {
            0x82635038 => {
    //   block [0x82635038..0x826350A0)
	// 82635038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263503C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635040: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635044: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263504C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82635050: 4BBEA209  bl 0x8221f258
	ctx.lr = 0x82635054;
	sub_8221F258(ctx, base);
	// 82635054: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635058: 419A0030  beq cr6, 0x82635088
	if ctx.cr[6].eq {
	pc = 0x82635088; continue 'dispatch;
	}
	// 8263505C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635060: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635068: 392A46C0  addi r9, r10, 0x46c0
	ctx.r[9].s64 = ctx.r[10].s64 + 18112;
	// 8263506C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635070: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263507C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635084: 4E800020  blr
	return;
	// 82635088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263508C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263509C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826350A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826350A0 size=112
    let mut pc: u32 = 0x826350A0;
    'dispatch: loop {
        match pc {
            0x826350A0 => {
    //   block [0x826350A0..0x82635110)
	// 826350A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826350A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826350A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826350AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826350B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826350B4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 826350B8: 4BBEA1A1  bl 0x8221f258
	ctx.lr = 0x826350BC;
	sub_8221F258(ctx, base);
	// 826350BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826350C0: 419A0038  beq cr6, 0x826350f8
	if ctx.cr[6].eq {
	pc = 0x826350F8; continue 'dispatch;
	}
	// 826350C4: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 826350C8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826350CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826350D0: 392A6640  addi r9, r10, 0x6640
	ctx.r[9].s64 = ctx.r[10].s64 + 26176;
	// 826350D4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826350D8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826350DC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 826350E0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826350E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826350E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826350EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826350F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826350F4: 4E800020  blr
	return;
	// 826350F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826350FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635110 size=120
    let mut pc: u32 = 0x82635110;
    'dispatch: loop {
        match pc {
            0x82635110 => {
    //   block [0x82635110..0x82635188)
	// 82635110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263511C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635124: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82635128: 4BBEA131  bl 0x8221f258
	ctx.lr = 0x8263512C;
	sub_8221F258(ctx, base);
	// 8263512C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635130: 419A0040  beq cr6, 0x82635170
	if ctx.cr[6].eq {
	pc = 0x82635170; continue 'dispatch;
	}
	// 82635134: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635138: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263513C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82635140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635144: 390A6720  addi r8, r10, 0x6720
	ctx.r[8].s64 = ctx.r[10].s64 + 26400;
	// 82635148: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 8263514C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635150: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82635154: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82635158: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263515C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263516C: 4E800020  blr
	return;
	// 82635170: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263517C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635188 size=88
    let mut pc: u32 = 0x82635188;
    'dispatch: loop {
        match pc {
            0x82635188 => {
    //   block [0x82635188..0x826351E0)
	// 82635188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263518C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263519C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 826351A0: 4BBEA0B9  bl 0x8221f258
	ctx.lr = 0x826351A4;
	sub_8221F258(ctx, base);
	// 826351A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826351A8: 419A0020  beq cr6, 0x826351c8
	if ctx.cr[6].eq {
	pc = 0x826351C8; continue 'dispatch;
	}
	// 826351AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826351B0: 482995B1  bl 0x828ce760
	ctx.lr = 0x826351B4;
	sub_828CE760(ctx, base);
	// 826351B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826351B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826351BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826351C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826351C4: 4E800020  blr
	return;
	// 826351C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826351CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826351D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826351D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826351D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826351DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826351E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826351E0 size=160
    let mut pc: u32 = 0x826351E0;
    'dispatch: loop {
        match pc {
            0x826351E0 => {
    //   block [0x826351E0..0x82635280)
	// 826351E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826351E4: 48674229  bl 0x82ca940c
	ctx.lr = 0x826351E8;
	sub_82CA93D0(ctx, base);
	// 826351E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826351EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826351F0: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 826351F4: 4BBEA065  bl 0x8221f258
	ctx.lr = 0x826351F8;
	sub_8221F258(ctx, base);
	// 826351F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826351FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635200: 419A0074  beq cr6, 0x82635274
	if ctx.cr[6].eq {
	pc = 0x82635274; continue 'dispatch;
	}
	// 82635204: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82635208: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263520C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635210: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635214: 392B39A8  addi r9, r11, 0x39a8
	ctx.r[9].s64 = ctx.r[11].s64 + 14760;
	// 82635218: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 8263521C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635220: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635224: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82635228: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8263522C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82635230: 4BB5EC09  bl 0x82193e38
	ctx.lr = 0x82635234;
	sub_82193E38(ctx, base);
	// 82635234: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 82635238: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8263523C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635240: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82635244: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82635248: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263524C: C0079484  lfs f0, -0x6b7c(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635250: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82635254: 4BD58265  bl 0x8238d4b8
	ctx.lr = 0x82635258;
	sub_8238D4B8(ctx, base);
	// 82635258: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8263525C: 88A60090  lbz r5, 0x90(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(144 as u32) ) } as u64;
	// 82635260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635264: 60A40001  ori r4, r5, 1
	ctx.r[4].u64 = ctx.r[5].u64 | 1;
	// 82635268: 98860090  stb r4, 0x90(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(144 as u32), ctx.r[4].u8 ) };
	// 8263526C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635270: 486741EC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82635274: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635278: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263527C: 486741E0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635280 size=88
    let mut pc: u32 = 0x82635280;
    'dispatch: loop {
        match pc {
            0x82635280 => {
    //   block [0x82635280..0x826352D8)
	// 82635280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263528C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635294: 38600104  li r3, 0x104
	ctx.r[3].s64 = 260;
	// 82635298: 4BBE9FC1  bl 0x8221f258
	ctx.lr = 0x8263529C;
	sub_8221F258(ctx, base);
	// 8263529C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826352A0: 419A0020  beq cr6, 0x826352c0
	if ctx.cr[6].eq {
	pc = 0x826352C0; continue 'dispatch;
	}
	// 826352A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826352A8: 481A1C79  bl 0x827d6f20
	ctx.lr = 0x826352AC;
	sub_827D6F20(ctx, base);
	// 826352AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826352B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826352B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826352B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826352BC: 4E800020  blr
	return;
	// 826352C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826352C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826352C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826352CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826352D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826352D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826352D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826352D8 size=124
    let mut pc: u32 = 0x826352D8;
    'dispatch: loop {
        match pc {
            0x826352D8 => {
    //   block [0x826352D8..0x82635354)
	// 826352D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826352DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826352E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826352E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826352E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826352EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826352F0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 826352F4: 4BBE9F65  bl 0x8221f258
	ctx.lr = 0x826352F8;
	sub_8221F258(ctx, base);
	// 826352F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826352FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635300: 419A0038  beq cr6, 0x82635338
	if ctx.cr[6].eq {
	pc = 0x82635338; continue 'dispatch;
	}
	// 82635304: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635308: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263530C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635310: 392A2A50  addi r9, r10, 0x2a50
	ctx.r[9].s64 = ctx.r[10].s64 + 10832;
	// 82635314: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635318: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8263531C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82635324: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635328: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8263532C: 4BD5818D  bl 0x8238d4b8
	ctx.lr = 0x82635330;
	sub_8238D4B8(ctx, base);
	// 82635330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635334: 48000008  b 0x8263533c
	pc = 0x8263533C; continue 'dispatch;
	// 82635338: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263533C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8263534C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635358 size=192
    let mut pc: u32 = 0x82635358;
    'dispatch: loop {
        match pc {
            0x82635358 => {
    //   block [0x82635358..0x82635418)
	// 82635358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263535C: 486740B1  bl 0x82ca940c
	ctx.lr = 0x82635360;
	sub_82CA93D0(ctx, base);
	// 82635360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635364: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82635368: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 8263536C: 4BBE9EED  bl 0x8221f258
	ctx.lr = 0x82635370;
	sub_8221F258(ctx, base);
	// 82635370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635374: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635378: 419A0094  beq cr6, 0x8263540c
	if ctx.cr[6].eq {
	pc = 0x8263540C; continue 'dispatch;
	}
	// 8263537C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82635380: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82635384: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635388: 394B3BF8  addi r10, r11, 0x3bf8
	ctx.r[10].s64 = ctx.r[11].s64 + 15352;
	// 8263538C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635390: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82635394: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82635398: 4BBE9EC1  bl 0x8221f258
	ctx.lr = 0x8263539C;
	sub_8221F258(ctx, base);
	// 8263539C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826353A0: 419A0008  beq cr6, 0x826353a8
	if ctx.cr[6].eq {
	pc = 0x826353A8; continue 'dispatch;
	}
	// 826353A4: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826353A8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 826353AC: 41820008  beq 0x826353b4
	if ctx.cr[0].eq {
	pc = 0x826353B4; continue 'dispatch;
	}
	// 826353B0: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826353B4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 826353B8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 826353BC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826353C0: 4BBE9E99  bl 0x8221f258
	ctx.lr = 0x826353C4;
	sub_8221F258(ctx, base);
	// 826353C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826353C8: 419A0008  beq cr6, 0x826353d0
	if ctx.cr[6].eq {
	pc = 0x826353D0; continue 'dispatch;
	}
	// 826353CC: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826353D0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 826353D4: 41820008  beq 0x826353dc
	if ctx.cr[0].eq {
	pc = 0x826353DC; continue 'dispatch;
	}
	// 826353D8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 826353DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 826353E0: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 826353E4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 826353E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826353EC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 826353F0: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 826353F4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 826353F8: 9BDF0030  stb r30, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u8 ) };
	// 826353FC: 9BDF0031  stb r30, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[30].u8 ) };
	// 82635400: 9BDF0032  stb r30, 0x32(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(50 as u32), ctx.r[30].u8 ) };
	// 82635404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635408: 48674054  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8263540C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635410: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635414: 48674048  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635418 size=88
    let mut pc: u32 = 0x82635418;
    'dispatch: loop {
        match pc {
            0x82635418 => {
    //   block [0x82635418..0x82635470)
	// 82635418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635424: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263542C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82635430: 4BBE9E29  bl 0x8221f258
	ctx.lr = 0x82635434;
	sub_8221F258(ctx, base);
	// 82635434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635438: 419A0020  beq cr6, 0x82635458
	if ctx.cr[6].eq {
	pc = 0x82635458; continue 'dispatch;
	}
	// 8263543C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635440: 48302C49  bl 0x82938088
	ctx.lr = 0x82635444;
	sub_82938088(ctx, base);
	// 82635444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263544C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635454: 4E800020  blr
	return;
	// 82635458: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263545C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635468: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263546C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635470 size=88
    let mut pc: u32 = 0x82635470;
    'dispatch: loop {
        match pc {
            0x82635470 => {
    //   block [0x82635470..0x826354C8)
	// 82635470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263547C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635484: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82635488: 4BBE9DD1  bl 0x8221f258
	ctx.lr = 0x8263548C;
	sub_8221F258(ctx, base);
	// 8263548C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635490: 419A0020  beq cr6, 0x826354b0
	if ctx.cr[6].eq {
	pc = 0x826354B0; continue 'dispatch;
	}
	// 82635494: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635498: 482C9DB1  bl 0x828ff248
	ctx.lr = 0x8263549C;
	sub_828FF248(ctx, base);
	// 8263549C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826354A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826354A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826354A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826354AC: 4E800020  blr
	return;
	// 826354B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826354B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826354B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826354BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826354C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826354C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826354C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826354C8 size=164
    let mut pc: u32 = 0x826354C8;
    'dispatch: loop {
        match pc {
            0x826354C8 => {
    //   block [0x826354C8..0x8263556C)
	// 826354C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826354CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826354D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826354D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826354D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826354DC: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 826354E0: 4BBE9D79  bl 0x8221f258
	ctx.lr = 0x826354E4;
	sub_8221F258(ctx, base);
	// 826354E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826354E8: 419A006C  beq cr6, 0x82635554
	if ctx.cr[6].eq {
	pc = 0x82635554; continue 'dispatch;
	}
	// 826354EC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 826354F0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826354F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 826354F8: 38E99490  addi r7, r9, -0x6b70
	ctx.r[7].s64 = ctx.r[9].s64 + -27504;
	// 826354FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635500: 390A4CE8  addi r8, r10, 0x4ce8
	ctx.r[8].s64 = ctx.r[10].s64 + 19688;
	// 82635504: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82635508: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263550C: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82635510: C1499490  lfs f10, -0x6b70(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82635514: C007FFF4  lfs f0, -0xc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635518: C1A75D38  lfs f13, 0x5d38(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(23864 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8263551C: C18729D4  lfs f12, 0x29d4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(10708 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82635520: C1671FF0  lfs f11, 0x1ff0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8176 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82635524: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82635528: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8263552C: D1830014  stfs f12, 0x14(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635530: D1630018  stfs f11, 0x18(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635534: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82635538: D143001C  stfs f10, 0x1c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8263553C: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82635540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263554C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635550: 4E800020  blr
	return;
	// 82635554: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263555C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635570 size=104
    let mut pc: u32 = 0x82635570;
    'dispatch: loop {
        match pc {
            0x82635570 => {
    //   block [0x82635570..0x826355D8)
	// 82635570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8263557C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635584: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82635588: 4BBE9CD1  bl 0x8221f258
	ctx.lr = 0x8263558C;
	sub_8221F258(ctx, base);
	// 8263558C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635590: 419A0030  beq cr6, 0x826355c0
	if ctx.cr[6].eq {
	pc = 0x826355C0; continue 'dispatch;
	}
	// 82635594: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635598: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8263559C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826355A0: 392A7248  addi r9, r10, 0x7248
	ctx.r[9].s64 = ctx.r[10].s64 + 29256;
	// 826355A4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826355A8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826355AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826355B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826355B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826355B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826355BC: 4E800020  blr
	return;
	// 826355C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826355C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826355C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826355CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826355D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826355D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826355D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826355D8 size=160
    let mut pc: u32 = 0x826355D8;
    'dispatch: loop {
        match pc {
            0x826355D8 => {
    //   block [0x826355D8..0x82635678)
	// 826355D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826355DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826355E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826355E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826355E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826355EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826355F0: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 826355F4: 4BBE9C65  bl 0x8221f258
	ctx.lr = 0x826355F8;
	sub_8221F258(ctx, base);
	// 826355F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826355FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635600: 419A005C  beq cr6, 0x8263565c
	if ctx.cr[6].eq {
	pc = 0x8263565C; continue 'dispatch;
	}
	// 82635604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82635608: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263560C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635610: 392BD5C8  addi r9, r11, -0x2a38
	ctx.r[9].s64 = ctx.r[11].s64 + -10808;
	// 82635614: 390A7468  addi r8, r10, 0x7468
	ctx.r[8].s64 = ctx.r[10].s64 + 29800;
	// 82635618: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263561C: C00BD5C8  lfs f0, -0x2a38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635620: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82635624: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82635628: 98FF0008  stb r7, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 8263562C: C009BEBC  lfs f0, -0x4144(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635630: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82635634: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82635638: 98DF0020  stb r6, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[6].u8 ) };
	// 8263563C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82635640: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635644: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8263564C: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635650: 4BD57E69  bl 0x8238d4b8
	ctx.lr = 0x82635654;
	sub_8238D4B8(ctx, base);
	// 82635654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635658: 48000008  b 0x82635660
	pc = 0x82635660; continue 'dispatch;
	// 8263565C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635660: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263566C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635678 size=88
    let mut pc: u32 = 0x82635678;
    'dispatch: loop {
        match pc {
            0x82635678 => {
    //   block [0x82635678..0x826356D0)
	// 82635678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263567C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263568C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82635690: 4BBE9BC9  bl 0x8221f258
	ctx.lr = 0x82635694;
	sub_8221F258(ctx, base);
	// 82635694: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635698: 419A0020  beq cr6, 0x826356b8
	if ctx.cr[6].eq {
	pc = 0x826356B8; continue 'dispatch;
	}
	// 8263569C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826356A0: 48316E59  bl 0x8294c4f8
	ctx.lr = 0x826356A4;
	sub_8294C4F8(ctx, base);
	// 826356A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826356A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826356AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826356B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826356B4: 4E800020  blr
	return;
	// 826356B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826356BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826356C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826356C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826356C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826356CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826356D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826356D0 size=88
    let mut pc: u32 = 0x826356D0;
    'dispatch: loop {
        match pc {
            0x826356D0 => {
    //   block [0x826356D0..0x82635728)
	// 826356D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826356D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826356D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826356DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826356E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826356E4: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 826356E8: 4BBE9B71  bl 0x8221f258
	ctx.lr = 0x826356EC;
	sub_8221F258(ctx, base);
	// 826356EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826356F0: 419A0020  beq cr6, 0x82635710
	if ctx.cr[6].eq {
	pc = 0x82635710; continue 'dispatch;
	}
	// 826356F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826356F8: 482C1D71  bl 0x828f7468
	ctx.lr = 0x826356FC;
	sub_828F7468(ctx, base);
	// 826356FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263570C: 4E800020  blr
	return;
	// 82635710: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263571C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635728 size=192
    let mut pc: u32 = 0x82635728;
    'dispatch: loop {
        match pc {
            0x82635728 => {
    //   block [0x82635728..0x826357E8)
	// 82635728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263572C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263573C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635740: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82635744: 4BBE9B15  bl 0x8221f258
	ctx.lr = 0x82635748;
	sub_8221F258(ctx, base);
	// 82635748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263574C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635750: 419A007C  beq cr6, 0x826357cc
	if ctx.cr[6].eq {
	pc = 0x826357CC; continue 'dispatch;
	}
	// 82635754: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82635758: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263575C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82635760: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82635764: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635768: 38E948A0  addi r7, r9, 0x48a0
	ctx.r[7].s64 = ctx.r[9].s64 + 18592;
	// 8263576C: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635770: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 82635774: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635778: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263577C: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635780: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635784: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82635788: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8263578C: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82635790: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82635794: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82635798: 997F0011  stb r11, 0x11(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8263579C: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 826357A0: 997F0012  stb r11, 0x12(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[11].u8 ) };
	// 826357A4: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 826357A8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 826357AC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 826357B0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 826357B4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826357B8: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 826357BC: 90DF003C  stw r6, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[6].u32 ) };
	// 826357C0: 4BB5E679  bl 0x82193e38
	ctx.lr = 0x826357C4;
	sub_82193E38(ctx, base);
	// 826357C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826357C8: 48000008  b 0x826357d0
	pc = 0x826357D0; continue 'dispatch;
	// 826357CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826357D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826357D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826357D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826357DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 826357E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826357E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826357E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826357E8 size=88
    let mut pc: u32 = 0x826357E8;
    'dispatch: loop {
        match pc {
            0x826357E8 => {
    //   block [0x826357E8..0x82635840)
	// 826357E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826357EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826357F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826357F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826357F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826357FC: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 82635800: 4BBE9A59  bl 0x8221f258
	ctx.lr = 0x82635804;
	sub_8221F258(ctx, base);
	// 82635804: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635808: 419A0020  beq cr6, 0x82635828
	if ctx.cr[6].eq {
	pc = 0x82635828; continue 'dispatch;
	}
	// 8263580C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635810: 482BDF99  bl 0x828f37a8
	ctx.lr = 0x82635814;
	sub_828F37A8(ctx, base);
	// 82635814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263581C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635824: 4E800020  blr
	return;
	// 82635828: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263582C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263583C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635840 size=188
    let mut pc: u32 = 0x82635840;
    'dispatch: loop {
        match pc {
            0x82635840 => {
    //   block [0x82635840..0x826358FC)
	// 82635840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635844: 48673BC9  bl 0x82ca940c
	ctx.lr = 0x82635848;
	sub_82CA93D0(ctx, base);
	// 82635848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263584C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82635850: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82635854: 4BBE9A05  bl 0x8221f258
	ctx.lr = 0x82635858;
	sub_8221F258(ctx, base);
	// 82635858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263585C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635860: 419A0090  beq cr6, 0x826358f0
	if ctx.cr[6].eq {
	pc = 0x826358F0; continue 'dispatch;
	}
	// 82635864: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82635868: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263586C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635870: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635874: 392B8150  addi r9, r11, -0x7eb0
	ctx.r[9].s64 = ctx.r[11].s64 + -32432;
	// 82635878: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 8263587C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635880: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635884: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82635888: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8263588C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82635890: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82635894: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82635898: 4BB5E5A1  bl 0x82193e38
	ctx.lr = 0x8263589C;
	sub_82193E38(ctx, base);
	// 8263589C: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 826358A0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 826358A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826358A8: 38C79484  addi r6, r7, -0x6b7c
	ctx.r[6].s64 = ctx.r[7].s64 + -27516;
	// 826358AC: 9BDF0030  stb r30, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u8 ) };
	// 826358B0: 9BDF0031  stb r30, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[30].u8 ) };
	// 826358B4: 9BDF0032  stb r30, 0x32(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(50 as u32), ctx.r[30].u8 ) };
	// 826358B8: C1A79484  lfs f13, -0x6b7c(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 826358BC: 9BDF0033  stb r30, 0x33(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(51 as u32), ctx.r[30].u8 ) };
	// 826358C0: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 826358C4: 9BDF0034  stb r30, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 826358C8: C006000C  lfs f0, 0xc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 826358CC: 9BDF0035  stb r30, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[30].u8 ) };
	// 826358D0: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 826358D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 826358D8: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 826358DC: D1BF002C  stfs f13, 0x2c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 826358E0: 4BD57BD9  bl 0x8238d4b8
	ctx.lr = 0x826358E4;
	sub_8238D4B8(ctx, base);
	// 826358E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826358E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826358EC: 48673B70  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 826358F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826358F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826358F8: 48673B64  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635900 size=132
    let mut pc: u32 = 0x82635900;
    'dispatch: loop {
        match pc {
            0x82635900 => {
    //   block [0x82635900..0x82635984)
	// 82635900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635904: 48673B09  bl 0x82ca940c
	ctx.lr = 0x82635908;
	sub_82CA93D0(ctx, base);
	// 82635908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263590C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82635910: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82635914: 4BBE9945  bl 0x8221f258
	ctx.lr = 0x82635918;
	sub_8221F258(ctx, base);
	// 82635918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263591C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635920: 419A0058  beq cr6, 0x82635978
	if ctx.cr[6].eq {
	pc = 0x82635978; continue 'dispatch;
	}
	// 82635924: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82635928: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8263592C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635930: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635934: 392B8190  addi r9, r11, -0x7e70
	ctx.r[9].s64 = ctx.r[11].s64 + -32368;
	// 82635938: 390A0B7C  addi r8, r10, 0xb7c
	ctx.r[8].s64 = ctx.r[10].s64 + 2940;
	// 8263593C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635940: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635944: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82635948: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8263594C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82635950: 4BB5E4E9  bl 0x82193e38
	ctx.lr = 0x82635954;
	sub_82193E38(ctx, base);
	// 82635954: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82635958: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8263595C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635960: 90FF0018  stw r7, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 82635964: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82635968: 4BD57B51  bl 0x8238d4b8
	ctx.lr = 0x8263596C;
	sub_8238D4B8(ctx, base);
	// 8263596C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635974: 48673AE8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82635978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263597C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635980: 48673ADC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635988 size=88
    let mut pc: u32 = 0x82635988;
    'dispatch: loop {
        match pc {
            0x82635988 => {
    //   block [0x82635988..0x826359E0)
	// 82635988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263599C: 386001C0  li r3, 0x1c0
	ctx.r[3].s64 = 448;
	// 826359A0: 4BBE98B9  bl 0x8221f258
	ctx.lr = 0x826359A4;
	sub_8221F258(ctx, base);
	// 826359A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826359A8: 419A0020  beq cr6, 0x826359c8
	if ctx.cr[6].eq {
	pc = 0x826359C8; continue 'dispatch;
	}
	// 826359AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826359B0: 48121D49  bl 0x827576f8
	ctx.lr = 0x826359B4;
	sub_827576F8(ctx, base);
	// 826359B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826359B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826359BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826359C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826359C4: 4E800020  blr
	return;
	// 826359C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826359CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826359D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826359D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826359D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826359DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826359E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826359E0 size=132
    let mut pc: u32 = 0x826359E0;
    'dispatch: loop {
        match pc {
            0x826359E0 => {
    //   block [0x826359E0..0x82635A64)
	// 826359E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826359E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826359E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826359EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826359F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826359F4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 826359F8: 4BBE9861  bl 0x8221f258
	ctx.lr = 0x826359FC;
	sub_8221F258(ctx, base);
	// 826359FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635A00: 419A004C  beq cr6, 0x82635a4c
	if ctx.cr[6].eq {
	pc = 0x82635A4C; continue 'dispatch;
	}
	// 82635A04: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635A08: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635A0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635A10: 392A6558  addi r9, r10, 0x6558
	ctx.r[9].s64 = ctx.r[10].s64 + 25944;
	// 82635A14: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635A18: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82635A1C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635A20: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635A24: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635A28: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82635A2C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82635A30: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82635A34: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82635A38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635A44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635A48: 4E800020  blr
	return;
	// 82635A4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635A5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635A68 size=228
    let mut pc: u32 = 0x82635A68;
    'dispatch: loop {
        match pc {
            0x82635A68 => {
    //   block [0x82635A68..0x82635B4C)
	// 82635A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635A70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635A74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635A78: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82635A7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635A80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635A84: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82635A88: 4BBE97D1  bl 0x8221f258
	ctx.lr = 0x82635A8C;
	sub_8221F258(ctx, base);
	// 82635A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635A90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635A94: 419A0098  beq cr6, 0x82635b2c
	if ctx.cr[6].eq {
	pc = 0x82635B2C; continue 'dispatch;
	}
	// 82635A98: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82635A9C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82635AA0: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82635AA4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82635AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635AAC: 38E95340  addi r7, r9, 0x5340
	ctx.r[7].s64 = ctx.r[9].s64 + 21312;
	// 82635AB0: 38C80B7C  addi r6, r8, 0xb7c
	ctx.r[6].s64 = ctx.r[8].s64 + 2940;
	// 82635AB4: C3EA9484  lfs f31, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82635AB8: D3FF0014  stfs f31, 0x14(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635ABC: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635AC0: D3FF0018  stfs f31, 0x18(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635AC4: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635AC8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635ACC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82635AD0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635AD4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82635AD8: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82635ADC: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82635AE0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82635AE4: 90DF0028  stw r6, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82635AE8: 4BB5E351  bl 0x82193e38
	ctx.lr = 0x82635AEC;
	sub_82193E38(ctx, base);
	// 82635AEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635AF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82635AF4: 4BD579C5  bl 0x8238d4b8
	ctx.lr = 0x82635AF8;
	sub_8238D4B8(ctx, base);
	// 82635AF8: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82635AFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82635B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635B04: C0250A54  lfs f1, 0xa54(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(2644 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82635B08: D03F0014  stfs f1, 0x14(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635B0C: 482D0CE5  bl 0x829067f0
	ctx.lr = 0x82635B10;
	sub_829067F0(ctx, base);
	// 82635B10: D3FF0018  stfs f31, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635B14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82635B18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82635B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635B20: 482D0FD9  bl 0x82906af8
	ctx.lr = 0x82635B24;
	sub_82906AF8(ctx, base);
	// 82635B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635B28: 48000008  b 0x82635b30
	pc = 0x82635B30; continue 'dispatch;
	// 82635B2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635B30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B3C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82635B40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635B44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635B50 size=88
    let mut pc: u32 = 0x82635B50;
    'dispatch: loop {
        match pc {
            0x82635B50 => {
    //   block [0x82635B50..0x82635BA8)
	// 82635B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635B64: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 82635B68: 4BBE96F1  bl 0x8221f258
	ctx.lr = 0x82635B6C;
	sub_8221F258(ctx, base);
	// 82635B6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635B70: 419A0020  beq cr6, 0x82635b90
	if ctx.cr[6].eq {
	pc = 0x82635B90; continue 'dispatch;
	}
	// 82635B74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635B78: 48144041  bl 0x82779bb8
	ctx.lr = 0x82635B7C;
	sub_82779BB8(ctx, base);
	// 82635B7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635B88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635B8C: 4E800020  blr
	return;
	// 82635B90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635BA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635BA8 size=152
    let mut pc: u32 = 0x82635BA8;
    'dispatch: loop {
        match pc {
            0x82635BA8 => {
    //   block [0x82635BA8..0x82635C40)
	// 82635BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635BB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635BB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635BB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635BBC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82635BC0: 4BBE9699  bl 0x8221f258
	ctx.lr = 0x82635BC4;
	sub_8221F258(ctx, base);
	// 82635BC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635BC8: 419A0060  beq cr6, 0x82635c28
	if ctx.cr[6].eq {
	pc = 0x82635C28; continue 'dispatch;
	}
	// 82635BCC: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635BD0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635BD8: 38EA6C98  addi r7, r10, 0x6c98
	ctx.r[7].s64 = ctx.r[10].s64 + 27800;
	// 82635BDC: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 82635BE0: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635BE4: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635BE8: 38A67088  addi r5, r6, 0x7088
	ctx.r[5].s64 = ctx.r[6].s64 + 28808;
	// 82635BEC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82635BF0: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82635BF4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82635BF8: 7D202828  lwarx r9, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82635BFC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82635C00: 7D20292D  stwcx. r9, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82635C04: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82635C08: 4082FFE8  bne 0x82635bf0
	if !ctx.cr[0].eq {
	pc = 0x82635BF0; continue 'dispatch;
	}
	// 82635C0C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635C10: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82635C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635C24: 4E800020  blr
	return;
	// 82635C28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635C2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635C38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635C40 size=160
    let mut pc: u32 = 0x82635C40;
    'dispatch: loop {
        match pc {
            0x82635C40 => {
    //   block [0x82635C40..0x82635CE0)
	// 82635C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635C48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635C4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635C50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635C54: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82635C58: 4BBE9601  bl 0x8221f258
	ctx.lr = 0x82635C5C;
	sub_8221F258(ctx, base);
	// 82635C5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635C60: 419A0068  beq cr6, 0x82635cc8
	if ctx.cr[6].eq {
	pc = 0x82635CC8; continue 'dispatch;
	}
	// 82635C64: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635C68: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635C6C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82635C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635C74: 390A6E18  addi r8, r10, 0x6e18
	ctx.r[8].s64 = ctx.r[10].s64 + 28184;
	// 82635C78: 38E90B7C  addi r7, r9, 0xb7c
	ctx.r[7].s64 = ctx.r[9].s64 + 2940;
	// 82635C7C: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635C80: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82635C84: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82635C88: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635C8C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82635C90: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82635C94: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 82635C98: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82635C9C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82635CA0: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82635CA4: 80C30004  lwz r6, 4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82635CA8: 88A60090  lbz r5, 0x90(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(144 as u32) ) } as u64;
	// 82635CAC: 60A40001  ori r4, r5, 1
	ctx.r[4].u64 = ctx.r[5].u64 | 1;
	// 82635CB0: 98860090  stb r4, 0x90(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(144 as u32), ctx.r[4].u8 ) };
	// 82635CB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635CC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635CC4: 4E800020  blr
	return;
	// 82635CC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635CCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635CE0 size=128
    let mut pc: u32 = 0x82635CE0;
    'dispatch: loop {
        match pc {
            0x82635CE0 => {
    //   block [0x82635CE0..0x82635D60)
	// 82635CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635CF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635CF8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82635CFC: 4BBE955D  bl 0x8221f258
	ctx.lr = 0x82635D00;
	sub_8221F258(ctx, base);
	// 82635D00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635D04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635D08: 419A003C  beq cr6, 0x82635d44
	if ctx.cr[6].eq {
	pc = 0x82635D44; continue 'dispatch;
	}
	// 82635D0C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635D10: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82635D14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635D18: 392A3A78  addi r9, r10, 0x3a78
	ctx.r[9].s64 = ctx.r[10].s64 + 14968;
	// 82635D1C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82635D20: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635D24: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635D28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635D2C: 991F000C  stb r8, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 82635D30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82635D34: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635D38: 4BD57781  bl 0x8238d4b8
	ctx.lr = 0x82635D3C;
	sub_8238D4B8(ctx, base);
	// 82635D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635D40: 48000008  b 0x82635d48
	pc = 0x82635D48; continue 'dispatch;
	// 82635D44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635D48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635D54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635D58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635D60 size=116
    let mut pc: u32 = 0x82635D60;
    'dispatch: loop {
        match pc {
            0x82635D60 => {
    //   block [0x82635D60..0x82635DD4)
	// 82635D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635D70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635D74: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82635D78: 4BBE94E1  bl 0x8221f258
	ctx.lr = 0x82635D7C;
	sub_8221F258(ctx, base);
	// 82635D7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635D80: 419A003C  beq cr6, 0x82635dbc
	if ctx.cr[6].eq {
	pc = 0x82635DBC; continue 'dispatch;
	}
	// 82635D84: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635D88: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635D8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635D90: 392A2EF0  addi r9, r10, 0x2ef0
	ctx.r[9].s64 = ctx.r[10].s64 + 12016;
	// 82635D94: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635D98: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635D9C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82635DA0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82635DA4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82635DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635DB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635DB8: 4E800020  blr
	return;
	// 82635DBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635DD8 size=116
    let mut pc: u32 = 0x82635DD8;
    'dispatch: loop {
        match pc {
            0x82635DD8 => {
    //   block [0x82635DD8..0x82635E4C)
	// 82635DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635DEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635DF0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82635DF4: 4BBE9465  bl 0x8221f258
	ctx.lr = 0x82635DF8;
	sub_8221F258(ctx, base);
	// 82635DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635DFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635E00: 419A0030  beq cr6, 0x82635e30
	if ctx.cr[6].eq {
	pc = 0x82635E30; continue 'dispatch;
	}
	// 82635E04: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82635E08: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82635E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82635E10: 392B7288  addi r9, r11, 0x7288
	ctx.r[9].s64 = ctx.r[11].s64 + 29320;
	// 82635E14: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82635E18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635E1C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635E20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82635E24: 4BD57695  bl 0x8238d4b8
	ctx.lr = 0x82635E28;
	sub_8238D4B8(ctx, base);
	// 82635E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635E2C: 48000008  b 0x82635e34
	pc = 0x82635E34; continue 'dispatch;
	// 82635E30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635E34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635E40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635E50 size=176
    let mut pc: u32 = 0x82635E50;
    'dispatch: loop {
        match pc {
            0x82635E50 => {
    //   block [0x82635E50..0x82635F00)
	// 82635E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635E58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82635E5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635E64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82635E68: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82635E6C: 4BBE93ED  bl 0x8221f258
	ctx.lr = 0x82635E70;
	sub_8221F258(ctx, base);
	// 82635E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635E74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635E78: 419A006C  beq cr6, 0x82635ee4
	if ctx.cr[6].eq {
	pc = 0x82635EE4; continue 'dispatch;
	}
	// 82635E7C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82635E80: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82635E84: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82635E88: 390A9490  addi r8, r10, -0x6b70
	ctx.r[8].s64 = ctx.r[10].s64 + -27504;
	// 82635E8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635E90: 38E93630  addi r7, r9, 0x3630
	ctx.r[7].s64 = ctx.r[9].s64 + 13872;
	// 82635E94: C00A9490  lfs f0, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635E98: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635E9C: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82635EA0: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635EA4: C008FFF4  lfs f0, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635EA8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82635EAC: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82635EB0: 997F0030  stb r11, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 82635EB4: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82635EB8: 997F0031  stb r11, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[11].u8 ) };
	// 82635EBC: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635EC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635EC4: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82635EC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82635ECC: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82635ED0: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82635ED4: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82635ED8: 4BD575E1  bl 0x8238d4b8
	ctx.lr = 0x82635EDC;
	sub_8238D4B8(ctx, base);
	// 82635EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635EE0: 48000008  b 0x82635ee8
	pc = 0x82635EE8; continue 'dispatch;
	// 82635EE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635EE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82635EF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82635EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82635EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82635F00 size=184
    let mut pc: u32 = 0x82635F00;
    'dispatch: loop {
        match pc {
            0x82635F00 => {
    //   block [0x82635F00..0x82635FB8)
	// 82635F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635F04: 48673509  bl 0x82ca940c
	ctx.lr = 0x82635F08;
	sub_82CA93D0(ctx, base);
	// 82635F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635F0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82635F10: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82635F14: 4BBE9345  bl 0x8221f258
	ctx.lr = 0x82635F18;
	sub_8221F258(ctx, base);
	// 82635F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635F1C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82635F20: 419A008C  beq cr6, 0x82635fac
	if ctx.cr[6].eq {
	pc = 0x82635FAC; continue 'dispatch;
	}
	// 82635F24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82635F28: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82635F2C: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82635F30: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 82635F34: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82635F38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82635F3C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635F40: 38EA46B0  addi r7, r10, 0x46b0
	ctx.r[7].s64 = ctx.r[10].s64 + 18096;
	// 82635F44: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82635F48: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82635F4C: 38A80B7C  addi r5, r8, 0xb7c
	ctx.r[5].s64 = ctx.r[8].s64 + 2940;
	// 82635F50: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82635F54: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82635F58: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82635F5C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82635F60: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82635F64: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82635F68: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82635F6C: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 82635F70: 98DF001D  stb r6, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[6].u8 ) };
	// 82635F74: 9BDF001E  stb r30, 0x1e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[30].u8 ) };
	// 82635F78: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82635F7C: 90BF0020  stw r5, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82635F80: 4BB5DEB9  bl 0x82193e38
	ctx.lr = 0x82635F84;
	sub_82193E38(ctx, base);
	// 82635F84: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82635F88: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82635F8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82635F90: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82635F94: 9BDF0038  stb r30, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 82635F98: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82635F9C: 4BD5751D  bl 0x8238d4b8
	ctx.lr = 0x82635FA0;
	sub_8238D4B8(ctx, base);
	// 82635FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82635FA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635FA8: 486734B4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82635FAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82635FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82635FB4: 486734A8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82635FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82635FB8 size=104
    let mut pc: u32 = 0x82635FB8;
    'dispatch: loop {
        match pc {
            0x82635FB8 => {
    //   block [0x82635FB8..0x82636020)
	// 82635FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82635FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82635FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82635FC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82635FC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82635FCC: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82635FD0: 4BBE9289  bl 0x8221f258
	ctx.lr = 0x82635FD4;
	sub_8221F258(ctx, base);
	// 82635FD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82635FD8: 419A0030  beq cr6, 0x82636008
	if ctx.cr[6].eq {
	pc = 0x82636008; continue 'dispatch;
	}
	// 82635FDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82635FE0: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82635FE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82635FE8: 392A4700  addi r9, r10, 0x4700
	ctx.r[9].s64 = ctx.r[10].s64 + 18176;
	// 82635FEC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82635FF0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82635FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82635FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82635FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636004: 4E800020  blr
	return;
	// 82636008: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263600C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263601C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636020 size=124
    let mut pc: u32 = 0x82636020;
    'dispatch: loop {
        match pc {
            0x82636020 => {
    //   block [0x82636020..0x8263609C)
	// 82636020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636024: 486733E9  bl 0x82ca940c
	ctx.lr = 0x82636028;
	sub_82CA93D0(ctx, base);
	// 82636028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263602C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636030: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82636034: 4BBE9225  bl 0x8221f258
	ctx.lr = 0x82636038;
	sub_8221F258(ctx, base);
	// 82636038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263603C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636040: 419A0050  beq cr6, 0x82636090
	if ctx.cr[6].eq {
	pc = 0x82636090; continue 'dispatch;
	}
	// 82636044: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82636048: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263604C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82636050: 394B4580  addi r10, r11, 0x4580
	ctx.r[10].s64 = ctx.r[11].s64 + 17792;
	// 82636054: 9BBF0008  stb r29, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82636058: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8263605C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82636060: 4BBE91F9  bl 0x8221f258
	ctx.lr = 0x82636064;
	sub_8221F258(ctx, base);
	// 82636064: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636068: 419A0008  beq cr6, 0x82636070
	if ctx.cr[6].eq {
	pc = 0x82636070; continue 'dispatch;
	}
	// 8263606C: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82636070: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82636074: 41820008  beq 0x8263607c
	if ctx.cr[0].eq {
	pc = 0x8263607C; continue 'dispatch;
	}
	// 82636078: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8263607C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82636080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636084: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82636088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8263608C: 486733D0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82636090: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636098: 486733C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826360A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826360A0 size=88
    let mut pc: u32 = 0x826360A0;
    'dispatch: loop {
        match pc {
            0x826360A0 => {
    //   block [0x826360A0..0x826360F8)
	// 826360A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826360A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826360A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826360AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826360B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826360B4: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 826360B8: 4BBE91A1  bl 0x8221f258
	ctx.lr = 0x826360BC;
	sub_8221F258(ctx, base);
	// 826360BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826360C0: 419A0020  beq cr6, 0x826360e0
	if ctx.cr[6].eq {
	pc = 0x826360E0; continue 'dispatch;
	}
	// 826360C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 826360C8: 4BDDD3E9  bl 0x824134b0
	ctx.lr = 0x826360CC;
	sub_824134B0(ctx, base);
	// 826360CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826360D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826360D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826360D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826360DC: 4E800020  blr
	return;
	// 826360E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826360E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826360E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826360EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826360F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826360F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826360F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826360F8 size=172
    let mut pc: u32 = 0x826360F8;
    'dispatch: loop {
        match pc {
            0x826360F8 => {
    //   block [0x826360F8..0x826361A4)
	// 826360F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826360FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263610C: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82636110: 4BBE9149  bl 0x8221f258
	ctx.lr = 0x82636114;
	sub_8221F258(ctx, base);
	// 82636114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636118: 419A0074  beq cr6, 0x8263618c
	if ctx.cr[6].eq {
	pc = 0x8263618C; continue 'dispatch;
	}
	// 8263611C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82636120: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636124: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82636128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263612C: 390940B8  addi r8, r9, 0x40b8
	ctx.r[8].s64 = ctx.r[9].s64 + 16568;
	// 82636130: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636134: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636138: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8263613C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82636140: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82636144: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82636148: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8263614C: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82636150: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82636154: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82636158: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263615C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82636160: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82636164: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82636168: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8263616C: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82636170: 99630044  stb r11, 0x44(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u8 ) };
	// 82636174: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82636178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8263617C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636188: 4E800020  blr
	return;
	// 8263618C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263619C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826361A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826361A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826361A8 size=140
    let mut pc: u32 = 0x826361A8;
    'dispatch: loop {
        match pc {
            0x826361A8 => {
    //   block [0x826361A8..0x82636234)
	// 826361A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826361AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826361B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 826361B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826361B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826361BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 826361C0: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 826361C4: 4BBE9095  bl 0x8221f258
	ctx.lr = 0x826361C8;
	sub_8221F258(ctx, base);
	// 826361C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826361CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826361D0: 419A0048  beq cr6, 0x82636218
	if ctx.cr[6].eq {
	pc = 0x82636218; continue 'dispatch;
	}
	// 826361D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 826361D8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 826361DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826361E0: 392B43C0  addi r9, r11, 0x43c0
	ctx.r[9].s64 = ctx.r[11].s64 + 17344;
	// 826361E4: 995F0008  stb r10, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 826361E8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 826361EC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826361F0: 4BE4A049  bl 0x82480238
	ctx.lr = 0x826361F4;
	sub_82480238(ctx, base);
	// 826361F4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 826361F8: 4BE4A041  bl 0x82480238
	ctx.lr = 0x826361FC;
	sub_82480238(ctx, base);
	// 826361FC: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 82636200: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82636204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636208: 98FF0024  stb r7, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[7].u8 ) };
	// 8263620C: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636210: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82636214: 48000008  b 0x8263621c
	pc = 0x8263621C; continue 'dispatch;
	// 82636218: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8263621C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636228: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8263622C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636238 size=180
    let mut pc: u32 = 0x82636238;
    'dispatch: loop {
        match pc {
            0x82636238 => {
    //   block [0x82636238..0x826362EC)
	// 82636238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263623C: 486731D1  bl 0x82ca940c
	ctx.lr = 0x82636240;
	sub_82CA93D0(ctx, base);
	// 82636240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636244: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82636248: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8263624C: 4BBE900D  bl 0x8221f258
	ctx.lr = 0x82636250;
	sub_8221F258(ctx, base);
	// 82636250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636254: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636258: 419A0088  beq cr6, 0x826362e0
	if ctx.cr[6].eq {
	pc = 0x826362E0; continue 'dispatch;
	}
	// 8263625C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82636260: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82636264: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82636268: 394B29F8  addi r10, r11, 0x29f8
	ctx.r[10].s64 = ctx.r[11].s64 + 10744;
	// 8263626C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82636270: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82636274: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82636278: 4BBE8FE1  bl 0x8221f258
	ctx.lr = 0x8263627C;
	sub_8221F258(ctx, base);
	// 8263627C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82636280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82636284: 419A0008  beq cr6, 0x8263628c
	if ctx.cr[6].eq {
	pc = 0x8263628C; continue 'dispatch;
	}
	// 82636288: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8263628C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82636290: 41820008  beq 0x82636298
	if ctx.cr[0].eq {
	pc = 0x82636298; continue 'dispatch;
	}
	// 82636294: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82636298: 354B0008  addic. r10, r11, 8
	ctx.xer.ca = (ctx.r[11].u32 > (!(8 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8263629C: 41820008  beq 0x826362a4
	if ctx.cr[0].eq {
	pc = 0x826362A4; continue 'dispatch;
	}
	// 826362A0: 93CA0000  stw r30, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 826362A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826362A8: 9BCB0021  stb r30, 0x21(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(33 as u32), ctx.r[30].u8 ) };
	// 826362AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 826362B0: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 826362B4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826362B8: 994B0021  stb r10, 0x21(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 826362BC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 826362C0: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 826362C4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 826362C8: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 826362CC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 826362D0: 91290008  stw r9, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 826362D4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 826362D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826362DC: 48673180  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 826362E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 826362E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 826362E8: 48673174  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826362F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826362F0 size=188
    let mut pc: u32 = 0x826362F0;
    'dispatch: loop {
        match pc {
            0x826362F0 => {
    //   block [0x826362F0..0x826363AC)
	// 826362F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826362F4: 48673119  bl 0x82ca940c
	ctx.lr = 0x826362F8;
	sub_82CA93D0(ctx, base);
	// 826362F8: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826363B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826363B0 size=232
    let mut pc: u32 = 0x826363B0;
    'dispatch: loop {
        match pc {
            0x826363B0 => {
    //   block [0x826363B0..0x82636498)
	// 826363B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826363B4: 48673059  bl 0x82ca940c
	ctx.lr = 0x826363B8;
	sub_82CA93D0(ctx, base);
	// 826363B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826363BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826363C0: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 826363C4: 4BBE8E95  bl 0x8221f258
	ctx.lr = 0x826363C8;
	sub_8221F258(ctx, base);
	// 826363C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826363CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826363D0: 419A00BC  beq cr6, 0x8263648c
	if ctx.cr[6].eq {
	pc = 0x8263648C; continue 'dispatch;
	}
	// 826363D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 826363D8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 826363DC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 826363E0: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 826363E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826363E8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 826363EC: 38EB04D4  addi r7, r11, 0x4d4
	ctx.r[7].s64 = ctx.r[11].s64 + 1236;
	// 826363F0: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 826363F4: 38CA6218  addi r6, r10, 0x6218
	ctx.r[6].s64 = ctx.r[10].s64 + 25112;
	// 826363F8: 38A96258  addi r5, r9, 0x6258
	ctx.r[5].s64 = ctx.r[9].s64 + 25176;
	// 826363FC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82636400: 3BA80B7C  addi r29, r8, 0xb7c
	ctx.r[29].s64 = ctx.r[8].s64 + 2940;
	// 82636404: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82636408: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8263640C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82636410: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82636414: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82636418: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8263641C: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82636420: 4BB5DA19  bl 0x82193e38
	ctx.lr = 0x82636424;
	sub_82193E38(ctx, base);
	// 82636424: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 82636428: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 8263642C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82636430: 39440E0C  addi r10, r4, 0xe0c
	ctx.r[10].s64 = ctx.r[4].s64 + 3596;
	// 82636434: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82636438: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 8263643C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82636440: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82636444: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82636448: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 8263644C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636450: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82636454: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82636458: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 8263645C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 82636460: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82636464: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82636468: D01F0054  stfs f0, 0x54(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8263646C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82636470: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82636474: 9BDF005C  stb r30, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u8 ) };
	// 82636478: 89090090  lbz r8, 0x90(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(144 as u32) ) } as u64;
	// 8263647C: 61070001  ori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 | 1;
	// 82636480: 98E90090  stb r7, 0x90(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(144 as u32), ctx.r[7].u8 ) };
	// 82636484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636488: 48672FD4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 8263648C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82636494: 48672FC8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82636498 size=132
    let mut pc: u32 = 0x82636498;
    'dispatch: loop {
        match pc {
            0x82636498 => {
    //   block [0x82636498..0x8263651C)
	// 82636498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826364A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 826364A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826364A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826364AC: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 826364B0: 4BBE8DA9  bl 0x8221f258
	ctx.lr = 0x826364B4;
	sub_8221F258(ctx, base);
	// 826364B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 826364B8: 419A004C  beq cr6, 0x82636504
	if ctx.cr[6].eq {
	pc = 0x82636504; continue 'dispatch;
	}
	// 826364BC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 826364C0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 826364C4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 826364C8: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 826364CC: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 826364D0: 392B6268  addi r9, r11, 0x6268
	ctx.r[9].s64 = ctx.r[11].s64 + 25192;
	// 826364D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826364D8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 826364DC: 98E30008  stb r7, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u8 ) };
	// 826364E0: C0089484  lfs f0, -0x6b7c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636520 size=148
    let mut pc: u32 = 0x82636520;
    'dispatch: loop {
        match pc {
            0x82636520 => {
    //   block [0x82636520..0x826365B4)
	// 82636520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82636524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8263652C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636534: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82636538: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 8263653C: 4BBE8D1D  bl 0x8221f258
	ctx.lr = 0x82636540;
	sub_8221F258(ctx, base);
	// 82636540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82636544: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82636548: 419A0050  beq cr6, 0x82636598
	if ctx.cr[6].eq {
	pc = 0x82636598; continue 'dispatch;
	}
	// 8263654C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 82636550: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 82636554: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82636558: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8263655C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636560: 390A62A8  addi r8, r10, 0x62a8
	ctx.r[8].s64 = ctx.r[10].s64 + 25256;
	// 82636564: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82636568: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8263656C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826365B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x826365B8 size=172
    let mut pc: u32 = 0x826365B8;
    'dispatch: loop {
        match pc {
            0x826365B8 => {
    //   block [0x826365B8..0x82636664)
	// 826365B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826365BC: 48672E51  bl 0x82ca940c
	ctx.lr = 0x826365C0;
	sub_82CA93D0(ctx, base);
	// 826365C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826365C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 826365C8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 826365CC: 4BBE8C8D  bl 0x8221f258
	ctx.lr = 0x826365D0;
	sub_8221F258(ctx, base);
	// 826365D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 826365D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 826365D8: 419A0080  beq cr6, 0x82636658
	if ctx.cr[6].eq {
	pc = 0x82636658; continue 'dispatch;
	}
	// 826365DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 826365E0: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 826365E4: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 826365E8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 826365EC: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 826365F0: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 826365F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 826365F8: 38EA8498  addi r7, r10, -0x7b68
	ctx.r[7].s64 = ctx.r[10].s64 + -31592;
	// 826365FC: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82636600: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82636604: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82636608: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8263660C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82636610: 38A60B7C  addi r5, r6, 0xb7c
	ctx.r[5].s64 = ctx.r[6].s64 + 2940;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636668 size=176
    let mut pc: u32 = 0x82636668;
    'dispatch: loop {
        match pc {
            0x82636668 => {
    //   block [0x82636668..0x82636718)
	// 82636668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263667C: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 82636680: 4BBE8BD9  bl 0x8221f258
	ctx.lr = 0x82636684;
	sub_8221F258(ctx, base);
	// 82636684: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636688: 419A0078  beq cr6, 0x82636700
	if ctx.cr[6].eq {
	pc = 0x82636700; continue 'dispatch;
	}
	// 8263668C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82636690: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636694: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82636698: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8263669C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826366A0: 38EA04D4  addi r7, r10, 0x4d4
	ctx.r[7].s64 = ctx.r[10].s64 + 1236;
	// 826366A4: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 826366A8: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 826366AC: 38A93EF8  addi r5, r9, 0x3ef8
	ctx.r[5].s64 = ctx.r[9].s64 + 16120;
	// 826366B0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 826366B4: 388885C8  addi r4, r8, -0x7a38
	ctx.r[4].s64 = ctx.r[8].s64 + -31288;
	// 826366B8: 39460E0C  addi r10, r6, 0xe0c
	ctx.r[10].s64 = ctx.r[6].s64 + 3596;
	// 826366BC: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 826366C0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 826366C4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826366C8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 826366CC: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 826366D0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 826366D4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 826366D8: 99230020  stb r9, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u8 ) };
	// 826366DC: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 826366E0: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 826366E4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 826366E8: 91030030  stw r8, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 826366EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826366F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826366F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826366F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826366FC: 4E800020  blr
	return;
	// 82636700: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636704: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263670C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82636714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82636718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82636718 size=144
    let mut pc: u32 = 0x82636718;
    'dispatch: loop {
        match pc {
            0x82636718 => {
    //   block [0x82636718..0x826367A8)
	// 82636718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263671C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82636720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82636724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82636728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8263672C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82636730: 4BBE8B29  bl 0x8221f258
	ctx.lr = 0x82636734;
	sub_8221F258(ctx, base);
	// 82636734: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82636738: 419A0058  beq cr6, 0x82636790
	if ctx.cr[6].eq {
	pc = 0x82636790; continue 'dispatch;
	}
	// 8263673C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 82636740: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82636744: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 82636748: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8263674C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82636750: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 82636754: 38CA04D4  addi r6, r10, 0x4d4
	ctx.r[6].s64 = ctx.r[10].s64 + 1236;
	// 82636758: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8263675C: 38A985D8  addi r5, r9, -0x7a28
	ctx.r[5].s64 = ctx.r[9].s64 + -31272;
	// 82636760: 38888618  addi r4, r8, -0x79e8
	ctx.r[4].s64 = ctx.r[8].s64 + -31208;
	// 82636764: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82636768: 39470E0C  addi r10, r7, 0xe0c
	ctx.r[10].s64 = ctx.r[7].s64 + 3596;
	// 8263676C: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82636770: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82636774: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82636778: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8263677C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82636784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82636788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8263678C: 4E800020  blr
	return;
	// 82636790: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82636794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82636798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263679C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826367A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 826367A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


