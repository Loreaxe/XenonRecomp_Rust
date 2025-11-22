pub fn sub_827D7F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D7F08 size=324
    let mut pc: u32 = 0x827D7F08;
    'dispatch: loop {
        match pc {
            0x827D7F08 => {
    //   block [0x827D7F08..0x827D804C)
	// 827D7F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D7F0C: 489D0259  bl 0x831a8164
	ctx.lr = 0x827D7F10;
	sub_831A8130(ctx, base);
	// 827D7F10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D7F14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827D7F18: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 827D7F1C: 815D00D0  lwz r10, 0xd0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 827D7F20: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D7F24: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827D7F28: 48000070  b 0x827d7f98
	pc = 0x827D7F98; continue 'dispatch;
	// 827D7F2C: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 827D7F30: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827D7F34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827D7F38: 419A0024  beq cr6, 0x827d7f5c
	if ctx.cr[6].eq {
	pc = 0x827D7F5C; continue 'dispatch;
	}
	// 827D7F3C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827D7F40: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827D7F44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D7F48: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827D7F4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827D7F50: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827D7F54: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D7F58: 4082FFE8  bne 0x827d7f40
	if !ctx.cr[0].eq {
	pc = 0x827D7F40; continue 'dispatch;
	}
	// 827D7F5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D7F60: C01D00D8  lfs f0, 0xd8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D7F64: C1BB0000  lfs f13, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827D7F68: EC200372  fmuls f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 827D7F6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D7F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D7F74: 4E800421  bctrl
	ctx.lr = 0x827D7F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D7F78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D7F7C: 4BBC970D  bl 0x823a1688
	ctx.lr = 0x827D7F80;
	sub_823A1688(ctx, base);
	// 827D7F80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827D7F84: 419A000C  beq cr6, 0x827d7f90
	if ctx.cr[6].eq {
	pc = 0x827D7F90; continue 'dispatch;
	}
	// 827D7F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D7F8C: 4BAE8905  bl 0x822c0890
	ctx.lr = 0x827D7F90;
	sub_822C0890(ctx, base);
	// 827D7F90: 815D00D0  lwz r10, 0xd0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 827D7F94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827D7F98: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827D7F9C: 409AFF90  bne cr6, 0x827d7f2c
	if !ctx.cr[6].eq {
	pc = 0x827D7F2C; continue 'dispatch;
	}
	// 827D7FA0: 817D00C4  lwz r11, 0xc4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(196 as u32) ) } as u64;
	// 827D7FA4: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D7FA8: 48000094  b 0x827d803c
	pc = 0x827D803C; continue 'dispatch;
	// 827D7FAC: 839F000C  lwz r28, 0xc(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827D7FB0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D7FB4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827D7FB8: 419A0024  beq cr6, 0x827d7fdc
	if ctx.cr[6].eq {
	pc = 0x827D7FDC; continue 'dispatch;
	}
	// 827D7FBC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 827D7FC0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827D7FC4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D7FC8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827D7FCC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827D7FD0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827D7FD4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D7FD8: 4082FFE8  bne 0x827d7fc0
	if !ctx.cr[0].eq {
	pc = 0x827D7FC0; continue 'dispatch;
	}
	// 827D7FDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D7FE0: 4BFFDAC9  bl 0x827d5aa8
	ctx.lr = 0x827D7FE4;
	sub_827D5AA8(ctx, base);
	// 827D7FE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D7FE8: 4182001C  beq 0x827d8004
	if ctx.cr[0].eq {
	pc = 0x827D8004; continue 'dispatch;
	}
	// 827D7FEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827D7FF0: 389D00C0  addi r4, r29, 0xc0
	ctx.r[4].s64 = ctx.r[29].s64 + 192;
	// 827D7FF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D7FF8: 4BEED171  bl 0x826c5168
	ctx.lr = 0x827D7FFC;
	sub_826C5168(ctx, base);
	// 827D7FFC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8000: 48000028  b 0x827d8028
	pc = 0x827D8028; continue 'dispatch;
	// 827D8004: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8008: C01D00D8  lfs f0, 0xd8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D800C: C1BB0000  lfs f13, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827D8010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D8014: EC200372  fmuls f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 827D8018: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827D801C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D8020: 4E800421  bctrl
	ctx.lr = 0x827D8024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D8024: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8028: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827D802C: 419A000C  beq cr6, 0x827d8038
	if ctx.cr[6].eq {
	pc = 0x827D8038; continue 'dispatch;
	}
	// 827D8030: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827D8034: 4BAE885D  bl 0x822c0890
	ctx.lr = 0x827D8038;
	sub_822C0890(ctx, base);
	// 827D8038: 817D00C4  lwz r11, 0xc4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(196 as u32) ) } as u64;
	// 827D803C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D8040: 409AFF6C  bne cr6, 0x827d7fac
	if !ctx.cr[6].eq {
	pc = 0x827D7FAC; continue 'dispatch;
	}
	// 827D8044: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827D8048: 489D016C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D8050 size=272
    let mut pc: u32 = 0x827D8050;
    'dispatch: loop {
        match pc {
            0x827D8050 => {
    //   block [0x827D8050..0x827D8160)
	// 827D8050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8054: 489D0115  bl 0x831a8168
	ctx.lr = 0x827D8058;
	sub_831A8130(ctx, base);
	// 827D8058: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D805C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D8060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D8064: 3BBF00CC  addi r29, r31, 0xcc
	ctx.r[29].s64 = ctx.r[31].s64 + 204;
	// 827D8068: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827D806C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827D8070: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827D8074: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8078: 4801E751  bl 0x827f67c8
	ctx.lr = 0x827D807C;
	sub_827F67C8(ctx, base);
	// 827D807C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 827D8080: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827D8084: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D8088: 409A00C0  bne cr6, 0x827d8148
	if !ctx.cr[6].eq {
	pc = 0x827D8148; continue 'dispatch;
	}
	// 827D808C: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D8090: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8094: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827D8098: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 827D809C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827D80A0: 419A0024  beq cr6, 0x827d80c4
	if ctx.cr[6].eq {
	pc = 0x827D80C4; continue 'dispatch;
	}
	// 827D80A4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827D80A8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827D80AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D80B0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827D80B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827D80B8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827D80BC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D80C0: 4082FFE8  bne 0x827d80a8
	if !ctx.cr[0].eq {
	pc = 0x827D80A8; continue 'dispatch;
	}
	// 827D80C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D80C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D80CC: 4861BB35  bl 0x82df3c00
	ctx.lr = 0x827D80D0;
	sub_82DF3C00(ctx, base);
	// 827D80D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D80D4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827D80D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D80DC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827D80E0: 482B3731  bl 0x82a8b810
	ctx.lr = 0x827D80E4;
	sub_82A8B810(ctx, base);
	// 827D80E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D80E8: 4861B341  bl 0x82df3428
	ctx.lr = 0x827D80EC;
	sub_82DF3428(ctx, base);
	// 827D80EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827D80F0: 419A000C  beq cr6, 0x827d80fc
	if ctx.cr[6].eq {
	pc = 0x827D80FC; continue 'dispatch;
	}
	// 827D80F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D80F8: 4BAE8799  bl 0x822c0890
	ctx.lr = 0x827D80FC;
	sub_822C0890(ctx, base);
	// 827D80FC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827D8100: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827D8104: 483AD6AD  bl 0x82b857b0
	ctx.lr = 0x827D8108;
	sub_82B857B0(ctx, base);
	// 827D8108: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827D810C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827D8110: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827D8114: 483AE8AD  bl 0x82b869c0
	ctx.lr = 0x827D8118;
	sub_82B869C0(ctx, base);
	// 827D8118: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 827D811C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D8120: 419A0008  beq cr6, 0x827d8128
	if ctx.cr[6].eq {
	pc = 0x827D8128; continue 'dispatch;
	}
	// 827D8124: 4BAE876D  bl 0x822c0890
	ctx.lr = 0x827D8128;
	sub_822C0890(ctx, base);
	// 827D8128: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827D812C: 4861B2FD  bl 0x82df3428
	ctx.lr = 0x827D8130;
	sub_82DF3428(ctx, base);
	// 827D8130: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 827D8134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D8138: 419A0008  beq cr6, 0x827d8140
	if ctx.cr[6].eq {
	pc = 0x827D8140; continue 'dispatch;
	}
	// 827D813C: 4BAE8755  bl 0x822c0890
	ctx.lr = 0x827D8140;
	sub_822C0890(ctx, base);
	// 827D8140: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827D8144: 4861B2E5  bl 0x82df3428
	ctx.lr = 0x827D8148;
	sub_82DF3428(ctx, base);
	// 827D8148: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D814C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D8150: 419A0008  beq cr6, 0x827d8158
	if ctx.cr[6].eq {
	pc = 0x827D8158; continue 'dispatch;
	}
	// 827D8154: 4BAE873D  bl 0x822c0890
	ctx.lr = 0x827D8158;
	sub_822C0890(ctx, base);
	// 827D8158: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827D815C: 489D005C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D8160 size=100
    let mut pc: u32 = 0x827D8160;
    'dispatch: loop {
        match pc {
            0x827D8160 => {
    //   block [0x827D8160..0x827D81C4)
	// 827D8160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8164: 489D0009  bl 0x831a816c
	ctx.lr = 0x827D8168;
	sub_831A8130(ctx, base);
	// 827D8168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D816C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D8170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D8174: 4BFFFBBD  bl 0x827d7d30
	ctx.lr = 0x827D8178;
	sub_827D7D30(ctx, base);
	// 827D8178: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827D817C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8180: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D8184: 4BD1E25D  bl 0x824f63e0
	ctx.lr = 0x827D8188;
	sub_824F63E0(ctx, base);
	// 827D8188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D818C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827D8190: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827D8194: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827D8198: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 827D819C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D81A0: 48146931  bl 0x8291ead0
	ctx.lr = 0x827D81A4;
	sub_8291EAD0(ctx, base);
	// 827D81A4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827D81A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827D81AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827D81B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827D81B4: 4BD3485D  bl 0x8250ca10
	ctx.lr = 0x827D81B8;
	sub_8250CA10(ctx, base);
	// 827D81B8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827D81BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D81C0: 489CFFFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D81C8 size=8
    let mut pc: u32 = 0x827D81C8;
    'dispatch: loop {
        match pc {
            0x827D81C8 => {
    //   block [0x827D81C8..0x827D81D0)
	// 827D81C8: 386300CC  addi r3, r3, 0xcc
	ctx.r[3].s64 = ctx.r[3].s64 + 204;
	// 827D81CC: 4BFFFF94  b 0x827d8160
	sub_827D8160(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D81D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D81D0 size=260
    let mut pc: u32 = 0x827D81D0;
    'dispatch: loop {
        match pc {
            0x827D81D0 => {
    //   block [0x827D81D0..0x827D82D4)
	// 827D81D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D81D4: 489CFF91  bl 0x831a8164
	ctx.lr = 0x827D81D8;
	sub_831A8130(ctx, base);
	// 827D81D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D81DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827D81E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D81E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827D81E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827D81EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827D81F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D81F4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 827D81F8: 4BFFFCA9  bl 0x827d7ea0
	ctx.lr = 0x827D81FC;
	sub_827D7EA0(ctx, base);
	// 827D81FC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8200: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 827D8204: 83610054  lwz r27, 0x54(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827D8208: 419A00A4  beq cr6, 0x827d82ac
	if ctx.cr[6].eq {
	pc = 0x827D82AC; continue 'dispatch;
	}
	// 827D820C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D8210: 4BFFD859  bl 0x827d5a68
	ctx.lr = 0x827D8214;
	sub_827D5A68(ctx, base);
	// 827D8214: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827D8218: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D821C: 4BFC0A75  bl 0x82798c90
	ctx.lr = 0x827D8220;
	sub_82798C90(ctx, base);
	// 827D8220: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827D8224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D8228: 419A004C  beq cr6, 0x827d8274
	if ctx.cr[6].eq {
	pc = 0x827D8274; continue 'dispatch;
	}
	// 827D822C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8230: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 827D8234: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827D8238: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 827D823C: 419A0024  beq cr6, 0x827d8260
	if ctx.cr[6].eq {
	pc = 0x827D8260; continue 'dispatch;
	}
	// 827D8240: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 827D8244: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827D8248: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D824C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827D8250: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827D8254: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827D8258: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D825C: 4082FFE8  bne 0x827d8244
	if !ctx.cr[0].eq {
	pc = 0x827D8244; continue 'dispatch;
	}
	// 827D8260: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8264: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827D8268: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D826C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D8270: 4E800421  bctrl
	ctx.lr = 0x827D8274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D8274: 83BC00C4  lwz r29, 0xc4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(196 as u32) ) } as u64;
	// 827D8278: 3BDC00C0  addi r30, r28, 0xc0
	ctx.r[30].s64 = ctx.r[28].s64 + 192;
	// 827D827C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 827D8280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D8284: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827D8288: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D828C: 4BEF1645  bl 0x826c98d0
	ctx.lr = 0x827D8290;
	sub_826C98D0(ctx, base);
	// 827D8290: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827D8294: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827D8298: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D829C: 48330085  bl 0x82b08320
	ctx.lr = 0x827D82A0;
	sub_82B08320(ctx, base);
	// 827D82A0: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827D82A4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D82A8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827D82AC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 827D82B0: 419A000C  beq cr6, 0x827d82bc
	if ctx.cr[6].eq {
	pc = 0x827D82BC; continue 'dispatch;
	}
	// 827D82B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827D82B8: 4BAE85D9  bl 0x822c0890
	ctx.lr = 0x827D82BC;
	sub_822C0890(ctx, base);
	// 827D82BC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D82C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D82C4: 419A0008  beq cr6, 0x827d82cc
	if ctx.cr[6].eq {
	pc = 0x827D82CC; continue 'dispatch;
	}
	// 827D82C8: 4BAE85C9  bl 0x822c0890
	ctx.lr = 0x827D82CC;
	sub_822C0890(ctx, base);
	// 827D82CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827D82D0: 489CFEE4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D82D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D82D8 size=128
    let mut pc: u32 = 0x827D82D8;
    'dispatch: loop {
        match pc {
            0x827D82D8 => {
    //   block [0x827D82D8..0x827D8358)
	// 827D82D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D82DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D82E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D82E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D82E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D82EC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D82F0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827D82F4: 396B49BC  addi r11, r11, 0x49bc
	ctx.r[11].s64 = ctx.r[11].s64 + 18876;
	// 827D82F8: 394A49A8  addi r10, r10, 0x49a8
	ctx.r[10].s64 = ctx.r[10].s64 + 18856;
	// 827D82FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D8300: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827D8304: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 827D8308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D830C: 419A0008  beq cr6, 0x827d8314
	if ctx.cr[6].eq {
	pc = 0x827D8314; continue 'dispatch;
	}
	// 827D8310: 4BAE8581  bl 0x822c0890
	ctx.lr = 0x827D8314;
	sub_822C0890(ctx, base);
	// 827D8314: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 827D8318: 4BDD7DF9  bl 0x825b0110
	ctx.lr = 0x827D831C;
	sub_825B0110(ctx, base);
	// 827D831C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 827D8320: 4802E199  bl 0x828064b8
	ctx.lr = 0x827D8324;
	sub_828064B8(ctx, base);
	// 827D8324: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827D8328: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 827D832C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827D8330: 48619E59  bl 0x82df2188
	ctx.lr = 0x827D8334;
	sub_82DF2188(ctx, base);
	// 827D8334: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D8338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D833C: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 827D8340: 4BD38E59  bl 0x82511198
	ctx.lr = 0x827D8344;
	sub_82511198(ctx, base);
	// 827D8344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827D8348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D834C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D8350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D8354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D8358 size=8
    let mut pc: u32 = 0x827D8358;
    'dispatch: loop {
        match pc {
            0x827D8358 => {
    //   block [0x827D8358..0x827D8360)
	// 827D8358: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827D835C: 4800008C  b 0x827d83e8
	sub_827D83E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D8360 size=136
    let mut pc: u32 = 0x827D8360;
    'dispatch: loop {
        match pc {
            0x827D8360 => {
    //   block [0x827D8360..0x827D83E8)
	// 827D8360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D8368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D836C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D8370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D8378: 4BD38D79  bl 0x825110f0
	ctx.lr = 0x827D837C;
	sub_825110F0(ctx, base);
	// 827D837C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D8380: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827D8384: 396B49BC  addi r11, r11, 0x49bc
	ctx.r[11].s64 = ctx.r[11].s64 + 18876;
	// 827D8388: 394A49A8  addi r10, r10, 0x49a8
	ctx.r[10].s64 = ctx.r[10].s64 + 18856;
	// 827D838C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D8390: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 827D8394: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827D8398: 4832D8F9  bl 0x82b05c90
	ctx.lr = 0x827D839C;
	sub_82B05C90(ctx, base);
	// 827D839C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827D83A0: 907F00C4  stw r3, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[3].u32 ) };
	// 827D83A4: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 827D83A8: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 827D83AC: 4BFC384D  bl 0x8279bbf8
	ctx.lr = 0x827D83B0;
	sub_8279BBF8(ctx, base);
	// 827D83B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827D83B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D83B8: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D83BC: D01F00D8  stfs f0, 0xd8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 827D83C0: 93DF00DC  stw r30, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[30].u32 ) };
	// 827D83C4: 93DF00E0  stw r30, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[30].u32 ) };
	// 827D83C8: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 827D83CC: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 827D83D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D83D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D83D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D83DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D83E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D83E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D83E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D83E8 size=76
    let mut pc: u32 = 0x827D83E8;
    'dispatch: loop {
        match pc {
            0x827D83E8 => {
    //   block [0x827D83E8..0x827D8434)
	// 827D83E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D83EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D83F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D83F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D83F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D83FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D8400: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D8404: 4BFFFED5  bl 0x827d82d8
	ctx.lr = 0x827D8408;
	sub_827D82D8(ctx, base);
	// 827D8408: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D840C: 4182000C  beq 0x827d8418
	if ctx.cr[0].eq {
	pc = 0x827D8418; continue 'dispatch;
	}
	// 827D8410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8414: 48619FC5  bl 0x82df23d8
	ctx.lr = 0x827D8418;
	sub_82DF23D8(ctx, base);
	// 827D8418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D841C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D8420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D8424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D8428: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D842C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D8430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D8438 size=36
    let mut pc: u32 = 0x827D8438;
    'dispatch: loop {
        match pc {
            0x827D8438 => {
    //   block [0x827D8438..0x827D845C)
	// 827D8438: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827D843C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 827D8440: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827D8444: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 827D8448: 394A49E8  addi r10, r10, 0x49e8
	ctx.r[10].s64 = ctx.r[10].s64 + 18920;
	// 827D844C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D8450: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827D8454: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827D8458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D8460 size=88
    let mut pc: u32 = 0x827D8460;
    'dispatch: loop {
        match pc {
            0x827D8460 => {
    //   block [0x827D8460..0x827D84B8)
	// 827D8460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D8468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D846C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D8470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D8478: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D847C: 48639A5D  bl 0x82e11ed8
	ctx.lr = 0x827D8480;
	sub_82E11ED8(ctx, base);
	// 827D8480: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827D8484: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 827D8488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D848C: 394A49F8  addi r10, r10, 0x49f8
	ctx.r[10].s64 = ctx.r[10].s64 + 18936;
	// 827D8490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8494: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827D8498: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827D849C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827D84A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D84A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D84A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D84AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D84B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D84B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D84B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D84B8 size=392
    let mut pc: u32 = 0x827D84B8;
    'dispatch: loop {
        match pc {
            0x827D84B8 => {
    //   block [0x827D84B8..0x827D8640)
	// 827D84B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D84BC: 489CFCAD  bl 0x831a8168
	ctx.lr = 0x827D84C0;
	sub_831A8130(ctx, base);
	// 827D84C0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 827D84C4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827D84C8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D8640 size=936
    let mut pc: u32 = 0x827D8640;
    'dispatch: loop {
        match pc {
            0x827D8640 => {
    //   block [0x827D8640..0x827D89E8)
	// 827D8640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8644: 489CFB15  bl 0x831a8158
	ctx.lr = 0x827D8648;
	sub_831A8130(ctx, base);
	// 827D8648: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D864C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827D8650: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827D8654: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 827D8658: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827D865C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D8660: 816B0130  lwz r11, 0x130(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(304 as u32) ) } as u64;
	// 827D8664: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8668: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 827D866C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 827D8670: 41980008  blt cr6, 0x827d8678
	if ctx.cr[6].lt {
	pc = 0x827D8678; continue 'dispatch;
	}
	// 827D8674: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827D8678: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D867C: 41820364  beq 0x827d89e0
	if ctx.cr[0].eq {
	pc = 0x827D89E0; continue 'dispatch;
	}
	// 827D8680: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827D8684: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827D8688: 3BEB87B0  addi r31, r11, -0x7850
	ctx.r[31].s64 = ctx.r[11].s64 + -30800;
	// 827D868C: 816A87B4  lwz r11, -0x784c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30796 as u32) ) } as u64;
	// 827D8690: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827D8694: 4082001C  bne 0x827d86b0
	if !ctx.cr[0].eq {
	pc = 0x827D86B0; continue 'dispatch;
	}
	// 827D8698: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827D869C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 827D86A0: 916A87B4  stw r11, -0x784c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30796 as u32), ctx.r[11].u32 ) };
	// 827D86A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D86A8: 38899430  addi r4, r9, -0x6bd0
	ctx.r[4].s64 = ctx.r[9].s64 + -27600;
	// 827D86AC: 48620E85  bl 0x82df9530
	ctx.lr = 0x827D86B0;
	sub_82DF9530(ctx, base);
	// 827D86B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D86B4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D86B8: 409A0328  bne cr6, 0x827d89e0
	if !ctx.cr[6].eq {
	pc = 0x827D89E0; continue 'dispatch;
	}
	// 827D86BC: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D86C0: 3B9D000C  addi r28, r29, 0xc
	ctx.r[28].s64 = ctx.r[29].s64 + 12;
	// 827D86C4: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 827D86C8: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 827D86CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D86D0: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D86D4: 409A00C0  bne cr6, 0x827d8794
	if !ctx.cr[6].eq {
	pc = 0x827D8794; continue 'dispatch;
	}
	// 827D86D8: 3D60002A  lis r11, 0x2a
	ctx.r[11].s64 = 2752512;
	// 827D86DC: B3C10050  sth r30, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u16 ) };
	// 827D86E0: 3D200018  lis r9, 0x18
	ctx.r[9].s64 = 1572864;
	// 827D86E4: B3C10052  sth r30, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[30].u16 ) };
	// 827D86E8: 616B23B9  ori r11, r11, 0x23b9
	ctx.r[11].u64 = ctx.r[11].u64 | 9145;
	// 827D86EC: 9BC10058  stb r30, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u8 ) };
	// 827D86F0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 827D86F4: 9BC10059  stb r30, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[30].u8 ) };
	// 827D86F8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 827D86FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827D8700: 3D40002C  lis r10, 0x2c
	ctx.r[10].s64 = 2883584;
	// 827D8704: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 827D8708: 612B2886  ori r11, r9, 0x2886
	ctx.r[11].u64 = ctx.r[9].u64 | 10374;
	// 827D870C: B101005E  sth r8, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[8].u16 ) };
	// 827D8710: 98E10065  stb r7, 0x65(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(101 as u32), ctx.r[7].u8 ) };
	// 827D8714: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 827D8718: 614A23A5  ori r10, r10, 0x23a5
	ctx.r[10].u64 = ctx.r[10].u64 | 9125;
	// 827D871C: 9BC1005A  stb r30, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[30].u8 ) };
	// 827D8720: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 827D8724: B3C1005C  sth r30, 0x5c(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u16 ) };
	// 827D8728: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 827D872C: 9BC10064  stb r30, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u8 ) };
	// 827D8730: 38E000FF  li r7, 0xff
	ctx.r[7].s64 = 255;
	// 827D8734: 9BC10066  stb r30, 0x66(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[30].u8 ) };
	// 827D8738: 3B00FFFF  li r24, -1
	ctx.r[24].s64 = -1;
	// 827D873C: B3C10068  sth r30, 0x68(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u16 ) };
	// 827D8740: B0C1006A  sth r6, 0x6a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(106 as u32), ctx.r[6].u16 ) };
	// 827D8744: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827D8748: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 827D874C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827D8750: 9BC10070  stb r30, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u8 ) };
	// 827D8754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8758: 99010071  stb r8, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[8].u8 ) };
	// 827D875C: 9BC10072  stb r30, 0x72(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(114 as u32), ctx.r[30].u8 ) };
	// 827D8760: B3C10074  sth r30, 0x74(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u16 ) };
	// 827D8764: B3410076  sth r26, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[26].u16 ) };
	// 827D8768: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 827D876C: 9BC1007C  stb r30, 0x7c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u8 ) };
	// 827D8770: 9921007D  stb r9, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[9].u8 ) };
	// 827D8774: 9BC1007E  stb r30, 0x7e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[30].u8 ) };
	// 827D8778: B0E10080  sth r7, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[7].u16 ) };
	// 827D877C: B3C10082  sth r30, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[30].u16 ) };
	// 827D8780: 93010084  stw r24, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[24].u32 ) };
	// 827D8784: 9BC10088  stb r30, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[30].u8 ) };
	// 827D8788: 9BC10089  stb r30, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[30].u8 ) };
	// 827D878C: 9BC1008A  stb r30, 0x8a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(138 as u32), ctx.r[30].u8 ) };
	// 827D8790: 486369D9  bl 0x82e0f168
	ctx.lr = 0x827D8794;
	sub_82E0F168(ctx, base);
	// 827D8794: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8798: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D879C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 827D87A0: 419A0010  beq cr6, 0x827d87b0
	if ctx.cr[6].eq {
	pc = 0x827D87B0; continue 'dispatch;
	}
	// 827D87A4: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 827D87A8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D87AC: 48400125  bl 0x82bd88d0
	ctx.lr = 0x827D87B0;
	sub_82BD88D0(ctx, base);
	// 827D87B0: 897F0026  lbz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 827D87B4: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 827D87B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D87BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827D87C0: 4082000C  bne 0x827d87cc
	if !ctx.cr[0].eq {
	pc = 0x827D87CC; continue 'dispatch;
	}
	// 827D87C4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 827D87C8: 48000028  b 0x827d87f0
	pc = 0x827D87F0; continue 'dispatch;
	// 827D87CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827D87D0: 4BB0CB19  bl 0x822e52e8
	ctx.lr = 0x827D87D4;
	sub_822E52E8(ctx, base);
	// 827D87D4: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 827D87D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D87DC: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 827D87E0: 4BB0CB09  bl 0x822e52e8
	ctx.lr = 0x827D87E4;
	sub_822E52E8(ctx, base);
	// 827D87E4: 80BF0034  lwz r5, 0x34(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827D87E8: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 827D87EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D87F0: 4BB0CAF9  bl 0x822e52e8
	ctx.lr = 0x827D87F4;
	sub_822E52E8(ctx, base);
	// 827D87F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827D87F8: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 827D87FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8800: 4BB0CAE9  bl 0x822e52e8
	ctx.lr = 0x827D8804;
	sub_822E52E8(ctx, base);
	// 827D8804: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827D8808: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 827D880C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8810: 4BB0CAD9  bl 0x822e52e8
	ctx.lr = 0x827D8814;
	sub_822E52E8(ctx, base);
	// 827D8814: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D8818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D881C: 388B0074  addi r4, r11, 0x74
	ctx.r[4].s64 = ctx.r[11].s64 + 116;
	// 827D8820: 3BCB007C  addi r30, r11, 0x7c
	ctx.r[30].s64 = ctx.r[11].s64 + 124;
	// 827D8824: 48636305  bl 0x82e0eb28
	ctx.lr = 0x827D8828;
	sub_82E0EB28(ctx, base);
	// 827D8828: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D882C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8830: 486363F1  bl 0x82e0ec20
	ctx.lr = 0x827D8834;
	sub_82E0EC20(ctx, base);
	// 827D8834: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827D8838: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 827D883C: C06B08A4  lfs f3, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 827D8840: FC401890  fmr f2, f3
	ctx.f[2].f64 = ctx.f[3].f64;
	// 827D8844: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 827D8848: 486A37D1  bl 0x82e7c018
	ctx.lr = 0x827D884C;
	sub_82E7C018(ctx, base);
	// 827D884C: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 827D8850: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D8854: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 827D8858: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 827D885C: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 827D8860: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 827D8864: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827D8868: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827D886C: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 827D8870: 3BC100E0  addi r30, r1, 0xe0
	ctx.r[30].s64 = ctx.r[1].s64 + 224;
	// 827D8874: 13C95407  vcmpneb. (lvlx128) v30, v9, v10
	tmp.u32 = ctx.r[9].u32 + ctx.r[10].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827D8878: 3B8100F0  addi r28, r1, 0xf0
	ctx.r[28].s64 = ctx.r[1].s64 + 240;
	// 827D887C: 13BA4407  vcmpneb. (lvlx128) v29, v26, v8
	tmp.u32 = ctx.r[26].u32 + ctx.r[8].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827D8880: 3B010100  addi r24, r1, 0x100
	ctx.r[24].s64 = ctx.r[1].s64 + 256;
	// 827D8884: 13863C07  vcmpneb. (lvlx128) v28, v6, v7
	tmp.u32 = ctx.r[6].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827D8888: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D89E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D89E8 size=108
    let mut pc: u32 = 0x827D89E8;
    'dispatch: loop {
        match pc {
            0x827D89E8 => {
    //   block [0x827D89E8..0x827D8A54)
	// 827D89E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D89EC: 489CF77D  bl 0x831a8168
	ctx.lr = 0x827D89F0;
	sub_831A8130(ctx, base);
	// 827D89F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D89F4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827D89F8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827D89FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D8A00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827D8A04: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827D8A08: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827D8A0C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827D8A10: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 827D8A14: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 827D8A18: 486196B1  bl 0x82df20c8
	ctx.lr = 0x827D8A1C;
	sub_82DF20C8(ctx, base);
	// 827D8A1C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827D8A20: 41820008  beq 0x827d8a28
	if ctx.cr[0].eq {
	pc = 0x827D8A28; continue 'dispatch;
	}
	// 827D8A24: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827D8A28: 357F0004  addic. r11, r31, 4
	ctx.xer.ca = (ctx.r[31].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D8A2C: 41820008  beq 0x827d8a34
	if ctx.cr[0].eq {
	pc = 0x827D8A34; continue 'dispatch;
	}
	// 827D8A30: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827D8A34: 347F0008  addic. r3, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 827D8A38: 41820010  beq 0x827d8a48
	if ctx.cr[0].eq {
	pc = 0x827D8A48; continue 'dispatch;
	}
	// 827D8A3C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 827D8A40: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827D8A44: 489CFACD  bl 0x831a8510
	ctx.lr = 0x827D8A48;
	sub_831A8510(ctx, base);
	// 827D8A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D8A50: 489CF768  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D8A58 size=108
    let mut pc: u32 = 0x827D8A58;
    'dispatch: loop {
        match pc {
            0x827D8A58 => {
    //   block [0x827D8A58..0x827D8AC4)
	// 827D8A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8A5C: 489CF70D  bl 0x831a8168
	ctx.lr = 0x827D8A60;
	sub_831A8130(ctx, base);
	// 827D8A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8A64: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827D8A68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827D8A6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D8A70: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827D8A74: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827D8A78: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 827D8A7C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827D8A80: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 827D8A84: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 827D8A88: 48619641  bl 0x82df20c8
	ctx.lr = 0x827D8A8C;
	sub_82DF20C8(ctx, base);
	// 827D8A8C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827D8A90: 41820008  beq 0x827d8a98
	if ctx.cr[0].eq {
	pc = 0x827D8A98; continue 'dispatch;
	}
	// 827D8A94: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827D8A98: 357F0004  addic. r11, r31, 4
	ctx.xer.ca = (ctx.r[31].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D8A9C: 41820008  beq 0x827d8aa4
	if ctx.cr[0].eq {
	pc = 0x827D8AA4; continue 'dispatch;
	}
	// 827D8AA0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827D8AA4: 347F0008  addic. r3, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 827D8AA8: 41820010  beq 0x827d8ab8
	if ctx.cr[0].eq {
	pc = 0x827D8AB8; continue 'dispatch;
	}
	// 827D8AAC: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 827D8AB0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827D8AB4: 489CFA5D  bl 0x831a8510
	ctx.lr = 0x827D8AB8;
	sub_831A8510(ctx, base);
	// 827D8AB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D8AC0: 489CF6F8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D8AC8 size=196
    let mut pc: u32 = 0x827D8AC8;
    'dispatch: loop {
        match pc {
            0x827D8AC8 => {
    //   block [0x827D8AC8..0x827D8B8C)
	// 827D8AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D8AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D8AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D8AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8ADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D8AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D8AE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827D8AE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827D8AEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D8AF0: 4BAE7E49  bl 0x822c0938
	ctx.lr = 0x827D8AF4;
	sub_822C0938(ctx, base);
	// 827D8AF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827D8AF8: 41820028  beq 0x827d8b20
	if ctx.cr[0].eq {
	pc = 0x827D8B20; continue 'dispatch;
	}
	// 827D8AFC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D8B00: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827D8B04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827D8B08: 392B4A08  addi r9, r11, 0x4a08
	ctx.r[9].s64 = ctx.r[11].s64 + 18952;
	// 827D8B0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827D8B10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827D8B14: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827D8B18: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827D8B1C: 48000008  b 0x827d8b24
	pc = 0x827D8B24; continue 'dispatch;
	// 827D8B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D8B24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D8B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D8B2C: 409A0044  bne cr6, 0x827d8b70
	if !ctx.cr[6].eq {
	pc = 0x827D8B70; continue 'dispatch;
	}
	// 827D8B30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827D8B34: 419A001C  beq cr6, 0x827d8b50
	if ctx.cr[6].eq {
	pc = 0x827D8B50; continue 'dispatch;
	}
	// 827D8B38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8B3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827D8B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8B44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D8B4C: 4E800421  bctrl
	ctx.lr = 0x827D8B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D8B50: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827D8B54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827D8B58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8B5C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827D8B60: 816BB5B0  lwz r11, -0x4a50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19024 as u32) ) } as u64;
	// 827D8B64: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827D8B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827D8B6C: 4BAE7495  bl 0x822c0000
	ctx.lr = 0x827D8B70;
	sub_822C0000(ctx, base);
	// 827D8B70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D8B74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D8B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D8B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D8B80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D8B84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D8B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D8B90 size=196
    let mut pc: u32 = 0x827D8B90;
    'dispatch: loop {
        match pc {
            0x827D8B90 => {
    //   block [0x827D8B90..0x827D8C54)
	// 827D8B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D8B98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D8B9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D8BA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8BA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D8BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D8BAC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827D8BB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827D8BB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D8BB8: 4BAE7D81  bl 0x822c0938
	ctx.lr = 0x827D8BBC;
	sub_822C0938(ctx, base);
	// 827D8BBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827D8BC0: 41820028  beq 0x827d8be8
	if ctx.cr[0].eq {
	pc = 0x827D8BE8; continue 'dispatch;
	}
	// 827D8BC4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D8BC8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827D8BCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827D8BD0: 392B4A1C  addi r9, r11, 0x4a1c
	ctx.r[9].s64 = ctx.r[11].s64 + 18972;
	// 827D8BD4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827D8BD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827D8BDC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827D8BE0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827D8BE4: 48000008  b 0x827d8bec
	pc = 0x827D8BEC; continue 'dispatch;
	// 827D8BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D8BEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D8BF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D8BF4: 409A0044  bne cr6, 0x827d8c38
	if !ctx.cr[6].eq {
	pc = 0x827D8C38; continue 'dispatch;
	}
	// 827D8BF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827D8BFC: 419A001C  beq cr6, 0x827d8c18
	if ctx.cr[6].eq {
	pc = 0x827D8C18; continue 'dispatch;
	}
	// 827D8C00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8C04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827D8C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8C0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8C10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D8C14: 4E800421  bctrl
	ctx.lr = 0x827D8C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D8C18: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827D8C1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827D8C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8C24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827D8C28: 816BB5B0  lwz r11, -0x4a50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19024 as u32) ) } as u64;
	// 827D8C2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827D8C30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827D8C34: 4BAE73CD  bl 0x822c0000
	ctx.lr = 0x827D8C38;
	sub_822C0000(ctx, base);
	// 827D8C38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D8C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D8C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D8C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D8C48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D8C4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D8C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D8C58 size=420
    let mut pc: u32 = 0x827D8C58;
    'dispatch: loop {
        match pc {
            0x827D8C58 => {
    //   block [0x827D8C58..0x827D8DFC)
	// 827D8C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D8C60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D8C64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D8C68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8C6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D8C70: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8C74: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827D8C78: 48651E71  bl 0x82e2aae8
	ctx.lr = 0x827D8C7C;
	sub_82E2AAE8(ctx, base);
	// 827D8C7C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D8C80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8C84: 388B4A3C  addi r4, r11, 0x4a3c
	ctx.r[4].s64 = ctx.r[11].s64 + 19004;
	// 827D8C88: 4861AD81  bl 0x82df3a08
	ctx.lr = 0x827D8C8C;
	sub_82DF3A08(ctx, base);
	// 827D8C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827D8C90: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827D8C94: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827D8C98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827D8C9C: 48652F45  bl 0x82e2bbe0
	ctx.lr = 0x827D8CA0;
	sub_82E2BBE0(ctx, base);
	// 827D8CA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827D8CA4: 395F0064  addi r10, r31, 0x64
	ctx.r[10].s64 = ctx.r[31].s64 + 100;
	// 827D8CA8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827D8CAC: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827D8CB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8CB4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827D8CB8: 4BAEB7A9  bl 0x822c4460
	ctx.lr = 0x827D8CBC;
	sub_822C4460(ctx, base);
	// 827D8CBC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827D8CC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D8CC4: 419A0008  beq cr6, 0x827d8ccc
	if ctx.cr[6].eq {
	pc = 0x827D8CCC; continue 'dispatch;
	}
	// 827D8CC8: 4BAE7BC9  bl 0x822c0890
	ctx.lr = 0x827D8CCC;
	sub_822C0890(ctx, base);
	// 827D8CCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8CD0: 4861A759  bl 0x82df3428
	ctx.lr = 0x827D8CD4;
	sub_82DF3428(ctx, base);
	// 827D8CD4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D8CD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8CDC: 388B4A2C  addi r4, r11, 0x4a2c
	ctx.r[4].s64 = ctx.r[11].s64 + 18988;
	// 827D8CE0: 4861AD29  bl 0x82df3a08
	ctx.lr = 0x827D8CE4;
	sub_82DF3A08(ctx, base);
	// 827D8CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827D8CE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827D8CEC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827D8CF0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827D8CF4: 48652EED  bl 0x82e2bbe0
	ctx.lr = 0x827D8CF8;
	sub_82E2BBE0(ctx, base);
	// 827D8CF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827D8CFC: 395F006C  addi r10, r31, 0x6c
	ctx.r[10].s64 = ctx.r[31].s64 + 108;
	// 827D8D00: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827D8D04: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827D8D08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8D0C: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827D8D10: 4BAEB751  bl 0x822c4460
	ctx.lr = 0x827D8D14;
	sub_822C4460(ctx, base);
	// 827D8D14: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827D8D18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D8D1C: 419A0008  beq cr6, 0x827d8d24
	if ctx.cr[6].eq {
	pc = 0x827D8D24; continue 'dispatch;
	}
	// 827D8D20: 4BAE7B71  bl 0x822c0890
	ctx.lr = 0x827D8D24;
	sub_822C0890(ctx, base);
	// 827D8D24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8D28: 4861A701  bl 0x82df3428
	ctx.lr = 0x827D8D2C;
	sub_82DF3428(ctx, base);
	// 827D8D2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827D8D30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8D34: 3BCB7C24  addi r30, r11, 0x7c24
	ctx.r[30].s64 = ctx.r[11].s64 + 31780;
	// 827D8D38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D8D3C: 4861ACCD  bl 0x82df3a08
	ctx.lr = 0x827D8D40;
	sub_82DF3A08(ctx, base);
	// 827D8D40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827D8D44: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827D8D48: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827D8D4C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827D8D50: 48655C11  bl 0x82e2e960
	ctx.lr = 0x827D8D54;
	sub_82E2E960(ctx, base);
	// 827D8D54: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827D8D58: 395F0074  addi r10, r31, 0x74
	ctx.r[10].s64 = ctx.r[31].s64 + 116;
	// 827D8D5C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827D8D60: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827D8D64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8D68: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827D8D6C: 4BAEB6F5  bl 0x822c4460
	ctx.lr = 0x827D8D70;
	sub_822C4460(ctx, base);
	// 827D8D70: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 827D8D74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D8D78: 419A0008  beq cr6, 0x827d8d80
	if ctx.cr[6].eq {
	pc = 0x827D8D80; continue 'dispatch;
	}
	// 827D8D7C: 4BAE7B15  bl 0x822c0890
	ctx.lr = 0x827D8D80;
	sub_822C0890(ctx, base);
	// 827D8D80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8D84: 4861A6A5  bl 0x82df3428
	ctx.lr = 0x827D8D88;
	sub_82DF3428(ctx, base);
	// 827D8D88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D8D8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8D90: 4861AC79  bl 0x82df3a08
	ctx.lr = 0x827D8D94;
	sub_82DF3A08(ctx, base);
	// 827D8D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827D8D98: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827D8D9C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827D8DA0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827D8DA4: 48655A6D  bl 0x82e2e810
	ctx.lr = 0x827D8DA8;
	sub_82E2E810(ctx, base);
	// 827D8DA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827D8DAC: 395F007C  addi r10, r31, 0x7c
	ctx.r[10].s64 = ctx.r[31].s64 + 124;
	// 827D8DB0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827D8DB4: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827D8DB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8DBC: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 827D8DC0: 4BAEB6A1  bl 0x822c4460
	ctx.lr = 0x827D8DC4;
	sub_822C4460(ctx, base);
	// 827D8DC4: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 827D8DC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D8DCC: 419A0008  beq cr6, 0x827d8dd4
	if ctx.cr[6].eq {
	pc = 0x827D8DD4; continue 'dispatch;
	}
	// 827D8DD0: 4BAE7AC1  bl 0x822c0890
	ctx.lr = 0x827D8DD4;
	sub_822C0890(ctx, base);
	// 827D8DD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8DD8: 4861A651  bl 0x82df3428
	ctx.lr = 0x827D8DDC;
	sub_82DF3428(ctx, base);
	// 827D8DDC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827D8DE0: 48651D21  bl 0x82e2ab00
	ctx.lr = 0x827D8DE4;
	sub_82E2AB00(ctx, base);
	// 827D8DE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827D8DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D8DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D8DF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D8DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D8DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D8E00 size=76
    let mut pc: u32 = 0x827D8E00;
    'dispatch: loop {
        match pc {
            0x827D8E00 => {
    //   block [0x827D8E00..0x827D8E4C)
	// 827D8E00: 81630144  lwz r11, 0x144(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(324 as u32) ) } as u64;
	// 827D8E04: D021001C  stfs f1, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 827D8E08: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827D8E0C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827D8E10: 40980008  bge cr6, 0x827d8e18
	if !ctx.cr[6].lt {
	pc = 0x827D8E18; continue 'dispatch;
	}
	// 827D8E14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827D8E18: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 827D8E1C: 81230140  lwz r9, 0x140(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(320 as u32) ) } as u64;
	// 827D8E20: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827D8E24: F961FFE0  std r11, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[11].u64 ) };
	// 827D8E28: C801FFE0  lfd f0, -0x20(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827D8E2C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 827D8E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827D8E34: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 827D8E38: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8E3C: C00808A8  lfs f0, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D8E40: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 827D8E44: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 827D8E48: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8E4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D8E4C size=172
    let mut pc: u32 = 0x827D8E4C;
    'dispatch: loop {
        match pc {
            0x827D8E4C => {
    //   block [0x827D8E4C..0x827D8EF8)
	// 827D8E4C: 38E30100  addi r7, r3, 0x100
	ctx.r[7].s64 = ctx.r[3].s64 + 256;
	// 827D8E50: 38C300E0  addi r6, r3, 0xe0
	ctx.r[6].s64 = ctx.r[3].s64 + 224;
	// 827D8E54: 7D4807B4  extsw r8, r10
	ctx.r[8].s64 = ctx.r[10].s32 as i64;
	// 827D8E58: 13E038C7  vcmpequd (lvx128) v31, v0, v7
	tmp.u32 = ctx.r[7].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827D8E5C: 38A1001C  addi r5, r1, 0x1c
	ctx.r[5].s64 = ctx.r[1].s64 + 28;
	// 827D8E60: 13C030C7  vcmpequd (lvx128) v30, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827D8E64: F901FFE8  std r8, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[8].u64 ) };
	// 827D8E68: C9A1FFE8  lfd f13, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D8E6C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 827D8E70: 3901001C  addi r8, r1, 0x1c
	ctx.r[8].s64 = ctx.r[1].s64 + 28;
	// 827D8E74: 13A02C07  vcmpneb. (lvlx128) v29, v0, v5
	tmp.u32 = ctx.r[5].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827D8E78: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 827D8E7C: 13804407  vcmpneb. (lvlx128) v28, v0, v8
	tmp.u32 = ctx.r[8].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D8EF8 size=228
    let mut pc: u32 = 0x827D8EF8;
    'dispatch: loop {
        match pc {
            0x827D8EF8 => {
    //   block [0x827D8EF8..0x827D8FDC)
	// 827D8EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8EFC: 489CF271  bl 0x831a816c
	ctx.lr = 0x827D8F00;
	sub_831A8130(ctx, base);
	// 827D8F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D8F08: 817F0138  lwz r11, 0x138(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 827D8F0C: 815F0144  lwz r10, 0x144(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 827D8F10: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 827D8F14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8F18: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 827D8F1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D8F20: 40980038  bge cr6, 0x827d8f58
	if !ctx.cr[6].lt {
	pc = 0x827D8F58; continue 'dispatch;
	}
	// 827D8F24: 4E800421  bctrl
	ctx.lr = 0x827D8F28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D8F28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D8F2C: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 827D8F30: 3BBF00E0  addi r29, r31, 0xe0
	ctx.r[29].s64 = ctx.r[31].s64 + 224;
	// 827D8F34: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8F38: 4BB2CB39  bl 0x82305a70
	ctx.lr = 0x827D8F3C;
	sub_82305A70(ctx, base);
	// 827D8F3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827D8F40: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 827D8F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D8F48: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827D8F4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D8F50: 4E800421  bctrl
	ctx.lr = 0x827D8F54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D8F54: 48000008  b 0x827d8f5c
	pc = 0x827D8F5C; continue 'dispatch;
	// 827D8F58: 4E800421  bctrl
	ctx.lr = 0x827D8F5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D8F5C: 817F0144  lwz r11, 0x144(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 827D8F60: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 827D8F64: 40990070  ble cr6, 0x827d8fd4
	if !ctx.cr[6].gt {
	pc = 0x827D8FD4; continue 'dispatch;
	}
	// 827D8F68: 815F0130  lwz r10, 0x130(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 827D8F6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D8F70: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D8F74: 916A01A8  stw r11, 0x1a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(424 as u32), ctx.r[11].u32 ) };
	// 827D8F78: 916A0350  stw r11, 0x350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(848 as u32), ctx.r[11].u32 ) };
	// 827D8F7C: 817F0140  lwz r11, 0x140(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 827D8F80: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8F84: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D8F88: 419A0044  beq cr6, 0x827d8fcc
	if ctx.cr[6].eq {
	pc = 0x827D8FCC; continue 'dispatch;
	}
	// 827D8F8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827D8F90: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D8F94: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827D8F98: C01E0014  lfs f0, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D8F9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827D8FA0: C1BE0018  lfs f13, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827D8FA4: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 827D8FA8: C19E001C  lfs f12, 0x1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827D8FAC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827D8FB0: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827D8FB4: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827D8FB8: 4BFFD7F9  bl 0x827d67b0
	ctx.lr = 0x827D8FBC;
	sub_827D67B0(ctx, base);
	// 827D8FBC: 817F0140  lwz r11, 0x140(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 827D8FC0: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8FC4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D8FC8: 409AFFD0  bne cr6, 0x827d8f98
	if !ctx.cr[6].eq {
	pc = 0x827D8F98; continue 'dispatch;
	}
	// 827D8FCC: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 827D8FD0: 4BFFD869  bl 0x827d6838
	ctx.lr = 0x827D8FD4;
	sub_827D6838(ctx, base);
	// 827D8FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D8FD8: 489CF1E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D8FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D8FE0 size=252
    let mut pc: u32 = 0x827D8FE0;
    'dispatch: loop {
        match pc {
            0x827D8FE0 => {
    //   block [0x827D8FE0..0x827D90DC)
	// 827D8FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D8FE4: 489CF189  bl 0x831a816c
	ctx.lr = 0x827D8FE8;
	sub_831A8130(ctx, base);
	// 827D8FE8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827D8FEC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D8FF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D8FF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D8FF8: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 827D8FFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827D9000: 4E800421  bctrl
	ctx.lr = 0x827D9004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827D9004: 817E0130  lwz r11, 0x130(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) } as u64;
	// 827D9008: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D900C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 827D9010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D9014: 41980008  blt cr6, 0x827d901c
	if ctx.cr[6].lt {
	pc = 0x827D901C; continue 'dispatch;
	}
	// 827D9018: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827D901C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D9020: 418200B0  beq 0x827d90d0
	if ctx.cr[0].eq {
	pc = 0x827D90D0; continue 'dispatch;
	}
	// 827D9024: 815E00B4  lwz r10, 0xb4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 827D9028: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827D902C: 817E00B8  lwz r11, 0xb8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 827D9030: 83EA0000  lwz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D9034: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827D9038: 419A0068  beq cr6, 0x827d90a0
	if ctx.cr[6].eq {
	pc = 0x827D90A0; continue 'dispatch;
	}
	// 827D903C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 827D9040: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 827D9044: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827D9048: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 827D904C: FFE00018  frsp f31, f0
	ctx.f[31].f64 = (ctx.f[0].f64 as f32) as f64;
	// 827D9050: 7FAB07B4  extsw r11, r29
	ctx.r[11].s64 = ctx.r[29].s32 as i64;
	// 827D9054: 807E0130  lwz r3, 0x130(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) } as u64;
	// 827D9058: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827D905C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 827D9060: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827D9064: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 827D9068: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 827D906C: EC20F824  fdivs f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[31].f64) as f32) as f64;
	// 827D9070: 4BFFD8B1  bl 0x827d6920
	ctx.lr = 0x827D9074;
	sub_827D6920(ctx, base);
	// 827D9074: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D9078: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 827D907C: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827D9080: C1810068  lfs f12, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827D9084: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827D9088: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827D908C: D19F0010  stfs f12, 0x10(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 827D9090: 817E00B4  lwz r11, 0xb4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 827D9094: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D9098: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D909C: 409AFFB4  bne cr6, 0x827d9050
	if !ctx.cr[6].eq {
	pc = 0x827D9050; continue 'dispatch;
	}
	// 827D90A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D90A4: 4BFFC9CD  bl 0x827d5a70
	ctx.lr = 0x827D90A8;
	sub_827D5A70(ctx, base);
	// 827D90A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D90AC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827D90B0: 4BFFC9C9  bl 0x827d5a78
	ctx.lr = 0x827D90B4;
	sub_827D5A78(ctx, base);
	// 827D90B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D90B8: EFFF0824  fdivs f31, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ((ctx.f[31].f64 / ctx.f[1].f64) as f32) as f64;
	// 827D90BC: C03E0110  lfs f1, 0x110(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827D90C0: 48000F31  bl 0x827d9ff0
	ctx.lr = 0x827D90C4;
	sub_827D9FF0(ctx, base);
	// 827D90C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827D90C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827D90CC: 48000EFD  bl 0x827d9fc8
	ctx.lr = 0x827D90D0;
	sub_827D9FC8(ctx, base);
	// 827D90D0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827D90D4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827D90D8: 489CF0E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D90E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D90E0 size=1104
    let mut pc: u32 = 0x827D90E0;
    'dispatch: loop {
        match pc {
            0x827D90E0 => {
    //   block [0x827D90E0..0x827D9530)
	// 827D90E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D90E4: 489CF07D  bl 0x831a8160
	ctx.lr = 0x827D90E8;
	sub_831A8130(ctx, base);
	// 827D90E8: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 827D90EC: 489CF989  bl 0x831a8a74
	ctx.lr = 0x827D90F0;
	sub_831A8A40(ctx, base);
	// 827D90F0: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D9530 size=260
    let mut pc: u32 = 0x827D9530;
    'dispatch: loop {
        match pc {
            0x827D9530 => {
    //   block [0x827D9530..0x827D9634)
	// 827D9530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9534: 489CEC39  bl 0x831a816c
	ctx.lr = 0x827D9538;
	sub_831A8130(ctx, base);
	// 827D9538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D953C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D9540: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D9544: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827D9548: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827D954C: 388B4A48  addi r4, r11, 0x4a48
	ctx.r[4].s64 = ctx.r[11].s64 + 19016;
	// 827D9550: 38A0007B  li r5, 0x7b
	ctx.r[5].s64 = 123;
	// 827D9554: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 827D9558: 48618E91  bl 0x82df23e8
	ctx.lr = 0x827D955C;
	sub_82DF23E8(ctx, base);
	// 827D955C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827D9560: 41820014  beq 0x827d9574
	if ctx.cr[0].eq {
	pc = 0x827D9574; continue 'dispatch;
	}
	// 827D9564: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D9568: 4BFFEEF9  bl 0x827d8460
	ctx.lr = 0x827D956C;
	sub_827D8460(ctx, base);
	// 827D956C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9570: 48000008  b 0x827d9578
	pc = 0x827D9578; continue 'dispatch;
	// 827D9574: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827D9578: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827D957C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827D9580: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827D9584: 4BFFF60D  bl 0x827d8b90
	ctx.lr = 0x827D9588;
	sub_827D8B90(ctx, base);
	// 827D9588: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827D958C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827D9590: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827D9594: 4BAE6A6D  bl 0x822c0000
	ctx.lr = 0x827D9598;
	sub_822C0000(ctx, base);
	// 827D9598: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827D959C: 3BFE0148  addi r31, r30, 0x148
	ctx.r[31].s64 = ctx.r[30].s64 + 328;
	// 827D95A0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827D95A4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827D95A8: 917E0148  stw r11, 0x148(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 827D95AC: 4BAEAEB5  bl 0x822c4460
	ctx.lr = 0x827D95B0;
	sub_822C4460(ctx, base);
	// 827D95B0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827D95B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D95B8: 419A0008  beq cr6, 0x827d95c0
	if ctx.cr[6].eq {
	pc = 0x827D95C0; continue 'dispatch;
	}
	// 827D95BC: 4BAE72D5  bl 0x822c0890
	ctx.lr = 0x827D95C0;
	sub_822C0890(ctx, base);
	// 827D95C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D95C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D95C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D95CC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827D95D0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 827D95D4: 419A0024  beq cr6, 0x827d95f8
	if ctx.cr[6].eq {
	pc = 0x827D95F8; continue 'dispatch;
	}
	// 827D95D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827D95DC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827D95E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D95E4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827D95E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827D95EC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827D95F0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D95F4: 4082FFE8  bne 0x827d95dc
	if !ctx.cr[0].eq {
	pc = 0x827D95DC; continue 'dispatch;
	}
	// 827D95F8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D95FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D9600: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827D9604: 409A0008  bne cr6, 0x827d960c
	if !ctx.cr[6].eq {
	pc = 0x827D960C; continue 'dispatch;
	}
	// 827D9608: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827D960C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827D9610: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827D9614: 808B70C4  lwz r4, 0x70c4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28868 as u32) ) } as u64;
	// 827D9618: 4BD50571  bl 0x82529b88
	ctx.lr = 0x827D961C;
	sub_82529B88(ctx, base);
	// 827D961C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827D9620: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D9624: 419A0008  beq cr6, 0x827d962c
	if ctx.cr[6].eq {
	pc = 0x827D962C; continue 'dispatch;
	}
	// 827D9628: 4BAE7269  bl 0x822c0890
	ctx.lr = 0x827D962C;
	sub_822C0890(ctx, base);
	// 827D962C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D9630: 489CEB8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D9638 size=192
    let mut pc: u32 = 0x827D9638;
    'dispatch: loop {
        match pc {
            0x827D9638 => {
    //   block [0x827D9638..0x827D96F8)
	// 827D9638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D963C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D9644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D964C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D9650: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D9654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827D9658: 388B4A48  addi r4, r11, 0x4a48
	ctx.r[4].s64 = ctx.r[11].s64 + 19016;
	// 827D965C: 38A0015A  li r5, 0x15a
	ctx.r[5].s64 = 346;
	// 827D9660: 386004F8  li r3, 0x4f8
	ctx.r[3].s64 = 1272;
	// 827D9664: 4BAE6D75  bl 0x822c03d8
	ctx.lr = 0x827D9668;
	sub_822C03D8(ctx, base);
	// 827D9668: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827D966C: 41820028  beq 0x827d9694
	if ctx.cr[0].eq {
	pc = 0x827D9694; continue 'dispatch;
	}
	// 827D9670: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 827D9674: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 827D9678: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827D967C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D9680: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827D9684: 394A01A8  addi r10, r10, 0x1a8
	ctx.r[10].s64 = ctx.r[10].s64 + 424;
	// 827D9688: 4080FFF0  bge 0x827d9678
	if !ctx.cr[0].lt {
	pc = 0x827D9678; continue 'dispatch;
	}
	// 827D968C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9690: 48000008  b 0x827d9698
	pc = 0x827D9698; continue 'dispatch;
	// 827D9694: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827D9698: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827D969C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827D96A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827D96A4: 4BFFD46D  bl 0x827d6b10
	ctx.lr = 0x827D96A8;
	sub_827D6B10(ctx, base);
	// 827D96A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827D96AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827D96B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827D96B4: 4BAE694D  bl 0x822c0000
	ctx.lr = 0x827D96B8;
	sub_822C0000(ctx, base);
	// 827D96B8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827D96BC: 397E0130  addi r11, r30, 0x130
	ctx.r[11].s64 = ctx.r[30].s64 + 304;
	// 827D96C0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827D96C4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 827D96C8: 915E0130  stw r10, 0x130(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(304 as u32), ctx.r[10].u32 ) };
	// 827D96CC: 4BAEAD95  bl 0x822c4460
	ctx.lr = 0x827D96D0;
	sub_822C4460(ctx, base);
	// 827D96D0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827D96D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D96D8: 419A0008  beq cr6, 0x827d96e0
	if ctx.cr[6].eq {
	pc = 0x827D96E0; continue 'dispatch;
	}
	// 827D96DC: 4BAE71B5  bl 0x822c0890
	ctx.lr = 0x827D96E0;
	sub_822C0890(ctx, base);
	// 827D96E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D96E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D96E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D96EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D96F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D96F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D96F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D96F8 size=296
    let mut pc: u32 = 0x827D96F8;
    'dispatch: loop {
        match pc {
            0x827D96F8 => {
    //   block [0x827D96F8..0x827D9820)
	// 827D96F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D96FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9700: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9704: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9708: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D970C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827D9710: 396B4A9C  addi r11, r11, 0x4a9c
	ctx.r[11].s64 = ctx.r[11].s64 + 19100;
	// 827D9714: 815F0148  lwz r10, 0x148(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 827D9718: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827D971C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827D9720: 419A00A4  beq cr6, 0x827d97c4
	if ctx.cr[6].eq {
	pc = 0x827D97C4; continue 'dispatch;
	}
	// 827D9724: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 827D9728: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827D972C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D9730: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 827D9734: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827D9738: 419A0024  beq cr6, 0x827d975c
	if ctx.cr[6].eq {
	pc = 0x827D975C; continue 'dispatch;
	}
	// 827D973C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827D9740: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827D9744: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D9748: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827D974C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827D9750: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827D9754: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827D9758: 4082FFE8  bne 0x827d9740
	if !ctx.cr[0].eq {
	pc = 0x827D9740; continue 'dispatch;
	}
	// 827D975C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D9760: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827D9764: 4BFFE5B5  bl 0x827d7d18
	ctx.lr = 0x827D9768;
	sub_827D7D18(ctx, base);
	// 827D9768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D976C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D9770: 388B0094  addi r4, r11, 0x94
	ctx.r[4].s64 = ctx.r[11].s64 + 148;
	// 827D9774: 409A0008  bne cr6, 0x827d977c
	if !ctx.cr[6].eq {
	pc = 0x827D977C; continue 'dispatch;
	}
	// 827D9778: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827D977C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827D9780: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827D9784: 48618475  bl 0x82df1bf8
	ctx.lr = 0x827D9788;
	sub_82DF1BF8(ctx, base);
	// 827D9788: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827D978C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827D9790: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827D9794: 409A0008  bne cr6, 0x827d979c
	if !ctx.cr[6].eq {
	pc = 0x827D979C; continue 'dispatch;
	}
	// 827D9798: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827D979C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827D97A0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827D97A4: 808B70C4  lwz r4, 0x70c4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28868 as u32) ) } as u64;
	// 827D97A8: 4BD4F469  bl 0x82528c10
	ctx.lr = 0x827D97AC;
	sub_82528C10(ctx, base);
	// 827D97AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827D97B0: 486184E1  bl 0x82df1c90
	ctx.lr = 0x827D97B4;
	sub_82DF1C90(ctx, base);
	// 827D97B4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827D97B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D97BC: 419A0008  beq cr6, 0x827d97c4
	if ctx.cr[6].eq {
	pc = 0x827D97C4; continue 'dispatch;
	}
	// 827D97C0: 4BAE70D1  bl 0x822c0890
	ctx.lr = 0x827D97C4;
	sub_822C0890(ctx, base);
	// 827D97C4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 827D97C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D97CC: 419A0008  beq cr6, 0x827d97d4
	if ctx.cr[6].eq {
	pc = 0x827D97D4; continue 'dispatch;
	}
	// 827D97D0: 4BAE70C1  bl 0x822c0890
	ctx.lr = 0x827D97D4;
	sub_822C0890(ctx, base);
	// 827D97D4: 387F013C  addi r3, r31, 0x13c
	ctx.r[3].s64 = ctx.r[31].s64 + 316;
	// 827D97D8: 4BC4EEA9  bl 0x82428680
	ctx.lr = 0x827D97DC;
	sub_82428680(ctx, base);
	// 827D97DC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827D97E0: 809F0140  lwz r4, 0x140(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 827D97E4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827D97E8: 486189A1  bl 0x82df2188
	ctx.lr = 0x827D97EC;
	sub_82DF2188(ctx, base);
	// 827D97EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827D97F0: 917F0140  stw r11, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 827D97F4: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 827D97F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827D97FC: 419A0008  beq cr6, 0x827d9804
	if ctx.cr[6].eq {
	pc = 0x827D9804; continue 'dispatch;
	}
	// 827D9800: 4BAE7091  bl 0x822c0890
	ctx.lr = 0x827D9804;
	sub_822C0890(ctx, base);
	// 827D9804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D9808: 480014D9  bl 0x827dace0
	ctx.lr = 0x827D980C;
	sub_827DACE0(ctx, base);
	// 827D980C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D9810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D9814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D981C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D9820 size=76
    let mut pc: u32 = 0x827D9820;
    'dispatch: loop {
        match pc {
            0x827D9820 => {
    //   block [0x827D9820..0x827D986C)
	// 827D9820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D982C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9838: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D983C: 4BFFFEBD  bl 0x827d96f8
	ctx.lr = 0x827D9840;
	sub_827D96F8(ctx, base);
	// 827D9840: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827D9844: 4182000C  beq 0x827d9850
	if ctx.cr[0].eq {
	pc = 0x827D9850; continue 'dispatch;
	}
	// 827D9848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D984C: 4BAE6A1D  bl 0x822c0268
	ctx.lr = 0x827D9850;
	sub_822C0268(ctx, base);
	// 827D9850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D9854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D9858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D985C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9860: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D9864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D9868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D9870 size=160
    let mut pc: u32 = 0x827D9870;
    'dispatch: loop {
        match pc {
            0x827D9870 => {
    //   block [0x827D9870..0x827D9910)
	// 827D9870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9878: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D987C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9884: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9888: 480016C9  bl 0x827daf50
	ctx.lr = 0x827D988C;
	sub_827DAF50(ctx, base);
	// 827D988C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827D9890: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827D9894: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 827D9898: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827D989C: 394A4A9C  addi r10, r10, 0x4a9c
	ctx.r[10].s64 = ctx.r[10].s64 + 19100;
	// 827D98A0: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D98A4: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 827D98A8: D01F00F0  stfs f0, 0xf0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 827D98AC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827D98B0: D01F00F4  stfs f0, 0xf4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 827D98B4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 827D98B8: D01F0110  stfs f0, 0x110(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 827D98BC: 93DF0130  stw r30, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[30].u32 ) };
	// 827D98C0: C1A9D2B0  lfs f13, -0x2d50(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-11600 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827D98C4: 93DF0134  stw r30, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[30].u32 ) };
	// 827D98C8: D01F0120  stfs f0, 0x120(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 827D98CC: 387F013C  addi r3, r31, 0x13c
	ctx.r[3].s64 = ctx.r[31].s64 + 316;
	// 827D98D0: D1BF0124  stfs f13, 0x124(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 827D98D4: D01F0128  stfs f0, 0x128(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 827D98D8: D01F012C  stfs f0, 0x12c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 827D98DC: 917F0138  stw r11, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 827D98E0: 4BD35E31  bl 0x8250f710
	ctx.lr = 0x827D98E4;
	sub_8250F710(ctx, base);
	// 827D98E4: 907F0140  stw r3, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[3].u32 ) };
	// 827D98E8: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 827D98EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D98F0: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 827D98F4: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 827D98F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D98FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D9900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9904: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D9908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D990C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D9910 size=156
    let mut pc: u32 = 0x827D9910;
    'dispatch: loop {
        match pc {
            0x827D9910 => {
    //   block [0x827D9910..0x827D99AC)
	// 827D9910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D991C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9920: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9928: 3D600666  lis r11, 0x666
	ctx.r[11].s64 = 107347968;
	// 827D992C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D9930: 616B6666  ori r11, r11, 0x6666
	ctx.r[11].u64 = ctx.r[11].u64 | 26214;
	// 827D9934: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D9938: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827D993C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827D9940: 40980048  bge cr6, 0x827d9988
	if !ctx.cr[6].lt {
	pc = 0x827D9988; continue 'dispatch;
	}
	// 827D9944: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827D9948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D994C: 388B960C  addi r4, r11, -0x69f4
	ctx.r[4].s64 = ctx.r[11].s64 + -27124;
	// 827D9950: 4BAEBF79  bl 0x822c58c8
	ctx.lr = 0x827D9954;
	sub_822C58C8(ctx, base);
	// 827D9954: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827D9958: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827D995C: 4BAEBEBD  bl 0x822c5818
	ctx.lr = 0x827D9960;
	sub_822C5818(ctx, base);
	// 827D9960: 4BAEA951  bl 0x822c42b0
	ctx.lr = 0x827D9964;
	sub_822C42B0(ctx, base);
	// 827D9964: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827D9968: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827D996C: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 827D9970: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827D9974: 4BAEBAFD  bl 0x822c5470
	ctx.lr = 0x827D9978;
	sub_822C5470(ctx, base);
	// 827D9978: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827D997C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827D9980: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D9984: 4BAEB35D  bl 0x822c4ce0
	ctx.lr = 0x827D9988;
	sub_822C4CE0(ctx, base);
	// 827D9988: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D998C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827D9990: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827D9994: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827D9998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D999C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D99A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D99A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D99A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D99B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D99B0 size=156
    let mut pc: u32 = 0x827D99B0;
    'dispatch: loop {
        match pc {
            0x827D99B0 => {
    //   block [0x827D99B0..0x827D9A4C)
	// 827D99B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D99B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D99B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D99BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D99C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D99C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D99C8: 3D60071C  lis r11, 0x71c
	ctx.r[11].s64 = 119275520;
	// 827D99CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D99D0: 616B71C7  ori r11, r11, 0x71c7
	ctx.r[11].u64 = ctx.r[11].u64 | 29127;
	// 827D99D4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D99D8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827D99DC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827D99E0: 40980048  bge cr6, 0x827d9a28
	if !ctx.cr[6].lt {
	pc = 0x827D9A28; continue 'dispatch;
	}
	// 827D99E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827D99E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D99EC: 388B960C  addi r4, r11, -0x69f4
	ctx.r[4].s64 = ctx.r[11].s64 + -27124;
	// 827D99F0: 4BAEBED9  bl 0x822c58c8
	ctx.lr = 0x827D99F4;
	sub_822C58C8(ctx, base);
	// 827D99F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827D99F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827D99FC: 4BAEBE1D  bl 0x822c5818
	ctx.lr = 0x827D9A00;
	sub_822C5818(ctx, base);
	// 827D9A00: 4BAEA8B1  bl 0x822c42b0
	ctx.lr = 0x827D9A04;
	sub_822C42B0(ctx, base);
	// 827D9A04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827D9A08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827D9A0C: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 827D9A10: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827D9A14: 4BAEBA5D  bl 0x822c5470
	ctx.lr = 0x827D9A18;
	sub_822C5470(ctx, base);
	// 827D9A18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827D9A1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827D9A20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D9A24: 4BAEB2BD  bl 0x822c4ce0
	ctx.lr = 0x827D9A28;
	sub_822C4CE0(ctx, base);
	// 827D9A28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D9A2C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827D9A30: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827D9A34: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827D9A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D9A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9A40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D9A44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D9A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D9A50 size=740
    let mut pc: u32 = 0x827D9A50;
    'dispatch: loop {
        match pc {
            0x827D9A50 => {
    //   block [0x827D9A50..0x827D9D34)
	// 827D9A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9A54: 489CE709  bl 0x831a815c
	ctx.lr = 0x827D9A58;
	sub_831A8130(ctx, base);
	// 827D9A58: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 827D9A5C: 489CF019  bl 0x831a8a74
	ctx.lr = 0x827D9A60;
	sub_831A8A40(ctx, base);
	// 827D9A60: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9A64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827D9A68: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 827D9A6C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D9A70: 4BB2C001  bl 0x82305a70
	ctx.lr = 0x827D9A74;
	sub_82305A70(ctx, base);
	// 827D9A74: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827D9A78: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D9A7C: 4BB4006D  bl 0x82319ae8
	ctx.lr = 0x827D9A80;
	sub_82319AE8(ctx, base);
	// 827D9A80: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 827D9A84: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 827D9A88: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827D9A8C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D9D38 size=96
    let mut pc: u32 = 0x827D9D38;
    'dispatch: loop {
        match pc {
            0x827D9D38 => {
    //   block [0x827D9D38..0x827D9D98)
	// 827D9D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9D44: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827D9D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9D50: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827D9D54: C01F0038  lfs f0, 0x38(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D9D58: C1BF0034  lfs f13, 0x34(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827D9D5C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827D9D60: 40990014  ble cr6, 0x827d9d74
	if !ctx.cr[6].gt {
	pc = 0x827D9D74; continue 'dispatch;
	}
	// 827D9D64: 4BFFFCED  bl 0x827d9a50
	ctx.lr = 0x827D9D68;
	sub_827D9A50(ctx, base);
	// 827D9D68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827D9D6C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D9D70: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827D9D74: C01F0034  lfs f0, 0x34(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D9D78: EC1F002A  fadds f0, f31, f0
	ctx.f[0].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 827D9D7C: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827D9D80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D9D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D9D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9D8C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D9D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D9D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D9D98 size=100
    let mut pc: u32 = 0x827D9D98;
    'dispatch: loop {
        match pc {
            0x827D9D98 => {
    //   block [0x827D9D98..0x827D9DFC)
	// 827D9D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9D9C: 489CE3C9  bl 0x831a8164
	ctx.lr = 0x827D9DA0;
	sub_831A8130(ctx, base);
	// 827D9DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9DA4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827D9DA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827D9DAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D9DB0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827D9DB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827D9DB8: 419A003C  beq cr6, 0x827d9df4
	if ctx.cr[6].eq {
	pc = 0x827D9DF4; continue 'dispatch;
	}
	// 827D9DBC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 827D9DC0: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D9DC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D9DC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827D9DCC: 4BFFEC8D  bl 0x827d8a58
	ctx.lr = 0x827D9DD0;
	sub_827D8A58(ctx, base);
	// 827D9DD0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 827D9DD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827D9DD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827D9DDC: 4BFFFBD5  bl 0x827d99b0
	ctx.lr = 0x827D9DE0;
	sub_827D99B0(ctx, base);
	// 827D9DE0: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 827D9DE4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D9DE8: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827D9DEC: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 827D9DF0: 4082FFCC  bne 0x827d9dbc
	if !ctx.cr[0].eq {
	pc = 0x827D9DBC; continue 'dispatch;
	}
	// 827D9DF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D9DF8: 489CE3BC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D9E00 size=140
    let mut pc: u32 = 0x827D9E00;
    'dispatch: loop {
        match pc {
            0x827D9E00 => {
    //   block [0x827D9E00..0x827D9E8C)
	// 827D9E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9E04: 489CE369  bl 0x831a816c
	ctx.lr = 0x827D9E08;
	sub_831A8130(ctx, base);
	// 827D9E08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9E0C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827D9E10: 81630140  lwz r11, 0x140(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(320 as u32) ) } as u64;
	// 827D9E14: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827D9E18: 3BE3013C  addi r31, r3, 0x13c
	ctx.r[31].s64 = ctx.r[3].s64 + 316;
	// 827D9E1C: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827D9E20: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 827D9E24: C1650000  lfs f11, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827D9E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D9E2C: C1450004  lfs f10, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 827D9E30: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 827D9E34: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827D9E38: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827D9E3C: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827D9E40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827D9E44: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827D9E48: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827D9E4C: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827D9E50: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D9E54: D1810064  stfs f12, 0x64(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827D9E58: D1610068  stfs f11, 0x68(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827D9E5C: D141006C  stfs f10, 0x6c(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 827D9E60: D1210070  stfs f9, 0x70(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 827D9E64: 4BFFEB85  bl 0x827d89e8
	ctx.lr = 0x827D9E68;
	sub_827D89E8(ctx, base);
	// 827D9E68: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827D9E6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827D9E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D9E74: 4BFFFA9D  bl 0x827d9910
	ctx.lr = 0x827D9E78;
	sub_827D9910(ctx, base);
	// 827D9E78: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 827D9E7C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D9E80: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827D9E84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827D9E88: 489CE334  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D9E90 size=140
    let mut pc: u32 = 0x827D9E90;
    'dispatch: loop {
        match pc {
            0x827D9E90 => {
    //   block [0x827D9E90..0x827D9F1C)
	// 827D9E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9E98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827D9E9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9EA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9EA8: F8A10090  std r5, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[5].u64 ) };
	// 827D9EAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827D9EB0: F8C10098  std r6, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[6].u64 ) };
	// 827D9EB4: F8E100A0  std r7, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[7].u64 ) };
	// 827D9EB8: F90100A8  std r8, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[8].u64 ) };
	// 827D9EBC: F92100B0  std r9, 0xb0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[9].u64 ) };
	// 827D9EC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D9EC4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D9EC8: 40990018  ble cr6, 0x827d9ee0
	if !ctx.cr[6].gt {
	pc = 0x827D9EE0; continue 'dispatch;
	}
	// 827D9ECC: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 827D9ED0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D9ED4: 7CABF050  subf r5, r11, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 827D9ED8: 4BFFFEC1  bl 0x827d9d98
	ctx.lr = 0x827D9EDC;
	sub_827D9D98(ctx, base);
	// 827D9EDC: 48000028  b 0x827d9f04
	pc = 0x827D9F04; continue 'dispatch;
	// 827D9EE0: 40980024  bge cr6, 0x827d9f04
	if !ctx.cr[6].lt {
	pc = 0x827D9F04; continue 'dispatch;
	}
	// 827D9EE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D9EE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827D9EEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827D9EF0: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827D9EF4: 4BB1143D  bl 0x822eb330
	ctx.lr = 0x827D9EF8;
	sub_822EB330(ctx, base);
	// 827D9EF8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827D9EFC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827D9F00: 4198FFE4  blt cr6, 0x827d9ee4
	if ctx.cr[6].lt {
	pc = 0x827D9EE4; continue 'dispatch;
	}
	// 827D9F04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827D9F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D9F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9F10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827D9F14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D9F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827D9F20 size=68
    let mut pc: u32 = 0x827D9F20;
    'dispatch: loop {
        match pc {
            0x827D9F20 => {
    //   block [0x827D9F20..0x827D9F64)
	// 827D9F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9F2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827D9F30: 81410070  lwz r10, 0x70(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 827D9F34: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827D9F38: 386B00B0  addi r3, r11, 0xb0
	ctx.r[3].s64 = ctx.r[11].s64 + 176;
	// 827D9F3C: E8C10058  ld r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 827D9F40: 794907E6  rldicr r9, r10, 0x20, 0x3f
	ctx.r[9].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 827D9F44: E8E10060  ld r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 827D9F48: E9010068  ld r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 827D9F4C: 808B00AC  lwz r4, 0xac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 827D9F50: 4BFFFF41  bl 0x827d9e90
	ctx.lr = 0x827D9F54;
	sub_827D9E90(ctx, base);
	// 827D9F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827D9F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D9F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827D9F68 size=72
    let mut pc: u32 = 0x827D9F68;
    'dispatch: loop {
        match pc {
            0x827D9F68 => {
    //   block [0x827D9F68..0x827D9FB0)
	// 827D9F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827D9F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827D9F70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827D9F74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827D9F78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827D9F7C: 48001145  bl 0x827db0c0
	ctx.lr = 0x827D9F80;
	sub_827DB0C0(ctx, base);
	// 827D9F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D9F84: 4BFFF6B5  bl 0x827d9638
	ctx.lr = 0x827D9F88;
	sub_827D9638(ctx, base);
	// 827D9F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D9F8C: 4BFFFF95  bl 0x827d9f20
	ctx.lr = 0x827D9F90;
	sub_827D9F20(ctx, base);
	// 827D9F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827D9F94: 4800002D  bl 0x827d9fc0
	ctx.lr = 0x827D9F98;
	sub_827D9FC0(ctx, base);
	// 827D9F98: D03F0110  stfs f1, 0x110(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 827D9F9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827D9FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827D9FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827D9FA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827D9FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D9FB0 size=8
    let mut pc: u32 = 0x827D9FB0;
    'dispatch: loop {
        match pc {
            0x827D9FB0 => {
    //   block [0x827D9FB0..0x827D9FB8)
	// 827D9FB0: 88630040  lbz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 827D9FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D9FB8 size=8
    let mut pc: u32 = 0x827D9FB8;
    'dispatch: loop {
        match pc {
            0x827D9FB8 => {
    //   block [0x827D9FB8..0x827D9FC0)
	// 827D9FB8: 908300AC  stw r4, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 827D9FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D9FC0 size=8
    let mut pc: u32 = 0x827D9FC0;
    'dispatch: loop {
        match pc {
            0x827D9FC0 => {
    //   block [0x827D9FC0..0x827D9FC8)
	// 827D9FC0: C0230034  lfs f1, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827D9FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D9FC8 size=8
    let mut pc: u32 = 0x827D9FC8;
    'dispatch: loop {
        match pc {
            0x827D9FC8 => {
    //   block [0x827D9FC8..0x827D9FD0)
	// 827D9FC8: D0230038  stfs f1, 0x38(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 827D9FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D9FD0 size=8
    let mut pc: u32 = 0x827D9FD0;
    'dispatch: loop {
        match pc {
            0x827D9FD0 => {
    //   block [0x827D9FD0..0x827D9FD8)
	// 827D9FD0: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 827D9FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D9FD8 size=8
    let mut pc: u32 = 0x827D9FD8;
    'dispatch: loop {
        match pc {
            0x827D9FD8 => {
    //   block [0x827D9FD8..0x827D9FE0)
	// 827D9FD8: 9083003C  stw r4, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 827D9FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D9FE0 size=8
    let mut pc: u32 = 0x827D9FE0;
    'dispatch: loop {
        match pc {
            0x827D9FE0 => {
    //   block [0x827D9FE0..0x827D9FE8)
	// 827D9FE0: D023002C  stfs f1, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 827D9FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D9FE8 size=8
    let mut pc: u32 = 0x827D9FE8;
    'dispatch: loop {
        match pc {
            0x827D9FE8 => {
    //   block [0x827D9FE8..0x827D9FF0)
	// 827D9FE8: D0230030  stfs f1, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 827D9FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827D9FF0 size=8
    let mut pc: u32 = 0x827D9FF0;
    'dispatch: loop {
        match pc {
            0x827D9FF0 => {
    //   block [0x827D9FF0..0x827D9FF8)
	// 827D9FF0: D0230034  stfs f1, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827D9FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827D9FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827D9FF8 size=8
    let mut pc: u32 = 0x827D9FF8;
    'dispatch: loop {
        match pc {
            0x827D9FF8 => {
    //   block [0x827D9FF8..0x827DA000)
	// 827D9FF8: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 827D9FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DA000 size=144
    let mut pc: u32 = 0x827DA000;
    'dispatch: loop {
        match pc {
            0x827DA000 => {
    //   block [0x827DA000..0x827DA090)
	// 827DA000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DA004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DA008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DA00C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827DA010: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DA014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DA018: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827DA01C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DA020: C01F002C  lfs f0, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DA024: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DA028: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 827DA02C: 40990010  ble cr6, 0x827da03c
	if !ctx.cr[6].gt {
	pc = 0x827DA03C; continue 'dispatch;
	}
	// 827DA030: EC00F828  fsubs f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 827DA034: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 827DA038: 48000040  b 0x827da078
	pc = 0x827DA078; continue 'dispatch;
	// 827DA03C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA040: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827DA044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA048: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DA04C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DA050: 4E800421  bctrl
	ctx.lr = 0x827DA054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DA054: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA058: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827DA05C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA060: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DA064: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DA068: 4E800421  bctrl
	ctx.lr = 0x827DA06C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DA06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA070: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827DA074: 4BFFBA25  bl 0x827d5a98
	ctx.lr = 0x827DA078;
	sub_827D5A98(ctx, base);
	// 827DA078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DA07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DA080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DA084: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DA088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DA08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DA090 size=88
    let mut pc: u32 = 0x827DA090;
    'dispatch: loop {
        match pc {
            0x827DA090 => {
    //   block [0x827DA090..0x827DA0E8)
	// 827DA090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DA094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DA098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DA09C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DA0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DA0A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DA0A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DA0AC: 48637E2D  bl 0x82e11ed8
	ctx.lr = 0x827DA0B0;
	sub_82E11ED8(ctx, base);
	// 827DA0B0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DA0B4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 827DA0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DA0BC: 394A4ADC  addi r10, r10, 0x4adc
	ctx.r[10].s64 = ctx.r[10].s64 + 19164;
	// 827DA0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA0C4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DA0C8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827DA0CC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827DA0D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DA0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DA0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DA0DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DA0E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DA0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DA0E8 size=8
    let mut pc: u32 = 0x827DA0E8;
    'dispatch: loop {
        match pc {
            0x827DA0E8 => {
    //   block [0x827DA0E8..0x827DA0F0)
	// 827DA0E8: 806300B8  lwz r3, 0xb8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 827DA0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DA0F0 size=16
    let mut pc: u32 = 0x827DA0F0;
    'dispatch: loop {
        match pc {
            0x827DA0F0 => {
    //   block [0x827DA0F0..0x827DA100)
	// 827DA0F0: 816300B8  lwz r11, 0xb8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 827DA0F4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DA0F8: 386BFFFE  addi r3, r11, -2
	ctx.r[3].s64 = ctx.r[11].s64 + -2;
	// 827DA0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DA100 size=8
    let mut pc: u32 = 0x827DA100;
    'dispatch: loop {
        match pc {
            0x827DA100 => {
    //   block [0x827DA100..0x827DA108)
	// 827DA100: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 827DA104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DA108 size=8
    let mut pc: u32 = 0x827DA108;
    'dispatch: loop {
        match pc {
            0x827DA108 => {
    //   block [0x827DA108..0x827DA110)
	// 827DA108: 806300A0  lwz r3, 0xa0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 827DA10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DA110 size=968
    let mut pc: u32 = 0x827DA110;
    'dispatch: loop {
        match pc {
            0x827DA110 => {
    //   block [0x827DA110..0x827DA4D8)
	// 827DA110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DA114: 489CE03D  bl 0x831a8150
	ctx.lr = 0x827DA118;
	sub_831A8130(ctx, base);
	// 827DA118: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DA11C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DA120: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 827DA124: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DA128: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DA12C: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 827DA130: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 827DA134: 4198039C  blt cr6, 0x827da4d0
	if ctx.cr[6].lt {
	pc = 0x827DA4D0; continue 'dispatch;
	}
	// 827DA138: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DA13C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA140: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DA144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DA148: 4E800421  bctrl
	ctx.lr = 0x827DA14C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DA14C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DA150: 41820380  beq 0x827da4d0
	if ctx.cr[0].eq {
	pc = 0x827DA4D0; continue 'dispatch;
	}
	// 827DA154: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DA158: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DA15C: 3BEB87B8  addi r31, r11, -0x7848
	ctx.r[31].s64 = ctx.r[11].s64 + -30792;
	// 827DA160: 816A87BC  lwz r11, -0x7844(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30788 as u32) ) } as u64;
	// 827DA164: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827DA168: 4082001C  bne 0x827da184
	if !ctx.cr[0].eq {
	pc = 0x827DA184; continue 'dispatch;
	}
	// 827DA16C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827DA170: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 827DA174: 916A87BC  stw r11, -0x7844(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30788 as u32), ctx.r[11].u32 ) };
	// 827DA178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA17C: 38899430  addi r4, r9, -0x6bd0
	ctx.r[4].s64 = ctx.r[9].s64 + -27600;
	// 827DA180: 4861F3B1  bl 0x82df9530
	ctx.lr = 0x827DA184;
	sub_82DF9530(ctx, base);
	// 827DA184: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA188: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DA18C: 409A0344  bne cr6, 0x827da4d0
	if !ctx.cr[6].eq {
	pc = 0x827DA4D0; continue 'dispatch;
	}
	// 827DA190: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA194: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 827DA198: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DA19C: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 827DA1A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DA1A4: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA1A8: 409A00C4  bne cr6, 0x827da26c
	if !ctx.cr[6].eq {
	pc = 0x827DA26C; continue 'dispatch;
	}
	// 827DA1AC: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 827DA1B0: B3610076  sth r27, 0x76(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(118 as u32), ctx.r[27].u16 ) };
	// 827DA1B4: 3D40002A  lis r10, 0x2a
	ctx.r[10].s64 = 2752512;
	// 827DA1B8: 3D20002C  lis r9, 0x2c
	ctx.r[9].s64 = 2883584;
	// 827DA1BC: B0E1005E  sth r7, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[7].u16 ) };
	// 827DA1C0: 3D000018  lis r8, 0x18
	ctx.r[8].s64 = 1572864;
	// 827DA1C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DA1C8: 614A23B9  ori r10, r10, 0x23b9
	ctx.r[10].u64 = ctx.r[10].u64 | 9145;
	// 827DA1CC: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 827DA1D0: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 827DA1D4: 612923A5  ori r9, r9, 0x23a5
	ctx.r[9].u64 = ctx.r[9].u64 | 9125;
	// 827DA1D8: B1610052  sth r11, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 827DA1DC: 61082886  ori r8, r8, 0x2886
	ctx.r[8].u64 = ctx.r[8].u64 | 10374;
	// 827DA1E0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 827DA1E4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 827DA1E8: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 827DA1EC: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 827DA1F0: 99610059  stb r11, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 827DA1F4: 3B000005  li r24, 5
	ctx.r[24].s64 = 5;
	// 827DA1F8: 9961005A  stb r11, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[11].u8 ) };
	// 827DA1FC: 3AE0000A  li r23, 0xa
	ctx.r[23].s64 = 10;
	// 827DA200: B161005C  sth r11, 0x5c(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u16 ) };
	// 827DA204: 3AC000FF  li r22, 0xff
	ctx.r[22].s64 = 255;
	// 827DA208: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 827DA20C: 99610064  stb r11, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u8 ) };
	// 827DA210: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827DA214: 98C10065  stb r6, 0x65(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(101 as u32), ctx.r[6].u8 ) };
	// 827DA218: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DA21C: 99610066  stb r11, 0x66(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[11].u8 ) };
	// 827DA220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA224: B1610068  sth r11, 0x68(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u16 ) };
	// 827DA228: B321006A  sth r25, 0x6a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(106 as u32), ctx.r[25].u16 ) };
	// 827DA22C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 827DA230: 99610070  stb r11, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 827DA234: 9B010071  stb r24, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[24].u8 ) };
	// 827DA238: 99610072  stb r11, 0x72(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(114 as u32), ctx.r[11].u8 ) };
	// 827DA23C: B1610074  sth r11, 0x74(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u16 ) };
	// 827DA240: 91010078  stw r8, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u32 ) };
	// 827DA244: 9961007C  stb r11, 0x7c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u8 ) };
	// 827DA248: 9AE1007D  stb r23, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[23].u8 ) };
	// 827DA24C: 9961007E  stb r11, 0x7e(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[11].u8 ) };
	// 827DA250: B2C10080  sth r22, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[22].u16 ) };
	// 827DA254: B1610082  sth r11, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[11].u16 ) };
	// 827DA258: 90E10084  stw r7, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[7].u32 ) };
	// 827DA25C: 99610088  stb r11, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 827DA260: 99610089  stb r11, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[11].u8 ) };
	// 827DA264: 9961008A  stb r11, 0x8a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(138 as u32), ctx.r[11].u8 ) };
	// 827DA268: 48634F01  bl 0x82e0f168
	ctx.lr = 0x827DA26C;
	sub_82E0F168(ctx, base);
	// 827DA26C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA270: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DA274: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 827DA278: 419A0010  beq cr6, 0x827da288
	if ctx.cr[6].eq {
	pc = 0x827DA288; continue 'dispatch;
	}
	// 827DA27C: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 827DA280: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA284: 483FE64D  bl 0x82bd88d0
	ctx.lr = 0x827DA288;
	sub_82BD88D0(ctx, base);
	// 827DA288: 897F0026  lbz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 827DA28C: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 827DA290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA294: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DA298: 4082000C  bne 0x827da2a4
	if !ctx.cr[0].eq {
	pc = 0x827DA2A4; continue 'dispatch;
	}
	// 827DA29C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 827DA2A0: 48000028  b 0x827da2c8
	pc = 0x827DA2C8; continue 'dispatch;
	// 827DA2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DA2A8: 4BB0B041  bl 0x822e52e8
	ctx.lr = 0x827DA2AC;
	sub_822E52E8(ctx, base);
	// 827DA2AC: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 827DA2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA2B4: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DA2B8: 4BB0B031  bl 0x822e52e8
	ctx.lr = 0x827DA2BC;
	sub_822E52E8(ctx, base);
	// 827DA2BC: 80BF0034  lwz r5, 0x34(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DA2C0: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 827DA2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA2C8: 4BB0B021  bl 0x822e52e8
	ctx.lr = 0x827DA2CC;
	sub_822E52E8(ctx, base);
	// 827DA2CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DA2D0: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 827DA2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA2D8: 4BB0B011  bl 0x822e52e8
	ctx.lr = 0x827DA2DC;
	sub_822E52E8(ctx, base);
	// 827DA2DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827DA2E0: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 827DA2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA2E8: 4BB0B001  bl 0x822e52e8
	ctx.lr = 0x827DA2EC;
	sub_822E52E8(ctx, base);
	// 827DA2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DA2F0: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 827DA2F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA2F8: 4BB0AFF1  bl 0x822e52e8
	ctx.lr = 0x827DA2FC;
	sub_822E52E8(ctx, base);
	// 827DA2FC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DA300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA304: 388B0074  addi r4, r11, 0x74
	ctx.r[4].s64 = ctx.r[11].s64 + 116;
	// 827DA308: 3BAB007C  addi r29, r11, 0x7c
	ctx.r[29].s64 = ctx.r[11].s64 + 124;
	// 827DA30C: 4863481D  bl 0x82e0eb28
	ctx.lr = 0x827DA310;
	sub_82E0EB28(ctx, base);
	// 827DA310: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DA314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA318: 48634909  bl 0x82e0ec20
	ctx.lr = 0x827DA31C;
	sub_82E0EC20(ctx, base);
	// 827DA31C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DA320: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 827DA324: C06B08A4  lfs f3, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 827DA328: FC401890  fmr f2, f3
	ctx.f[2].f64 = ctx.f[3].f64;
	// 827DA32C: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 827DA330: 486A1CE9  bl 0x82e7c018
	ctx.lr = 0x827DA334;
	sub_82E7C018(ctx, base);
	// 827DA334: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 827DA338: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DA33C: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 827DA340: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 827DA344: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 827DA348: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 827DA34C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827DA350: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827DA354: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 827DA358: 3BA100E0  addi r29, r1, 0xe0
	ctx.r[29].s64 = ctx.r[1].s64 + 224;
	// 827DA35C: 13C95407  vcmpneb. (lvlx128) v30, v9, v10
	tmp.u32 = ctx.r[9].u32 + ctx.r[10].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827DA360: 3B2100F0  addi r25, r1, 0xf0
	ctx.r[25].s64 = ctx.r[1].s64 + 240;
	// 827DA364: 13BB4407  vcmpneb. (lvlx128) v29, v27, v8
	tmp.u32 = ctx.r[27].u32 + ctx.r[8].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827DA368: 3B010100  addi r24, r1, 0x100
	ctx.r[24].s64 = ctx.r[1].s64 + 256;
	// 827DA36C: 13863C07  vcmpneb. (lvlx128) v28, v6, v7
	tmp.u32 = ctx.r[6].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827DA370: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DA4D8 size=80
    let mut pc: u32 = 0x827DA4D8;
    'dispatch: loop {
        match pc {
            0x827DA4D8 => {
    //   block [0x827DA4D8..0x827DA528)
	// 827DA4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DA4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DA4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DA4E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827DA4E8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827DA4EC: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 827DA4F0: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 827DA4F4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 827DA4F8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827DA4FC: 48617BCD  bl 0x82df20c8
	ctx.lr = 0x827DA500;
	sub_82DF20C8(ctx, base);
	// 827DA500: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DA504: 41820008  beq 0x827da50c
	if ctx.cr[0].eq {
	pc = 0x827DA50C; continue 'dispatch;
	}
	// 827DA508: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 827DA50C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DA510: 41820008  beq 0x827da518
	if ctx.cr[0].eq {
	pc = 0x827DA518; continue 'dispatch;
	}
	// 827DA514: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 827DA518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DA51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DA520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DA524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DA528 size=196
    let mut pc: u32 = 0x827DA528;
    'dispatch: loop {
        match pc {
            0x827DA528 => {
    //   block [0x827DA528..0x827DA5EC)
	// 827DA528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DA52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DA530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DA534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DA538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DA53C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DA540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DA544: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827DA548: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DA54C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DA550: 4BAE63E9  bl 0x822c0938
	ctx.lr = 0x827DA554;
	sub_822C0938(ctx, base);
	// 827DA554: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DA558: 41820028  beq 0x827da580
	if ctx.cr[0].eq {
	pc = 0x827DA580; continue 'dispatch;
	}
	// 827DA55C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DA560: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827DA564: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DA568: 392B4AEC  addi r9, r11, 0x4aec
	ctx.r[9].s64 = ctx.r[11].s64 + 19180;
	// 827DA56C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DA570: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DA574: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DA578: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DA57C: 48000008  b 0x827da584
	pc = 0x827DA584; continue 'dispatch;
	// 827DA580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DA584: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DA588: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DA58C: 409A0044  bne cr6, 0x827da5d0
	if !ctx.cr[6].eq {
	pc = 0x827DA5D0; continue 'dispatch;
	}
	// 827DA590: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DA594: 419A001C  beq cr6, 0x827da5b0
	if ctx.cr[6].eq {
	pc = 0x827DA5B0; continue 'dispatch;
	}
	// 827DA598: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA59C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DA5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DA5A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA5A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DA5AC: 4E800421  bctrl
	ctx.lr = 0x827DA5B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DA5B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DA5B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827DA5B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DA5BC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827DA5C0: 816BB710  lwz r11, -0x48f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18672 as u32) ) } as u64;
	// 827DA5C4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827DA5C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827DA5CC: 4BAE5A35  bl 0x822c0000
	ctx.lr = 0x827DA5D0;
	sub_822C0000(ctx, base);
	// 827DA5D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DA5D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DA5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DA5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DA5E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DA5E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DA5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DA5F0 size=244
    let mut pc: u32 = 0x827DA5F0;
    'dispatch: loop {
        match pc {
            0x827DA5F0 => {
    //   block [0x827DA5F0..0x827DA6E4)
	// 827DA5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DA5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DA5F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DA5FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DA600: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DA604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DA608: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA60C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827DA610: 486504D9  bl 0x82e2aae8
	ctx.lr = 0x827DA614;
	sub_82E2AAE8(ctx, base);
	// 827DA614: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DA618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DA61C: 3BCB7C24  addi r30, r11, 0x7c24
	ctx.r[30].s64 = ctx.r[11].s64 + 31780;
	// 827DA620: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DA624: 486193E5  bl 0x82df3a08
	ctx.lr = 0x827DA628;
	sub_82DF3A08(ctx, base);
	// 827DA628: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827DA62C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DA630: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827DA634: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DA638: 48654329  bl 0x82e2e960
	ctx.lr = 0x827DA63C;
	sub_82E2E960(ctx, base);
	// 827DA63C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DA640: 395F0074  addi r10, r31, 0x74
	ctx.r[10].s64 = ctx.r[31].s64 + 116;
	// 827DA644: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827DA648: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827DA64C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA650: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827DA654: 4BAE9E0D  bl 0x822c4460
	ctx.lr = 0x827DA658;
	sub_822C4460(ctx, base);
	// 827DA658: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827DA65C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DA660: 419A0008  beq cr6, 0x827da668
	if ctx.cr[6].eq {
	pc = 0x827DA668; continue 'dispatch;
	}
	// 827DA664: 4BAE622D  bl 0x822c0890
	ctx.lr = 0x827DA668;
	sub_822C0890(ctx, base);
	// 827DA668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DA66C: 48618DBD  bl 0x82df3428
	ctx.lr = 0x827DA670;
	sub_82DF3428(ctx, base);
	// 827DA670: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DA674: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DA678: 48619391  bl 0x82df3a08
	ctx.lr = 0x827DA67C;
	sub_82DF3A08(ctx, base);
	// 827DA67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827DA680: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DA684: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827DA688: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827DA68C: 48654185  bl 0x82e2e810
	ctx.lr = 0x827DA690;
	sub_82E2E810(ctx, base);
	// 827DA690: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DA694: 395F007C  addi r10, r31, 0x7c
	ctx.r[10].s64 = ctx.r[31].s64 + 124;
	// 827DA698: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827DA69C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827DA6A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DA6A4: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 827DA6A8: 4BAE9DB9  bl 0x822c4460
	ctx.lr = 0x827DA6AC;
	sub_822C4460(ctx, base);
	// 827DA6AC: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827DA6B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DA6B4: 419A0008  beq cr6, 0x827da6bc
	if ctx.cr[6].eq {
	pc = 0x827DA6BC; continue 'dispatch;
	}
	// 827DA6B8: 4BAE61D9  bl 0x822c0890
	ctx.lr = 0x827DA6BC;
	sub_822C0890(ctx, base);
	// 827DA6BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DA6C0: 48618D69  bl 0x82df3428
	ctx.lr = 0x827DA6C4;
	sub_82DF3428(ctx, base);
	// 827DA6C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827DA6C8: 48650439  bl 0x82e2ab00
	ctx.lr = 0x827DA6CC;
	sub_82E2AB00(ctx, base);
	// 827DA6CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DA6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DA6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DA6D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DA6DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DA6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DA6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DA6E8 size=1264
    let mut pc: u32 = 0x827DA6E8;
    'dispatch: loop {
        match pc {
            0x827DA6E8 => {
    //   block [0x827DA6E8..0x827DABD8)
	// 827DA6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DA6EC: 489CDA79  bl 0x831a8164
	ctx.lr = 0x827DA6F0;
	sub_831A8130(ctx, base);
	// 827DA6F0: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 827DA6F4: 489CE381  bl 0x831a8a74
	ctx.lr = 0x827DA6F8;
	sub_831A8A40(ctx, base);
	// 827DA6F8: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DABD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DABD8 size=260
    let mut pc: u32 = 0x827DABD8;
    'dispatch: loop {
        match pc {
            0x827DABD8 => {
    //   block [0x827DABD8..0x827DACDC)
	// 827DABD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DABDC: 489CD591  bl 0x831a816c
	ctx.lr = 0x827DABE0;
	sub_831A8130(ctx, base);
	// 827DABE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DABE4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DABE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DABEC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827DABF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827DABF4: 388B4B00  addi r4, r11, 0x4b00
	ctx.r[4].s64 = ctx.r[11].s64 + 19200;
	// 827DABF8: 38A00082  li r5, 0x82
	ctx.r[5].s64 = 130;
	// 827DABFC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 827DAC00: 486177E9  bl 0x82df23e8
	ctx.lr = 0x827DAC04;
	sub_82DF23E8(ctx, base);
	// 827DAC04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DAC08: 41820014  beq 0x827dac1c
	if ctx.cr[0].eq {
	pc = 0x827DAC1C; continue 'dispatch;
	}
	// 827DAC0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DAC10: 4BFFF481  bl 0x827da090
	ctx.lr = 0x827DAC14;
	sub_827DA090(ctx, base);
	// 827DAC14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DAC18: 48000008  b 0x827dac20
	pc = 0x827DAC20; continue 'dispatch;
	// 827DAC1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827DAC20: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827DAC24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DAC28: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DAC2C: 4BFFF8FD  bl 0x827da528
	ctx.lr = 0x827DAC30;
	sub_827DA528(ctx, base);
	// 827DAC30: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827DAC34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DAC38: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DAC3C: 4BAE53C5  bl 0x822c0000
	ctx.lr = 0x827DAC40;
	sub_822C0000(ctx, base);
	// 827DAC40: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DAC44: 3BFE0084  addi r31, r30, 0x84
	ctx.r[31].s64 = ctx.r[30].s64 + 132;
	// 827DAC48: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827DAC4C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827DAC50: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 827DAC54: 4BAE980D  bl 0x822c4460
	ctx.lr = 0x827DAC58;
	sub_822C4460(ctx, base);
	// 827DAC58: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827DAC5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DAC60: 419A0008  beq cr6, 0x827dac68
	if ctx.cr[6].eq {
	pc = 0x827DAC68; continue 'dispatch;
	}
	// 827DAC64: 4BAE5C2D  bl 0x822c0890
	ctx.lr = 0x827DAC68;
	sub_822C0890(ctx, base);
	// 827DAC68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DAC6C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DAC70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DAC74: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827DAC78: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 827DAC7C: 419A0024  beq cr6, 0x827daca0
	if ctx.cr[6].eq {
	pc = 0x827DACA0; continue 'dispatch;
	}
	// 827DAC80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DAC84: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DAC88: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DAC8C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DAC90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DAC94: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DAC98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DAC9C: 4082FFE8  bne 0x827dac84
	if !ctx.cr[0].eq {
	pc = 0x827DAC84; continue 'dispatch;
	}
	// 827DACA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DACA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DACA8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827DACAC: 409A0008  bne cr6, 0x827dacb4
	if !ctx.cr[6].eq {
	pc = 0x827DACB4; continue 'dispatch;
	}
	// 827DACB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827DACB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827DACB8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827DACBC: 808B70C4  lwz r4, 0x70c4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28868 as u32) ) } as u64;
	// 827DACC0: 4BD4EEC9  bl 0x82529b88
	ctx.lr = 0x827DACC4;
	sub_82529B88(ctx, base);
	// 827DACC4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827DACC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DACCC: 419A0008  beq cr6, 0x827dacd4
	if ctx.cr[6].eq {
	pc = 0x827DACD4; continue 'dispatch;
	}
	// 827DACD0: 4BAE5BC1  bl 0x822c0890
	ctx.lr = 0x827DACD4;
	sub_822C0890(ctx, base);
	// 827DACD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DACD8: 489CD4E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DACE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DACE0 size=368
    let mut pc: u32 = 0x827DACE0;
    'dispatch: loop {
        match pc {
            0x827DACE0 => {
    //   block [0x827DACE0..0x827DAE50)
	// 827DACE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DACE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DACE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DACEC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DACF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DACF4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DACF8: 396B4B54  addi r11, r11, 0x4b54
	ctx.r[11].s64 = ctx.r[11].s64 + 19284;
	// 827DACFC: 815F0084  lwz r10, 0x84(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 827DAD00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DAD04: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DAD08: 419A00A4  beq cr6, 0x827dadac
	if ctx.cr[6].eq {
	pc = 0x827DADAC; continue 'dispatch;
	}
	// 827DAD0C: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 827DAD10: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DAD14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DAD18: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 827DAD1C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827DAD20: 419A0024  beq cr6, 0x827dad44
	if ctx.cr[6].eq {
	pc = 0x827DAD44; continue 'dispatch;
	}
	// 827DAD24: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DAD28: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DAD2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DAD30: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DAD34: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DAD38: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DAD3C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DAD40: 4082FFE8  bne 0x827dad28
	if !ctx.cr[0].eq {
	pc = 0x827DAD28; continue 'dispatch;
	}
	// 827DAD44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DAD48: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DAD4C: 4BFFCFCD  bl 0x827d7d18
	ctx.lr = 0x827DAD50;
	sub_827D7D18(ctx, base);
	// 827DAD50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DAD54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DAD58: 388B0094  addi r4, r11, 0x94
	ctx.r[4].s64 = ctx.r[11].s64 + 148;
	// 827DAD5C: 409A0008  bne cr6, 0x827dad64
	if !ctx.cr[6].eq {
	pc = 0x827DAD64; continue 'dispatch;
	}
	// 827DAD60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DAD64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DAD68: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DAD6C: 48616E8D  bl 0x82df1bf8
	ctx.lr = 0x827DAD70;
	sub_82DF1BF8(ctx, base);
	// 827DAD70: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827DAD74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DAD78: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827DAD7C: 409A0008  bne cr6, 0x827dad84
	if !ctx.cr[6].eq {
	pc = 0x827DAD84; continue 'dispatch;
	}
	// 827DAD80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827DAD84: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827DAD88: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827DAD8C: 808B70C4  lwz r4, 0x70c4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28868 as u32) ) } as u64;
	// 827DAD90: 4BD4DE81  bl 0x82528c10
	ctx.lr = 0x827DAD94;
	sub_82528C10(ctx, base);
	// 827DAD94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DAD98: 48616EF9  bl 0x82df1c90
	ctx.lr = 0x827DAD9C;
	sub_82DF1C90(ctx, base);
	// 827DAD9C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827DADA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DADA4: 419A0008  beq cr6, 0x827dadac
	if ctx.cr[6].eq {
	pc = 0x827DADAC; continue 'dispatch;
	}
	// 827DADA8: 4BAE5AE9  bl 0x822c0890
	ctx.lr = 0x827DADAC;
	sub_822C0890(ctx, base);
	// 827DADAC: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 827DADB0: 4BC4D8D1  bl 0x82428680
	ctx.lr = 0x827DADB4;
	sub_82428680(ctx, base);
	// 827DADB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827DADB8: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DADBC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827DADC0: 486173C9  bl 0x82df2188
	ctx.lr = 0x827DADC4;
	sub_82DF2188(ctx, base);
	// 827DADC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DADC8: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 827DADCC: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 827DADD0: 4BC8F231  bl 0x8246a000
	ctx.lr = 0x827DADD4;
	sub_8246A000(ctx, base);
	// 827DADD4: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 827DADD8: 4BC8F229  bl 0x8246a000
	ctx.lr = 0x827DADDC;
	sub_8246A000(ctx, base);
	// 827DADDC: 807F0088  lwz r3, 0x88(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 827DADE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DADE4: 419A0008  beq cr6, 0x827dadec
	if ctx.cr[6].eq {
	pc = 0x827DADEC; continue 'dispatch;
	}
	// 827DADE8: 4BAE5AA9  bl 0x822c0890
	ctx.lr = 0x827DADEC;
	sub_822C0890(ctx, base);
	// 827DADEC: 807F0080  lwz r3, 0x80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 827DADF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DADF4: 419A0008  beq cr6, 0x827dadfc
	if ctx.cr[6].eq {
	pc = 0x827DADFC; continue 'dispatch;
	}
	// 827DADF8: 4BAE5A99  bl 0x822c0890
	ctx.lr = 0x827DADFC;
	sub_822C0890(ctx, base);
	// 827DADFC: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 827DAE00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DAE04: 419A0008  beq cr6, 0x827dae0c
	if ctx.cr[6].eq {
	pc = 0x827DAE0C; continue 'dispatch;
	}
	// 827DAE08: 4BAE5A89  bl 0x822c0890
	ctx.lr = 0x827DAE0C;
	sub_822C0890(ctx, base);
	// 827DAE0C: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827DAE10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DAE14: 419A0008  beq cr6, 0x827dae1c
	if ctx.cr[6].eq {
	pc = 0x827DAE1C; continue 'dispatch;
	}
	// 827DAE18: 4BAE5A79  bl 0x822c0890
	ctx.lr = 0x827DAE1C;
	sub_822C0890(ctx, base);
	// 827DAE1C: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 827DAE20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DAE24: 419A0008  beq cr6, 0x827dae2c
	if ctx.cr[6].eq {
	pc = 0x827DAE2C; continue 'dispatch;
	}
	// 827DAE28: 4BAE5A69  bl 0x822c0890
	ctx.lr = 0x827DAE2C;
	sub_822C0890(ctx, base);
	// 827DAE2C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 827DAE30: 486185F9  bl 0x82df3428
	ctx.lr = 0x827DAE34;
	sub_82DF3428(ctx, base);
	// 827DAE34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DAE38: 4BFFB849  bl 0x827d6680
	ctx.lr = 0x827DAE3C;
	sub_827D6680(ctx, base);
	// 827DAE3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DAE40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DAE44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DAE48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DAE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DAE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827DAE50 size=96
    let mut pc: u32 = 0x827DAE50;
    'dispatch: loop {
        match pc {
            0x827DAE50 => {
    //   block [0x827DAE50..0x827DAEB0)
	// 827DAE50: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DAE54: C1830038  lfs f12, 0x38(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827DAE58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 827DAE5C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 827DAE60: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DAE64: ED601028  fsubs f11, f0, f2
	ctx.f[11].f64 = (((ctx.f[0].f64 - ctx.f[2].f64) as f32) as f64);
	// 827DAE68: C1AA9524  lfs f13, -0x6adc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DAE6C: C009C3C8  lfs f0, -0x3c38(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DAE70: ED6B00B2  fmuls f11, f11, f2
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[2].f64) as f32) as f64);
	// 827DAE74: EDAB0372  fmuls f13, f11, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 827DAE78: ED8C0372  fmuls f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 827DAE7C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827DAE80: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 827DAE84: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 827DAE88: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 827DAE8C: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 827DAE90: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 827DAE94: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 827DAE98: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 827DAE9C: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 827DAEA0: 556B801E  slwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DAEA4: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 827DAEA8: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DAEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827DAEB0 size=76
    let mut pc: u32 = 0x827DAEB0;
    'dispatch: loop {
        match pc {
            0x827DAEB0 => {
    //   block [0x827DAEB0..0x827DAEFC)
	// 827DAEB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DAEB4: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DAEB8: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DAEBC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 827DAEC0: ED806824  fdivs f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 827DAEC4: C1630034  lfs f11, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827DAEC8: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 827DAECC: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DAED0: ED401028  fsubs f10, f0, f2
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[2].f64) as f32) as f64);
	// 827DAED4: C00A9524  lfs f0, -0x6adc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DAED8: C1A9D7BC  lfs f13, -0x2844(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DAEDC: ED4A00B2  fmuls f10, f10, f2
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[2].f64) as f32) as f64);
	// 827DAEE0: EC0A0032  fmuls f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 827DAEE4: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 827DAEE8: EC0002F2  fmuls f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 827DAEEC: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827DAEF0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 827DAEF4: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827DAEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DAF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DAF00 size=76
    let mut pc: u32 = 0x827DAF00;
    'dispatch: loop {
        match pc {
            0x827DAF00 => {
    //   block [0x827DAF00..0x827DAF4C)
	// 827DAF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DAF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DAF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DAF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DAF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DAF14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DAF18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DAF1C: 4BFFFDC5  bl 0x827dace0
	ctx.lr = 0x827DAF20;
	sub_827DACE0(ctx, base);
	// 827DAF20: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DAF24: 4182000C  beq 0x827daf30
	if ctx.cr[0].eq {
	pc = 0x827DAF30; continue 'dispatch;
	}
	// 827DAF28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DAF2C: 4BAE533D  bl 0x822c0268
	ctx.lr = 0x827DAF30;
	sub_822C0268(ctx, base);
	// 827DAF30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DAF34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DAF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DAF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DAF40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DAF44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DAF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DAF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DAF50 size=216
    let mut pc: u32 = 0x827DAF50;
    'dispatch: loop {
        match pc {
            0x827DAF50 => {
    //   block [0x827DAF50..0x827DB028)
	// 827DAF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DAF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DAF58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DAF5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DAF60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DAF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DAF68: 4BFFB779  bl 0x827d66e0
	ctx.lr = 0x827DAF6C;
	sub_827D66E0(ctx, base);
	// 827DAF6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DAF70: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827DAF74: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827DAF78: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 827DAF7C: 39294B54  addi r9, r9, 0x4b54
	ctx.r[9].s64 = ctx.r[9].s64 + 19284;
	// 827DAF80: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827DAF84: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DAF88: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DAF8C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DAF90: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 827DAF94: 911F003C  stw r8, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[8].u32 ) };
	// 827DAF98: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 827DAF9C: 98FF0040  stb r7, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[7].u8 ) };
	// 827DAFA0: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827DAFA4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 827DAFA8: D1BF0038  stfs f13, 0x38(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 827DAFAC: 48618145  bl 0x82df30f0
	ctx.lr = 0x827DAFB0;
	sub_82DF30F0(ctx, base);
	// 827DAFB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827DAFB4: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 827DAFB8: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 827DAFBC: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 827DAFC0: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 827DAFC4: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 827DAFC8: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 827DAFCC: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 827DAFD0: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 827DAFD4: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 827DAFD8: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 827DAFDC: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 827DAFE0: 93DF0090  stw r30, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 827DAFE4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 827DAFE8: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 827DAFEC: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 827DAFF0: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 827DAFF4: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 827DAFF8: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 827DAFFC: 4BFFF4DD  bl 0x827da4d8
	ctx.lr = 0x827DB000;
	sub_827DA4D8(ctx, base);
	// 827DB000: 907F00B4  stw r3, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[3].u32 ) };
	// 827DB004: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 827DB008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB00C: 93DF00BC  stw r30, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 827DB010: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DB014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DB018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DB01C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DB020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DB024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DB028 size=148
    let mut pc: u32 = 0x827DB028;
    'dispatch: loop {
        match pc {
            0x827DB028 => {
    //   block [0x827DB028..0x827DB0BC)
	// 827DB028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB02C: 489CD13D  bl 0x831a8168
	ctx.lr = 0x827DB030;
	sub_831A8130(ctx, base);
	// 827DB030: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827DB034: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB03C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827DB040: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 827DB044: 556B083D  rlwinm. r11, r11, 1, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DB048: 40810068  ble 0x827db0b0
	if !ctx.cr[0].gt {
	pc = 0x827DB0B0; continue 'dispatch;
	}
	// 827DB04C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DB050: 3BBF008C  addi r29, r31, 0x8c
	ctx.r[29].s64 = ctx.r[31].s64 + 140;
	// 827DB054: 3B9F009C  addi r28, r31, 0x9c
	ctx.r[28].s64 = ctx.r[31].s64 + 156;
	// 827DB058: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827DB05C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 827DB060: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827DB064: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827DB068: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DB06C: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827DB070: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827DB074: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827DB078: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DB07C: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827DB080: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827DB084: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827DB088: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 827DB08C: 4BFFB4C5  bl 0x827d6550
	ctx.lr = 0x827DB090;
	sub_827D6550(ctx, base);
	// 827DB090: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DB094: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827DB098: 4BFFB4B9  bl 0x827d6550
	ctx.lr = 0x827DB09C;
	sub_827D6550(ctx, base);
	// 827DB09C: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 827DB0A0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 827DB0A4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DB0A8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 827DB0AC: 4198FFB0  blt cr6, 0x827db05c
	if ctx.cr[6].lt {
	pc = 0x827DB05C; continue 'dispatch;
	}
	// 827DB0B0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827DB0B4: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827DB0B8: 489CD100  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DB0C0 size=52
    let mut pc: u32 = 0x827DB0C0;
    'dispatch: loop {
        match pc {
            0x827DB0C0 => {
    //   block [0x827DB0C0..0x827DB0F4)
	// 827DB0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DB0C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DB0CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB0D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB0D4: 4BFFAA05  bl 0x827d5ad8
	ctx.lr = 0x827DB0D8;
	sub_827D5AD8(ctx, base);
	// 827DB0D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB0DC: 4BFFFF4D  bl 0x827db028
	ctx.lr = 0x827DB0E0;
	sub_827DB028(ctx, base);
	// 827DB0E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DB0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DB0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DB0EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DB0F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB0F8 size=20
    let mut pc: u32 = 0x827DB0F8;
    'dispatch: loop {
        match pc {
            0x827DB0F8 => {
    //   block [0x827DB0F8..0x827DB10C)
	// 827DB0F8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DB0FC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 827DB100: 396B4B88  addi r11, r11, 0x4b88
	ctx.r[11].s64 = ctx.r[11].s64 + 19336;
	// 827DB104: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DB108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DB110 size=392
    let mut pc: u32 = 0x827DB110;
    'dispatch: loop {
        match pc {
            0x827DB110 => {
    //   block [0x827DB110..0x827DB298)
	// 827DB110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB114: 489CD051  bl 0x831a8164
	ctx.lr = 0x827DB118;
	sub_831A8130(ctx, base);
	// 827DB118: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB11C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DB120: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DB124: 419A016C  beq cr6, 0x827db290
	if ctx.cr[6].eq {
	pc = 0x827DB290; continue 'dispatch;
	}
	// 827DB128: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 827DB12C: 4BB2A2BD  bl 0x823053e8
	ctx.lr = 0x827DB130;
	sub_823053E8(ctx, base);
	// 827DB130: 483D16D9  bl 0x82bac808
	ctx.lr = 0x827DB134;
	sub_82BAC808(ctx, base);
	// 827DB134: 4800F135  bl 0x827ea268
	ctx.lr = 0x827DB138;
	sub_827EA268(ctx, base);
	// 827DB138: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827DB13C: 41820154  beq 0x827db290
	if ctx.cr[0].eq {
	pc = 0x827DB290; continue 'dispatch;
	}
	// 827DB140: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 827DB144: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DB148: 409A0080  bne cr6, 0x827db1c8
	if !ctx.cr[6].eq {
	pc = 0x827DB1C8; continue 'dispatch;
	}
	// 827DB14C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DB150: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 827DB154: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827DB158: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 827DB15C: 394A4C50  addi r10, r10, 0x4c50
	ctx.r[10].s64 = ctx.r[10].s64 + 19536;
	// 827DB160: 39294C44  addi r9, r9, 0x4c44
	ctx.r[9].s64 = ctx.r[9].s64 + 19524;
	// 827DB164: 3CE08207  lis r7, -0x7df9
	ctx.r[7].s64 = -2113470464;
	// 827DB168: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827DB16C: 3CC08207  lis r6, -0x7df9
	ctx.r[6].s64 = -2113470464;
	// 827DB170: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 827DB174: 39085900  addi r8, r8, 0x5900
	ctx.r[8].s64 = ctx.r[8].s64 + 22784;
	// 827DB178: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DB17C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DB180: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 827DB184: 3C808207  lis r4, -0x7df9
	ctx.r[4].s64 = -2113470464;
	// 827DB188: 39474C3C  addi r10, r7, 0x4c3c
	ctx.r[10].s64 = ctx.r[7].s64 + 19516;
	// 827DB18C: 39264C30  addi r9, r6, 0x4c30
	ctx.r[9].s64 = ctx.r[6].s64 + 19504;
	// 827DB190: 39044C24  addi r8, r4, 0x4c24
	ctx.r[8].s64 = ctx.r[4].s64 + 19492;
	// 827DB194: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 827DB198: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 827DB19C: 3C608207  lis r3, -0x7df9
	ctx.r[3].s64 = -2113470464;
	// 827DB1A0: 3FA08207  lis r29, -0x7df9
	ctx.r[29].s64 = -2113470464;
	// 827DB1A4: 7C8B282E  lwzx r4, r11, r5
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 827DB1A8: 3F808207  lis r28, -0x7df9
	ctx.r[28].s64 = -2113470464;
	// 827DB1AC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 827DB1B0: 3F608207  lis r27, -0x7df9
	ctx.r[27].s64 = -2113470464;
	// 827DB1B4: 39434C18  addi r10, r3, 0x4c18
	ctx.r[10].s64 = ctx.r[3].s64 + 19480;
	// 827DB1B8: 393D4C0C  addi r9, r29, 0x4c0c
	ctx.r[9].s64 = ctx.r[29].s64 + 19468;
	// 827DB1BC: 397C4C04  addi r11, r28, 0x4c04
	ctx.r[11].s64 = ctx.r[28].s64 + 19460;
	// 827DB1C0: 391B4BF8  addi r8, r27, 0x4bf8
	ctx.r[8].s64 = ctx.r[27].s64 + 19448;
	// 827DB1C4: 48000084  b 0x827db248
	pc = 0x827DB248; continue 'dispatch;
	// 827DB1C8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 827DB1CC: 409A00C4  bne cr6, 0x827db290
	if !ctx.cr[6].eq {
	pc = 0x827DB290; continue 'dispatch;
	}
	// 827DB1D0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DB1D4: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 827DB1D8: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827DB1DC: 394A4BEC  addi r10, r10, 0x4bec
	ctx.r[10].s64 = ctx.r[10].s64 + 19436;
	// 827DB1E0: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 827DB1E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827DB1E8: 39294BE0  addi r9, r9, 0x4be0
	ctx.r[9].s64 = ctx.r[9].s64 + 19424;
	// 827DB1EC: 3CE08207  lis r7, -0x7df9
	ctx.r[7].s64 = -2113470464;
	// 827DB1F0: 3CC08207  lis r6, -0x7df9
	ctx.r[6].s64 = -2113470464;
	// 827DB1F4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 827DB1F8: 3908590C  addi r8, r8, 0x590c
	ctx.r[8].s64 = ctx.r[8].s64 + 22796;
	// 827DB1FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DB200: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DB204: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 827DB208: 3C808207  lis r4, -0x7df9
	ctx.r[4].s64 = -2113470464;
	// 827DB20C: 39474BD8  addi r10, r7, 0x4bd8
	ctx.r[10].s64 = ctx.r[7].s64 + 19416;
	// 827DB210: 39264BCC  addi r9, r6, 0x4bcc
	ctx.r[9].s64 = ctx.r[6].s64 + 19404;
	// 827DB214: 39044BC0  addi r8, r4, 0x4bc0
	ctx.r[8].s64 = ctx.r[4].s64 + 19392;
	// 827DB218: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 827DB21C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 827DB220: 3C608207  lis r3, -0x7df9
	ctx.r[3].s64 = -2113470464;
	// 827DB224: 3FA08207  lis r29, -0x7df9
	ctx.r[29].s64 = -2113470464;
	// 827DB228: 3F808207  lis r28, -0x7df9
	ctx.r[28].s64 = -2113470464;
	// 827DB22C: 3F608207  lis r27, -0x7df9
	ctx.r[27].s64 = -2113470464;
	// 827DB230: 39434BB4  addi r10, r3, 0x4bb4
	ctx.r[10].s64 = ctx.r[3].s64 + 19380;
	// 827DB234: 393D4BA8  addi r9, r29, 0x4ba8
	ctx.r[9].s64 = ctx.r[29].s64 + 19368;
	// 827DB238: 7C8B282E  lwzx r4, r11, r5
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 827DB23C: 397C4BA0  addi r11, r28, 0x4ba0
	ctx.r[11].s64 = ctx.r[28].s64 + 19360;
	// 827DB240: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 827DB244: 391B4B94  addi r8, r27, 0x4b94
	ctx.r[8].s64 = ctx.r[27].s64 + 19348;
	// 827DB248: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DB24C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827DB250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DB254: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 827DB258: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 827DB25C: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 827DB260: 487D1AB1  bl 0x82facd10
	ctx.lr = 0x827DB264;
	sub_82FACD10(ctx, base);
	// 827DB264: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 827DB268: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 827DB26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DB270: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 827DB274: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DB278: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DB27C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DB280: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 827DB284: 487D1A8D  bl 0x82facd10
	ctx.lr = 0x827DB288;
	sub_82FACD10(ctx, base);
	// 827DB288: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 827DB28C: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 827DB290: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827DB294: 489CCF20  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB298 size=8
    let mut pc: u32 = 0x827DB298;
    'dispatch: loop {
        match pc {
            0x827DB298 => {
    //   block [0x827DB298..0x827DB2A0)
	// 827DB298: 908300C0  stw r4, 0xc0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), ctx.r[4].u32 ) };
	// 827DB29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB2A0 size=8
    let mut pc: u32 = 0x827DB2A0;
    'dispatch: loop {
        match pc {
            0x827DB2A0 => {
    //   block [0x827DB2A0..0x827DB2A8)
	// 827DB2A0: 908300C4  stw r4, 0xc4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), ctx.r[4].u32 ) };
	// 827DB2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB2A8 size=12
    let mut pc: u32 = 0x827DB2A8;
    'dispatch: loop {
        match pc {
            0x827DB2A8 => {
    //   block [0x827DB2A8..0x827DB2B4)
	// 827DB2A8: 908300C8  stw r4, 0xc8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[4].u32 ) };
	// 827DB2AC: 90A300CC  stw r5, 0xcc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), ctx.r[5].u32 ) };
	// 827DB2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB2B8 size=8
    let mut pc: u32 = 0x827DB2B8;
    'dispatch: loop {
        match pc {
            0x827DB2B8 => {
    //   block [0x827DB2B8..0x827DB2C0)
	// 827DB2B8: 9083010C  stw r4, 0x10c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(268 as u32), ctx.r[4].u32 ) };
	// 827DB2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827DB2C0 size=8
    let mut pc: u32 = 0x827DB2C0;
    'dispatch: loop {
        match pc {
            0x827DB2C0 => {
    //   block [0x827DB2C0..0x827DB2C8)
	// 827DB2C0: D0230100  stfs f1, 0x100(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 827DB2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DB2C8 size=164
    let mut pc: u32 = 0x827DB2C8;
    'dispatch: loop {
        match pc {
            0x827DB2C8 => {
    //   block [0x827DB2C8..0x827DB36C)
	// 827DB2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB2CC: 489CCEA1  bl 0x831a816c
	ctx.lr = 0x827DB2D0;
	sub_831A8130(ctx, base);
	// 827DB2D0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827DB2D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB2D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB2DC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827DB2E0: 4BFFECF1  bl 0x827d9fd0
	ctx.lr = 0x827DB2E4;
	sub_827D9FD0(ctx, base);
	// 827DB2E4: 547D463E  srwi r29, r3, 0x18
	ctx.r[29].u32 = ctx.r[3].u32.wrapping_shr(24);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 827DB2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB2EC: 4BFFECE5  bl 0x827d9fd0
	ctx.lr = 0x827DB2F0;
	sub_827D9FD0(ctx, base);
	// 827DB2F0: 546B023E  clrlwi r11, r3, 8
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00FFFFFFu64;
	// 827DB2F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB2F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DB2FC: 4BFFA775  bl 0x827d5a70
	ctx.lr = 0x827DB300;
	sub_827D5A70(ctx, base);
	// 827DB300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB304: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827DB308: 4BFFA771  bl 0x827d5a78
	ctx.lr = 0x827DB30C;
	sub_827D5A78(ctx, base);
	// 827DB30C: FBA10050  std r29, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u64 ) };
	// 827DB310: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827DB314: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 827DB318: EDBF0824  fdivs f13, f31, f1
	ctx.f[13].f64 = ((ctx.f[31].f64 / ctx.f[1].f64) as f32) as f64;
	// 827DB31C: C19F0100  lfs f12, 0x100(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827DB320: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 827DB324: 897E0001  lbz r11, 1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 827DB328: 895E0002  lbz r10, 2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 827DB32C: 893E0003  lbz r9, 3(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(3 as u32) ) } as u64;
	// 827DB330: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 827DB334: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827DB338: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 827DB33C: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 827DB340: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827DB344: 5508402E  slwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 827DB348: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 827DB34C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DB350: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 827DB354: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DB358: 7D6B4B78  or r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 827DB35C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DB360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DB364: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827DB368: 489CCE54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB370 size=32
    let mut pc: u32 = 0x827DB370;
    'dispatch: loop {
        match pc {
            0x827DB370 => {
    //   block [0x827DB370..0x827DB390)
	// 827DB370: 81630104  lwz r11, 0x104(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(260 as u32) ) } as u64;
	// 827DB374: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DB378: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 827DB37C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DB380: 41980008  blt cr6, 0x827db388
	if ctx.cr[6].lt {
	pc = 0x827DB388; continue 'dispatch;
	}
	// 827DB384: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DB388: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DB38C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB390 size=8
    let mut pc: u32 = 0x827DB390;
    'dispatch: loop {
        match pc {
            0x827DB390 => {
    //   block [0x827DB390..0x827DB398)
	// 827DB390: 4BFFF358  b 0x827da6e8
	sub_827DA6E8(ctx, base);
	return;
	// 827DB394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DB398 size=200
    let mut pc: u32 = 0x827DB398;
    'dispatch: loop {
        match pc {
            0x827DB398 => {
    //   block [0x827DB398..0x827DB460)
	// 827DB398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB39C: 489CCDD1  bl 0x831a816c
	ctx.lr = 0x827DB3A0;
	sub_831A8130(ctx, base);
	// 827DB3A0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB3A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DB3A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DB3AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DB3B0: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 827DB3B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DB3B8: 419A009C  beq cr6, 0x827db454
	if ctx.cr[6].eq {
	pc = 0x827DB454; continue 'dispatch;
	}
	// 827DB3BC: 4BB2A02D  bl 0x823053e8
	ctx.lr = 0x827DB3C0;
	sub_823053E8(ctx, base);
	// 827DB3C0: 483084D1  bl 0x82ae3890
	ctx.lr = 0x827DB3C4;
	sub_82AE3890(ctx, base);
	// 827DB3C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DB3C8: 4182008C  beq 0x827db454
	if ctx.cr[0].eq {
	pc = 0x827DB454; continue 'dispatch;
	}
	// 827DB3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DB3D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DB3D4: 487D42B5  bl 0x82faf688
	ctx.lr = 0x827DB3D8;
	sub_82FAF688(ctx, base);
	// 827DB3D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DB3DC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 827DB3E0: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 827DB3E4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 827DB3E8: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 827DB3EC: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 827DB3F0: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 827DB3F4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DB460 size=548
    let mut pc: u32 = 0x827DB460;
    'dispatch: loop {
        match pc {
            0x827DB460 => {
    //   block [0x827DB460..0x827DB684)
	// 827DB460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DB468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DB46C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DB470: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DB688 size=340
    let mut pc: u32 = 0x827DB688;
    'dispatch: loop {
        match pc {
            0x827DB688 => {
    //   block [0x827DB688..0x827DB7DC)
	// 827DB688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB68C: 489CCADD  bl 0x831a8168
	ctx.lr = 0x827DB690;
	sub_831A8130(ctx, base);
	// 827DB690: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB694: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DB698: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DB69C: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827DB6A0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827DB6A4: 396B4CC0  addi r11, r11, 0x4cc0
	ctx.r[11].s64 = ctx.r[11].s64 + 19648;
	// 827DB6A8: 394A4CAC  addi r10, r10, 0x4cac
	ctx.r[10].s64 = ctx.r[10].s64 + 19628;
	// 827DB6AC: 39294C9C  addi r9, r9, 0x4c9c
	ctx.r[9].s64 = ctx.r[9].s64 + 19612;
	// 827DB6B0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827DB6B4: 3D008207  lis r8, -0x7df9
	ctx.r[8].s64 = -2113470464;
	// 827DB6B8: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 827DB6BC: 3CE08207  lis r7, -0x7df9
	ctx.r[7].s64 = -2113470464;
	// 827DB6C0: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 827DB6C4: 3CC08207  lis r6, -0x7df9
	ctx.r[6].s64 = -2113470464;
	// 827DB6C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB6CC: 39684C84  addi r11, r8, 0x4c84
	ctx.r[11].s64 = ctx.r[8].s64 + 19588;
	// 827DB6D0: 39474C74  addi r10, r7, 0x4c74
	ctx.r[10].s64 = ctx.r[7].s64 + 19572;
	// 827DB6D4: 39264C5C  addi r9, r6, 0x4c5c
	ctx.r[9].s64 = ctx.r[6].s64 + 19548;
	// 827DB6D8: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 827DB6DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827DB6E0: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 827DB6E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827DB6E8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DB6EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827DB6F0: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 827DB6F4: 4864F3F5  bl 0x82e2aae8
	ctx.lr = 0x827DB6F8;
	sub_82E2AAE8(ctx, base);
	// 827DB6F8: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 827DB6FC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 827DB700: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DB704: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DB708: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 827DB70C: 486182FD  bl 0x82df3a08
	ctx.lr = 0x827DB710;
	sub_82DF3A08(ctx, base);
	// 827DB710: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827DB714: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DB718: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827DB71C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DB720: 486504C1  bl 0x82e2bbe0
	ctx.lr = 0x827DB724;
	sub_82E2BBE0(ctx, base);
	// 827DB724: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DB728: 395F0064  addi r10, r31, 0x64
	ctx.r[10].s64 = ctx.r[31].s64 + 100;
	// 827DB72C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827DB730: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827DB734: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DB738: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827DB73C: 4BAE8D25  bl 0x822c4460
	ctx.lr = 0x827DB740;
	sub_822C4460(ctx, base);
	// 827DB740: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827DB744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DB748: 419A0008  beq cr6, 0x827db750
	if ctx.cr[6].eq {
	pc = 0x827DB750; continue 'dispatch;
	}
	// 827DB74C: 4BAE5145  bl 0x822c0890
	ctx.lr = 0x827DB750;
	sub_822C0890(ctx, base);
	// 827DB750: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DB754: 48617CD5  bl 0x82df3428
	ctx.lr = 0x827DB758;
	sub_82DF3428(ctx, base);
	// 827DB758: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 827DB75C: 39610074  addi r11, r1, 0x74
	ctx.r[11].s64 = ctx.r[1].s64 + 116;
	// 827DB760: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DB764: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DB768: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 827DB76C: 4861829D  bl 0x82df3a08
	ctx.lr = 0x827DB770;
	sub_82DF3A08(ctx, base);
	// 827DB770: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827DB774: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DB778: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827DB77C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827DB780: 48650461  bl 0x82e2bbe0
	ctx.lr = 0x827DB784;
	sub_82E2BBE0(ctx, base);
	// 827DB784: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DB788: 395F006C  addi r10, r31, 0x6c
	ctx.r[10].s64 = ctx.r[31].s64 + 108;
	// 827DB78C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827DB790: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827DB794: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DB798: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827DB79C: 4BAE8CC5  bl 0x822c4460
	ctx.lr = 0x827DB7A0;
	sub_822C4460(ctx, base);
	// 827DB7A0: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827DB7A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DB7A8: 419A0008  beq cr6, 0x827db7b0
	if ctx.cr[6].eq {
	pc = 0x827DB7B0; continue 'dispatch;
	}
	// 827DB7AC: 4BAE50E5  bl 0x822c0890
	ctx.lr = 0x827DB7B0;
	sub_822C0890(ctx, base);
	// 827DB7B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DB7B4: 48617C75  bl 0x82df3428
	ctx.lr = 0x827DB7B8;
	sub_82DF3428(ctx, base);
	// 827DB7B8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827DB7BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827DB7C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DB7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB7C8: 4BFFEE29  bl 0x827da5f0
	ctx.lr = 0x827DB7CC;
	sub_827DA5F0(ctx, base);
	// 827DB7CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827DB7D0: 4864F331  bl 0x82e2ab00
	ctx.lr = 0x827DB7D4;
	sub_82E2AB00(ctx, base);
	// 827DB7D4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827DB7D8: 489CC9E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DB7E0 size=192
    let mut pc: u32 = 0x827DB7E0;
    'dispatch: loop {
        match pc {
            0x827DB7E0 => {
    //   block [0x827DB7E0..0x827DB8A0)
	// 827DB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DB7E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DB7EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DB7F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB7F4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DB7F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DB7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827DB800: 388B4CD0  addi r4, r11, 0x4cd0
	ctx.r[4].s64 = ctx.r[11].s64 + 19664;
	// 827DB804: 38A0010B  li r5, 0x10b
	ctx.r[5].s64 = 267;
	// 827DB808: 386004F8  li r3, 0x4f8
	ctx.r[3].s64 = 1272;
	// 827DB80C: 4BAE4BCD  bl 0x822c03d8
	ctx.lr = 0x827DB810;
	sub_822C03D8(ctx, base);
	// 827DB810: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DB814: 41820028  beq 0x827db83c
	if ctx.cr[0].eq {
	pc = 0x827DB83C; continue 'dispatch;
	}
	// 827DB818: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 827DB81C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 827DB820: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827DB824: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DB828: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DB82C: 394A01A8  addi r10, r10, 0x1a8
	ctx.r[10].s64 = ctx.r[10].s64 + 424;
	// 827DB830: 4080FFF0  bge 0x827db820
	if !ctx.cr[0].lt {
	pc = 0x827DB820; continue 'dispatch;
	}
	// 827DB834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB838: 48000008  b 0x827db840
	pc = 0x827DB840; continue 'dispatch;
	// 827DB83C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827DB840: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827DB844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DB848: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DB84C: 4BFFB2C5  bl 0x827d6b10
	ctx.lr = 0x827DB850;
	sub_827D6B10(ctx, base);
	// 827DB850: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827DB854: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DB858: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DB85C: 4BAE47A5  bl 0x822c0000
	ctx.lr = 0x827DB860;
	sub_822C0000(ctx, base);
	// 827DB860: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DB864: 397E0104  addi r11, r30, 0x104
	ctx.r[11].s64 = ctx.r[30].s64 + 260;
	// 827DB868: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827DB86C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 827DB870: 915E0104  stw r10, 0x104(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 827DB874: 4BAE8BED  bl 0x822c4460
	ctx.lr = 0x827DB878;
	sub_822C4460(ctx, base);
	// 827DB878: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827DB87C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DB880: 419A0008  beq cr6, 0x827db888
	if ctx.cr[6].eq {
	pc = 0x827DB888; continue 'dispatch;
	}
	// 827DB884: 4BAE500D  bl 0x822c0890
	ctx.lr = 0x827DB888;
	sub_822C0890(ctx, base);
	// 827DB888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DB88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DB890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DB894: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DB898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DB89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DB8A0 size=108
    let mut pc: u32 = 0x827DB8A0;
    'dispatch: loop {
        match pc {
            0x827DB8A0 => {
    //   block [0x827DB8A0..0x827DB90C)
	// 827DB8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DB8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DB8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB8B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB8B4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DB8B8: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 827DB8BC: 396B4D28  addi r11, r11, 0x4d28
	ctx.r[11].s64 = ctx.r[11].s64 + 19752;
	// 827DB8C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DB8C4: 4BC4CDBD  bl 0x82428680
	ctx.lr = 0x827DB8C8;
	sub_82428680(ctx, base);
	// 827DB8C8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827DB8CC: 809F0114  lwz r4, 0x114(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 827DB8D0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827DB8D4: 486168B5  bl 0x82df2188
	ctx.lr = 0x827DB8D8;
	sub_82DF2188(ctx, base);
	// 827DB8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DB8DC: 917F0114  stw r11, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 827DB8E0: 807F0108  lwz r3, 0x108(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827DB8E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DB8E8: 419A0008  beq cr6, 0x827db8f0
	if ctx.cr[6].eq {
	pc = 0x827DB8F0; continue 'dispatch;
	}
	// 827DB8EC: 4BAE4FA5  bl 0x822c0890
	ctx.lr = 0x827DB8F0;
	sub_822C0890(ctx, base);
	// 827DB8F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB8F4: 4BFFF3ED  bl 0x827dace0
	ctx.lr = 0x827DB8F8;
	sub_827DACE0(ctx, base);
	// 827DB8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DB8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DB900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DB904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DB908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DB910 size=76
    let mut pc: u32 = 0x827DB910;
    'dispatch: loop {
        match pc {
            0x827DB910 => {
    //   block [0x827DB910..0x827DB95C)
	// 827DB910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DB918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DB91C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DB920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DB92C: 4BFFFF75  bl 0x827db8a0
	ctx.lr = 0x827DB930;
	sub_827DB8A0(ctx, base);
	// 827DB930: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DB934: 4182000C  beq 0x827db940
	if ctx.cr[0].eq {
	pc = 0x827DB940; continue 'dispatch;
	}
	// 827DB938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB93C: 4BAE492D  bl 0x822c0268
	ctx.lr = 0x827DB940;
	sub_822C0268(ctx, base);
	// 827DB940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DB944: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DB948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DB94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DB950: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DB954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DB958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DB960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DB960 size=168
    let mut pc: u32 = 0x827DB960;
    'dispatch: loop {
        match pc {
            0x827DB960 => {
    //   block [0x827DB960..0x827DBA08)
	// 827DB960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DB964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DB968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DB96C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DB970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DB974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DB978: 4BFFF5D9  bl 0x827daf50
	ctx.lr = 0x827DB97C;
	sub_827DAF50(ctx, base);
	// 827DB97C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827DB980: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DB984: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827DB988: 392000E0  li r9, 0xe0
	ctx.r[9].s64 = 224;
	// 827DB98C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827DB990: 394A4D28  addi r10, r10, 0x4d28
	ctx.r[10].s64 = ctx.r[10].s64 + 19752;
	// 827DB994: 390000F0  li r8, 0xf0
	ctx.r[8].s64 = 240;
	// 827DB998: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 827DB99C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 827DB9A0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DB9A4: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 827DB9A8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 827DB9AC: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 827DB9B0: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 827DB9B4: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 827DB9B8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DBA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DBA08 size=504
    let mut pc: u32 = 0x827DBA08;
    'dispatch: loop {
        match pc {
            0x827DBA08 => {
    //   block [0x827DBA08..0x827DBC00)
	// 827DBA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DBA0C: 489CC75D  bl 0x831a8168
	ctx.lr = 0x827DBA10;
	sub_831A8130(ctx, base);
	// 827DBA10: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827DBA14: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DBA18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DBA1C: 897F011C  lbz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 827DBA20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DBA24: 408201D0  bne 0x827dbbf4
	if !ctx.cr[0].eq {
	pc = 0x827DBBF4; continue 'dispatch;
	}
	// 827DBA28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DBA2C: 80BF00D0  lwz r5, 0xd0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 827DBA30: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 827DBA34: 4BFFFA2D  bl 0x827db460
	ctx.lr = 0x827DBA38;
	sub_827DB460(ctx, base);
	// 827DBA38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DBA3C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 827DBA40: 80BF00D4  lwz r5, 0xd4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 827DBA44: 4BFFFA1D  bl 0x827db460
	ctx.lr = 0x827DBA48;
	sub_827DB460(ctx, base);
	// 827DBA48: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DBA4C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 827DBA50: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 827DBA54: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 827DBA58: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827DBA5C: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 827DBA60: D3E10074  stfs f31, 0x74(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 827DBA64: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 827DBA68: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 827DBA6C: 486A025D  bl 0x82e7bcc8
	ctx.lr = 0x827DBA70;
	sub_82E7BCC8(ctx, base);
	// 827DBA70: D3E10080  stfs f31, 0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 827DBA74: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 827DBA78: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 827DBA7C: D3E10088  stfs f31, 0x88(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 827DBA80: 38810100  addi r4, r1, 0x100
	ctx.r[4].s64 = ctx.r[1].s64 + 256;
	// 827DBA84: D3E1008C  stfs f31, 0x8c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 827DBA88: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 827DBA8C: 486A023D  bl 0x82e7bcc8
	ctx.lr = 0x827DBA90;
	sub_82E7BCC8(ctx, base);
	// 827DBA90: C1BF0018  lfs f13, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DBA94: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DBA98: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 827DBA9C: C1BF0030  lfs f13, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DBAA0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 827DBAA4: 409800D4  bge cr6, 0x827dbb78
	if !ctx.cr[6].lt {
	pc = 0x827DBB78; continue 'dispatch;
	}
	// 827DBAA8: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 827DBAAC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 827DBAB0: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 827DBAB4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827DBAB8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827DBABC: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DBC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DBC00 size=232
    let mut pc: u32 = 0x827DBC00;
    'dispatch: loop {
        match pc {
            0x827DBC00 => {
    //   block [0x827DBC00..0x827DBCE8)
	// 827DBC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DBC04: 489CC569  bl 0x831a816c
	ctx.lr = 0x827DBC08;
	sub_831A8130(ctx, base);
	// 827DBC08: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827DBC0C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DBC10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DBC14: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827DBC18: 396000F0  li r11, 0xf0
	ctx.r[11].s64 = 240;
	// 827DBC1C: 3BFE00E0  addi r31, r30, 0xe0
	ctx.r[31].s64 = ctx.r[30].s64 + 224;
	// 827DBC20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DBC24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DBC28: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DBCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DBCE8 size=80
    let mut pc: u32 = 0x827DBCE8;
    'dispatch: loop {
        match pc {
            0x827DBCE8 => {
    //   block [0x827DBCE8..0x827DBD38)
	// 827DBCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DBCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DBCF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DBCF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DBCF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DBCFC: 4BFFE2FD  bl 0x827d9ff8
	ctx.lr = 0x827DBD00;
	sub_827D9FF8(ctx, base);
	// 827DBD00: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 827DBD04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DBD08: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 827DBD0C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827DBD10: 796907E6  rldicr r9, r11, 0x20, 0x3f
	ctx.r[9].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 827DBD14: E8C10058  ld r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 827DBD18: E8E10060  ld r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 827DBD1C: E9010068  ld r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 827DBD20: 4BFFE171  bl 0x827d9e90
	ctx.lr = 0x827DBD24;
	sub_827D9E90(ctx, base);
	// 827DBD24: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DBD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DBD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DBD30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DBD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DBD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DBD38 size=92
    let mut pc: u32 = 0x827DBD38;
    'dispatch: loop {
        match pc {
            0x827DBD38 => {
    //   block [0x827DBD38..0x827DBD94)
	// 827DBD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DBD3C: 489CC42D  bl 0x831a8168
	ctx.lr = 0x827DBD40;
	sub_831A8130(ctx, base);
	// 827DBD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DBD44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DBD48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DBD4C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DBD50: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827DBD54: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 827DBD58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DBD5C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DBD60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DBD64: 4E800421  bctrl
	ctx.lr = 0x827DBD68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DBD68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DBD6C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 827DBD70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827DBD74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DBD78: 4BFFF349  bl 0x827db0c0
	ctx.lr = 0x827DBD7C;
	sub_827DB0C0(ctx, base);
	// 827DBD7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DBD80: 4BFFFA61  bl 0x827db7e0
	ctx.lr = 0x827DBD84;
	sub_827DB7E0(ctx, base);
	// 827DBD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DBD88: 4BFFFF61  bl 0x827dbce8
	ctx.lr = 0x827DBD8C;
	sub_827DBCE8(ctx, base);
	// 827DBD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DBD90: 489CC428  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DBD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DBD98 size=208
    let mut pc: u32 = 0x827DBD98;
    'dispatch: loop {
        match pc {
            0x827DBD98 => {
    //   block [0x827DBD98..0x827DBE68)
	// 827DBD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DBD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DBDA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DBDA4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827DBDA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DBDAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DBDB0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827DBDB4: 489CFF55  bl 0x831abd08
	ctx.lr = 0x827DBDB8;
	sub_831ABD08(ctx, base);
	// 827DBDB8: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 827DBDBC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 827DBDC0: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 827DBDC4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827DBDC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DBDCC: 4098000C  bge cr6, 0x827dbdd8
	if !ctx.cr[6].lt {
	pc = 0x827DBDD8; continue 'dispatch;
	}
	// 827DBDD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DBDD4: 48000014  b 0x827dbde8
	pc = 0x827DBDE8; continue 'dispatch;
	// 827DBDD8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DBDDC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 827DBDE0: 41980008  blt cr6, 0x827dbde8
	if ctx.cr[6].lt {
	pc = 0x827DBDE8; continue 'dispatch;
	}
	// 827DBDE4: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 827DBDE8: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 827DBDEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DBDF0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 827DBDF4: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827DBDF8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 827DBDFC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 827DBE00: EC1F0028  fsubs f0, f31, f0
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 827DBE04: 40980010  bge cr6, 0x827dbe14
	if !ctx.cr[6].lt {
	pc = 0x827DBE14; continue 'dispatch;
	}
	// 827DBE08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DBE0C: C02B08A4  lfs f1, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827DBE10: 48000040  b 0x827dbe50
	pc = 0x827DBE50; continue 'dispatch;
	// 827DBE14: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 827DBE18: 392B002B  addi r9, r11, 0x2b
	ctx.r[9].s64 = ctx.r[11].s64 + 43;
	// 827DBE1C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DBE20: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827DBE24: 390B0016  addi r8, r11, 0x16
	ctx.r[8].s64 = ctx.r[11].s64 + 22;
	// 827DBE28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 827DBE2C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 827DBE30: 7DAAFC2E  lfsx f13, r10, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DBE34: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DBE38: 7D89FC2E  lfsx f12, r9, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827DBE3C: EDAD603A  fmadds f13, f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 827DBE40: 7D88FC2E  lfsx f12, r8, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827DBE44: 7D6BFC2E  lfsx f11, r11, r31
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827DBE48: EDAD603A  fmadds f13, f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 827DBE4C: EC2D583A  fmadds f1, f13, f0, f11
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 827DBE50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DBE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DBE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DBE5C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DBE60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DBE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DBE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DBE68 size=24
    let mut pc: u32 = 0x827DBE68;
    'dispatch: loop {
        match pc {
            0x827DBE68 => {
    //   block [0x827DBE68..0x827DBE80)
	// 827DBE68: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DBE6C: 3D208336  lis r9, -0x7cca
	ctx.r[9].s64 = -2093613056;
	// 827DBE70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DBE74: 996A87D4  stb r11, -0x782c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-30764 as u32), ctx.r[11].u8 ) };
	// 827DBE78: 986987D5  stb r3, -0x782b(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(-30763 as u32), ctx.r[3].u8 ) };
	// 827DBE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DBE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DBE80 size=464
    let mut pc: u32 = 0x827DBE80;
    'dispatch: loop {
        match pc {
            0x827DBE80 => {
    //   block [0x827DBE80..0x827DC050)
	// 827DBE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DBE84: 489CC2D5  bl 0x831a8158
	ctx.lr = 0x827DBE88;
	sub_831A8130(ctx, base);
	// 827DBE88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DBE8C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 827DBE90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DBE94: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827DBE98: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 827DBE9C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 827DBEA0: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DBEA4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DBEA8: 41980010  blt cr6, 0x827dbeb8
	if ctx.cr[6].lt {
	pc = 0x827DBEB8; continue 'dispatch;
	}
	// 827DBEAC: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DBEB0: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 827DBEB4: 40980008  bge cr6, 0x827dbebc
	if !ctx.cr[6].lt {
	pc = 0x827DBEBC; continue 'dispatch;
	}
	// 827DBEB8: 486149C1  bl 0x82df0878
	ctx.lr = 0x827DBEBC;
	sub_82DF0878(ctx, base);
	// 827DBEBC: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DBEC0: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 827DBEC4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 827DBEC8: 40980008  bge cr6, 0x827dbed0
	if !ctx.cr[6].lt {
	pc = 0x827DBED0; continue 'dispatch;
	}
	// 827DBECC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 827DBED0: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DBED4: 216BFFFF  subfic r11, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[11].s64 = (-1 as i64) - ctx.r[11].s64;
	// 827DBED8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 827DBEDC: 41990008  bgt cr6, 0x827dbee4
	if ctx.cr[6].gt {
	pc = 0x827DBEE4; continue 'dispatch;
	}
	// 827DBEE0: 48614861  bl 0x82df0740
	ctx.lr = 0x827DBEE4;
	sub_82DF0740(ctx, base);
	// 827DBEE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DBEE8: 419A015C  beq cr6, 0x827dc044
	if ctx.cr[6].eq {
	pc = 0x827DC044; continue 'dispatch;
	}
	// 827DBEEC: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DBEF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DBEF4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DBEF8: 7F1F5A14  add r24, r31, r11
	ctx.r[24].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 827DBEFC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 827DBF00: 4BAED171  bl 0x822c9070
	ctx.lr = 0x827DBF04;
	sub_822C9070(ctx, base);
	// 827DBF04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DBF08: 4182013C  beq 0x827dc044
	if ctx.cr[0].eq {
	pc = 0x827DC044; continue 'dispatch;
	}
	// 827DBF0C: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DBF10: 3B5B0004  addi r26, r27, 4
	ctx.r[26].s64 = ctx.r[27].s64 + 4;
	// 827DBF14: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DBF18: 4198000C  blt cr6, 0x827dbf24
	if ctx.cr[6].lt {
	pc = 0x827DBF24; continue 'dispatch;
	}
	// 827DBF1C: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DBF20: 48000008  b 0x827dbf28
	pc = 0x827DBF28; continue 'dispatch;
	// 827DBF24: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 827DBF28: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DBF2C: 4198000C  blt cr6, 0x827dbf38
	if ctx.cr[6].lt {
	pc = 0x827DBF38; continue 'dispatch;
	}
	// 827DBF30: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DBF34: 48000008  b 0x827dbf3c
	pc = 0x827DBF3C; continue 'dispatch;
	// 827DBF38: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 827DBF3C: 7CFE5850  subf r7, r30, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 827DBF40: 811B0014  lwz r8, 0x14(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DBF44: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 827DBF48: 7D1E4050  subf r8, r30, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[30].s64;
	// 827DBF4C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DBF50: 7CFF3850  subf r7, r31, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[31].s64;
	// 827DBF54: 57DD083C  slwi r29, r30, 1
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 827DBF58: 5506083C  slwi r6, r8, 1
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 827DBF5C: 7CBD4A14  add r5, r29, r9
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[9].u64;
	// 827DBF60: 54E4083C  slwi r4, r7, 1
	ctx.r[4].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DBF64: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827DBF68: 489CCDA1  bl 0x831a8d08
	ctx.lr = 0x827DBF6C;
	sub_831A8D08(ctx, base);
	// 827DBF6C: 7F1BE040  cmplw cr6, r27, r28
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[28].u32, &mut ctx.xer);
	// 827DBF70: 409A0060  bne cr6, 0x827dbfd0
	if !ctx.cr[6].eq {
	pc = 0x827DBFD0; continue 'dispatch;
	}
	// 827DBF74: 7F1EC840  cmplw cr6, r30, r25
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[25].u32, &mut ctx.xer);
	// 827DBF78: 7D79FA14  add r11, r25, r31
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[31].u64;
	// 827DBF7C: 41980008  blt cr6, 0x827dbf84
	if ctx.cr[6].lt {
	pc = 0x827DBF84; continue 'dispatch;
	}
	// 827DBF80: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 827DBF84: 811B0018  lwz r8, 0x18(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DBF88: 2B080008  cmplwi cr6, r8, 8
	ctx.cr[6].compare_u32(ctx.r[8].u32, 8 as u32, &mut ctx.xer);
	// 827DBF8C: 4198000C  blt cr6, 0x827dbf98
	if ctx.cr[6].lt {
	pc = 0x827DBF98; continue 'dispatch;
	}
	// 827DBF90: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DBF94: 48000008  b 0x827dbf9c
	pc = 0x827DBF9C; continue 'dispatch;
	// 827DBF98: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 827DBF9C: 2B080008  cmplwi cr6, r8, 8
	ctx.cr[6].compare_u32(ctx.r[8].u32, 8 as u32, &mut ctx.xer);
	// 827DBFA0: 4198000C  blt cr6, 0x827dbfac
	if ctx.cr[6].lt {
	pc = 0x827DBFAC; continue 'dispatch;
	}
	// 827DBFA4: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DBFA8: 48000008  b 0x827dbfb0
	pc = 0x827DBFB0; continue 'dispatch;
	// 827DBFAC: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 827DBFB0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DBFB4: 7D1E4050  subf r8, r30, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[30].s64;
	// 827DBFB8: 57E6083C  slwi r6, r31, 1
	ctx.r[6].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 827DBFBC: 7CAB4A14  add r5, r11, r9
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 827DBFC0: 5504083C  slwi r4, r8, 1
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DBFC4: 7C7D5214  add r3, r29, r10
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 827DBFC8: 489CCD41  bl 0x831a8d08
	ctx.lr = 0x827DBFCC;
	sub_831A8D08(ctx, base);
	// 827DBFCC: 48000050  b 0x827dc01c
	pc = 0x827DC01C; continue 'dispatch;
	// 827DBFD0: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DBFD4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DBFD8: 4198000C  blt cr6, 0x827dbfe4
	if ctx.cr[6].lt {
	pc = 0x827DBFE4; continue 'dispatch;
	}
	// 827DBFDC: 811C0004  lwz r8, 4(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DBFE0: 48000008  b 0x827dbfe8
	pc = 0x827DBFE8; continue 'dispatch;
	// 827DBFE4: 391C0004  addi r8, r28, 4
	ctx.r[8].s64 = ctx.r[28].s64 + 4;
	// 827DBFE8: 813B0018  lwz r9, 0x18(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DBFEC: 2B090008  cmplwi cr6, r9, 8
	ctx.cr[6].compare_u32(ctx.r[9].u32, 8 as u32, &mut ctx.xer);
	// 827DBFF0: 4198000C  blt cr6, 0x827dbffc
	if ctx.cr[6].lt {
	pc = 0x827DBFFC; continue 'dispatch;
	}
	// 827DBFF4: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DBFF8: 48000008  b 0x827dc000
	pc = 0x827DC000; continue 'dispatch;
	// 827DBFFC: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 827DC000: 572B083C  slwi r11, r25, 1
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DC004: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 827DC008: 57E6083C  slwi r6, r31, 1
	ctx.r[6].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 827DC00C: 7CAB4214  add r5, r11, r8
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 827DC010: 5524083C  slwi r4, r9, 1
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DC014: 7C7D5214  add r3, r29, r10
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 827DC018: 489CCC19  bl 0x831a8c30
	ctx.lr = 0x827DC01C;
	sub_831A8C30(ctx, base);
	// 827DC01C: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DC020: 931B0014  stw r24, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[24].u32 ) };
	// 827DC024: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DC028: 4198000C  blt cr6, 0x827dc034
	if ctx.cr[6].lt {
	pc = 0x827DC034; continue 'dispatch;
	}
	// 827DC02C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC030: 48000008  b 0x827dc038
	pc = 0x827DC038; continue 'dispatch;
	// 827DC034: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 827DC038: 570A083C  slwi r10, r24, 1
	ctx.r[10].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DC03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827DC040: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 827DC044: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DC048: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827DC04C: 489CC15C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC050 size=372
    let mut pc: u32 = 0x827DC050;
    'dispatch: loop {
        match pc {
            0x827DC050 => {
    //   block [0x827DC050..0x827DC1C4)
	// 827DC050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC054: 489CC109  bl 0x831a815c
	ctx.lr = 0x827DC058;
	sub_831A8130(ctx, base);
	// 827DC058: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC05C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 827DC060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DC064: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 827DC068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DC06C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827DC070: 4BAECBF9  bl 0x822c8c68
	ctx.lr = 0x827DC074;
	sub_822C8C68(ctx, base);
	// 827DC074: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC078: 4182003C  beq 0x827dc0b4
	if ctx.cr[0].eq {
	pc = 0x827DC0B4; continue 'dispatch;
	}
	// 827DC07C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DC080: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DC084: 4198000C  blt cr6, 0x827dc090
	if ctx.cr[6].lt {
	pc = 0x827DC090; continue 'dispatch;
	}
	// 827DC088: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC08C: 48000008  b 0x827dc094
	pc = 0x827DC094; continue 'dispatch;
	// 827DC090: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827DC094: 7D6BC850  subf r11, r11, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[11].s64;
	// 827DC098: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 827DC09C: 7D660E70  srawi r6, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DC0A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827DC0A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DC0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC0AC: 4BFFFDD5  bl 0x827dbe80
	ctx.lr = 0x827DC0B0;
	sub_827DBE80(ctx, base);
	// 827DC0B0: 4800010C  b 0x827dc1bc
	pc = 0x827DC1BC; continue 'dispatch;
	// 827DC0B4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DC0B8: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DC0BC: 40980008  bge cr6, 0x827dc0c4
	if !ctx.cr[6].lt {
	pc = 0x827DC0C4; continue 'dispatch;
	}
	// 827DC0C0: 486147B9  bl 0x82df0878
	ctx.lr = 0x827DC0C4;
	sub_82DF0878(ctx, base);
	// 827DC0C4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DC0C8: 216BFFFF  subfic r11, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[11].s64 = (-1 as i64) - ctx.r[11].s64;
	// 827DC0CC: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 827DC0D0: 41990008  bgt cr6, 0x827dc0d8
	if ctx.cr[6].gt {
	pc = 0x827DC0D8; continue 'dispatch;
	}
	// 827DC0D4: 4861466D  bl 0x82df0740
	ctx.lr = 0x827DC0D8;
	sub_82DF0740(ctx, base);
	// 827DC0D8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827DC0DC: 419A00DC  beq cr6, 0x827dc1b8
	if ctx.cr[6].eq {
	pc = 0x827DC1B8; continue 'dispatch;
	}
	// 827DC0E0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DC0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DC0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC0EC: 7F4BE214  add r26, r11, r28
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 827DC0F0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827DC0F4: 4BAECF7D  bl 0x822c9070
	ctx.lr = 0x827DC0F8;
	sub_822C9070(ctx, base);
	// 827DC0F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC0FC: 418200BC  beq 0x827dc1b8
	if ctx.cr[0].eq {
	pc = 0x827DC1B8; continue 'dispatch;
	}
	// 827DC100: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DC104: 3B7F0004  addi r27, r31, 4
	ctx.r[27].s64 = ctx.r[31].s64 + 4;
	// 827DC108: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DC10C: 4198000C  blt cr6, 0x827dc118
	if ctx.cr[6].lt {
	pc = 0x827DC118; continue 'dispatch;
	}
	// 827DC110: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC114: 48000008  b 0x827dc11c
	pc = 0x827DC11C; continue 'dispatch;
	// 827DC118: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 827DC11C: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DC120: 4198000C  blt cr6, 0x827dc12c
	if ctx.cr[6].lt {
	pc = 0x827DC12C; continue 'dispatch;
	}
	// 827DC124: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC128: 48000008  b 0x827dc130
	pc = 0x827DC130; continue 'dispatch;
	// 827DC12C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 827DC130: 7CFE5850  subf r7, r30, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 827DC134: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DC138: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 827DC13C: 7D1E4050  subf r8, r30, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[30].s64;
	// 827DC140: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DC144: 7CFC3850  subf r7, r28, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[28].s64;
	// 827DC148: 57DD083C  slwi r29, r30, 1
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 827DC14C: 5506083C  slwi r6, r8, 1
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 827DC150: 7CBD4A14  add r5, r29, r9
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[9].u64;
	// 827DC154: 54E4083C  slwi r4, r7, 1
	ctx.r[4].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DC158: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827DC15C: 489CCBAD  bl 0x831a8d08
	ctx.lr = 0x827DC160;
	sub_831A8D08(ctx, base);
	// 827DC160: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DC164: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DC168: 4198000C  blt cr6, 0x827dc174
	if ctx.cr[6].lt {
	pc = 0x827DC174; continue 'dispatch;
	}
	// 827DC16C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC170: 48000008  b 0x827dc178
	pc = 0x827DC178; continue 'dispatch;
	// 827DC174: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 827DC178: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 827DC17C: 5786083C  slwi r6, r28, 1
	ctx.r[6].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 827DC180: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 827DC184: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DC188: 7C7D5214  add r3, r29, r10
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 827DC18C: 489CCAA5  bl 0x831a8c30
	ctx.lr = 0x827DC190;
	sub_831A8C30(ctx, base);
	// 827DC190: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DC194: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 827DC198: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 827DC19C: 4198000C  blt cr6, 0x827dc1a8
	if ctx.cr[6].lt {
	pc = 0x827DC1A8; continue 'dispatch;
	}
	// 827DC1A0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC1A4: 48000008  b 0x827dc1ac
	pc = 0x827DC1AC; continue 'dispatch;
	// 827DC1A8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 827DC1AC: 574A083C  slwi r10, r26, 1
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DC1B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827DC1B4: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 827DC1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC1BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DC1C0: 489CBFEC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC1C8 size=28
    let mut pc: u32 = 0x827DC1C8;
    'dispatch: loop {
        match pc {
            0x827DC1C8 => {
    //   block [0x827DC1C8..0x827DC1E4)
	// 827DC1C8: 89630100  lbz r11, 0x100(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 827DC1CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DC1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DC1D4: 41820010  beq 0x827dc1e4
	if ctx.cr[0].eq {
		sub_827DC1E4(ctx, base);
		return;
	}
	// 827DC1D8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DC1DC: 916AA0AC  stw r11, -0x5f54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24404 as u32), ctx.r[11].u32 ) };
	// 827DC1E0: 4800000C  b 0x827dc1ec
	sub_827DC1E4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC1E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC1E4 size=16
    let mut pc: u32 = 0x827DC1E4;
    'dispatch: loop {
        match pc {
            0x827DC1E4 => {
    //   block [0x827DC1E4..0x827DC1F4)
	// 827DC1E4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DC1E8: 916AA0B0  stw r11, -0x5f50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24400 as u32), ctx.r[11].u32 ) };
	// 827DC1EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827DC1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC1F8 size=12
    let mut pc: u32 = 0x827DC1F8;
    'dispatch: loop {
        match pc {
            0x827DC1F8 => {
    //   block [0x827DC1F8..0x827DC204)
	// 827DC1F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC1FC: 988BA0B4  stb r4, -0x5f4c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-24396 as u32), ctx.r[4].u8 ) };
	// 827DC200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC208 size=32
    let mut pc: u32 = 0x827DC208;
    'dispatch: loop {
        match pc {
            0x827DC208 => {
    //   block [0x827DC208..0x827DC228)
	// 827DC208: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC20C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DC210: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 827DC214: 816BB84C  lwz r11, -0x47b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18356 as u32) ) } as u64;
	// 827DC218: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 827DC21C: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 827DC220: 9169B864  stw r11, -0x479c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18332 as u32), ctx.r[11].u32 ) };
	// 827DC224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC228 size=12
    let mut pc: u32 = 0x827DC228;
    'dispatch: loop {
        match pc {
            0x827DC228 => {
    //   block [0x827DC228..0x827DC234)
	// 827DC228: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC22C: 988BB861  stb r4, -0x479f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-18335 as u32), ctx.r[4].u8 ) };
	// 827DC230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC238 size=24
    let mut pc: u32 = 0x827DC238;
    'dispatch: loop {
        match pc {
            0x827DC238 => {
    //   block [0x827DC238..0x827DC250)
	// 827DC238: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827DC23C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC240: 409A0010  bne cr6, 0x827dc250
	if !ctx.cr[6].eq {
		sub_827DC250(ctx, base);
		return;
	}
	// 827DC244: 396B9F48  addi r11, r11, -0x60b8
	ctx.r[11].s64 = ctx.r[11].s64 + -24760;
	// 827DC248: 806B0154  lwz r3, 0x154(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 827DC24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC250 size=52
    let mut pc: u32 = 0x827DC250;
    'dispatch: loop {
        match pc {
            0x827DC250 => {
    //   block [0x827DC250..0x827DC284)
	// 827DC250: 89440030  lbz r10, 0x30(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DC254: 392B9F48  addi r9, r11, -0x60b8
	ctx.r[9].s64 = ctx.r[11].s64 + -24760;
	// 827DC258: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DC25C: 41820034  beq 0x827dc290
	if ctx.cr[0].eq {
		sub_827DC290(ctx, base);
		return;
	}
	// 827DC260: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 827DC264: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC268: 41980074  blt cr6, 0x827dc2dc
	if ctx.cr[6].lt {
		sub_827DC2DC(ctx, base);
		return;
	}
	// 827DC26C: 8144002C  lwz r10, 0x2c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 827DC270: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DC274: 41980010  blt cr6, 0x827dc284
	if ctx.cr[6].lt {
		sub_827DC284(ctx, base);
		return;
	}
	// 827DC278: 1D4A0058  mulli r10, r10, 0x58
	ctx.r[10].s64 = ctx.r[10].s64 * 88;
	// 827DC27C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827DC280: 48000028  b 0x827dc2a8
	sub_827DC290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC284 size=12
    let mut pc: u32 = 0x827DC284;
    'dispatch: loop {
        match pc {
            0x827DC284 => {
    //   block [0x827DC284..0x827DC290)
	// 827DC284: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DC288: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 827DC28C: 4800002C  b 0x827dc2b8
	sub_827DC290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC290 size=48
    let mut pc: u32 = 0x827DC290;
    'dispatch: loop {
        match pc {
            0x827DC290 => {
    //   block [0x827DC290..0x827DC2C0)
	// 827DC290: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 827DC294: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC298: 41980044  blt cr6, 0x827dc2dc
	if ctx.cr[6].lt {
		sub_827DC2DC(ctx, base);
		return;
	}
	// 827DC29C: 81440028  lwz r10, 0x28(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 827DC2A0: 1D6B0058  mulli r11, r11, 0x58
	ctx.r[11].s64 = ctx.r[11].s64 * 88;
	// 827DC2A4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827DC2A8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DC2AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DC2B0: 394A87E8  addi r10, r10, -0x7818
	ctx.r[10].s64 = ctx.r[10].s64 + -30744;
	// 827DC2B4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 827DC2B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DC2BC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC2C0 size=20
    let mut pc: u32 = 0x827DC2C0;
    'dispatch: loop {
        match pc {
            0x827DC2C0 => {
    //   block [0x827DC2C0..0x827DC2D4)
	// 827DC2C0: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DC2C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC2C8: 419A000C  beq cr6, 0x827dc2d4
	if ctx.cr[6].eq {
		sub_827DC2D4(ctx, base);
		return;
	}
	// 827DC2CC: 806B0038  lwz r3, 0x38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 827DC2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC2D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC2D4 size=8
    let mut pc: u32 = 0x827DC2D4;
    'dispatch: loop {
        match pc {
            0x827DC2D4 => {
    //   block [0x827DC2D4..0x827DC2DC)
	// 827DC2D4: 80690154  lwz r3, 0x154(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(340 as u32) ) } as u64;
	// 827DC2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC2DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC2DC size=8
    let mut pc: u32 = 0x827DC2DC;
    'dispatch: loop {
        match pc {
            0x827DC2DC => {
    //   block [0x827DC2DC..0x827DC2E4)
	// 827DC2DC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827DC2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC2E8 size=12
    let mut pc: u32 = 0x827DC2E8;
    'dispatch: loop {
        match pc {
            0x827DC2E8 => {
    //   block [0x827DC2E8..0x827DC2F4)
	// 827DC2E8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC2EC: 988BA0B5  stb r4, -0x5f4b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-24395 as u32), ctx.r[4].u8 ) };
	// 827DC2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC2F8 size=12
    let mut pc: u32 = 0x827DC2F8;
    'dispatch: loop {
        match pc {
            0x827DC2F8 => {
    //   block [0x827DC2F8..0x827DC304)
	// 827DC2F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC2FC: 988BA0B6  stb r4, -0x5f4a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-24394 as u32), ctx.r[4].u8 ) };
	// 827DC300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC308 size=4
    let mut pc: u32 = 0x827DC308;
    'dispatch: loop {
        match pc {
            0x827DC308 => {
    //   block [0x827DC308..0x827DC30C)
	// 827DC308: 48000014  b 0x827dc31c
	sub_827DC30C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC30C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC30C size=28
    let mut pc: u32 = 0x827DC30C;
    'dispatch: loop {
        match pc {
            0x827DC30C => {
    //   block [0x827DC30C..0x827DC328)
	// 827DC30C: 89640030  lbz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DC310: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DC314: 41820014  beq 0x827dc328
	if ctx.cr[0].eq {
		sub_827DC328(ctx, base);
		return;
	}
	// 827DC318: 80840020  lwz r4, 0x20(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DC31C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827DC320: 409AFFEC  bne cr6, 0x827dc30c
	if !ctx.cr[6].eq {
	pc = 0x827DC30C; continue 'dispatch;
	}
	// 827DC324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC328 size=16
    let mut pc: u32 = 0x827DC328;
    'dispatch: loop {
        match pc {
            0x827DC328 => {
    //   block [0x827DC328..0x827DC338)
	// 827DC328: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC32C: 8084002C  lwz r4, 0x2c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 827DC330: 806BA104  lwz r3, -0x5efc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24316 as u32) ) } as u64;
	// 827DC334: 4800A07C  b 0x827e63b0
	sub_827E63B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC338 size=132
    let mut pc: u32 = 0x827DC338;
    'dispatch: loop {
        match pc {
            0x827DC338 => {
    //   block [0x827DC338..0x827DC3BC)
	// 827DC338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC33C: 489CBE31  bl 0x831a816c
	ctx.lr = 0x827DC340;
	sub_831A8130(ctx, base);
	// 827DC340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DC348: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DC34C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DC350: 48616DA1  bl 0x82df30f0
	ctx.lr = 0x827DC354;
	sub_82DF30F0(ctx, base);
	// 827DC354: 80BD0020  lwz r5, 0x20(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DC358: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 827DC35C: 419A0034  beq cr6, 0x827dc390
	if ctx.cr[6].eq {
	pc = 0x827DC390; continue 'dispatch;
	}
	// 827DC360: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DC364: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DC368: 4BFFFFD1  bl 0x827dc338
	ctx.lr = 0x827DC36C;
	sub_827DC338(ctx, base);
	// 827DC36C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DC370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC374: 4861785D  bl 0x82df3bd0
	ctx.lr = 0x827DC378;
	sub_82DF3BD0(ctx, base);
	// 827DC378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DC37C: 486170AD  bl 0x82df3428
	ctx.lr = 0x827DC380;
	sub_82DF3428(ctx, base);
	// 827DC380: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 827DC384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC388: 388BD398  addi r4, r11, -0x2c68
	ctx.r[4].s64 = ctx.r[11].s64 + -11368;
	// 827DC38C: 486171ED  bl 0x82df3578
	ctx.lr = 0x827DC390;
	sub_82DF3578(ctx, base);
	// 827DC390: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DC394: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DC398: 4861CFB9  bl 0x82df9350
	ctx.lr = 0x827DC39C;
	sub_82DF9350(ctx, base);
	// 827DC39C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DC3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC3A4: 48617335  bl 0x82df36d8
	ctx.lr = 0x827DC3A8;
	sub_82DF36D8(ctx, base);
	// 827DC3A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DC3AC: 4861707D  bl 0x82df3428
	ctx.lr = 0x827DC3B0;
	sub_82DF3428(ctx, base);
	// 827DC3B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DC3B8: 489CBE04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC3C0 size=12
    let mut pc: u32 = 0x827DC3C0;
    'dispatch: loop {
        match pc {
            0x827DC3C0 => {
    //   block [0x827DC3C0..0x827DC3CC)
	// 827DC3C0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC3C4: 988BB862  stb r4, -0x479e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-18334 as u32), ctx.r[4].u8 ) };
	// 827DC3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC3D0 size=136
    let mut pc: u32 = 0x827DC3D0;
    'dispatch: loop {
        match pc {
            0x827DC3D0 => {
    //   block [0x827DC3D0..0x827DC458)
	// 827DC3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC3D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DC3DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DC3E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC3E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DC3E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DC3EC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827DC3F0: 409A0020  bne cr6, 0x827dc410
	if !ctx.cr[6].eq {
	pc = 0x827DC410; continue 'dispatch;
	}
	// 827DC3F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DC3F8: 419A0048  beq cr6, 0x827dc440
	if ctx.cr[6].eq {
	pc = 0x827DC440; continue 'dispatch;
	}
	// 827DC3FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC400: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC404: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC408: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DC40C: 48000034  b 0x827dc440
	pc = 0x827DC440; continue 'dispatch;
	// 827DC410: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827DC414: 419A002C  beq cr6, 0x827dc440
	if ctx.cr[6].eq {
	pc = 0x827DC440; continue 'dispatch;
	}
	// 827DC418: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC41C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC420: 388BB990  addi r4, r11, -0x4670
	ctx.r[4].s64 = ctx.r[11].s64 + -18032;
	// 827DC424: 489CBCD5  bl 0x831a80f8
	ctx.lr = 0x827DC428;
	sub_831A80F8(ctx, base);
	// 827DC428: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC42C: 4182000C  beq 0x827dc438
	if ctx.cr[0].eq {
	pc = 0x827DC438; continue 'dispatch;
	}
	// 827DC430: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DC434: 4800000C  b 0x827dc440
	pc = 0x827DC440; continue 'dispatch;
	// 827DC438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DC43C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC440: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DC444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC44C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DC450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DC454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC458 size=136
    let mut pc: u32 = 0x827DC458;
    'dispatch: loop {
        match pc {
            0x827DC458 => {
    //   block [0x827DC458..0x827DC4E0)
	// 827DC458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DC464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DC468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC46C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DC470: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DC474: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827DC478: 409A0020  bne cr6, 0x827dc498
	if !ctx.cr[6].eq {
	pc = 0x827DC498; continue 'dispatch;
	}
	// 827DC47C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DC480: 419A0048  beq cr6, 0x827dc4c8
	if ctx.cr[6].eq {
	pc = 0x827DC4C8; continue 'dispatch;
	}
	// 827DC484: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC488: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC48C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC490: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DC494: 48000034  b 0x827dc4c8
	pc = 0x827DC4C8; continue 'dispatch;
	// 827DC498: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827DC49C: 419A002C  beq cr6, 0x827dc4c8
	if ctx.cr[6].eq {
	pc = 0x827DC4C8; continue 'dispatch;
	}
	// 827DC4A0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC4A4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC4A8: 388BBA78  addi r4, r11, -0x4588
	ctx.r[4].s64 = ctx.r[11].s64 + -17800;
	// 827DC4AC: 489CBC4D  bl 0x831a80f8
	ctx.lr = 0x827DC4B0;
	sub_831A80F8(ctx, base);
	// 827DC4B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC4B4: 4182000C  beq 0x827dc4c0
	if ctx.cr[0].eq {
	pc = 0x827DC4C0; continue 'dispatch;
	}
	// 827DC4B8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DC4BC: 4800000C  b 0x827dc4c8
	pc = 0x827DC4C8; continue 'dispatch;
	// 827DC4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DC4C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC4C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DC4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC4D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DC4D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DC4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC4E0 size=136
    let mut pc: u32 = 0x827DC4E0;
    'dispatch: loop {
        match pc {
            0x827DC4E0 => {
    //   block [0x827DC4E0..0x827DC568)
	// 827DC4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC4E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DC4EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DC4F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC4F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DC4F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DC4FC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827DC500: 409A0020  bne cr6, 0x827dc520
	if !ctx.cr[6].eq {
	pc = 0x827DC520; continue 'dispatch;
	}
	// 827DC504: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DC508: 419A0048  beq cr6, 0x827dc550
	if ctx.cr[6].eq {
	pc = 0x827DC550; continue 'dispatch;
	}
	// 827DC50C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC510: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC514: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC518: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DC51C: 48000034  b 0x827dc550
	pc = 0x827DC550; continue 'dispatch;
	// 827DC520: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827DC524: 419A002C  beq cr6, 0x827dc550
	if ctx.cr[6].eq {
	pc = 0x827DC550; continue 'dispatch;
	}
	// 827DC528: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC52C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC530: 388BBB20  addi r4, r11, -0x44e0
	ctx.r[4].s64 = ctx.r[11].s64 + -17632;
	// 827DC534: 489CBBC5  bl 0x831a80f8
	ctx.lr = 0x827DC538;
	sub_831A80F8(ctx, base);
	// 827DC538: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC53C: 4182000C  beq 0x827dc548
	if ctx.cr[0].eq {
	pc = 0x827DC548; continue 'dispatch;
	}
	// 827DC540: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DC544: 4800000C  b 0x827dc550
	pc = 0x827DC550; continue 'dispatch;
	// 827DC548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DC54C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC550: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DC554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC55C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DC560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DC564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC568 size=136
    let mut pc: u32 = 0x827DC568;
    'dispatch: loop {
        match pc {
            0x827DC568 => {
    //   block [0x827DC568..0x827DC5F0)
	// 827DC568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC570: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DC574: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DC578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC57C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DC580: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DC584: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827DC588: 409A0020  bne cr6, 0x827dc5a8
	if !ctx.cr[6].eq {
	pc = 0x827DC5A8; continue 'dispatch;
	}
	// 827DC58C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DC590: 419A0048  beq cr6, 0x827dc5d8
	if ctx.cr[6].eq {
	pc = 0x827DC5D8; continue 'dispatch;
	}
	// 827DC594: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC598: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC59C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC5A0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DC5A4: 48000034  b 0x827dc5d8
	pc = 0x827DC5D8; continue 'dispatch;
	// 827DC5A8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827DC5AC: 419A002C  beq cr6, 0x827dc5d8
	if ctx.cr[6].eq {
	pc = 0x827DC5D8; continue 'dispatch;
	}
	// 827DC5B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC5B4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC5B8: 388BBBC8  addi r4, r11, -0x4438
	ctx.r[4].s64 = ctx.r[11].s64 + -17464;
	// 827DC5BC: 489CBB3D  bl 0x831a80f8
	ctx.lr = 0x827DC5C0;
	sub_831A80F8(ctx, base);
	// 827DC5C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC5C4: 4182000C  beq 0x827dc5d0
	if ctx.cr[0].eq {
	pc = 0x827DC5D0; continue 'dispatch;
	}
	// 827DC5C8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DC5CC: 4800000C  b 0x827dc5d8
	pc = 0x827DC5D8; continue 'dispatch;
	// 827DC5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DC5D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC5D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DC5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC5E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DC5E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DC5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC5F0 size=16
    let mut pc: u32 = 0x827DC5F0;
    'dispatch: loop {
        match pc {
            0x827DC5F0 => {
    //   block [0x827DC5F0..0x827DC600)
	// 827DC5F0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC5F4: 816BA0AC  lwz r11, -0x5f54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24404 as u32) ) } as u64;
	// 827DC5F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC5FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC600 size=12
    let mut pc: u32 = 0x827DC600;
    'dispatch: loop {
        match pc {
            0x827DC600 => {
    //   block [0x827DC600..0x827DC60C)
	// 827DC600: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DC604: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 827DC608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC610 size=32
    let mut pc: u32 = 0x827DC610;
    'dispatch: loop {
        match pc {
            0x827DC610 => {
    //   block [0x827DC610..0x827DC630)
	// 827DC610: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC614: 816BA0AC  lwz r11, -0x5f54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24404 as u32) ) } as u64;
	// 827DC618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC61C: 419A0014  beq cr6, 0x827dc630
	if ctx.cr[6].eq {
		sub_827DC630(ctx, base);
		return;
	}
	// 827DC620: 896B0014  lbz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DC624: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827DC628: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827DC62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC630 size=8
    let mut pc: u32 = 0x827DC630;
    'dispatch: loop {
        match pc {
            0x827DC630 => {
    //   block [0x827DC630..0x827DC638)
	// 827DC630: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827DC634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC638 size=16
    let mut pc: u32 = 0x827DC638;
    'dispatch: loop {
        match pc {
            0x827DC638 => {
    //   block [0x827DC638..0x827DC648)
	// 827DC638: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC63C: 816BA0B0  lwz r11, -0x5f50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24400 as u32) ) } as u64;
	// 827DC640: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC644: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC648 size=12
    let mut pc: u32 = 0x827DC648;
    'dispatch: loop {
        match pc {
            0x827DC648 => {
    //   block [0x827DC648..0x827DC654)
	// 827DC648: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DC64C: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 827DC650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC658 size=32
    let mut pc: u32 = 0x827DC658;
    'dispatch: loop {
        match pc {
            0x827DC658 => {
    //   block [0x827DC658..0x827DC678)
	// 827DC658: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DC65C: 816BA0B0  lwz r11, -0x5f50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24400 as u32) ) } as u64;
	// 827DC660: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC664: 419A0014  beq cr6, 0x827dc678
	if ctx.cr[6].eq {
		sub_827DC678(ctx, base);
		return;
	}
	// 827DC668: 896B0014  lbz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DC66C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827DC670: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827DC674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC678 size=8
    let mut pc: u32 = 0x827DC678;
    'dispatch: loop {
        match pc {
            0x827DC678 => {
    //   block [0x827DC678..0x827DC680)
	// 827DC678: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827DC67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC680 size=20
    let mut pc: u32 = 0x827DC680;
    'dispatch: loop {
        match pc {
            0x827DC680 => {
    //   block [0x827DC680..0x827DC694)
	// 827DC680: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC684: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC688: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DC68C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DC690: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC694 size=44
    let mut pc: u32 = 0x827DC694;
    'dispatch: loop {
        match pc {
            0x827DC694 => {
    //   block [0x827DC694..0x827DC6C0)
	// 827DC694: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC698: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DC69C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DC6A0: 41990020  bgt cr6, 0x827dc6c0
	if ctx.cr[6].gt {
		sub_827DC6C0(ctx, base);
		return;
	}
	// 827DC6A4: 41980014  blt cr6, 0x827dc6b8
	if ctx.cr[6].lt {
	pc = 0x827DC6B8; continue 'dispatch;
	}
	// 827DC6A8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DC6AC: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC6B0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 827DC6B4: 4198000C  blt cr6, 0x827dc6c0
	if ctx.cr[6].lt {
		sub_827DC6C0(ctx, base);
		return;
	}
	// 827DC6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DC6BC: 48000008  b 0x827dc6c4
	sub_827DC6C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC6C0 size=20
    let mut pc: u32 = 0x827DC6C0;
    'dispatch: loop {
        match pc {
            0x827DC6C0 => {
    //   block [0x827DC6C0..0x827DC6D4)
	// 827DC6C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DC6C4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DC6C8: 4182000C  beq 0x827dc6d4
	if ctx.cr[0].eq {
		sub_827DC6D4(ctx, base);
		return;
	}
	// 827DC6CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DC6D0: 4800000C  b 0x827dc6dc
	sub_827DC6D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC6D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DC6D4 size=24
    let mut pc: u32 = 0x827DC6D4;
    'dispatch: loop {
        match pc {
            0x827DC6D4 => {
    //   block [0x827DC6D4..0x827DC6EC)
	// 827DC6D4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827DC6D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC6DC: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DC6E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DC6E4: 419AFFB4  beq cr6, 0x827dc698
	if ctx.cr[6].eq {
		sub_827DC694(ctx, base);
		return;
	}
	// 827DC6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC6F0 size=196
    let mut pc: u32 = 0x827DC6F0;
    'dispatch: loop {
        match pc {
            0x827DC6F0 => {
    //   block [0x827DC6F0..0x827DC7B4)
	// 827DC6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC6F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DC6FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DC700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC704: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DC708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DC70C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827DC710: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DC714: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC718: 4BAE4221  bl 0x822c0938
	ctx.lr = 0x827DC71C;
	sub_822C0938(ctx, base);
	// 827DC71C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DC720: 41820028  beq 0x827dc748
	if ctx.cr[0].eq {
	pc = 0x827DC748; continue 'dispatch;
	}
	// 827DC724: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DC728: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827DC72C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DC730: 392B558C  addi r9, r11, 0x558c
	ctx.r[9].s64 = ctx.r[11].s64 + 21900;
	// 827DC734: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DC738: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DC73C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DC740: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DC744: 48000008  b 0x827dc74c
	pc = 0x827DC74C; continue 'dispatch;
	// 827DC748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DC74C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC754: 409A0044  bne cr6, 0x827dc798
	if !ctx.cr[6].eq {
	pc = 0x827DC798; continue 'dispatch;
	}
	// 827DC758: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DC75C: 419A001C  beq cr6, 0x827dc778
	if ctx.cr[6].eq {
	pc = 0x827DC778; continue 'dispatch;
	}
	// 827DC760: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC764: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DC768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC76C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DC774: 4E800421  bctrl
	ctx.lr = 0x827DC778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DC778: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC77C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827DC780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DC784: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827DC788: 816BB86C  lwz r11, -0x4794(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18324 as u32) ) } as u64;
	// 827DC78C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827DC790: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827DC794: 4BAE386D  bl 0x822c0000
	ctx.lr = 0x827DC798;
	sub_822C0000(ctx, base);
	// 827DC798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DC79C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DC7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC7A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DC7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DC7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC7B8 size=88
    let mut pc: u32 = 0x827DC7B8;
    'dispatch: loop {
        match pc {
            0x827DC7B8 => {
    //   block [0x827DC7B8..0x827DC810)
	// 827DC7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC7C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC7C4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 827DC7C8: 486156E1  bl 0x82df1ea8
	ctx.lr = 0x827DC7CC;
	sub_82DF1EA8(ctx, base);
	// 827DC7CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DC7D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DC7D4: 41820008  beq 0x827dc7dc
	if ctx.cr[0].eq {
	pc = 0x827DC7DC; continue 'dispatch;
	}
	// 827DC7D8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DC7DC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC7E0: 41820008  beq 0x827dc7e8
	if ctx.cr[0].eq {
	pc = 0x827DC7E8; continue 'dispatch;
	}
	// 827DC7E4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DC7E8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DC7EC: 41820008  beq 0x827dc7f4
	if ctx.cr[0].eq {
	pc = 0x827DC7F4; continue 'dispatch;
	}
	// 827DC7F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DC7F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DC7F8: 99430019  stb r10, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 827DC7FC: 99630018  stb r11, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 827DC800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DC804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC810 size=72
    let mut pc: u32 = 0x827DC810;
    'dispatch: loop {
        match pc {
            0x827DC810 => {
    //   block [0x827DC810..0x827DC858)
	// 827DC810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC81C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827DC820: 419A001C  beq cr6, 0x827dc83c
	if ctx.cr[6].eq {
	pc = 0x827DC83C; continue 'dispatch;
	}
	// 827DC824: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DC828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DC82C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827DC830: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DC834: 4BFFFB9D  bl 0x827dc3d0
	ctx.lr = 0x827DC838;
	sub_827DC3D0(ctx, base);
	// 827DC838: 48000010  b 0x827dc848
	pc = 0x827DC848; continue 'dispatch;
	// 827DC83C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC840: 396BB990  addi r11, r11, -0x4670
	ctx.r[11].s64 = ctx.r[11].s64 + -18032;
	// 827DC844: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DC84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC858 size=72
    let mut pc: u32 = 0x827DC858;
    'dispatch: loop {
        match pc {
            0x827DC858 => {
    //   block [0x827DC858..0x827DC8A0)
	// 827DC858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC864: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827DC868: 419A001C  beq cr6, 0x827dc884
	if ctx.cr[6].eq {
	pc = 0x827DC884; continue 'dispatch;
	}
	// 827DC86C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DC870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DC874: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827DC878: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DC87C: 4BFFFBDD  bl 0x827dc458
	ctx.lr = 0x827DC880;
	sub_827DC458(ctx, base);
	// 827DC880: 48000010  b 0x827dc890
	pc = 0x827DC890; continue 'dispatch;
	// 827DC884: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC888: 396BBA78  addi r11, r11, -0x4588
	ctx.r[11].s64 = ctx.r[11].s64 + -17800;
	// 827DC88C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DC894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC8A0 size=72
    let mut pc: u32 = 0x827DC8A0;
    'dispatch: loop {
        match pc {
            0x827DC8A0 => {
    //   block [0x827DC8A0..0x827DC8E8)
	// 827DC8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC8AC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827DC8B0: 419A001C  beq cr6, 0x827dc8cc
	if ctx.cr[6].eq {
	pc = 0x827DC8CC; continue 'dispatch;
	}
	// 827DC8B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DC8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DC8BC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827DC8C0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DC8C4: 4BFFFC1D  bl 0x827dc4e0
	ctx.lr = 0x827DC8C8;
	sub_827DC4E0(ctx, base);
	// 827DC8C8: 48000010  b 0x827dc8d8
	pc = 0x827DC8D8; continue 'dispatch;
	// 827DC8CC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC8D0: 396BBB20  addi r11, r11, -0x44e0
	ctx.r[11].s64 = ctx.r[11].s64 + -17632;
	// 827DC8D4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DC8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC8E8 size=72
    let mut pc: u32 = 0x827DC8E8;
    'dispatch: loop {
        match pc {
            0x827DC8E8 => {
    //   block [0x827DC8E8..0x827DC930)
	// 827DC8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC8F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC8F4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827DC8F8: 419A001C  beq cr6, 0x827dc914
	if ctx.cr[6].eq {
	pc = 0x827DC914; continue 'dispatch;
	}
	// 827DC8FC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DC900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DC904: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827DC908: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DC90C: 4BFFFC5D  bl 0x827dc568
	ctx.lr = 0x827DC910;
	sub_827DC568(ctx, base);
	// 827DC910: 48000010  b 0x827dc920
	pc = 0x827DC920; continue 'dispatch;
	// 827DC914: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DC918: 396BBBC8  addi r11, r11, -0x4438
	ctx.r[11].s64 = ctx.r[11].s64 + -17464;
	// 827DC91C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DC920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DC924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC930 size=148
    let mut pc: u32 = 0x827DC930;
    'dispatch: loop {
        match pc {
            0x827DC930 => {
    //   block [0x827DC930..0x827DC9C4)
	// 827DC930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DC93C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC940: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827DC944: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 827DC948: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827DC94C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC950: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC954: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC958: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827DC95C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 827DC960: 419A0024  beq cr6, 0x827dc984
	if ctx.cr[6].eq {
	pc = 0x827DC984; continue 'dispatch;
	}
	// 827DC964: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DC968: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 827DC96C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DC970: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 827DC974: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 827DC978: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DC97C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DC980: 4082FFE8  bne 0x827dc968
	if !ctx.cr[0].eq {
	pc = 0x827DC968; continue 'dispatch;
	}
	// 827DC984: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC988: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DC98C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DC990: 4E800421  bctrl
	ctx.lr = 0x827DC994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DC994: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DC99C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC9A0: 419A000C  beq cr6, 0x827dc9ac
	if ctx.cr[6].eq {
	pc = 0x827DC9AC; continue 'dispatch;
	}
	// 827DC9A4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827DC9A8: 4BAE3EE9  bl 0x822c0890
	ctx.lr = 0x827DC9AC;
	sub_822C0890(ctx, base);
	// 827DC9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DC9B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DC9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DC9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DC9BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DC9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DC9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DC9C8 size=112
    let mut pc: u32 = 0x827DC9C8;
    'dispatch: loop {
        match pc {
            0x827DC9C8 => {
    //   block [0x827DC9C8..0x827DCA38)
	// 827DC9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DC9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DC9D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DC9D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DC9D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DC9DC: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DC9E0: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DC9E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DC9E8: 419A0034  beq cr6, 0x827dca1c
	if ctx.cr[6].eq {
	pc = 0x827DCA1C; continue 'dispatch;
	}
	// 827DC9EC: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DC9F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DC9F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DC9F8: 4800001C  b 0x827dca14
	pc = 0x827DCA14; continue 'dispatch;
	// 827DC9FC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DCA00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DCA04: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCA08: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827DCA0C: 4BBC4C7D  bl 0x823a1688
	ctx.lr = 0x827DCA10;
	sub_823A1688(ctx, base);
	// 827DCA10: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DCA14: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 827DCA18: 409AFFE4  bne cr6, 0x827dc9fc
	if !ctx.cr[6].eq {
	pc = 0x827DC9FC; continue 'dispatch;
	}
	// 827DCA1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DCA20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DCA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCA2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DCA30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCA38 size=112
    let mut pc: u32 = 0x827DCA38;
    'dispatch: loop {
        match pc {
            0x827DCA38 => {
    //   block [0x827DCA38..0x827DCAA8)
	// 827DCA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCA40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DCA44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCA48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCA4C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DCA50: 83C30008  lwz r30, 8(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DCA54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DCA58: 419A0034  beq cr6, 0x827dca8c
	if ctx.cr[6].eq {
	pc = 0x827DCA8C; continue 'dispatch;
	}
	// 827DCA5C: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DCA60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCA64: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DCA68: 4800001C  b 0x827dca84
	pc = 0x827DCA84; continue 'dispatch;
	// 827DCA6C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DCA70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DCA74: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DCA78: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827DCA7C: 4BBC4C0D  bl 0x823a1688
	ctx.lr = 0x827DCA80;
	sub_823A1688(ctx, base);
	// 827DCA80: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DCA84: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 827DCA88: 409AFFE4  bne cr6, 0x827dca6c
	if !ctx.cr[6].eq {
	pc = 0x827DCA6C; continue 'dispatch;
	}
	// 827DCA8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DCA90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DCA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCA9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DCAA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCAA8 size=160
    let mut pc: u32 = 0x827DCAA8;
    'dispatch: loop {
        match pc {
            0x827DCAA8 => {
    //   block [0x827DCAA8..0x827DCB48)
	// 827DCAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCAB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DCAB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCAB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCABC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DCAC0: 83FE00A4  lwz r31, 0xa4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) } as u64;
	// 827DCAC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCAC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DCACC: 48000028  b 0x827dcaf4
	pc = 0x827DCAF4; continue 'dispatch;
	// 827DCAD0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DCAD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DCAD8: 812A003C  lwz r9, 0x3c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCADC: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 827DCAE0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DCAE4: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCAE8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827DCAEC: 4BBC4B9D  bl 0x823a1688
	ctx.lr = 0x827DCAF0;
	sub_823A1688(ctx, base);
	// 827DCAF0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DCAF4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 827DCAF8: 409AFFD8  bne cr6, 0x827dcad0
	if !ctx.cr[6].eq {
	pc = 0x827DCAD0; continue 'dispatch;
	}
	// 827DCAFC: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 827DCB00: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827DCB04: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCB08: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 827DCB0C: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 827DCB10: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCB14: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827DCB18: 817E00B4  lwz r11, 0xb4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DCB1C: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCB20: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 827DCB24: 817E00B4  lwz r11, 0xb4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DCB28: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCB2C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827DCB30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DCB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCB3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DCB40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCB48 size=80
    let mut pc: u32 = 0x827DCB48;
    'dispatch: loop {
        match pc {
            0x827DCB48 => {
    //   block [0x827DCB48..0x827DCB98)
	// 827DCB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCB50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCB54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCB58: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 827DCB5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DCB60: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 827DCB64: 916AB868  stw r11, -0x4798(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18328 as u32), ctx.r[11].u32 ) };
	// 827DCB68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCB6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DCB70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DCB74: 4E800421  bctrl
	ctx.lr = 0x827DCB78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DCB78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DCB7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DCB80: 4BFFFF29  bl 0x827dcaa8
	ctx.lr = 0x827DCB84;
	sub_827DCAA8(ctx, base);
	// 827DCB84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DCB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCB90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCB98 size=140
    let mut pc: u32 = 0x827DCB98;
    'dispatch: loop {
        match pc {
            0x827DCB98 => {
    //   block [0x827DCB98..0x827DCC24)
	// 827DCB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCBA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DCBA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCBA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCBAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DCBB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DCBB4: 897F0030  lbz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DCBB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DCBBC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DCBC0: 806BA104  lwz r3, -0x5efc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24316 as u32) ) } as u64;
	// 827DCBC4: 41820010  beq 0x827dcbd4
	if ctx.cr[0].eq {
	pc = 0x827DCBD4; continue 'dispatch;
	}
	// 827DCBC8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 827DCBCC: 480097B5  bl 0x827e6380
	ctx.lr = 0x827DCBD0;
	sub_827E6380(ctx, base);
	// 827DCBD0: 4800000C  b 0x827dcbdc
	pc = 0x827DCBDC; continue 'dispatch;
	// 827DCBD4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 827DCBD8: 480097D9  bl 0x827e63b0
	ctx.lr = 0x827DCBDC;
	sub_827E63B0(ctx, base);
	// 827DCBDC: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DCBE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCBE4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DCBE8: 4800001C  b 0x827dcc04
	pc = 0x827DCC04; continue 'dispatch;
	// 827DCBEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DCBF0: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DCBF4: 4BFFFFA5  bl 0x827dcb98
	ctx.lr = 0x827DCBF8;
	sub_827DCB98(ctx, base);
	// 827DCBF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DCBFC: 4BBC4A8D  bl 0x823a1688
	ctx.lr = 0x827DCC00;
	sub_823A1688(ctx, base);
	// 827DCC00: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DCC04: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 827DCC08: 409AFFE4  bne cr6, 0x827dcbec
	if !ctx.cr[6].eq {
	pc = 0x827DCBEC; continue 'dispatch;
	}
	// 827DCC0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DCC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCC18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DCC1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCC28 size=88
    let mut pc: u32 = 0x827DCC28;
    'dispatch: loop {
        match pc {
            0x827DCC28 => {
    //   block [0x827DCC28..0x827DCC80)
	// 827DCC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCC30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCC34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCC38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DCC3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DCC40: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 827DCC44: 394B0110  addi r10, r11, 0x110
	ctx.r[10].s64 = ctx.r[11].s64 + 272;
	// 827DCC48: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCC4C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827DCC50: 912B0110  stw r9, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[9].u32 ) };
	// 827DCC54: 4BAE780D  bl 0x822c4460
	ctx.lr = 0x827DCC58;
	sub_822C4460(ctx, base);
	// 827DCC58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DCC5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DCC60: 419A0008  beq cr6, 0x827dcc68
	if ctx.cr[6].eq {
	pc = 0x827DCC68; continue 'dispatch;
	}
	// 827DCC64: 4BAE3C2D  bl 0x822c0890
	ctx.lr = 0x827DCC68;
	sub_822C0890(ctx, base);
	// 827DCC68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827DCC6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DCC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCC78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCC80 size=88
    let mut pc: u32 = 0x827DCC80;
    'dispatch: loop {
        match pc {
            0x827DCC80 => {
    //   block [0x827DCC80..0x827DCCD8)
	// 827DCC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCC88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCC8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCC90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DCC94: 4BFFFB25  bl 0x827dc7b8
	ctx.lr = 0x827DCC98;
	sub_827DC7B8(ctx, base);
	// 827DCC98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DCC9C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 827DCCA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DCCA4: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 827DCCA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DCCAC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DCCB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DCCB4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DCCB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DCCBC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DCCC0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DCCC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DCCC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCCCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCCD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCCD8 size=112
    let mut pc: u32 = 0x827DCCD8;
    'dispatch: loop {
        match pc {
            0x827DCCD8 => {
    //   block [0x827DCCD8..0x827DCD48)
	// 827DCCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCCE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DCCE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCCE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCCEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DCCF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DCCF4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827DCCF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DCCFC: 4BAECB95  bl 0x822c9890
	ctx.lr = 0x827DCD00;
	sub_822C9890(ctx, base);
	// 827DCD00: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827DCD04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DCD08: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DCD0C: 4BAE32F5  bl 0x822c0000
	ctx.lr = 0x827DCD10;
	sub_822C0000(ctx, base);
	// 827DCD10: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DCD14: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827DCD18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DCD1C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DCD20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DCD24: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DCD28: 419A0008  beq cr6, 0x827dcd30
	if ctx.cr[6].eq {
	pc = 0x827DCD30; continue 'dispatch;
	}
	// 827DCD2C: 4BAE3B65  bl 0x822c0890
	ctx.lr = 0x827DCD30;
	sub_822C0890(ctx, base);
	// 827DCD30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DCD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCD3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DCD40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCD48 size=88
    let mut pc: u32 = 0x827DCD48;
    'dispatch: loop {
        match pc {
            0x827DCD48 => {
    //   block [0x827DCD48..0x827DCDA0)
	// 827DCD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCD50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DCD54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCD58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCD5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DCD60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DCD64: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DCD68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DCD6C: 419A0008  beq cr6, 0x827dcd74
	if ctx.cr[6].eq {
	pc = 0x827DCD74; continue 'dispatch;
	}
	// 827DCD70: 4BAE3B21  bl 0x822c0890
	ctx.lr = 0x827DCD74;
	sub_822C0890(ctx, base);
	// 827DCD74: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DCD78: 4182000C  beq 0x827dcd84
	if ctx.cr[0].eq {
	pc = 0x827DCD84; continue 'dispatch;
	}
	// 827DCD7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DCD80: 4BAE34E9  bl 0x822c0268
	ctx.lr = 0x827DCD84;
	sub_822C0268(ctx, base);
	// 827DCD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DCD88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DCD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCD94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DCD98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCDA0 size=88
    let mut pc: u32 = 0x827DCDA0;
    'dispatch: loop {
        match pc {
            0x827DCDA0 => {
    //   block [0x827DCDA0..0x827DCDF8)
	// 827DCDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCDA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DCDAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DCDB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCDB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DCDB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DCDBC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DCDC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DCDC4: 419A0008  beq cr6, 0x827dcdcc
	if ctx.cr[6].eq {
	pc = 0x827DCDCC; continue 'dispatch;
	}
	// 827DCDC8: 4BAE3AC9  bl 0x822c0890
	ctx.lr = 0x827DCDCC;
	sub_822C0890(ctx, base);
	// 827DCDCC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DCDD0: 4182000C  beq 0x827dcddc
	if ctx.cr[0].eq {
	pc = 0x827DCDDC; continue 'dispatch;
	}
	// 827DCDD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DCDD8: 4BAE3491  bl 0x822c0268
	ctx.lr = 0x827DCDDC;
	sub_822C0268(ctx, base);
	// 827DCDDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DCDE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DCDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCDEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DCDF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DCDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCDF8 size=108
    let mut pc: u32 = 0x827DCDF8;
    'dispatch: loop {
        match pc {
            0x827DCDF8 => {
    //   block [0x827DCDF8..0x827DCE64)
	// 827DCDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DCE00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCE04: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCE08: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 827DCE0C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 827DCE10: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DCE14: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCE18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DCE1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827DCE20: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 827DCE24: 419A0024  beq cr6, 0x827dce48
	if ctx.cr[6].eq {
	pc = 0x827DCE48; continue 'dispatch;
	}
	// 827DCE28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DCE2C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 827DCE30: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DCE34: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 827DCE38: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 827DCE3C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DCE40: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DCE44: 4082FFE8  bne 0x827dce2c
	if !ctx.cr[0].eq {
	pc = 0x827DCE2C; continue 'dispatch;
	}
	// 827DCE48: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DCE4C: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCE50: 4BFFFAE1  bl 0x827dc930
	ctx.lr = 0x827DCE54;
	sub_827DC930(ctx, base);
	// 827DCE54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DCE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DCE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DCE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DCE68 size=264
    let mut pc: u32 = 0x827DCE68;
    'dispatch: loop {
        match pc {
            0x827DCE68 => {
    //   block [0x827DCE68..0x827DCF70)
	// 827DCE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCE6C: 489CB301  bl 0x831a816c
	ctx.lr = 0x827DCE70;
	sub_831A8130(ctx, base);
	// 827DCE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCE74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827DCE78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DCE7C: 4BFFF3BD  bl 0x827dc238
	ctx.lr = 0x827DCE80;
	sub_827DC238(ctx, base);
	// 827DCE80: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DCE84: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 827DCE88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DCE8C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DCE90: 4800001C  b 0x827dceac
	pc = 0x827DCEAC; continue 'dispatch;
	// 827DCE94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DCE98: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DCE9C: 4BFFFFCD  bl 0x827dce68
	ctx.lr = 0x827DCEA0;
	sub_827DCE68(ctx, base);
	// 827DCEA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DCEA4: 4BBC47E5  bl 0x823a1688
	ctx.lr = 0x827DCEA8;
	sub_823A1688(ctx, base);
	// 827DCEA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DCEAC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DCEB0: 409AFFE4  bne cr6, 0x827dce94
	if !ctx.cr[6].eq {
	pc = 0x827DCE94; continue 'dispatch;
	}
	// 827DCEB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DCEB8: 4BFFFB11  bl 0x827dc9c8
	ctx.lr = 0x827DCEBC;
	sub_827DC9C8(ctx, base);
	// 827DCEBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DCEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DCEC4: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 827DCEC8: 4BFFFB71  bl 0x827dca38
	ctx.lr = 0x827DCECC;
	sub_827DCA38(ctx, base);
	// 827DCECC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DCED0: 907F0040  stw r3, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[3].u32 ) };
	// 827DCED4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DCED8: 41980008  blt cr6, 0x827dcee0
	if ctx.cr[6].lt {
	pc = 0x827DCEE0; continue 'dispatch;
	}
	// 827DCEDC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 827DCEE0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827DCEE4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DCEE8: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DCEEC: 41990008  bgt cr6, 0x827dcef4
	if ctx.cr[6].gt {
	pc = 0x827DCEF4; continue 'dispatch;
	}
	// 827DCEF0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 827DCEF4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DCEF8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827DCEFC: 894AA0B5  lbz r10, -0x5f4b(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-24395 as u32) ) } as u64;
	// 827DCF00: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DCF04: 41820008  beq 0x827dcf0c
	if ctx.cr[0].eq {
	pc = 0x827DCF0C; continue 'dispatch;
	}
	// 827DCF08: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 827DCF0C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 827DCF10: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DCF14: 4099000C  ble cr6, 0x827dcf20
	if !ctx.cr[6].gt {
	pc = 0x827DCF20; continue 'dispatch;
	}
	// 827DCF18: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 827DCF1C: 48000048  b 0x827dcf64
	pc = 0x827DCF64; continue 'dispatch;
	// 827DCF20: 7BCA0020  clrldi r10, r30, 0x20
	ctx.r[10].u64 = ctx.r[30].u64 & 0x00000000FFFFFFFFu64;
	// 827DCF24: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 827DCF28: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 827DCF2C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827DCF30: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 827DCF34: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827DCF38: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 827DCF3C: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 827DCF40: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DCF44: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 827DCF48: C00BB854  lfs f0, -0x47ac(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18348 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DCF4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DCF50: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 827DCF54: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827DCF58: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 827DCF5C: 41990008  bgt cr6, 0x827dcf64
	if ctx.cr[6].gt {
	pc = 0x827DCF64; continue 'dispatch;
	}
	// 827DCF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DCF64: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 827DCF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DCF6C: 489CB250  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DCF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DCF70 size=340
    let mut pc: u32 = 0x827DCF70;
    'dispatch: loop {
        match pc {
            0x827DCF70 => {
    //   block [0x827DCF70..0x827DD0C4)
	// 827DCF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DCF74: 489CB1F9  bl 0x831a816c
	ctx.lr = 0x827DCF78;
	sub_831A8130(ctx, base);
	// 827DCF78: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DCF7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827DCF80: 817D0110  lwz r11, 0x110(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(272 as u32) ) } as u64;
	// 827DCF84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DCF88: 419A0130  beq cr6, 0x827dd0b8
	if ctx.cr[6].eq {
	pc = 0x827DD0B8; continue 'dispatch;
	}
	// 827DCF8C: 83CB003C  lwz r30, 0x3c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DCF90: 48008D59  bl 0x827e5ce8
	ctx.lr = 0x827DCF94;
	sub_827E5CE8(ctx, base);
	// 827DCF94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DCF98: 40820008  bne 0x827dcfa0
	if !ctx.cr[0].eq {
	pc = 0x827DCFA0; continue 'dispatch;
	}
	// 827DCF9C: 48009D2D  bl 0x827e6cc8
	ctx.lr = 0x827DCFA0;
	sub_827E6CC8(ctx, base);
	// 827DCFA0: 3FE08336  lis r31, -0x7cca
	ctx.r[31].s64 = -2093613056;
	// 827DCFA4: 807FA104  lwz r3, -0x5efc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24316 as u32) ) } as u64;
	// 827DCFA8: 48009061  bl 0x827e6008
	ctx.lr = 0x827DCFAC;
	sub_827E6008(ctx, base);
	// 827DCFAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DCFB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DCFB4: 4BFFFBE5  bl 0x827dcb98
	ctx.lr = 0x827DCFB8;
	sub_827DCB98(ctx, base);
	// 827DCFB8: 897D0100  lbz r11, 0x100(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(256 as u32) ) } as u64;
	// 827DCFBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DCFC0: 40820044  bne 0x827dd004
	if !ctx.cr[0].eq {
	pc = 0x827DD004; continue 'dispatch;
	}
	// 827DCFC4: 897E0030  lbz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DCFC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DCFCC: 41820038  beq 0x827dd004
	if ctx.cr[0].eq {
	pc = 0x827DD004; continue 'dispatch;
	}
	// 827DCFD0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DCFD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DCFD8: 419A002C  beq cr6, 0x827dd004
	if ctx.cr[6].eq {
	pc = 0x827DD004; continue 'dispatch;
	}
	// 827DCFDC: 894B0030  lbz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DCFE0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DCFE4: 41820014  beq 0x827dcff8
	if ctx.cr[0].eq {
	pc = 0x827DCFF8; continue 'dispatch;
	}
	// 827DCFE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DCFEC: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DCFF0: 4BFFF319  bl 0x827dc308
	ctx.lr = 0x827DCFF4;
	sub_827DC308(ctx, base);
	// 827DCFF4: 48000010  b 0x827dd004
	pc = 0x827DD004; continue 'dispatch;
	// 827DCFF8: 807FA104  lwz r3, -0x5efc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24316 as u32) ) } as u64;
	// 827DCFFC: 808B002C  lwz r4, 0x2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 827DD000: 480093B1  bl 0x827e63b0
	ctx.lr = 0x827DD004;
	sub_827E63B0(ctx, base);
	// 827DD004: 897D0100  lbz r11, 0x100(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(256 as u32) ) } as u64;
	// 827DD008: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827DD00C: 83DFA104  lwz r30, -0x5efc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24316 as u32) ) } as u64;
	// 827DD010: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DD014: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DD018: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD01C: 41820028  beq 0x827dd044
	if ctx.cr[0].eq {
	pc = 0x827DD044; continue 'dispatch;
	}
	// 827DD020: 4BFFF319  bl 0x827dc338
	ctx.lr = 0x827DD024;
	sub_827DC338(ctx, base);
	// 827DD024: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DD028: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DD02C: 4861C75D  bl 0x82df9788
	ctx.lr = 0x827DD030;
	sub_82DF9788(ctx, base);
	// 827DD030: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DD034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD038: 48009021  bl 0x827e6058
	ctx.lr = 0x827DD03C;
	sub_827E6058(ctx, base);
	// 827DD03C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DD040: 48000024  b 0x827dd064
	pc = 0x827DD064; continue 'dispatch;
	// 827DD044: 4BFFF2F5  bl 0x827dc338
	ctx.lr = 0x827DD048;
	sub_827DC338(ctx, base);
	// 827DD048: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DD04C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827DD050: 4861C739  bl 0x82df9788
	ctx.lr = 0x827DD054;
	sub_82DF9788(ctx, base);
	// 827DD054: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DD058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD05C: 4800900D  bl 0x827e6068
	ctx.lr = 0x827DD060;
	sub_827E6068(ctx, base);
	// 827DD060: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827DD064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DD068: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DD06C: 4BAEBDC5  bl 0x822c8e30
	ctx.lr = 0x827DD070;
	sub_822C8E30(ctx, base);
	// 827DD070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD074: 486163B5  bl 0x82df3428
	ctx.lr = 0x827DD078;
	sub_82DF3428(ctx, base);
	// 827DD078: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DD07C: 807FA104  lwz r3, -0x5efc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24316 as u32) ) } as u64;
	// 827DD080: 816BB868  lwz r11, -0x4798(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18328 as u32) ) } as u64;
	// 827DD084: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DD088: 4198001C  blt cr6, 0x827dd0a4
	if ctx.cr[6].lt {
	pc = 0x827DD0A4; continue 'dispatch;
	}
	// 827DD08C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 827DD090: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827DD094: 394AB830  addi r10, r10, -0x47d0
	ctx.r[10].s64 = ctx.r[10].s64 + -18384;
	// 827DD098: 7D640734  extsh r4, r11
	ctx.r[4].s64 = ctx.r[11].s16 as i64;
	// 827DD09C: 7CA9502E  lwzx r5, r9, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 827DD0A0: 4800000C  b 0x827dd0ac
	pc = 0x827DD0AC; continue 'dispatch;
	// 827DD0A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DD0A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DD0AC: 48008AFD  bl 0x827e5ba8
	ctx.lr = 0x827DD0B0;
	sub_827E5BA8(ctx, base);
	// 827DD0B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DD0B4: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 827DD0B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827DD0BC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 827DD0C0: 489CB0FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD0C8 size=100
    let mut pc: u32 = 0x827DD0C8;
    'dispatch: loop {
        match pc {
            0x827DD0C8 => {
    //   block [0x827DD0C8..0x827DD12C)
	// 827DD0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD0CC: 489CB095  bl 0x831a8160
	ctx.lr = 0x827DD0D0;
	sub_831A8130(ctx, base);
	// 827DD0D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD0D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DD0D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DD0DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827DD0E0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DD0E4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827DD0E8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 827DD0EC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 827DD0F0: 480D4B71  bl 0x828b1c60
	ctx.lr = 0x827DD0F4;
	sub_828B1C60(ctx, base);
	// 827DD0F4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827DD0F8: 41820028  beq 0x827dd120
	if ctx.cr[0].eq {
	pc = 0x827DD120; continue 'dispatch;
	}
	// 827DD0FC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DD100: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827DD104: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 827DD108: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 827DD10C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DD110: 483D2B79  bl 0x82bafc88
	ctx.lr = 0x827DD114;
	sub_82BAFC88(ctx, base);
	// 827DD114: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DD118: 9B5F001C  stb r26, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u8 ) };
	// 827DD11C: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 827DD120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DD124: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DD128: 489CB088  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD130 size=92
    let mut pc: u32 = 0x827DD130;
    'dispatch: loop {
        match pc {
            0x827DD130 => {
    //   block [0x827DD130..0x827DD18C)
	// 827DD130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD134: 489CB039  bl 0x831a816c
	ctx.lr = 0x827DD138;
	sub_831A8130(ctx, base);
	// 827DD138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD13C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DD140: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827DD144: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 827DD148: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD14C: 48000030  b 0x827dd17c
	pc = 0x827DD17C; continue 'dispatch;
	// 827DD150: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DD154: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD158: 4BFFFFD9  bl 0x827dd130
	ctx.lr = 0x827DD15C;
	sub_827DD130(ctx, base);
	// 827DD15C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DD160: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD164: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD168: 4BFFFC39  bl 0x827dcda0
	ctx.lr = 0x827DD16C;
	sub_827DCDA0(ctx, base);
	// 827DD16C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD170: 48614D41  bl 0x82df1eb0
	ctx.lr = 0x827DD174;
	sub_82DF1EB0(ctx, base);
	// 827DD174: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD178: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 827DD17C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DD180: 419AFFD0  beq cr6, 0x827dd150
	if ctx.cr[6].eq {
	pc = 0x827DD150; continue 'dispatch;
	}
	// 827DD184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DD188: 489CB034  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD190 size=92
    let mut pc: u32 = 0x827DD190;
    'dispatch: loop {
        match pc {
            0x827DD190 => {
    //   block [0x827DD190..0x827DD1EC)
	// 827DD190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD194: 489CAFD9  bl 0x831a816c
	ctx.lr = 0x827DD198;
	sub_831A8130(ctx, base);
	// 827DD198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD19C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DD1A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827DD1A4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 827DD1A8: 897E001D  lbz r11, 0x1d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DD1AC: 48000030  b 0x827dd1dc
	pc = 0x827DD1DC; continue 'dispatch;
	// 827DD1B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DD1B4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD1B8: 4BFFFFD9  bl 0x827dd190
	ctx.lr = 0x827DD1BC;
	sub_827DD190(ctx, base);
	// 827DD1BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DD1C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD1C4: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD1C8: 4BFFFB81  bl 0x827dcd48
	ctx.lr = 0x827DD1CC;
	sub_827DCD48(ctx, base);
	// 827DD1CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD1D0: 4BAE3099  bl 0x822c0268
	ctx.lr = 0x827DD1D4;
	sub_822C0268(ctx, base);
	// 827DD1D4: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DD1D8: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 827DD1DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DD1E0: 419AFFD0  beq cr6, 0x827dd1b0
	if ctx.cr[6].eq {
	pc = 0x827DD1B0; continue 'dispatch;
	}
	// 827DD1E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DD1E8: 489CAFD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD1F0 size=84
    let mut pc: u32 = 0x827DD1F0;
    'dispatch: loop {
        match pc {
            0x827DD1F0 => {
    //   block [0x827DD1F0..0x827DD244)
	// 827DD1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DD1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DD1FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD200: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DD204: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD208: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD20C: 4BFFFF25  bl 0x827dd130
	ctx.lr = 0x827DD210;
	sub_827DD130(ctx, base);
	// 827DD210: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD214: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DD218: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DD21C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DD220: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD224: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DD228: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD22C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DD230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DD234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DD238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DD23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DD240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD248 size=84
    let mut pc: u32 = 0x827DD248;
    'dispatch: loop {
        match pc {
            0x827DD248 => {
    //   block [0x827DD248..0x827DD29C)
	// 827DD248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DD250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DD254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DD25C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD260: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD264: 4BFFFF2D  bl 0x827dd190
	ctx.lr = 0x827DD268;
	sub_827DD190(ctx, base);
	// 827DD268: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD26C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DD270: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DD274: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DD278: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD27C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DD280: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD284: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DD288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DD28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DD290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DD294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DD298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD2A0 size=564
    let mut pc: u32 = 0x827DD2A0;
    'dispatch: loop {
        match pc {
            0x827DD2A0 => {
    //   block [0x827DD2A0..0x827DD4D4)
	// 827DD2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD2A4: 489CAEB9  bl 0x831a815c
	ctx.lr = 0x827DD2A8;
	sub_831A8130(ctx, base);
	// 827DD2A8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD2AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DD2B0: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 827DD2B4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 827DD2B8: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 827DD2BC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 827DD2C0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD2C4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 827DD2C8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 827DD2CC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD2D0: 41980048  blt cr6, 0x827dd318
	if ctx.cr[6].lt {
	pc = 0x827DD318; continue 'dispatch;
	}
	// 827DD2D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DD2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD2DC: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 827DD2E0: 4BAE85E9  bl 0x822c58c8
	ctx.lr = 0x827DD2E4;
	sub_822C58C8(ctx, base);
	// 827DD2E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DD2E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DD2EC: 4BAE852D  bl 0x822c5818
	ctx.lr = 0x827DD2F0;
	sub_822C5818(ctx, base);
	// 827DD2F0: 4BAE6FC1  bl 0x822c42b0
	ctx.lr = 0x827DD2F4;
	sub_822C42B0(ctx, base);
	// 827DD2F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DD2F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DD2FC: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 827DD300: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827DD304: 4BAE816D  bl 0x822c5470
	ctx.lr = 0x827DD308;
	sub_822C5470(ctx, base);
	// 827DD308: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DD30C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DD310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD314: 4BAE79CD  bl 0x822c4ce0
	ctx.lr = 0x827DD318;
	sub_822C4CE0(ctx, base);
	// 827DD318: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 827DD31C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD320: 48614B89  bl 0x82df1ea8
	ctx.lr = 0x827DD324;
	sub_82DF1EA8(ctx, base);
	// 827DD324: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 827DD328: 41820020  beq 0x827dd348
	if ctx.cr[0].eq {
	pc = 0x827DD348; continue 'dispatch;
	}
	// 827DD32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827DD330: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 827DD334: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827DD338: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827DD33C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DD340: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827DD344: 4BDDCB05  bl 0x825b9e48
	ctx.lr = 0x827DD348;
	sub_825B9E48(ctx, base);
	// 827DD348: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD34C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD350: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DD354: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD358: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DD35C: 409A0018  bne cr6, 0x827dd374
	if !ctx.cr[6].eq {
	pc = 0x827DD374; continue 'dispatch;
	}
	// 827DD360: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DD364: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD368: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD36C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD370: 4800003C  b 0x827dd3ac
	pc = 0x827DD3AC; continue 'dispatch;
	// 827DD374: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DD378: 41820020  beq 0x827dd398
	if ctx.cr[0].eq {
	pc = 0x827DD398; continue 'dispatch;
	}
	// 827DD37C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD380: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD384: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD388: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DD38C: 409A0024  bne cr6, 0x827dd3b0
	if !ctx.cr[6].eq {
	pc = 0x827DD3B0; continue 'dispatch;
	}
	// 827DD390: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD394: 4800001C  b 0x827dd3b0
	pc = 0x827DD3B0; continue 'dispatch;
	// 827DD398: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DD39C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD3A0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD3A4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DD3A8: 409A0008  bne cr6, 0x827dd3b0
	if !ctx.cr[6].eq {
	pc = 0x827DD3B0; continue 'dispatch;
	}
	// 827DD3AC: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DD3B0: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD3B4: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 827DD3B8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 827DD3BC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 827DD3C0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD3C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD3C8: 409A00F0  bne cr6, 0x827dd4b8
	if !ctx.cr[6].eq {
	pc = 0x827DD4B8; continue 'dispatch;
	}
	// 827DD3CC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 827DD3D0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD3D4: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD3D8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD3DC: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 827DD3E0: 409A0054  bne cr6, 0x827dd434
	if !ctx.cr[6].eq {
	pc = 0x827DD434; continue 'dispatch;
	}
	// 827DD3E4: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD3E8: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD3EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 827DD3F0: 419A0054  beq cr6, 0x827dd444
	if ctx.cr[6].eq {
	pc = 0x827DD444; continue 'dispatch;
	}
	// 827DD3F4: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD3F8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD3FC: 409A0010  bne cr6, 0x827dd40c
	if !ctx.cr[6].eq {
	pc = 0x827DD40C; continue 'dispatch;
	}
	// 827DD400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD404: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DD408: 4BC66B61  bl 0x82443f68
	ctx.lr = 0x827DD40C;
	sub_82443F68(ctx, base);
	// 827DD40C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD414: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DD418: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD41C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD420: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 827DD424: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD428: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD42C: 4BC66AD5  bl 0x82443f00
	ctx.lr = 0x827DD430;
	sub_82443F00(ctx, base);
	// 827DD430: 48000074  b 0x827dd4a4
	pc = 0x827DD4A4; continue 'dispatch;
	// 827DD434: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD438: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD43C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 827DD440: 409A0028  bne cr6, 0x827dd468
	if !ctx.cr[6].eq {
	pc = 0x827DD468; continue 'dispatch;
	}
	// 827DD444: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD448: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DD44C: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DD450: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD454: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD458: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 827DD45C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD460: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD464: 48000040  b 0x827dd4a4
	pc = 0x827DD4A4; continue 'dispatch;
	// 827DD468: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD46C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD470: 409A0010  bne cr6, 0x827dd480
	if !ctx.cr[6].eq {
	pc = 0x827DD480; continue 'dispatch;
	}
	// 827DD474: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD478: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DD47C: 4BC66A85  bl 0x82443f00
	ctx.lr = 0x827DD480;
	sub_82443F00(ctx, base);
	// 827DD480: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD484: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD488: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DD48C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD490: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD494: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 827DD498: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD49C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD4A0: 4BC66AC9  bl 0x82443f68
	ctx.lr = 0x827DD4A4;
	sub_82443F68(ctx, base);
	// 827DD4A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD4A8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827DD4AC: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD4B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD4B4: 419AFF1C  beq cr6, 0x827dd3d0
	if ctx.cr[6].eq {
	pc = 0x827DD3D0; continue 'dispatch;
	}
	// 827DD4B8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD4BC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 827DD4C0: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD4C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD4C8: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DD4CC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827DD4D0: 489CACDC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD4D8 size=548
    let mut pc: u32 = 0x827DD4D8;
    'dispatch: loop {
        match pc {
            0x827DD4D8 => {
    //   block [0x827DD4D8..0x827DD6FC)
	// 827DD4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD4DC: 489CAC85  bl 0x831a8160
	ctx.lr = 0x827DD4E0;
	sub_831A8130(ctx, base);
	// 827DD4E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD4E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DD4E8: 3D600FFF  lis r11, 0xfff
	ctx.r[11].s64 = 268369920;
	// 827DD4EC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 827DD4F0: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 827DD4F4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 827DD4F8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD4FC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 827DD500: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 827DD504: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD508: 41980048  blt cr6, 0x827dd550
	if ctx.cr[6].lt {
	pc = 0x827DD550; continue 'dispatch;
	}
	// 827DD50C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DD510: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD514: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 827DD518: 4BAE83B1  bl 0x822c58c8
	ctx.lr = 0x827DD51C;
	sub_822C58C8(ctx, base);
	// 827DD51C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DD520: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DD524: 4BAE82F5  bl 0x822c5818
	ctx.lr = 0x827DD528;
	sub_822C5818(ctx, base);
	// 827DD528: 4BAE6D89  bl 0x822c42b0
	ctx.lr = 0x827DD52C;
	sub_822C42B0(ctx, base);
	// 827DD52C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DD530: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DD534: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 827DD538: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827DD53C: 4BAE7F35  bl 0x822c5470
	ctx.lr = 0x827DD540;
	sub_822C5470(ctx, base);
	// 827DD540: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DD544: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DD548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD54C: 4BAE7795  bl 0x822c4ce0
	ctx.lr = 0x827DD550;
	sub_822C4CE0(ctx, base);
	// 827DD550: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827DD558: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 827DD55C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 827DD560: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827DD564: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD568: 4BFFFB61  bl 0x827dd0c8
	ctx.lr = 0x827DD56C;
	sub_827DD0C8(ctx, base);
	// 827DD56C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD570: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD574: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827DD578: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DD57C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD580: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DD584: 409A0018  bne cr6, 0x827dd59c
	if !ctx.cr[6].eq {
	pc = 0x827DD59C; continue 'dispatch;
	}
	// 827DD588: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DD58C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD590: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD594: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD598: 4800003C  b 0x827dd5d4
	pc = 0x827DD5D4; continue 'dispatch;
	// 827DD59C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DD5A0: 41820020  beq 0x827dd5c0
	if ctx.cr[0].eq {
	pc = 0x827DD5C0; continue 'dispatch;
	}
	// 827DD5A4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD5A8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD5AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD5B0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DD5B4: 409A0024  bne cr6, 0x827dd5d8
	if !ctx.cr[6].eq {
	pc = 0x827DD5D8; continue 'dispatch;
	}
	// 827DD5B8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD5BC: 4800001C  b 0x827dd5d8
	pc = 0x827DD5D8; continue 'dispatch;
	// 827DD5C0: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DD5C4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD5C8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD5CC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DD5D0: 409A0008  bne cr6, 0x827dd5d8
	if !ctx.cr[6].eq {
	pc = 0x827DD5D8; continue 'dispatch;
	}
	// 827DD5D4: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DD5D8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD5DC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 827DD5E0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 827DD5E4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 827DD5E8: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DD5EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD5F0: 409A00F0  bne cr6, 0x827dd6e0
	if !ctx.cr[6].eq {
	pc = 0x827DD6E0; continue 'dispatch;
	}
	// 827DD5F4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 827DD5F8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD5FC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD600: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD604: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 827DD608: 409A0054  bne cr6, 0x827dd65c
	if !ctx.cr[6].eq {
	pc = 0x827DD65C; continue 'dispatch;
	}
	// 827DD60C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD610: 892A001C  lbz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DD614: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 827DD618: 419A0054  beq cr6, 0x827dd66c
	if ctx.cr[6].eq {
	pc = 0x827DD66C; continue 'dispatch;
	}
	// 827DD61C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD620: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD624: 409A0010  bne cr6, 0x827dd634
	if !ctx.cr[6].eq {
	pc = 0x827DD634; continue 'dispatch;
	}
	// 827DD628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD62C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DD630: 4BD51F59  bl 0x8252f588
	ctx.lr = 0x827DD634;
	sub_8252F588(ctx, base);
	// 827DD634: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD638: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD63C: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DD640: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD644: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD648: 9B6B001C  stb r27, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 827DD64C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD650: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD654: 4BD51F9D  bl 0x8252f5f0
	ctx.lr = 0x827DD658;
	sub_8252F5F0(ctx, base);
	// 827DD658: 48000074  b 0x827dd6cc
	pc = 0x827DD6CC; continue 'dispatch;
	// 827DD65C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD660: 892A001C  lbz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DD664: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 827DD668: 409A0028  bne cr6, 0x827dd690
	if !ctx.cr[6].eq {
	pc = 0x827DD690; continue 'dispatch;
	}
	// 827DD66C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD670: 9BA9001C  stb r29, 0x1c(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DD674: 9BAA001C  stb r29, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DD678: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD67C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD680: 9B6A001C  stb r27, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 827DD684: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD688: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD68C: 48000040  b 0x827dd6cc
	pc = 0x827DD6CC; continue 'dispatch;
	// 827DD690: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD694: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD698: 409A0010  bne cr6, 0x827dd6a8
	if !ctx.cr[6].eq {
	pc = 0x827DD6A8; continue 'dispatch;
	}
	// 827DD69C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD6A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DD6A4: 4BD51F4D  bl 0x8252f5f0
	ctx.lr = 0x827DD6A8;
	sub_8252F5F0(ctx, base);
	// 827DD6A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DD6B0: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DD6B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6BC: 9B6B001C  stb r27, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 827DD6C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6C4: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6C8: 4BD51EC1  bl 0x8252f588
	ctx.lr = 0x827DD6CC;
	sub_8252F588(ctx, base);
	// 827DD6CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6D0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827DD6D4: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DD6D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD6DC: 419AFF1C  beq cr6, 0x827dd5f8
	if ctx.cr[6].eq {
	pc = 0x827DD5F8; continue 'dispatch;
	}
	// 827DD6E0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DD6E8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD6EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD6F0: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DD6F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827DD6F8: 489CAAB8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DD700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DD700 size=1020
    let mut pc: u32 = 0x827DD700;
    'dispatch: loop {
        match pc {
            0x827DD700 => {
    //   block [0x827DD700..0x827DDAFC)
	// 827DD700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DD704: 489CAA55  bl 0x831a8158
	ctx.lr = 0x827DD708;
	sub_831A8130(ctx, base);
	// 827DD708: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DD70C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827DD710: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 827DD714: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 827DD718: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 827DD71C: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DD724: 419A0048  beq cr6, 0x827dd76c
	if ctx.cr[6].eq {
	pc = 0x827DD76C; continue 'dispatch;
	}
	// 827DD728: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DD72C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD730: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 827DD734: 4BAE8195  bl 0x822c58c8
	ctx.lr = 0x827DD738;
	sub_822C58C8(ctx, base);
	// 827DD738: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DD73C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DD740: 4BAEC771  bl 0x822c9eb0
	ctx.lr = 0x827DD744;
	sub_822C9EB0(ctx, base);
	// 827DD744: 4BAE6B6D  bl 0x822c42b0
	ctx.lr = 0x827DD748;
	sub_822C42B0(ctx, base);
	// 827DD748: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DD74C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DD750: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 827DD754: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827DD758: 4BAE7D19  bl 0x822c5470
	ctx.lr = 0x827DD75C;
	sub_822C5470(ctx, base);
	// 827DD75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DD760: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DD764: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DD768: 4BAE7579  bl 0x822c4ce0
	ctx.lr = 0x827DD76C;
	sub_822C4CE0(ctx, base);
	// 827DD76C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 827DD770: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 827DD774: 4BBC3F15  bl 0x823a1688
	ctx.lr = 0x827DD778;
	sub_823A1688(ctx, base);
	// 827DD778: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD77C: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD780: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD784: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 827DD788: 419A000C  beq cr6, 0x827dd794
	if ctx.cr[6].eq {
	pc = 0x827DD794; continue 'dispatch;
	}
	// 827DD78C: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD790: 48000028  b 0x827dd7b8
	pc = 0x827DD7B8; continue 'dispatch;
	// 827DD794: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD798: 894A0019  lbz r10, 0x19(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD79C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD7A0: 419A000C  beq cr6, 0x827dd7ac
	if ctx.cr[6].eq {
	pc = 0x827DD7AC; continue 'dispatch;
	}
	// 827DD7A4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 827DD7A8: 48000010  b 0x827dd7b8
	pc = 0x827DD7B8; continue 'dispatch;
	// 827DD7AC: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD7B0: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DD7B4: 409A00DC  bne cr6, 0x827dd890
	if !ctx.cr[6].eq {
	pc = 0x827DD890; continue 'dispatch;
	}
	// 827DD7B8: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD7BC: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD7C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DD7C4: 409A0008  bne cr6, 0x827dd7cc
	if !ctx.cr[6].eq {
	pc = 0x827DD7CC; continue 'dispatch;
	}
	// 827DD7C8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827DD7CC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD7D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD7D4: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DD7D8: 409A000C  bne cr6, 0x827dd7e4
	if !ctx.cr[6].eq {
	pc = 0x827DD7E4; continue 'dispatch;
	}
	// 827DD7DC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DD7E0: 4800001C  b 0x827dd7fc
	pc = 0x827DD7FC; continue 'dispatch;
	// 827DD7E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD7E8: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DD7EC: 409A000C  bne cr6, 0x827dd7f8
	if !ctx.cr[6].eq {
	pc = 0x827DD7F8; continue 'dispatch;
	}
	// 827DD7F0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD7F4: 48000008  b 0x827dd7fc
	pc = 0x827DD7FC; continue 'dispatch;
	// 827DD7F8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DD7FC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD800: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD804: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DD808: 409A003C  bne cr6, 0x827dd844
	if !ctx.cr[6].eq {
	pc = 0x827DD844; continue 'dispatch;
	}
	// 827DD80C: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DD814: 419A000C  beq cr6, 0x827dd820
	if ctx.cr[6].eq {
	pc = 0x827DD820; continue 'dispatch;
	}
	// 827DD818: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827DD81C: 48000024  b 0x827dd840
	pc = 0x827DD840; continue 'dispatch;
	// 827DD820: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD824: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827DD828: 4800000C  b 0x827dd834
	pc = 0x827DD834; continue 'dispatch;
	// 827DD82C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827DD830: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD834: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD838: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827DD83C: 419AFFF0  beq cr6, 0x827dd82c
	if ctx.cr[6].eq {
	pc = 0x827DD82C; continue 'dispatch;
	}
	// 827DD840: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DD844: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD848: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD84C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DD850: 409A00D4  bne cr6, 0x827dd924
	if !ctx.cr[6].eq {
	pc = 0x827DD924; continue 'dispatch;
	}
	// 827DD854: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD858: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DD85C: 419A000C  beq cr6, 0x827dd868
	if ctx.cr[6].eq {
	pc = 0x827DD868; continue 'dispatch;
	}
	// 827DD860: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827DD864: 48000024  b 0x827dd888
	pc = 0x827DD888; continue 'dispatch;
	// 827DD868: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD86C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827DD870: 4800000C  b 0x827dd87c
	pc = 0x827DD87C; continue 'dispatch;
	// 827DD874: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827DD878: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD87C: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD880: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827DD884: 419AFFF0  beq cr6, 0x827dd874
	if ctx.cr[6].eq {
	pc = 0x827DD874; continue 'dispatch;
	}
	// 827DD888: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DD88C: 48000098  b 0x827dd924
	pc = 0x827DD924; continue 'dispatch;
	// 827DD890: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827DD894: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD898: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DD89C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD8A0: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD8A4: 409A000C  bne cr6, 0x827dd8b0
	if !ctx.cr[6].eq {
	pc = 0x827DD8B0; continue 'dispatch;
	}
	// 827DD8A8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 827DD8AC: 4800002C  b 0x827dd8d8
	pc = 0x827DD8D8; continue 'dispatch;
	// 827DD8B0: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD8B4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD8B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DD8BC: 409A0008  bne cr6, 0x827dd8c4
	if !ctx.cr[6].eq {
	pc = 0x827DD8C4; continue 'dispatch;
	}
	// 827DD8C0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827DD8C4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DD8C8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD8CC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DD8D0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD8D4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827DD8D8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD8DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD8E0: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DD8E4: 409A000C  bne cr6, 0x827dd8f0
	if !ctx.cr[6].eq {
	pc = 0x827DD8F0; continue 'dispatch;
	}
	// 827DD8E8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827DD8EC: 48000020  b 0x827dd90c
	pc = 0x827DD90C; continue 'dispatch;
	// 827DD8F0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD8F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD8F8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DD8FC: 409A000C  bne cr6, 0x827dd908
	if !ctx.cr[6].eq {
	pc = 0x827DD908; continue 'dispatch;
	}
	// 827DD900: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827DD904: 48000008  b 0x827dd90c
	pc = 0x827DD90C; continue 'dispatch;
	// 827DD908: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 827DD90C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD910: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DD914: 897A0018  lbz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD918: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD91C: 99790018  stb r11, 0x18(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 827DD920: 995A0018  stb r10, 0x18(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827DD924: 897A0018  lbz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD928: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827DD92C: 409A0198  bne cr6, 0x827ddac4
	if !ctx.cr[6].eq {
	pc = 0x827DDAC4; continue 'dispatch;
	}
	// 827DD930: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD934: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 827DD938: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DD93C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD940: 419A0180  beq cr6, 0x827ddac0
	if ctx.cr[6].eq {
	pc = 0x827DDAC0; continue 'dispatch;
	}
	// 827DD944: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827DD948: 897C0018  lbz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD94C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827DD950: 409A0170  bne cr6, 0x827ddac0
	if !ctx.cr[6].eq {
	pc = 0x827DDAC0; continue 'dispatch;
	}
	// 827DD954: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD958: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DD95C: 409A00A8  bne cr6, 0x827dda04
	if !ctx.cr[6].eq {
	pc = 0x827DDA04; continue 'dispatch;
	}
	// 827DD960: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD964: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD968: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD96C: 409A001C  bne cr6, 0x827dd988
	if !ctx.cr[6].eq {
	pc = 0x827DD988; continue 'dispatch;
	}
	// 827DD970: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DD974: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DD978: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DD97C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DD980: 4BC665E9  bl 0x82443f68
	ctx.lr = 0x827DD984;
	sub_82443F68(ctx, base);
	// 827DD984: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD988: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DD98C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DD990: 409A00C8  bne cr6, 0x827dda58
	if !ctx.cr[6].eq {
	pc = 0x827DDA58; continue 'dispatch;
	}
	// 827DD994: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD998: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD99C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DD9A0: 409A0014  bne cr6, 0x827dd9b4
	if !ctx.cr[6].eq {
	pc = 0x827DD9B4; continue 'dispatch;
	}
	// 827DD9A4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD9A8: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD9AC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DD9B0: 419A00A4  beq cr6, 0x827dda54
	if ctx.cr[6].eq {
	pc = 0x827DDA54; continue 'dispatch;
	}
	// 827DD9B4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD9B8: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD9BC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DD9C0: 409A0020  bne cr6, 0x827dd9e0
	if !ctx.cr[6].eq {
	pc = 0x827DD9E0; continue 'dispatch;
	}
	// 827DD9C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DD9C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827DD9CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DD9D0: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DD9D4: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DD9D8: 4BC66529  bl 0x82443f00
	ctx.lr = 0x827DD9DC;
	sub_82443F00(ctx, base);
	// 827DD9DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD9E0: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DD9E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DD9E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DD9EC: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827DD9F0: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DD9F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DD9F8: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DD9FC: 4BC6656D  bl 0x82443f68
	ctx.lr = 0x827DDA00;
	sub_82443F68(ctx, base);
	// 827DDA00: 480000C0  b 0x827ddac0
	pc = 0x827DDAC0; continue 'dispatch;
	// 827DDA04: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DDA08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDA0C: 409A001C  bne cr6, 0x827dda28
	if !ctx.cr[6].eq {
	pc = 0x827DDA28; continue 'dispatch;
	}
	// 827DDA10: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DDA14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DDA18: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DDA1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDA20: 4BC664E1  bl 0x82443f00
	ctx.lr = 0x827DDA24;
	sub_82443F00(ctx, base);
	// 827DDA24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDA28: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DDA2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDA30: 409A0028  bne cr6, 0x827dda58
	if !ctx.cr[6].eq {
	pc = 0x827DDA58; continue 'dispatch;
	}
	// 827DDA34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDA38: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DDA3C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDA40: 409A0034  bne cr6, 0x827dda74
	if !ctx.cr[6].eq {
	pc = 0x827DDA74; continue 'dispatch;
	}
	// 827DDA44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDA48: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DDA4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDA50: 409A0024  bne cr6, 0x827dda74
	if !ctx.cr[6].eq {
	pc = 0x827DDA74; continue 'dispatch;
	}
	// 827DDA54: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DDA58: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDA5C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 827DDA60: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDA64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDA68: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DDA6C: 409AFEDC  bne cr6, 0x827dd948
	if !ctx.cr[6].eq {
	pc = 0x827DD948; continue 'dispatch;
	}
	// 827DDA70: 48000050  b 0x827ddac0
	pc = 0x827DDAC0; continue 'dispatch;
	// 827DDA74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDA78: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DDA7C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDA80: 409A0020  bne cr6, 0x827ddaa0
	if !ctx.cr[6].eq {
	pc = 0x827DDAA0; continue 'dispatch;
	}
	// 827DDA84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDA88: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827DDA8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDA90: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DDA94: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827DDA98: 4BC664D1  bl 0x82443f68
	ctx.lr = 0x827DDA9C;
	sub_82443F68(ctx, base);
	// 827DDA9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDAA0: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827DDAA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DDAA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDAAC: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827DDAB0: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DDAB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDAB8: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DDABC: 4BC66445  bl 0x82443f00
	ctx.lr = 0x827DDAC0;
	sub_82443F00(ctx, base);
	// 827DDAC0: 9BDC0018  stb r30, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827DDAC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DDAC8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DDACC: 4BFFF2D5  bl 0x827dcda0
	ctx.lr = 0x827DDAD0;
	sub_827DCDA0(ctx, base);
	// 827DDAD0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DDAD4: 486143DD  bl 0x82df1eb0
	ctx.lr = 0x827DDAD8;
	sub_82DF1EB0(ctx, base);
	// 827DDAD8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDADC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DDAE0: 419A000C  beq cr6, 0x827ddaec
	if ctx.cr[6].eq {
	pc = 0x827DDAEC; continue 'dispatch;
	}
	// 827DDAE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827DDAE8: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DDAEC: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827DDAF0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 827DDAF4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827DDAF8: 489CA6B0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DDB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DDB00 size=1020
    let mut pc: u32 = 0x827DDB00;
    'dispatch: loop {
        match pc {
            0x827DDB00 => {
    //   block [0x827DDB00..0x827DDEFC)
	// 827DDB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DDB04: 489CA655  bl 0x831a8158
	ctx.lr = 0x827DDB08;
	sub_831A8130(ctx, base);
	// 827DDB08: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DDB0C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827DDB10: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 827DDB14: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 827DDB18: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 827DDB1C: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDB20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DDB24: 419A0048  beq cr6, 0x827ddb6c
	if ctx.cr[6].eq {
	pc = 0x827DDB6C; continue 'dispatch;
	}
	// 827DDB28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DDB2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DDB30: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 827DDB34: 4BAE7D95  bl 0x822c58c8
	ctx.lr = 0x827DDB38;
	sub_822C58C8(ctx, base);
	// 827DDB38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DDB3C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DDB40: 4BAEC371  bl 0x822c9eb0
	ctx.lr = 0x827DDB44;
	sub_822C9EB0(ctx, base);
	// 827DDB44: 4BAE676D  bl 0x822c42b0
	ctx.lr = 0x827DDB48;
	sub_822C42B0(ctx, base);
	// 827DDB48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827DDB4C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827DDB50: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 827DDB54: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827DDB58: 4BAE7919  bl 0x822c5470
	ctx.lr = 0x827DDB5C;
	sub_822C5470(ctx, base);
	// 827DDB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DDB60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DDB64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DDB68: 4BAE7179  bl 0x822c4ce0
	ctx.lr = 0x827DDB6C;
	sub_822C4CE0(ctx, base);
	// 827DDB6C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 827DDB70: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 827DDB74: 4807CE45  bl 0x8285a9b8
	ctx.lr = 0x827DDB78;
	sub_8285A9B8(ctx, base);
	// 827DDB78: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDB7C: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDB80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDB84: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 827DDB88: 419A000C  beq cr6, 0x827ddb94
	if ctx.cr[6].eq {
	pc = 0x827DDB94; continue 'dispatch;
	}
	// 827DDB8C: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDB90: 48000028  b 0x827ddbb8
	pc = 0x827DDBB8; continue 'dispatch;
	// 827DDB94: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDB98: 894A001D  lbz r10, 0x1d(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDB9C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDBA0: 419A000C  beq cr6, 0x827ddbac
	if ctx.cr[6].eq {
	pc = 0x827DDBAC; continue 'dispatch;
	}
	// 827DDBA4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 827DDBA8: 48000010  b 0x827ddbb8
	pc = 0x827DDBB8; continue 'dispatch;
	// 827DDBAC: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDBB0: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DDBB4: 409A00DC  bne cr6, 0x827ddc90
	if !ctx.cr[6].eq {
	pc = 0x827DDC90; continue 'dispatch;
	}
	// 827DDBB8: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDBBC: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDBC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DDBC4: 409A0008  bne cr6, 0x827ddbcc
	if !ctx.cr[6].eq {
	pc = 0x827DDBCC; continue 'dispatch;
	}
	// 827DDBC8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827DDBCC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDBD0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDBD4: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DDBD8: 409A000C  bne cr6, 0x827ddbe4
	if !ctx.cr[6].eq {
	pc = 0x827DDBE4; continue 'dispatch;
	}
	// 827DDBDC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DDBE0: 4800001C  b 0x827ddbfc
	pc = 0x827DDBFC; continue 'dispatch;
	// 827DDBE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDBE8: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DDBEC: 409A000C  bne cr6, 0x827ddbf8
	if !ctx.cr[6].eq {
	pc = 0x827DDBF8; continue 'dispatch;
	}
	// 827DDBF0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DDBF4: 48000008  b 0x827ddbfc
	pc = 0x827DDBFC; continue 'dispatch;
	// 827DDBF8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DDBFC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDC00: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDC04: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DDC08: 409A003C  bne cr6, 0x827ddc44
	if !ctx.cr[6].eq {
	pc = 0x827DDC44; continue 'dispatch;
	}
	// 827DDC0C: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDC10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DDC14: 419A000C  beq cr6, 0x827ddc20
	if ctx.cr[6].eq {
	pc = 0x827DDC20; continue 'dispatch;
	}
	// 827DDC18: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827DDC1C: 48000024  b 0x827ddc40
	pc = 0x827DDC40; continue 'dispatch;
	// 827DDC20: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDC24: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827DDC28: 4800000C  b 0x827ddc34
	pc = 0x827DDC34; continue 'dispatch;
	// 827DDC2C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827DDC30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDC34: 890B001D  lbz r8, 0x1d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDC38: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827DDC3C: 419AFFF0  beq cr6, 0x827ddc2c
	if ctx.cr[6].eq {
	pc = 0x827DDC2C; continue 'dispatch;
	}
	// 827DDC40: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DDC44: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDC48: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDC4C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DDC50: 409A00D4  bne cr6, 0x827ddd24
	if !ctx.cr[6].eq {
	pc = 0x827DDD24; continue 'dispatch;
	}
	// 827DDC54: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDC58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DDC5C: 419A000C  beq cr6, 0x827ddc68
	if ctx.cr[6].eq {
	pc = 0x827DDC68; continue 'dispatch;
	}
	// 827DDC60: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827DDC64: 48000024  b 0x827ddc88
	pc = 0x827DDC88; continue 'dispatch;
	// 827DDC68: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDC6C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827DDC70: 4800000C  b 0x827ddc7c
	pc = 0x827DDC7C; continue 'dispatch;
	// 827DDC74: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827DDC78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDC7C: 890B001D  lbz r8, 0x1d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDC80: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827DDC84: 419AFFF0  beq cr6, 0x827ddc74
	if ctx.cr[6].eq {
	pc = 0x827DDC74; continue 'dispatch;
	}
	// 827DDC88: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DDC8C: 48000098  b 0x827ddd24
	pc = 0x827DDD24; continue 'dispatch;
	// 827DDC90: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827DDC94: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDC98: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DDC9C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDCA0: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DDCA4: 409A000C  bne cr6, 0x827ddcb0
	if !ctx.cr[6].eq {
	pc = 0x827DDCB0; continue 'dispatch;
	}
	// 827DDCA8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 827DDCAC: 4800002C  b 0x827ddcd8
	pc = 0x827DDCD8; continue 'dispatch;
	// 827DDCB0: 897C001D  lbz r11, 0x1d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDCB4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDCB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DDCBC: 409A0008  bne cr6, 0x827ddcc4
	if !ctx.cr[6].eq {
	pc = 0x827DDCC4; continue 'dispatch;
	}
	// 827DDCC0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827DDCC4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827DDCC8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDCCC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DDCD0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDCD4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827DDCD8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDCDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDCE0: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DDCE4: 409A000C  bne cr6, 0x827ddcf0
	if !ctx.cr[6].eq {
	pc = 0x827DDCF0; continue 'dispatch;
	}
	// 827DDCE8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827DDCEC: 48000020  b 0x827ddd0c
	pc = 0x827DDD0C; continue 'dispatch;
	// 827DDCF0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDCF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDCF8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 827DDCFC: 409A000C  bne cr6, 0x827ddd08
	if !ctx.cr[6].eq {
	pc = 0x827DDD08; continue 'dispatch;
	}
	// 827DDD00: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827DDD04: 48000008  b 0x827ddd0c
	pc = 0x827DDD0C; continue 'dispatch;
	// 827DDD08: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 827DDD0C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDD10: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DDD14: 897A001C  lbz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDD18: 8959001C  lbz r10, 0x1c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDD1C: 9979001C  stb r11, 0x1c(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 827DDD20: 995A001C  stb r10, 0x1c(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 827DDD24: 897A001C  lbz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDD28: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827DDD2C: 409A0198  bne cr6, 0x827ddec4
	if !ctx.cr[6].eq {
	pc = 0x827DDEC4; continue 'dispatch;
	}
	// 827DDD30: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDD34: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 827DDD38: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDD3C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DDD40: 419A0180  beq cr6, 0x827ddec0
	if ctx.cr[6].eq {
	pc = 0x827DDEC0; continue 'dispatch;
	}
	// 827DDD44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827DDD48: 897C001C  lbz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDD4C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827DDD50: 409A0170  bne cr6, 0x827ddec0
	if !ctx.cr[6].eq {
	pc = 0x827DDEC0; continue 'dispatch;
	}
	// 827DDD54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDD58: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DDD5C: 409A00A8  bne cr6, 0x827dde04
	if !ctx.cr[6].eq {
	pc = 0x827DDE04; continue 'dispatch;
	}
	// 827DDD60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDD64: 894B001C  lbz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDD68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDD6C: 409A001C  bne cr6, 0x827ddd88
	if !ctx.cr[6].eq {
	pc = 0x827DDD88; continue 'dispatch;
	}
	// 827DDD70: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDD74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DDD78: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DDD7C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDD80: 4BD51809  bl 0x8252f588
	ctx.lr = 0x827DDD84;
	sub_8252F588(ctx, base);
	// 827DDD84: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDD88: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDD8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDD90: 409A00C8  bne cr6, 0x827dde58
	if !ctx.cr[6].eq {
	pc = 0x827DDE58; continue 'dispatch;
	}
	// 827DDD94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDD98: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDD9C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDDA0: 409A0014  bne cr6, 0x827dddb4
	if !ctx.cr[6].eq {
	pc = 0x827DDDB4; continue 'dispatch;
	}
	// 827DDDA4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDDA8: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDDAC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDDB0: 419A00A4  beq cr6, 0x827dde54
	if ctx.cr[6].eq {
	pc = 0x827DDE54; continue 'dispatch;
	}
	// 827DDDB4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDDB8: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDDBC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDDC0: 409A0020  bne cr6, 0x827ddde0
	if !ctx.cr[6].eq {
	pc = 0x827DDDE0; continue 'dispatch;
	}
	// 827DDDC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDDC8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827DDDCC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDDD0: 9BCA001C  stb r30, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDDD4: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DDDD8: 4BD51819  bl 0x8252f5f0
	ctx.lr = 0x827DDDDC;
	sub_8252F5F0(ctx, base);
	// 827DDDDC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDDE0: 895F001C  lbz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDDE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DDDE8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDDEC: 994B001C  stb r10, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 827DDDF0: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDDF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDDF8: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDDFC: 4BD5178D  bl 0x8252f588
	ctx.lr = 0x827DDE00;
	sub_8252F588(ctx, base);
	// 827DDE00: 480000C0  b 0x827ddec0
	pc = 0x827DDEC0; continue 'dispatch;
	// 827DDE04: 894B001C  lbz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDE08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDE0C: 409A001C  bne cr6, 0x827dde28
	if !ctx.cr[6].eq {
	pc = 0x827DDE28; continue 'dispatch;
	}
	// 827DDE10: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDE14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DDE18: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DDE1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDE20: 4BD517D1  bl 0x8252f5f0
	ctx.lr = 0x827DDE24;
	sub_8252F5F0(ctx, base);
	// 827DDE24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDE28: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DDE2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDE30: 409A0028  bne cr6, 0x827dde58
	if !ctx.cr[6].eq {
	pc = 0x827DDE58; continue 'dispatch;
	}
	// 827DDE34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDE38: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDE3C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDE40: 409A0034  bne cr6, 0x827dde74
	if !ctx.cr[6].eq {
	pc = 0x827DDE74; continue 'dispatch;
	}
	// 827DDE44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDE48: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDE4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDE50: 409A0024  bne cr6, 0x827dde74
	if !ctx.cr[6].eq {
	pc = 0x827DDE74; continue 'dispatch;
	}
	// 827DDE54: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DDE58: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDE5C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 827DDE60: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDE64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDE68: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DDE6C: 409AFEDC  bne cr6, 0x827ddd48
	if !ctx.cr[6].eq {
	pc = 0x827DDD48; continue 'dispatch;
	}
	// 827DDE70: 48000050  b 0x827ddec0
	pc = 0x827DDEC0; continue 'dispatch;
	// 827DDE74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDE78: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDE7C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827DDE80: 409A0020  bne cr6, 0x827ddea0
	if !ctx.cr[6].eq {
	pc = 0x827DDEA0; continue 'dispatch;
	}
	// 827DDE84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDE88: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827DDE8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDE90: 9BCA001C  stb r30, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDE94: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 827DDE98: 4BD516F1  bl 0x8252f588
	ctx.lr = 0x827DDE9C;
	sub_8252F588(ctx, base);
	// 827DDE9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDEA0: 895F001C  lbz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 827DDEA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DDEA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DDEAC: 994B001C  stb r10, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 827DDEB0: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDEB4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDEB8: 9BCB001C  stb r30, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDEBC: 4BD51735  bl 0x8252f5f0
	ctx.lr = 0x827DDEC0;
	sub_8252F5F0(ctx, base);
	// 827DDEC0: 9BDC001C  stb r30, 0x1c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 827DDEC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827DDEC8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DDECC: 4BFFEE7D  bl 0x827dcd48
	ctx.lr = 0x827DDED0;
	sub_827DCD48(ctx, base);
	// 827DDED0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DDED4: 4BAE2395  bl 0x822c0268
	ctx.lr = 0x827DDED8;
	sub_822C0268(ctx, base);
	// 827DDED8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDEDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DDEE0: 419A000C  beq cr6, 0x827ddeec
	if ctx.cr[6].eq {
	pc = 0x827DDEEC; continue 'dispatch;
	}
	// 827DDEE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827DDEE8: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DDEEC: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827DDEF0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 827DDEF4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827DDEF8: 489CA2B0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DDF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DDF00 size=236
    let mut pc: u32 = 0x827DDF00;
    'dispatch: loop {
        match pc {
            0x827DDF00 => {
    //   block [0x827DDF00..0x827DDFEC)
	// 827DDF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DDF04: 489CA25D  bl 0x831a8160
	ctx.lr = 0x827DDF08;
	sub_831A8130(ctx, base);
	// 827DDF08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DDF0C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827DDF10: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 827DDF14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DDF18: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 827DDF1C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 827DDF20: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDF24: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDF28: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DDF2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DDF30: 409A0038  bne cr6, 0x827ddf68
	if !ctx.cr[6].eq {
	pc = 0x827DDF68; continue 'dispatch;
	}
	// 827DDF34: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDF38: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DDF3C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 827DDF40: 7D295010  subfc r9, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 827DDF44: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 827DDF48: 553D07FF  clrlwi. r29, r9, 0x1f
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827DDF4C: 4182000C  beq 0x827ddf58
	if ctx.cr[0].eq {
	pc = 0x827DDF58; continue 'dispatch;
	}
	// 827DDF50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDF54: 48000008  b 0x827ddf5c
	pc = 0x827DDF5C; continue 'dispatch;
	// 827DDF58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DDF5C: 892B0019  lbz r9, 0x19(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827DDF60: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 827DDF64: 419AFFD4  beq cr6, 0x827ddf38
	if ctx.cr[6].eq {
	pc = 0x827DDF38; continue 'dispatch;
	}
	// 827DDF68: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 827DDF6C: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DDF70: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DDF74: 41820044  beq 0x827ddfb8
	if ctx.cr[0].eq {
	pc = 0x827DDFB8; continue 'dispatch;
	}
	// 827DDF78: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DDF7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DDF80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDF84: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DDF88: 409A0028  bne cr6, 0x827ddfb0
	if !ctx.cr[6].eq {
	pc = 0x827DDFB0; continue 'dispatch;
	}
	// 827DDF8C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827DDF90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827DDF94: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827DDF98: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 827DDF9C: 4BFFF305  bl 0x827dd2a0
	ctx.lr = 0x827DDFA0;
	sub_827DD2A0(ctx, base);
	// 827DDFA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DDFA4: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 827DDFA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDFAC: 48000030  b 0x827ddfdc
	pc = 0x827DDFDC; continue 'dispatch;
	// 827DDFB0: 4BFCBE69  bl 0x827a9e18
	ctx.lr = 0x827DDFB4;
	sub_827A9E18(ctx, base);
	// 827DDFB4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DDFB8: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DDFBC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DDFC0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DDFC4: 40980010  bge cr6, 0x827ddfd4
	if !ctx.cr[6].lt {
	pc = 0x827DDFD4; continue 'dispatch;
	}
	// 827DDFC8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827DDFCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DDFD0: 4BFFFFC0  b 0x827ddf90
	pc = 0x827DDF90; continue 'dispatch;
	// 827DDFD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DDFD8: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 827DDFDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DDFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DDFE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DDFE8: 489CA1C8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DDFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DDFF0 size=132
    let mut pc: u32 = 0x827DDFF0;
    'dispatch: loop {
        match pc {
            0x827DDFF0 => {
    //   block [0x827DDFF0..0x827DE074)
	// 827DDFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DDFF4: 489CA175  bl 0x831a8168
	ctx.lr = 0x827DDFF8;
	sub_831A8130(ctx, base);
	// 827DDFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DDFFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DE000: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 827DE004: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827DE008: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827DE00C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE010: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE014: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE018: 409A0044  bne cr6, 0x827de05c
	if !ctx.cr[6].eq {
	pc = 0x827DE05C; continue 'dispatch;
	}
	// 827DE01C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DE020: 409A003C  bne cr6, 0x827de05c
	if !ctx.cr[6].eq {
	pc = 0x827DE05C; continue 'dispatch;
	}
	// 827DE024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE028: 4BFFF1C9  bl 0x827dd1f0
	ctx.lr = 0x827DE02C;
	sub_827DD1F0(ctx, base);
	// 827DE02C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE030: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE034: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DE038: 48000030  b 0x827de068
	pc = 0x827DE068; continue 'dispatch;
	// 827DE03C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 827DE040: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827DE044: 4BBC3645  bl 0x823a1688
	ctx.lr = 0x827DE048;
	sub_823A1688(ctx, base);
	// 827DE048: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827DE04C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DE050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DE054: 4BFFF6AD  bl 0x827dd700
	ctx.lr = 0x827DE058;
	sub_827DD700(ctx, base);
	// 827DE058: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 827DE05C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DE060: 409AFFDC  bne cr6, 0x827de03c
	if !ctx.cr[6].eq {
	pc = 0x827DE03C; continue 'dispatch;
	}
	// 827DE064: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 827DE068: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DE06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DE070: 489CA148  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DE078 size=312
    let mut pc: u32 = 0x827DE078;
    'dispatch: loop {
        match pc {
            0x827DE078 => {
    //   block [0x827DE078..0x827DE1B0)
	// 827DE078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE07C: 489CA0E5  bl 0x831a8160
	ctx.lr = 0x827DE080;
	sub_831A8130(ctx, base);
	// 827DE080: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE084: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 827DE088: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 827DE08C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DE090: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827DE094: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 827DE098: 83DB0004  lwz r30, 4(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE09C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE0A0: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DE0A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DE0A8: 409A0058  bne cr6, 0x827de100
	if !ctx.cr[6].eq {
	pc = 0x827DE100; continue 'dispatch;
	}
	// 827DE0AC: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE0B0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DE0B4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 827DE0B8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 827DE0BC: 41990020  bgt cr6, 0x827de0dc
	if ctx.cr[6].gt {
	pc = 0x827DE0DC; continue 'dispatch;
	}
	// 827DE0C0: 41980014  blt cr6, 0x827de0d4
	if ctx.cr[6].lt {
	pc = 0x827DE0D4; continue 'dispatch;
	}
	// 827DE0C4: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE0C8: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DE0CC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 827DE0D0: 4198000C  blt cr6, 0x827de0dc
	if ctx.cr[6].lt {
	pc = 0x827DE0DC; continue 'dispatch;
	}
	// 827DE0D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DE0D8: 48000008  b 0x827de0e0
	pc = 0x827DE0E0; continue 'dispatch;
	// 827DE0DC: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 827DE0E0: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827DE0E4: 4182000C  beq 0x827de0f0
	if ctx.cr[0].eq {
	pc = 0x827DE0F0; continue 'dispatch;
	}
	// 827DE0E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE0EC: 48000008  b 0x827de0f4
	pc = 0x827DE0F4; continue 'dispatch;
	// 827DE0F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DE0F4: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DE0F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DE0FC: 419AFFB4  beq cr6, 0x827de0b0
	if ctx.cr[6].eq {
	pc = 0x827DE0B0; continue 'dispatch;
	}
	// 827DE100: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 827DE104: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE108: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 827DE10C: 41820048  beq 0x827de154
	if ctx.cr[0].eq {
	pc = 0x827DE154; continue 'dispatch;
	}
	// 827DE110: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE114: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DE118: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE11C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DE120: 409A002C  bne cr6, 0x827de14c
	if !ctx.cr[6].eq {
	pc = 0x827DE14C; continue 'dispatch;
	}
	// 827DE124: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827DE128: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827DE12C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827DE130: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 827DE134: 4BFFF3A5  bl 0x827dd4d8
	ctx.lr = 0x827DE138;
	sub_827DD4D8(ctx, base);
	// 827DE138: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DE13C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 827DE140: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE144: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DE148: 4800005C  b 0x827de1a4
	pc = 0x827DE1A4; continue 'dispatch;
	// 827DE14C: 4807C93D  bl 0x8285aa88
	ctx.lr = 0x827DE150;
	sub_8285AA88(ctx, base);
	// 827DE150: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DE154: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE158: 8149000C  lwz r10, 0xc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DE15C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE160: 41990020  bgt cr6, 0x827de180
	if ctx.cr[6].gt {
	pc = 0x827DE180; continue 'dispatch;
	}
	// 827DE164: 41980014  blt cr6, 0x827de178
	if ctx.cr[6].lt {
	pc = 0x827DE178; continue 'dispatch;
	}
	// 827DE168: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DE16C: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE170: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE174: 4198000C  blt cr6, 0x827de180
	if ctx.cr[6].lt {
	pc = 0x827DE180; continue 'dispatch;
	}
	// 827DE178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DE17C: 48000008  b 0x827de184
	pc = 0x827DE184; continue 'dispatch;
	// 827DE180: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 827DE184: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE188: 41820010  beq 0x827de198
	if ctx.cr[0].eq {
	pc = 0x827DE198; continue 'dispatch;
	}
	// 827DE18C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827DE190: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DE194: 4BFFFF94  b 0x827de128
	pc = 0x827DE128; continue 'dispatch;
	// 827DE198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DE19C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DE1A0: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 827DE1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE1A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DE1AC: 489CA004  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DE1B0 size=132
    let mut pc: u32 = 0x827DE1B0;
    'dispatch: loop {
        match pc {
            0x827DE1B0 => {
    //   block [0x827DE1B0..0x827DE234)
	// 827DE1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE1B4: 489C9FB5  bl 0x831a8168
	ctx.lr = 0x827DE1B8;
	sub_831A8130(ctx, base);
	// 827DE1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE1BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DE1C0: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 827DE1C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827DE1C8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827DE1CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE1D0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE1D4: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE1D8: 409A0044  bne cr6, 0x827de21c
	if !ctx.cr[6].eq {
	pc = 0x827DE21C; continue 'dispatch;
	}
	// 827DE1DC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DE1E0: 409A003C  bne cr6, 0x827de21c
	if !ctx.cr[6].eq {
	pc = 0x827DE21C; continue 'dispatch;
	}
	// 827DE1E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE1E8: 4BFFF061  bl 0x827dd248
	ctx.lr = 0x827DE1EC;
	sub_827DD248(ctx, base);
	// 827DE1EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE1F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE1F4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DE1F8: 48000030  b 0x827de228
	pc = 0x827DE228; continue 'dispatch;
	// 827DE1FC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 827DE200: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827DE204: 4807C7B5  bl 0x8285a9b8
	ctx.lr = 0x827DE208;
	sub_8285A9B8(ctx, base);
	// 827DE208: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827DE20C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DE210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DE214: 4BFFF8ED  bl 0x827ddb00
	ctx.lr = 0x827DE218;
	sub_827DDB00(ctx, base);
	// 827DE218: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 827DE21C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DE220: 409AFFDC  bne cr6, 0x827de1fc
	if !ctx.cr[6].eq {
	pc = 0x827DE1FC; continue 'dispatch;
	}
	// 827DE224: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 827DE228: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DE22C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DE230: 489C9F88  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DE238 size=128
    let mut pc: u32 = 0x827DE238;
    'dispatch: loop {
        match pc {
            0x827DE238 => {
    //   block [0x827DE238..0x827DE2B8)
	// 827DE238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE23C: 489C9F31  bl 0x831a816c
	ctx.lr = 0x827DE240;
	sub_831A8130(ctx, base);
	// 827DE240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE244: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DE248: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DE24C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DE250: 3BEBA0B8  addi r31, r11, -0x5f48
	ctx.r[31].s64 = ctx.r[11].s64 + -24392;
	// 827DE254: 816AA0C0  lwz r11, -0x5f40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24384 as u32) ) } as u64;
	// 827DE258: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827DE25C: 40820024  bne 0x827de280
	if !ctx.cr[0].eq {
	pc = 0x827DE280; continue 'dispatch;
	}
	// 827DE260: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 827DE264: 3D00827E  lis r8, -0x7d82
	ctx.r[8].s64 = -2105671680;
	// 827DE268: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827DE26C: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 827DE270: 3908C858  addi r8, r8, -0x37a8
	ctx.r[8].s64 = ctx.r[8].s64 + -14248;
	// 827DE274: 916AA0C0  stw r11, -0x5f40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24384 as u32), ctx.r[11].u32 ) };
	// 827DE278: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827DE27C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827DE280: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DE284: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827DE288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE28C: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 827DE290: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827DE294: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DE298: 4BDF4689  bl 0x825d2920
	ctx.lr = 0x827DE29C;
	sub_825D2920(ctx, base);
	// 827DE29C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE2A0: 4182000C  beq 0x827de2ac
	if ctx.cr[0].eq {
	pc = 0x827DE2AC; continue 'dispatch;
	}
	// 827DE2A4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827DE2A8: 48000008  b 0x827de2b0
	pc = 0x827DE2B0; continue 'dispatch;
	// 827DE2AC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827DE2B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DE2B4: 489C9F08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DE2B8 size=128
    let mut pc: u32 = 0x827DE2B8;
    'dispatch: loop {
        match pc {
            0x827DE2B8 => {
    //   block [0x827DE2B8..0x827DE338)
	// 827DE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE2BC: 489C9EB1  bl 0x831a816c
	ctx.lr = 0x827DE2C0;
	sub_831A8130(ctx, base);
	// 827DE2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE2C4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DE2C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DE2CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DE2D0: 3BEBA0C4  addi r31, r11, -0x5f3c
	ctx.r[31].s64 = ctx.r[11].s64 + -24380;
	// 827DE2D4: 816AA0CC  lwz r11, -0x5f34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24372 as u32) ) } as u64;
	// 827DE2D8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827DE2DC: 40820024  bne 0x827de300
	if !ctx.cr[0].eq {
	pc = 0x827DE300; continue 'dispatch;
	}
	// 827DE2E0: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 827DE2E4: 3D00827E  lis r8, -0x7d82
	ctx.r[8].s64 = -2105671680;
	// 827DE2E8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827DE2EC: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 827DE2F0: 3908C8A0  addi r8, r8, -0x3760
	ctx.r[8].s64 = ctx.r[8].s64 + -14176;
	// 827DE2F4: 916AA0CC  stw r11, -0x5f34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24372 as u32), ctx.r[11].u32 ) };
	// 827DE2F8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827DE2FC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827DE300: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DE304: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827DE308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE30C: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 827DE310: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827DE314: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DE318: 4BDF4609  bl 0x825d2920
	ctx.lr = 0x827DE31C;
	sub_825D2920(ctx, base);
	// 827DE31C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE320: 4182000C  beq 0x827de32c
	if ctx.cr[0].eq {
	pc = 0x827DE32C; continue 'dispatch;
	}
	// 827DE324: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827DE328: 48000008  b 0x827de330
	pc = 0x827DE330; continue 'dispatch;
	// 827DE32C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827DE330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DE334: 489C9E88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DE338 size=128
    let mut pc: u32 = 0x827DE338;
    'dispatch: loop {
        match pc {
            0x827DE338 => {
    //   block [0x827DE338..0x827DE3B8)
	// 827DE338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE33C: 489C9E31  bl 0x831a816c
	ctx.lr = 0x827DE340;
	sub_831A8130(ctx, base);
	// 827DE340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE344: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DE348: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DE34C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DE350: 3BEBA0D0  addi r31, r11, -0x5f30
	ctx.r[31].s64 = ctx.r[11].s64 + -24368;
	// 827DE354: 816AA0D8  lwz r11, -0x5f28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24360 as u32) ) } as u64;
	// 827DE358: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827DE35C: 40820024  bne 0x827de380
	if !ctx.cr[0].eq {
	pc = 0x827DE380; continue 'dispatch;
	}
	// 827DE360: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 827DE364: 3D00827E  lis r8, -0x7d82
	ctx.r[8].s64 = -2105671680;
	// 827DE368: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827DE36C: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 827DE370: 3908C8E8  addi r8, r8, -0x3718
	ctx.r[8].s64 = ctx.r[8].s64 + -14104;
	// 827DE374: 916AA0D8  stw r11, -0x5f28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24360 as u32), ctx.r[11].u32 ) };
	// 827DE378: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827DE37C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827DE380: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DE384: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827DE388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE38C: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 827DE390: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827DE394: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DE398: 4BDF4589  bl 0x825d2920
	ctx.lr = 0x827DE39C;
	sub_825D2920(ctx, base);
	// 827DE39C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE3A0: 4182000C  beq 0x827de3ac
	if ctx.cr[0].eq {
	pc = 0x827DE3AC; continue 'dispatch;
	}
	// 827DE3A4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827DE3A8: 48000008  b 0x827de3b0
	pc = 0x827DE3B0; continue 'dispatch;
	// 827DE3AC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827DE3B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DE3B4: 489C9E08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DE3B8 size=552
    let mut pc: u32 = 0x827DE3B8;
    'dispatch: loop {
        match pc {
            0x827DE3B8 => {
    //   block [0x827DE3B8..0x827DE5E0)
	// 827DE3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE3BC: 489C9DA5  bl 0x831a8160
	ctx.lr = 0x827DE3C0;
	sub_831A8130(ctx, base);
	// 827DE3C0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE3C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 827DE3C8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827DE3CC: 93610124  stw r27, 0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), ctx.r[27].u32 ) };
	// 827DE3D0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 827DE3D4: 3B9D0104  addi r28, r29, 0x104
	ctx.r[28].s64 = ctx.r[29].s64 + 260;
	// 827DE3D8: 38A10124  addi r5, r1, 0x124
	ctx.r[5].s64 = ctx.r[1].s64 + 292;
	// 827DE3DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827DE3E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DE3E4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827DE3E8: 48140AE1  bl 0x8291eec8
	ctx.lr = 0x827DE3EC;
	sub_8291EEC8(ctx, base);
	// 827DE3EC: 817D0108  lwz r11, 0x108(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(264 as u32) ) } as u64;
	// 827DE3F0: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DE3F4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DE3F8: 409A01A4  bne cr6, 0x827de59c
	if !ctx.cr[6].eq {
	pc = 0x827DE59C; continue 'dispatch;
	}
	// 827DE3FC: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 827DE400: 4BAE2539  bl 0x822c0938
	ctx.lr = 0x827DE404;
	sub_822C0938(ctx, base);
	// 827DE404: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827DE408: 41820068  beq 0x827de470
	if ctx.cr[0].eq {
	pc = 0x827DE470; continue 'dispatch;
	}
	// 827DE40C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827DE410: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827DE414: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 827DE418: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827DE41C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827DE420: C1AB6218  lfs f13, 0x6218(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827DE424: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DE428: C18989AC  lfs f12, -0x7654(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827DE42C: D1A10080  stfs f13, 0x80(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 827DE430: D1A10084  stfs f13, 0x84(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 827DE434: D0010088  stfs f0, 0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 827DE438: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 827DE43C: D1A10090  stfs f13, 0x90(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827DE440: D1810094  stfs f12, 0x94(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827DE444: D0010098  stfs f0, 0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 827DE448: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 827DE44C: 4861B1DD  bl 0x82df9628
	ctx.lr = 0x827DE450;
	sub_82DF9628(ctx, base);
	// 827DE450: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 827DE454: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 827DE458: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 827DE45C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DE460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE464: 488E9D05  bl 0x830c8168
	ctx.lr = 0x827DE468;
	sub_830C8168(ctx, base);
	// 827DE468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DE46C: 48000008  b 0x827de474
	pc = 0x827DE474; continue 'dispatch;
	// 827DE470: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827DE474: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827DE478: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DE47C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DE480: 4BAEB1B9  bl 0x822c9638
	ctx.lr = 0x827DE484;
	sub_822C9638(ctx, base);
	// 827DE484: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827DE488: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DE48C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DE490: 4BAE1B71  bl 0x822c0000
	ctx.lr = 0x827DE494;
	sub_822C0000(ctx, base);
	// 827DE494: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827DE498: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DE49C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827DE4A0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 827DE4A4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827DE4A8: 419A0024  beq cr6, 0x827de4cc
	if ctx.cr[6].eq {
	pc = 0x827DE4CC; continue 'dispatch;
	}
	// 827DE4AC: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 827DE4B0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DE4B4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE4B8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DE4BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DE4C0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE4C4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE4C8: 4082FFE8  bne 0x827de4b0
	if !ctx.cr[0].eq {
	pc = 0x827DE4B0; continue 'dispatch;
	}
	// 827DE4CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DE4D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827DE4D4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827DE4D8: 4BCAE739  bl 0x8248cc10
	ctx.lr = 0x827DE4DC;
	sub_8248CC10(ctx, base);
	// 827DE4DC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DE4E0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE4E4: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE4E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DE4EC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827DE4F0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827DE4F4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 827DE4F8: 419A0024  beq cr6, 0x827de51c
	if ctx.cr[6].eq {
	pc = 0x827DE51C; continue 'dispatch;
	}
	// 827DE4FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DE500: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DE504: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE508: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DE50C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DE510: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE514: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE518: 4082FFE8  bne 0x827de500
	if !ctx.cr[0].eq {
	pc = 0x827DE500; continue 'dispatch;
	}
	// 827DE51C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827DE520: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827DE524: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DE528: 4BFF19B9  bl 0x827cfee0
	ctx.lr = 0x827DE52C;
	sub_827CFEE0(ctx, base);
	// 827DE52C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827DE530: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DE534: 419A0008  beq cr6, 0x827de53c
	if ctx.cr[6].eq {
	pc = 0x827DE53C; continue 'dispatch;
	}
	// 827DE538: 4BAE2359  bl 0x822c0890
	ctx.lr = 0x827DE53C;
	sub_822C0890(ctx, base);
	// 827DE53C: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 827DE540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DE544: 419A0008  beq cr6, 0x827de54c
	if ctx.cr[6].eq {
	pc = 0x827DE54C; continue 'dispatch;
	}
	// 827DE548: 4BAE2349  bl 0x822c0890
	ctx.lr = 0x827DE54C;
	sub_822C0890(ctx, base);
	// 827DE54C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827DE550: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827DE554: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 827DE558: 419A0024  beq cr6, 0x827de57c
	if ctx.cr[6].eq {
	pc = 0x827DE57C; continue 'dispatch;
	}
	// 827DE55C: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 827DE560: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DE564: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE568: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DE56C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DE570: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE574: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE578: 4082FFE8  bne 0x827de560
	if !ctx.cr[0].eq {
	pc = 0x827DE560; continue 'dispatch;
	}
	// 827DE57C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DE580: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DE584: 488E5395  bl 0x830c3918
	ctx.lr = 0x827DE588;
	sub_830C3918(ctx, base);
	// 827DE588: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827DE58C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827DE590: 419A000C  beq cr6, 0x827de59c
	if ctx.cr[6].eq {
	pc = 0x827DE59C; continue 'dispatch;
	}
	// 827DE594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DE598: 4BAE22F9  bl 0x822c0890
	ctx.lr = 0x827DE59C;
	sub_822C0890(ctx, base);
	// 827DE59C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DE5A0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DE5A4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DE5A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DE5AC: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DE5B0: 419A0024  beq cr6, 0x827de5d4
	if ctx.cr[6].eq {
	pc = 0x827DE5D4; continue 'dispatch;
	}
	// 827DE5B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DE5B8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DE5BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE5C0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DE5C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DE5C8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE5CC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE5D0: 4082FFE8  bne 0x827de5b8
	if !ctx.cr[0].eq {
	pc = 0x827DE5B8; continue 'dispatch;
	}
	// 827DE5D4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DE5D8: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 827DE5DC: 489C9BD4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DE5E0 size=696
    let mut pc: u32 = 0x827DE5E0;
    'dispatch: loop {
        match pc {
            0x827DE5E0 => {
    //   block [0x827DE5E0..0x827DE898)
	// 827DE5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE5E4: 489C9B79  bl 0x831a815c
	ctx.lr = 0x827DE5E8;
	sub_831A8130(ctx, base);
	// 827DE5E8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 827DE5EC: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE5F0: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 827DE5F4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 827DE5F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 827DE5FC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DE600: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 827DE604: C3E989AC  lfs f31, -0x7654(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827DE608: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 827DE60C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 827DE610: 3BCAB814  addi r30, r10, -0x47ec
	ctx.r[30].s64 = ctx.r[10].s64 + -18412;
	// 827DE614: 3B6BB870  addi r27, r11, -0x4790
	ctx.r[27].s64 = ctx.r[11].s64 + -18320;
	// 827DE618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DE61C: 7C9FF02E  lwzx r4, r31, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 827DE620: 7D7FF214  add r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 827DE624: 4861B055  bl 0x82df9678
	ctx.lr = 0x827DE628;
	sub_82DF9678(ctx, base);
	// 827DE628: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827DE62C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827DE630: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 827DE634: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DE638: 4BFFFD81  bl 0x827de3b8
	ctx.lr = 0x827DE63C;
	sub_827DE3B8(ctx, base);
	// 827DE63C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827DE640: 7C9FF02E  lwzx r4, r31, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 827DE644: 4BAEACAD  bl 0x822c92f0
	ctx.lr = 0x827DE648;
	sub_822C92F0(ctx, base);
	// 827DE648: 83A10060  lwz r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827DE64C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 827DE650: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DE654: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 827DE658: 387D0090  addi r3, r29, 0x90
	ctx.r[3].s64 = ctx.r[29].s64 + 144;
	// 827DE65C: 4BAEAAD5  bl 0x822c9130
	ctx.lr = 0x827DE660;
	sub_822C9130(ctx, base);
	// 827DE660: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DE664: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DE668: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827DE66C: 4BAEA7C5  bl 0x822c8e30
	ctx.lr = 0x827DE670;
	sub_822C8E30(ctx, base);
	// 827DE670: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827DE674: 7C1FDC2E  lfsx f0, r31, r27
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DE678: D01D0030  stfs f0, 0x30(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 827DE67C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DE680: D3FD0034  stfs f31, 0x34(r29)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827DE684: 419A0008  beq cr6, 0x827de68c
	if ctx.cr[6].eq {
	pc = 0x827DE68C; continue 'dispatch;
	}
	// 827DE688: 4BAE2209  bl 0x822c0890
	ctx.lr = 0x827DE68C;
	sub_822C0890(ctx, base);
	// 827DE68C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 827DE690: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 827DE694: 2B1F001C  cmplwi cr6, r31, 0x1c
	ctx.cr[6].compare_u32(ctx.r[31].u32, 28 as u32, &mut ctx.xer);
	// 827DE698: 4198FF80  blt cr6, 0x827de618
	if ctx.cr[6].lt {
	pc = 0x827DE618; continue 'dispatch;
	}
	// 827DE69C: 83B900D4  lwz r29, 0xd4(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(212 as u32) ) } as u64;
	// 827DE6A0: 83D900D0  lwz r30, 0xd0(r25)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(208 as u32) ) } as u64;
	// 827DE6A4: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 827DE6A8: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 827DE6AC: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 827DE6B0: 419A01DC  beq cr6, 0x827de88c
	if ctx.cr[6].eq {
	pc = 0x827DE88C; continue 'dispatch;
	}
	// 827DE6B4: 3F808336  lis r28, -0x7cca
	ctx.r[28].s64 = -2093613056;
	// 827DE6B8: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE6BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE6C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DE6C4: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 827DE6C8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 827DE6CC: 419A0024  beq cr6, 0x827de6f0
	if ctx.cr[6].eq {
	pc = 0x827DE6F0; continue 'dispatch;
	}
	// 827DE6D0: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 827DE6D4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 827DE6D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE6DC: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 827DE6E0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 827DE6E4: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE6E8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE6EC: 4082FFE8  bne 0x827de6d4
	if !ctx.cr[0].eq {
	pc = 0x827DE6D4; continue 'dispatch;
	}
	// 827DE6F0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DE6F4: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 827DE6F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827DE6FC: 4BAE5D65  bl 0x822c4460
	ctx.lr = 0x827DE700;
	sub_822C4460(ctx, base);
	// 827DE700: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DE704: 419A000C  beq cr6, 0x827de710
	if ctx.cr[6].eq {
	pc = 0x827DE710; continue 'dispatch;
	}
	// 827DE708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE70C: 4BAE2185  bl 0x822c0890
	ctx.lr = 0x827DE710;
	sub_822C0890(ctx, base);
	// 827DE710: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DE714: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827DE718: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DE71C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DE720: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DE724: 419900B8  bgt cr6, 0x827de7dc
	if ctx.cr[6].gt {
	pc = 0x827DE7DC; continue 'dispatch;
	}
	// 827DE728: 895CA0B6  lbz r10, -0x5f4a(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(-24394 as u32) ) } as u64;
	// 827DE72C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DE730: 408200AC  bne 0x827de7dc
	if !ctx.cr[0].eq {
	pc = 0x827DE7DC; continue 'dispatch;
	}
	// 827DE734: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE738: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DE73C: 419A0134  beq cr6, 0x827de870
	if ctx.cr[6].eq {
	pc = 0x827DE870; continue 'dispatch;
	}
	// 827DE740: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827DE744: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DE748: 419A0054  beq cr6, 0x827de79c
	if ctx.cr[6].eq {
	pc = 0x827DE79C; continue 'dispatch;
	}
	// 827DE74C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827DE750: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DE754: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 827DE758: 419A0024  beq cr6, 0x827de77c
	if ctx.cr[6].eq {
	pc = 0x827DE77C; continue 'dispatch;
	}
	// 827DE75C: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 827DE760: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 827DE764: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE768: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 827DE76C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 827DE770: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE774: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE778: 4082FFE8  bne 0x827de760
	if !ctx.cr[0].eq {
	pc = 0x827DE760; continue 'dispatch;
	}
	// 827DE77C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827DE780: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827DE784: 488E79BD  bl 0x830c6140
	ctx.lr = 0x827DE788;
	sub_830C6140(ctx, base);
	// 827DE788: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827DE78C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DE790: 419A00E0  beq cr6, 0x827de870
	if ctx.cr[6].eq {
	pc = 0x827DE870; continue 'dispatch;
	}
	// 827DE794: 4BAE20FD  bl 0x822c0890
	ctx.lr = 0x827DE798;
	sub_822C0890(ctx, base);
	// 827DE798: 480000D8  b 0x827de870
	pc = 0x827DE870; continue 'dispatch;
	// 827DE79C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 827DE7A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DE7A4: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 827DE7A8: 419A0024  beq cr6, 0x827de7cc
	if ctx.cr[6].eq {
	pc = 0x827DE7CC; continue 'dispatch;
	}
	// 827DE7AC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827DE7B0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DE7B4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE7B8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DE7BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DE7C0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE7C4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE7C8: 4082FFE8  bne 0x827de7b0
	if !ctx.cr[0].eq {
	pc = 0x827DE7B0; continue 'dispatch;
	}
	// 827DE7CC: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 827DE7D0: 807900C4  lwz r3, 0xc4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(196 as u32) ) } as u64;
	// 827DE7D4: 488E79E5  bl 0x830c61b8
	ctx.lr = 0x827DE7D8;
	sub_830C61B8(ctx, base);
	// 827DE7D8: 48000098  b 0x827de870
	pc = 0x827DE870; continue 'dispatch;
	// 827DE7DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE7E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DE7E4: 409A008C  bne cr6, 0x827de870
	if !ctx.cr[6].eq {
	pc = 0x827DE870; continue 'dispatch;
	}
	// 827DE7E8: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827DE7EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DE7F0: 419A0044  beq cr6, 0x827de834
	if ctx.cr[6].eq {
	pc = 0x827DE834; continue 'dispatch;
	}
	// 827DE7F4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827DE7F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DE7FC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 827DE800: 419A0024  beq cr6, 0x827de824
	if ctx.cr[6].eq {
	pc = 0x827DE824; continue 'dispatch;
	}
	// 827DE804: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 827DE808: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 827DE80C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE810: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 827DE814: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 827DE818: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE81C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE820: 4082FFE8  bne 0x827de808
	if !ctx.cr[0].eq {
	pc = 0x827DE808; continue 'dispatch;
	}
	// 827DE824: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827DE828: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827DE82C: 488E76D5  bl 0x830c5f00
	ctx.lr = 0x827DE830;
	sub_830C5F00(ctx, base);
	// 827DE830: 48000040  b 0x827de870
	pc = 0x827DE870; continue 'dispatch;
	// 827DE834: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 827DE838: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DE83C: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 827DE840: 419A0024  beq cr6, 0x827de864
	if ctx.cr[6].eq {
	pc = 0x827DE864; continue 'dispatch;
	}
	// 827DE844: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827DE848: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DE84C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE850: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DE854: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DE858: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DE85C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DE860: 4082FFE8  bne 0x827de848
	if !ctx.cr[0].eq {
	pc = 0x827DE848; continue 'dispatch;
	}
	// 827DE864: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 827DE868: 807900C4  lwz r3, 0xc4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(196 as u32) ) } as u64;
	// 827DE86C: 488E777D  bl 0x830c5fe8
	ctx.lr = 0x827DE870;
	sub_830C5FE8(ctx, base);
	// 827DE870: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 827DE874: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 827DE878: 409AFE40  bne cr6, 0x827de6b8
	if !ctx.cr[6].eq {
	pc = 0x827DE6B8; continue 'dispatch;
	}
	// 827DE87C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DE880: 419A000C  beq cr6, 0x827de88c
	if ctx.cr[6].eq {
	pc = 0x827DE88C; continue 'dispatch;
	}
	// 827DE884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DE888: 4BAE2009  bl 0x822c0890
	ctx.lr = 0x827DE88C;
	sub_822C0890(ctx, base);
	// 827DE88C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 827DE890: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 827DE894: 489C9918  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DE898 size=80
    let mut pc: u32 = 0x827DE898;
    'dispatch: loop {
        match pc {
            0x827DE898 => {
    //   block [0x827DE898..0x827DE8E8)
	// 827DE898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DE8A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DE8A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE8A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DE8AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DE8B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DE8B4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE8B8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE8BC: 4BFFF735  bl 0x827ddff0
	ctx.lr = 0x827DE8C0;
	sub_827DDFF0(ctx, base);
	// 827DE8C0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE8C4: 486135ED  bl 0x82df1eb0
	ctx.lr = 0x827DE8C8;
	sub_82DF1EB0(ctx, base);
	// 827DE8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DE8CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DE8D0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DE8D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DE8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DE8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DE8E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DE8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DE8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DE8E8 size=600
    let mut pc: u32 = 0x827DE8E8;
    'dispatch: loop {
        match pc {
            0x827DE8E8 => {
    //   block [0x827DE8E8..0x827DEB40)
	// 827DE8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DE8EC: 489C9875  bl 0x831a8160
	ctx.lr = 0x827DE8F0;
	sub_831A8130(ctx, base);
	// 827DE8F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DE8F4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 827DE8F8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827DE8FC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827DE900: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 827DE904: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DE908: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DE90C: 409A0020  bne cr6, 0x827de92c
	if !ctx.cr[6].eq {
	pc = 0x827DE92C; continue 'dispatch;
	}
	// 827DE910: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE914: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827DE918: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 827DE91C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827DE920: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827DE924: 4BFFEBB5  bl 0x827dd4d8
	ctx.lr = 0x827DE928;
	sub_827DD4D8(ctx, base);
	// 827DE928: 4800020C  b 0x827deb34
	pc = 0x827DEB34; continue 'dispatch;
	// 827DE92C: 837A0004  lwz r27, 4(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE930: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE934: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DE938: 409A0044  bne cr6, 0x827de97c
	if !ctx.cr[6].eq {
	pc = 0x827DE97C; continue 'dispatch;
	}
	// 827DE93C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE940: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DE944: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DE948: 41990020  bgt cr6, 0x827de968
	if ctx.cr[6].gt {
	pc = 0x827DE968; continue 'dispatch;
	}
	// 827DE94C: 41980014  blt cr6, 0x827de960
	if ctx.cr[6].lt {
	pc = 0x827DE960; continue 'dispatch;
	}
	// 827DE950: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE954: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DE958: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE95C: 4198000C  blt cr6, 0x827de968
	if ctx.cr[6].lt {
	pc = 0x827DE968; continue 'dispatch;
	}
	// 827DE960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DE964: 48000008  b 0x827de96c
	pc = 0x827DE96C; continue 'dispatch;
	// 827DE968: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DE96C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE970: 418201A8  beq 0x827deb18
	if ctx.cr[0].eq {
	pc = 0x827DEB18; continue 'dispatch;
	}
	// 827DE974: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 827DE978: 4BFFFF9C  b 0x827de914
	pc = 0x827DE914; continue 'dispatch;
	// 827DE97C: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827DE980: 409A0048  bne cr6, 0x827de9c8
	if !ctx.cr[6].eq {
	pc = 0x827DE9C8; continue 'dispatch;
	}
	// 827DE984: 80DB0008  lwz r6, 8(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DE988: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE98C: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DE990: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE994: 41990020  bgt cr6, 0x827de9b4
	if ctx.cr[6].gt {
	pc = 0x827DE9B4; continue 'dispatch;
	}
	// 827DE998: 41980014  blt cr6, 0x827de9ac
	if ctx.cr[6].lt {
	pc = 0x827DE9AC; continue 'dispatch;
	}
	// 827DE99C: 81660010  lwz r11, 0x10(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DE9A0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE9A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE9A8: 4198000C  blt cr6, 0x827de9b4
	if ctx.cr[6].lt {
	pc = 0x827DE9B4; continue 'dispatch;
	}
	// 827DE9AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DE9B0: 48000008  b 0x827de9b8
	pc = 0x827DE9B8; continue 'dispatch;
	// 827DE9B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DE9B8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE9BC: 4182015C  beq 0x827deb18
	if ctx.cr[0].eq {
	pc = 0x827DEB18; continue 'dispatch;
	}
	// 827DE9C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DE9C4: 4BFFFF54  b 0x827de918
	pc = 0x827DE918; continue 'dispatch;
	// 827DE9C8: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DE9CC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DE9D0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DE9D4: 41990020  bgt cr6, 0x827de9f4
	if ctx.cr[6].gt {
	pc = 0x827DE9F4; continue 'dispatch;
	}
	// 827DE9D8: 41980014  blt cr6, 0x827de9ec
	if ctx.cr[6].lt {
	pc = 0x827DE9EC; continue 'dispatch;
	}
	// 827DE9DC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DE9E0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DE9E4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DE9E8: 4198000C  blt cr6, 0x827de9f4
	if ctx.cr[6].lt {
	pc = 0x827DE9F4; continue 'dispatch;
	}
	// 827DE9EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DE9F0: 48000008  b 0x827de9f8
	pc = 0x827DE9F8; continue 'dispatch;
	// 827DE9F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DE9F8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DE9FC: 41820078  beq 0x827dea74
	if ctx.cr[0].eq {
	pc = 0x827DEA74; continue 'dispatch;
	}
	// 827DEA00: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827DEA04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DEA08: 4807C081  bl 0x8285aa88
	ctx.lr = 0x827DEA0C;
	sub_8285AA88(ctx, base);
	// 827DEA0C: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DEA10: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DEA14: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DEA18: 41990020  bgt cr6, 0x827dea38
	if ctx.cr[6].gt {
	pc = 0x827DEA38; continue 'dispatch;
	}
	// 827DEA1C: 41980014  blt cr6, 0x827dea30
	if ctx.cr[6].lt {
	pc = 0x827DEA30; continue 'dispatch;
	}
	// 827DEA20: 81660010  lwz r11, 0x10(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DEA24: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEA28: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DEA2C: 4198000C  blt cr6, 0x827dea38
	if ctx.cr[6].lt {
	pc = 0x827DEA38; continue 'dispatch;
	}
	// 827DEA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DEA34: 48000008  b 0x827dea3c
	pc = 0x827DEA3C; continue 'dispatch;
	// 827DEA38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DEA3C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DEA40: 41820034  beq 0x827dea74
	if ctx.cr[0].eq {
	pc = 0x827DEA74; continue 'dispatch;
	}
	// 827DEA44: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DEA48: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 827DEA4C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827DEA50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827DEA54: 896B001D  lbz r11, 0x1d(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DEA58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DEA5C: 419A000C  beq cr6, 0x827dea68
	if ctx.cr[6].eq {
	pc = 0x827DEA68; continue 'dispatch;
	}
	// 827DEA60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DEA64: 4BFFFEC0  b 0x827de924
	pc = 0x827DE924; continue 'dispatch;
	// 827DEA68: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 827DEA6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827DEA70: 4BFFFEB4  b 0x827de924
	pc = 0x827DE924; continue 'dispatch;
	// 827DEA74: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DEA78: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DEA7C: 41990020  bgt cr6, 0x827dea9c
	if ctx.cr[6].gt {
	pc = 0x827DEA9C; continue 'dispatch;
	}
	// 827DEA80: 41980014  blt cr6, 0x827dea94
	if ctx.cr[6].lt {
	pc = 0x827DEA94; continue 'dispatch;
	}
	// 827DEA84: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DEA88: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEA8C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DEA90: 4198000C  blt cr6, 0x827dea9c
	if ctx.cr[6].lt {
	pc = 0x827DEA9C; continue 'dispatch;
	}
	// 827DEA94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DEA98: 48000008  b 0x827deaa0
	pc = 0x827DEAA0; continue 'dispatch;
	// 827DEA9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DEAA0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DEAA4: 41820074  beq 0x827deb18
	if ctx.cr[0].eq {
	pc = 0x827DEB18; continue 'dispatch;
	}
	// 827DEAA8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827DEAAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DEAB0: 4807BF09  bl 0x8285a9b8
	ctx.lr = 0x827DEAB4;
	sub_8285A9B8(ctx, base);
	// 827DEAB4: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DEAB8: 7F06D840  cmplw cr6, r6, r27
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827DEABC: 419A0038  beq cr6, 0x827deaf4
	if ctx.cr[6].eq {
	pc = 0x827DEAF4; continue 'dispatch;
	}
	// 827DEAC0: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DEAC4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827DEAC8: 41990020  bgt cr6, 0x827deae8
	if ctx.cr[6].gt {
	pc = 0x827DEAE8; continue 'dispatch;
	}
	// 827DEACC: 41980014  blt cr6, 0x827deae0
	if ctx.cr[6].lt {
	pc = 0x827DEAE0; continue 'dispatch;
	}
	// 827DEAD0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEAD4: 81460010  lwz r10, 0x10(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DEAD8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DEADC: 4198000C  blt cr6, 0x827deae8
	if ctx.cr[6].lt {
	pc = 0x827DEAE8; continue 'dispatch;
	}
	// 827DEAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DEAE4: 48000008  b 0x827deaec
	pc = 0x827DEAEC; continue 'dispatch;
	// 827DEAE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DEAEC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DEAF0: 41820028  beq 0x827deb18
	if ctx.cr[0].eq {
	pc = 0x827DEB18; continue 'dispatch;
	}
	// 827DEAF4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DEAF8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 827DEAFC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827DEB00: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827DEB04: 896B001D  lbz r11, 0x1d(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 827DEB08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DEB0C: 419AFF60  beq cr6, 0x827dea6c
	if ctx.cr[6].eq {
	pc = 0x827DEA6C; continue 'dispatch;
	}
	// 827DEB10: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 827DEB14: 4BFFFF4C  b 0x827dea60
	pc = 0x827DEA60; continue 'dispatch;
	// 827DEB18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827DEB1C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827DEB20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DEB24: 4BFFF555  bl 0x827de078
	ctx.lr = 0x827DEB28;
	sub_827DE078(ctx, base);
	// 827DEB28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DEB2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DEB30: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DEB34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827DEB38: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DEB3C: 489C9674  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DEB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DEB40 size=80
    let mut pc: u32 = 0x827DEB40;
    'dispatch: loop {
        match pc {
            0x827DEB40 => {
    //   block [0x827DEB40..0x827DEB90)
	// 827DEB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DEB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DEB48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DEB4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DEB50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DEB54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DEB58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827DEB5C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEB60: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DEB64: 4BFFF64D  bl 0x827de1b0
	ctx.lr = 0x827DEB68;
	sub_827DE1B0(ctx, base);
	// 827DEB68: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEB6C: 4BAE16FD  bl 0x822c0268
	ctx.lr = 0x827DEB70;
	sub_822C0268(ctx, base);
	// 827DEB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DEB74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DEB78: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DEB7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DEB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DEB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DEB88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DEB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DEB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DEB90 size=128
    let mut pc: u32 = 0x827DEB90;
    'dispatch: loop {
        match pc {
            0x827DEB90 => {
    //   block [0x827DEB90..0x827DEC10)
	// 827DEB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DEB94: 489C95D9  bl 0x831a816c
	ctx.lr = 0x827DEB98;
	sub_831A8130(ctx, base);
	// 827DEB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DEB9C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827DEBA0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827DEBA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DEBA8: 3BEBA0DC  addi r31, r11, -0x5f24
	ctx.r[31].s64 = ctx.r[11].s64 + -24356;
	// 827DEBAC: 816AA0E4  lwz r11, -0x5f1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24348 as u32) ) } as u64;
	// 827DEBB0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827DEBB4: 40820024  bne 0x827debd8
	if !ctx.cr[0].eq {
	pc = 0x827DEBD8; continue 'dispatch;
	}
	// 827DEBB8: 3D20827E  lis r9, -0x7d82
	ctx.r[9].s64 = -2105671680;
	// 827DEBBC: 3D00827E  lis r8, -0x7d82
	ctx.r[8].s64 = -2105671680;
	// 827DEBC0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827DEBC4: 39297A40  addi r9, r9, 0x7a40
	ctx.r[9].s64 = ctx.r[9].s64 + 31296;
	// 827DEBC8: 3908C810  addi r8, r8, -0x37f0
	ctx.r[8].s64 = ctx.r[8].s64 + -14320;
	// 827DEBCC: 916AA0E4  stw r11, -0x5f1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24348 as u32), ctx.r[11].u32 ) };
	// 827DEBD0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827DEBD4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827DEBD8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827DEBDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827DEBE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DEBE4: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 827DEBE8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827DEBEC: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827DEBF0: 4BDF3D31  bl 0x825d2920
	ctx.lr = 0x827DEBF4;
	sub_825D2920(ctx, base);
	// 827DEBF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DEBF8: 4182000C  beq 0x827dec04
	if ctx.cr[0].eq {
	pc = 0x827DEC04; continue 'dispatch;
	}
	// 827DEBFC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827DEC00: 48000008  b 0x827dec08
	pc = 0x827DEC08; continue 'dispatch;
	// 827DEC04: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827DEC08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827DEC0C: 489C95B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DEC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DEC10 size=172
    let mut pc: u32 = 0x827DEC10;
    'dispatch: loop {
        match pc {
            0x827DEC10 => {
    //   block [0x827DEC10..0x827DECBC)
	// 827DEC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DEC14: 489C9559  bl 0x831a816c
	ctx.lr = 0x827DEC18;
	sub_831A8130(ctx, base);
	// 827DEC18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DEC1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827DEC20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DEC24: 4BFFDA5D  bl 0x827dc680
	ctx.lr = 0x827DEC28;
	sub_827DC680(ctx, base);
	// 827DEC28: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEC2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DEC30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827DEC34: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DEC38: 419A003C  beq cr6, 0x827dec74
	if ctx.cr[6].eq {
	pc = 0x827DEC74; continue 'dispatch;
	}
	// 827DEC3C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DEC40: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DEC44: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DEC48: 41990020  bgt cr6, 0x827dec68
	if ctx.cr[6].gt {
	pc = 0x827DEC68; continue 'dispatch;
	}
	// 827DEC4C: 41980014  blt cr6, 0x827dec60
	if ctx.cr[6].lt {
	pc = 0x827DEC60; continue 'dispatch;
	}
	// 827DEC50: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEC54: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DEC58: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DEC5C: 4198000C  blt cr6, 0x827dec68
	if ctx.cr[6].lt {
	pc = 0x827DEC68; continue 'dispatch;
	}
	// 827DEC60: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 827DEC64: 48000008  b 0x827dec6c
	pc = 0x827DEC6C; continue 'dispatch;
	// 827DEC68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DEC6C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DEC70: 41820040  beq 0x827decb0
	if ctx.cr[0].eq {
	pc = 0x827DECB0; continue 'dispatch;
	}
	// 827DEC74: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 827DEC78: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 827DEC7C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 827DEC80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827DEC84: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827DEC88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827DEC8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DEC90: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 827DEC94: 4BFFFC55  bl 0x827de8e8
	ctx.lr = 0x827DEC98;
	sub_827DE8E8(ctx, base);
	// 827DEC98: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827DEC9C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DECA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DECA4: 419A000C  beq cr6, 0x827decb0
	if ctx.cr[6].eq {
	pc = 0x827DECB0; continue 'dispatch;
	}
	// 827DECA8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827DECAC: 4BAE1BE5  bl 0x822c0890
	ctx.lr = 0x827DECB0;
	sub_822C0890(ctx, base);
	// 827DECB0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 827DECB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DECB8: 489C9504  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827DECC0 size=672
    let mut pc: u32 = 0x827DECC0;
    'dispatch: loop {
        match pc {
            0x827DECC0 => {
    //   block [0x827DECC0..0x827DEF60)
	// 827DECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DECC4: 489C94A1  bl 0x831a8164
	ctx.lr = 0x827DECC8;
	sub_831A8130(ctx, base);
	// 827DECC8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DECCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DECD0: 488E2791  bl 0x830c1460
	ctx.lr = 0x827DECD4;
	sub_830C1460(ctx, base);
	// 827DECD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DECD8: 4BFFF909  bl 0x827de5e0
	ctx.lr = 0x827DECDC;
	sub_827DE5E0(ctx, base);
	// 827DECDC: 3F60832B  lis r27, -0x7cd5
	ctx.r[27].s64 = -2094333952;
	// 827DECE0: 817BB868  lwz r11, -0x4798(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18328 as u32) ) } as u64;
	// 827DECE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DECE8: 897F0100  lbz r11, 0x100(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 827DECEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DECF0: 41980090  blt cr6, 0x827ded80
	if ctx.cr[6].lt {
	pc = 0x827DED80; continue 'dispatch;
	}
	// 827DECF4: 41820040  beq 0x827ded34
	if ctx.cr[0].eq {
	pc = 0x827DED34; continue 'dispatch;
	}
	// 827DECF8: 3BC00044  li r30, 0x44
	ctx.r[30].s64 = 68;
	// 827DECFC: 3BBF00E8  addi r29, r31, 0xe8
	ctx.r[29].s64 = ctx.r[31].s64 + 232;
	// 827DED00: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827DED04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DED08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DED0C: 483E0B55  bl 0x82bbf860
	ctx.lr = 0x827DED10;
	sub_82BBF860(ctx, base);
	// 827DED10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DED14: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 827DED18: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827DED1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DED20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DED24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DED28: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DED2C: 483E0B35  bl 0x82bbf860
	ctx.lr = 0x827DED30;
	sub_82BBF860(ctx, base);
	// 827DED30: 4800003C  b 0x827ded6c
	pc = 0x827DED6C; continue 'dispatch;
	// 827DED34: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 827DED38: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 827DED3C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DED40: 3BDF00F4  addi r30, r31, 0xf4
	ctx.r[30].s64 = ctx.r[31].s64 + 244;
	// 827DED44: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 827DED48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DED4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DED50: 4BFFFEC1  bl 0x827dec10
	ctx.lr = 0x827DED54;
	sub_827DEC10(ctx, base);
	// 827DED54: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DED58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DED5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DED60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DED64: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DED68: 4BFFFEA9  bl 0x827dec10
	ctx.lr = 0x827DED6C;
	sub_827DEC10(ctx, base);
	// 827DED6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DED70: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DED74: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 827DED78: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DED7C: 48000098  b 0x827dee14
	pc = 0x827DEE14; continue 'dispatch;
	// 827DED80: 41820044  beq 0x827dedc4
	if ctx.cr[0].eq {
	pc = 0x827DEDC4; continue 'dispatch;
	}
	// 827DED84: 3BC00044  li r30, 0x44
	ctx.r[30].s64 = 68;
	// 827DED88: 3BBF00E8  addi r29, r31, 0xe8
	ctx.r[29].s64 = ctx.r[31].s64 + 232;
	// 827DED8C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827DED90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DED94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DED98: 483E0AC9  bl 0x82bbf860
	ctx.lr = 0x827DED9C;
	sub_82BBF860(ctx, base);
	// 827DED9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DEDA0: 815F00E0  lwz r10, 0xe0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 827DEDA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DEDA8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827DEDAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827DEDB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DEDB4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DEDB8: 483E0AA9  bl 0x82bbf860
	ctx.lr = 0x827DEDBC;
	sub_82BBF860(ctx, base);
	// 827DEDBC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 827DEDC0: 48000040  b 0x827dee00
	pc = 0x827DEE00; continue 'dispatch;
	// 827DEDC4: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 827DEDC8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 827DEDCC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827DEDD0: 3BDF00F4  addi r30, r31, 0xf4
	ctx.r[30].s64 = ctx.r[31].s64 + 244;
	// 827DEDD4: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 827DEDD8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DEDDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DEDE0: 4BFFFE31  bl 0x827dec10
	ctx.lr = 0x827DEDE4;
	sub_827DEC10(ctx, base);
	// 827DEDE4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DEDE8: 815F00E0  lwz r10, 0xe0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 827DEDEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827DEDF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DEDF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DEDF8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DEDFC: 4BFFFE15  bl 0x827dec10
	ctx.lr = 0x827DEE00;
	sub_827DEC10(ctx, base);
	// 827DEE00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DEE04: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DEE08: 817F00DC  lwz r11, 0xdc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 827DEE0C: 815F00BC  lwz r10, 0xbc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 827DEE10: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DEE14: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 827DEE18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DEE1C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827DEE20: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 827DEE24: 4BFFE045  bl 0x827dce68
	ctx.lr = 0x827DEE28;
	sub_827DCE68(ctx, base);
	// 827DEE28: 817BB868  lwz r11, -0x4798(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18328 as u32) ) } as u64;
	// 827DEE2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DEE30: 41980040  blt cr6, 0x827dee70
	if ctx.cr[6].lt {
	pc = 0x827DEE70; continue 'dispatch;
	}
	// 827DEE34: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 827DEE38: 5564043E  clrlwi r4, r11, 0x10
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 827DEE3C: 806A110C  lwz r3, 0x110c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827DEE40: 48613129  bl 0x82df1f68
	ctx.lr = 0x827DEE44;
	sub_82DF1F68(ctx, base);
	// 827DEE44: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 827DEE48: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DEE4C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DEE50: 40990014  ble cr6, 0x827dee64
	if !ctx.cr[6].gt {
	pc = 0x827DEE64; continue 'dispatch;
	}
	// 827DEE54: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEE58: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 827DEE5C: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827DEE60: 48000024  b 0x827dee84
	pc = 0x827DEE84; continue 'dispatch;
	// 827DEE64: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEE68: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827DEE6C: 48000018  b 0x827dee84
	pc = 0x827DEE84; continue 'dispatch;
	// 827DEE70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DEE74: 483EE33D  bl 0x82bcd1b0
	ctx.lr = 0x827DEE78;
	sub_82BCD1B0(ctx, base);
	// 827DEE78: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEE7C: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827DEE80: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DEE84: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEE88: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEE8C: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 827DEE90: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEE94: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DEE98: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEE9C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DEEA0: 40990008  ble cr6, 0x827deea8
	if !ctx.cr[6].gt {
	pc = 0x827DEEA8; continue 'dispatch;
	}
	// 827DEEA4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 827DEEA8: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEEAC: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827DEEB0: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEEB4: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DEEB8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEEBC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DEEC0: 41980008  blt cr6, 0x827deec8
	if ctx.cr[6].lt {
	pc = 0x827DEEC8; continue 'dispatch;
	}
	// 827DEEC4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827DEEC8: 813F00B4  lwz r9, 0xb4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEECC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DEED0: 396BB858  addi r11, r11, -0x47a8
	ctx.r[11].s64 = ctx.r[11].s64 + -18344;
	// 827DEED4: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 827DEED8: 811F00B4  lwz r8, 0xb4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEEDC: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DEEE0: 815F00AC  lwz r10, 0xac(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 827DEEE4: 812A003C  lwz r9, 0x3c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DEEE8: 81480004  lwz r10, 4(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEEEC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 827DEEF0: 91480038  stw r10, 0x38(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 827DEEF4: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEEF8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEEFC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 827DEF00: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827DEF04: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 827DEF08: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 827DEF0C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827DEF10: 40980014  bge cr6, 0x827def24
	if !ctx.cr[6].lt {
	pc = 0x827DEF24; continue 'dispatch;
	}
	// 827DEF14: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEF18: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 827DEF1C: 914B0034  stw r10, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 827DEF20: 48000038  b 0x827def58
	pc = 0x827DEF58; continue 'dispatch;
	// 827DEF24: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827DEF28: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827DEF2C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DEF30: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 827DEF34: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827DEF38: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 827DEF3C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 827DEF40: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827DEF44: 40980010  bge cr6, 0x827def54
	if !ctx.cr[6].lt {
	pc = 0x827DEF54; continue 'dispatch;
	}
	// 827DEF48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827DEF4C: 916A0034  stw r11, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 827DEF50: 48000008  b 0x827def58
	pc = 0x827DEF58; continue 'dispatch;
	// 827DEF54: 938A0034  stw r28, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[28].u32 ) };
	// 827DEF58: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827DEF5C: 489C9258  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DEF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DEF60 size=120
    let mut pc: u32 = 0x827DEF60;
    'dispatch: loop {
        match pc {
            0x827DEF60 => {
    //   block [0x827DEF60..0x827DEFD8)
	// 827DEF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DEF64: 489C91F9  bl 0x831a815c
	ctx.lr = 0x827DEF68;
	sub_831A8130(ctx, base);
	// 827DEF68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DEF6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DEF70: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827DEF74: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 827DEF78: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DEF7C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827DEF80: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 827DEF84: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 827DEF88: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 827DEF8C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 827DEF90: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 827DEF94: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 827DEF98: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 827DEF9C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 827DEFA0: 4BFFDCE1  bl 0x827dcc80
	ctx.lr = 0x827DEFA4;
	sub_827DCC80(ctx, base);
	// 827DEFA4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 827DEFA8: 935F0020  stw r26, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 827DEFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DEFB0: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 827DEFB4: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 827DEFB8: 937F002C  stw r27, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 827DEFBC: 9B3F0030  stb r25, 0x30(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[25].u8 ) };
	// 827DEFC0: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 827DEFC4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 827DEFC8: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 827DEFCC: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 827DEFD0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DEFD4: 489C91D8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DEFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DEFD8 size=180
    let mut pc: u32 = 0x827DEFD8;
    'dispatch: loop {
        match pc {
            0x827DEFD8 => {
    //   block [0x827DEFD8..0x827DF08C)
	// 827DEFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DEFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DEFE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DEFE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DEFE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DEFEC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DEFF0: 396B55CC  addi r11, r11, 0x55cc
	ctx.r[11].s64 = ctx.r[11].s64 + 21964;
	// 827DEFF4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DEFF8: 807F0114  lwz r3, 0x114(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 827DEFFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DF000: 419A0008  beq cr6, 0x827df008
	if ctx.cr[6].eq {
	pc = 0x827DF008; continue 'dispatch;
	}
	// 827DF004: 4BAE188D  bl 0x822c0890
	ctx.lr = 0x827DF008;
	sub_822C0890(ctx, base);
	// 827DF008: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 827DF00C: 4830E2ED  bl 0x82aed2f8
	ctx.lr = 0x827DF010;
	sub_82AED2F8(ctx, base);
	// 827DF010: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 827DF014: 4BFFFB2D  bl 0x827deb40
	ctx.lr = 0x827DF018;
	sub_827DEB40(ctx, base);
	// 827DF018: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 827DF01C: 483E090D  bl 0x82bbf928
	ctx.lr = 0x827DF020;
	sub_82BBF928(ctx, base);
	// 827DF020: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 827DF024: 4BCD227D  bl 0x824b12a0
	ctx.lr = 0x827DF028;
	sub_824B12A0(ctx, base);
	// 827DF028: 807F00C8  lwz r3, 0xc8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 827DF02C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DF030: 419A0008  beq cr6, 0x827df038
	if ctx.cr[6].eq {
	pc = 0x827DF038; continue 'dispatch;
	}
	// 827DF034: 4BAE185D  bl 0x822c0890
	ctx.lr = 0x827DF038;
	sub_822C0890(ctx, base);
	// 827DF038: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 827DF03C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DF040: 419A0008  beq cr6, 0x827df048
	if ctx.cr[6].eq {
	pc = 0x827DF048; continue 'dispatch;
	}
	// 827DF044: 4BAE184D  bl 0x822c0890
	ctx.lr = 0x827DF048;
	sub_822C0890(ctx, base);
	// 827DF048: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 827DF04C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DF050: 419A0008  beq cr6, 0x827df058
	if ctx.cr[6].eq {
	pc = 0x827DF058; continue 'dispatch;
	}
	// 827DF054: 4BAE183D  bl 0x822c0890
	ctx.lr = 0x827DF058;
	sub_822C0890(ctx, base);
	// 827DF058: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 827DF05C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DF060: 419A0008  beq cr6, 0x827df068
	if ctx.cr[6].eq {
	pc = 0x827DF068; continue 'dispatch;
	}
	// 827DF064: 4BAE182D  bl 0x822c0890
	ctx.lr = 0x827DF068;
	sub_822C0890(ctx, base);
	// 827DF068: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 827DF06C: 4BFFF82D  bl 0x827de898
	ctx.lr = 0x827DF070;
	sub_827DE898(ctx, base);
	// 827DF070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF074: 488E3A75  bl 0x830c2ae8
	ctx.lr = 0x827DF078;
	sub_830C2AE8(ctx, base);
	// 827DF078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DF07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF090 size=76
    let mut pc: u32 = 0x827DF090;
    'dispatch: loop {
        match pc {
            0x827DF090 => {
    //   block [0x827DF090..0x827DF0DC)
	// 827DF090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DF09C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF0A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF0A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DF0AC: 4BFFFF2D  bl 0x827defd8
	ctx.lr = 0x827DF0B0;
	sub_827DEFD8(ctx, base);
	// 827DF0B0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DF0B4: 4182000C  beq 0x827df0c0
	if ctx.cr[0].eq {
	pc = 0x827DF0C0; continue 'dispatch;
	}
	// 827DF0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF0BC: 4BAE11AD  bl 0x822c0268
	ctx.lr = 0x827DF0C0;
	sub_822C0268(ctx, base);
	// 827DF0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF0C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF0D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DF0D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF0E0 size=188
    let mut pc: u32 = 0x827DF0E0;
    'dispatch: loop {
        match pc {
            0x827DF0E0 => {
    //   block [0x827DF0E0..0x827DF19C)
	// 827DF0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF0E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DF0EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF0F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF0F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DF0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DF0FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827DF100: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827DF104: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF108: 4BAE1831  bl 0x822c0938
	ctx.lr = 0x827DF10C;
	sub_822C0938(ctx, base);
	// 827DF10C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827DF110: 41820028  beq 0x827df138
	if ctx.cr[0].eq {
	pc = 0x827DF138; continue 'dispatch;
	}
	// 827DF114: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DF118: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827DF11C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DF120: 392B55B4  addi r9, r11, 0x55b4
	ctx.r[9].s64 = ctx.r[11].s64 + 21940;
	// 827DF124: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827DF128: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DF12C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DF130: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827DF134: 48000008  b 0x827df13c
	pc = 0x827DF13C; continue 'dispatch;
	// 827DF138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DF13C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DF144: 409A003C  bne cr6, 0x827df180
	if !ctx.cr[6].eq {
	pc = 0x827DF180; continue 'dispatch;
	}
	// 827DF148: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DF14C: 419A0014  beq cr6, 0x827df160
	if ctx.cr[6].eq {
	pc = 0x827DF160; continue 'dispatch;
	}
	// 827DF150: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 827DF154: 4BFFF745  bl 0x827de898
	ctx.lr = 0x827DF158;
	sub_827DE898(ctx, base);
	// 827DF158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF15C: 4BAE110D  bl 0x822c0268
	ctx.lr = 0x827DF160;
	sub_822C0268(ctx, base);
	// 827DF160: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827DF164: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827DF168: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DF16C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827DF170: 816BB86C  lwz r11, -0x4794(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18324 as u32) ) } as u64;
	// 827DF174: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827DF178: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827DF17C: 4BAE0E85  bl 0x822c0000
	ctx.lr = 0x827DF180;
	sub_822C0000(ctx, base);
	// 827DF180: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DF184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF190: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DF194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF1A0 size=64
    let mut pc: u32 = 0x827DF1A0;
    'dispatch: loop {
        match pc {
            0x827DF1A0 => {
    //   block [0x827DF1A0..0x827DF1E0)
	// 827DF1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF1A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF1AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF1B0: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DF1B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DF1B8: 419A0014  beq cr6, 0x827df1cc
	if ctx.cr[6].eq {
	pc = 0x827DF1CC; continue 'dispatch;
	}
	// 827DF1BC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 827DF1C0: 4BFFF6D9  bl 0x827de898
	ctx.lr = 0x827DF1C4;
	sub_827DE898(ctx, base);
	// 827DF1C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF1C8: 4BAE10A1  bl 0x822c0268
	ctx.lr = 0x827DF1CC;
	sub_822C0268(ctx, base);
	// 827DF1CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DF1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF1D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF1E0 size=80
    let mut pc: u32 = 0x827DF1E0;
    'dispatch: loop {
        match pc {
            0x827DF1E0 => {
    //   block [0x827DF1E0..0x827DF230)
	// 827DF1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF1E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF1EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF1F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF1F4: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DF1F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DF1FC: 419A0008  beq cr6, 0x827df204
	if ctx.cr[6].eq {
	pc = 0x827DF204; continue 'dispatch;
	}
	// 827DF200: 4BAE1691  bl 0x822c0890
	ctx.lr = 0x827DF204;
	sub_822C0890(ctx, base);
	// 827DF204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827DF208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DF20C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 827DF210: 4BAE9C21  bl 0x822c8e30
	ctx.lr = 0x827DF214;
	sub_822C8E30(ctx, base);
	// 827DF214: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 827DF218: 4BAEB3D9  bl 0x822ca5f0
	ctx.lr = 0x827DF21C;
	sub_822CA5F0(ctx, base);
	// 827DF21C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DF220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF230 size=164
    let mut pc: u32 = 0x827DF230;
    'dispatch: loop {
        match pc {
            0x827DF230 => {
    //   block [0x827DF230..0x827DF2D4)
	// 827DF230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF234: 489C8F39  bl 0x831a816c
	ctx.lr = 0x827DF238;
	sub_831A8130(ctx, base);
	// 827DF238: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF23C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DF240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DF248: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DF24C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF250: 4861A101  bl 0x82df9350
	ctx.lr = 0x827DF254;
	sub_82DF9350(ctx, base);
	// 827DF254: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DF258: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827DF25C: 4861A52D  bl 0x82df9788
	ctx.lr = 0x827DF260;
	sub_82DF9788(ctx, base);
	// 827DF260: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827DF264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF268: 4BAEB3E1  bl 0x822ca648
	ctx.lr = 0x827DF26C;
	sub_822CA648(ctx, base);
	// 827DF26C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DF270: 486141B9  bl 0x82df3428
	ctx.lr = 0x827DF274;
	sub_82DF3428(ctx, base);
	// 827DF274: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DF278: 397F003C  addi r11, r31, 0x3c
	ctx.r[11].s64 = ctx.r[31].s64 + 60;
	// 827DF27C: 396A561C  addi r11, r10, 0x561c
	ctx.r[11].s64 = ctx.r[10].s64 + 22044;
	// 827DF280: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF284: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF288: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 827DF28C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DF294: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 827DF298: 419A0024  beq cr6, 0x827df2bc
	if ctx.cr[6].eq {
	pc = 0x827DF2BC; continue 'dispatch;
	}
	// 827DF29C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DF2A0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827DF2A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DF2A8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827DF2AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DF2B0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827DF2B4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827DF2B8: 4082FFE8  bne 0x827df2a0
	if !ctx.cr[0].eq {
	pc = 0x827DF2A0; continue 'dispatch;
	}
	// 827DF2BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DF2C0: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 827DF2C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF2C8: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 827DF2CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827DF2D0: 489C8EEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF2D8 size=92
    let mut pc: u32 = 0x827DF2D8;
    'dispatch: loop {
        match pc {
            0x827DF2D8 => {
    //   block [0x827DF2D8..0x827DF334)
	// 827DF2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF2E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF2E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF2E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF2EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827DF2F0: 4BBC1CA1  bl 0x823a0f90
	ctx.lr = 0x827DF2F4;
	sub_823A0F90(ctx, base);
	// 827DF2F4: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 827DF2F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF2FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827DF300: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF304: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827DF308: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 827DF30C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827DF310: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827DF314: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 827DF318: 48611741  bl 0x82df0a58
	ctx.lr = 0x827DF31C;
	sub_82DF0A58(ctx, base);
	// 827DF31C: F87F0018  std r3, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u64 ) };
	// 827DF320: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF32C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF338 size=84
    let mut pc: u32 = 0x827DF338;
    'dispatch: loop {
        match pc {
            0x827DF338 => {
    //   block [0x827DF338..0x827DF38C)
	// 827DF338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF34C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 827DF350: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827DF354: 419A0010  beq cr6, 0x827df364
	if ctx.cr[6].eq {
	pc = 0x827DF364; continue 'dispatch;
	}
	// 827DF358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF35C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF360: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 827DF364: 808A0004  lwz r4, 4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF368: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF36C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DF370: 4E800421  bctrl
	ctx.lr = 0x827DF374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DF374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DF37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF384: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DF390 size=100
    let mut pc: u32 = 0x827DF390;
    'dispatch: loop {
        match pc {
            0x827DF390 => {
    //   block [0x827DF390..0x827DF3F4)
	// 827DF390: 8163FFF4  lwz r11, -0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-12 as u32) ) } as u64;
	// 827DF394: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DF398: 3523FFF4  addic. r9, r3, -0xc
	ctx.xer.ca = (ctx.r[3].u32 > (!(-12 as u32)));
	ctx.r[9].s64 = ctx.r[3].s64 + -12;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827DF39C: 394A5640  addi r10, r10, 0x5640
	ctx.r[10].s64 = ctx.r[10].s64 + 22080;
	// 827DF3A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF3A4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 827DF3A8: 914BFFF4  stw r10, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[10].u32 ) };
	// 827DF3AC: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 827DF3B0: 40820008  bne 0x827df3b8
	if !ctx.cr[0].eq {
	pc = 0x827DF3B8; continue 'dispatch;
	}
	// 827DF3B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DF3B8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF3BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827DF3C0: 3D008207  lis r8, -0x7df9
	ctx.r[8].s64 = -2113470464;
	// 827DF3C4: 3943FFFC  addi r10, r3, -4
	ctx.r[10].s64 = ctx.r[3].s64 + -4;
	// 827DF3C8: 39085630  addi r8, r8, 0x5630
	ctx.r[8].s64 = ctx.r[8].s64 + 22064;
	// 827DF3CC: 3CE08207  lis r7, -0x7df9
	ctx.r[7].s64 = -2113470464;
	// 827DF3D0: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF3D4: 38E75638  addi r7, r7, 0x5638
	ctx.r[7].s64 = ctx.r[7].s64 + 22072;
	// 827DF3D8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 827DF3DC: 910BFFFC  stw r8, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 827DF3E0: 8163FFF4  lwz r11, -0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-12 as u32) ) } as u64;
	// 827DF3E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF3E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827DF3EC: 90EBFFF8  stw r7, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 827DF3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF3F8 size=80
    let mut pc: u32 = 0x827DF3F8;
    'dispatch: loop {
        match pc {
            0x827DF3F8 => {
    //   block [0x827DF3F8..0x827DF448)
	// 827DF3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DF404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF40C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DF410: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 827DF414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF418: 4BFFFF79  bl 0x827df390
	ctx.lr = 0x827DF41C;
	sub_827DF390(ctx, base);
	// 827DF41C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827DF420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF424: 396BBF8C  addi r11, r11, -0x4074
	ctx.r[11].s64 = ctx.r[11].s64 + -16500;
	// 827DF428: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827DF42C: 48611FB5  bl 0x82df13e0
	ctx.lr = 0x827DF430;
	sub_82DF13E0(ctx, base);
	// 827DF430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF43C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DF440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF448 size=188
    let mut pc: u32 = 0x827DF448;
    'dispatch: loop {
        match pc {
            0x827DF448 => {
    //   block [0x827DF448..0x827DF504)
	// 827DF448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF44C: 489C8D21  bl 0x831a816c
	ctx.lr = 0x827DF450;
	sub_831A8130(ctx, base);
	// 827DF450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF458: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DF45C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827DF460: 396B5648  addi r11, r11, 0x5648
	ctx.r[11].s64 = ctx.r[11].s64 + 22088;
	// 827DF464: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF468: 48611FF1  bl 0x82df1458
	ctx.lr = 0x827DF46C;
	sub_82DF1458(ctx, base);
	// 827DF46C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 827DF470: 4BAE14C9  bl 0x822c0938
	ctx.lr = 0x827DF474;
	sub_822C0938(ctx, base);
	// 827DF474: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827DF478: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827DF47C: 41820018  beq 0x827df494
	if ctx.cr[0].eq {
	pc = 0x827DF494; continue 'dispatch;
	}
	// 827DF480: 48611C61  bl 0x82df10e0
	ctx.lr = 0x827DF484;
	sub_82DF10E0(ctx, base);
	// 827DF484: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 827DF488: 48611861  bl 0x82df0ce8
	ctx.lr = 0x827DF48C;
	sub_82DF0CE8(ctx, base);
	// 827DF48C: 4BBC1BCD  bl 0x823a1058
	ctx.lr = 0x827DF490;
	sub_823A1058(ctx, base);
	// 827DF490: 48000008  b 0x827df498
	pc = 0x827DF498; continue 'dispatch;
	// 827DF494: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 827DF498: 393F0008  addi r9, r31, 8
	ctx.r[9].s64 = ctx.r[31].s64 + 8;
	// 827DF49C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 827DF4A0: 393F0018  addi r9, r31, 0x18
	ctx.r[9].s64 = ctx.r[31].s64 + 24;
	// 827DF4A4: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 827DF4A8: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 827DF4AC: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 827DF4B0: 393F0028  addi r9, r31, 0x28
	ctx.r[9].s64 = ctx.r[31].s64 + 40;
	// 827DF4B4: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 827DF4B8: 395F001C  addi r10, r31, 0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + 28;
	// 827DF4BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 827DF4C0: 393F002C  addi r9, r31, 0x2c
	ctx.r[9].s64 = ctx.r[31].s64 + 44;
	// 827DF4C4: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 827DF4C8: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 827DF4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF4D0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 827DF4D4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DF4D8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DF4DC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF4E0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DF4E4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DF4E8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DF4EC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DF4F0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DF4F4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DF4F8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DF4FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF500: 489C8CBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DF508 size=8
    let mut pc: u32 = 0x827DF508;
    'dispatch: loop {
        match pc {
            0x827DF508 => {
    //   block [0x827DF508..0x827DF510)
	// 827DF508: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827DF50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF510 size=80
    let mut pc: u32 = 0x827DF510;
    'dispatch: loop {
        match pc {
            0x827DF510 => {
    //   block [0x827DF510..0x827DF560)
	// 827DF510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF51C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF520: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF524: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827DF528: 396B5648  addi r11, r11, 0x5648
	ctx.r[11].s64 = ctx.r[11].s64 + 22088;
	// 827DF52C: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 827DF530: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827DF538: 419A000C  beq cr6, 0x827df544
	if ctx.cr[6].eq {
	pc = 0x827DF544; continue 'dispatch;
	}
	// 827DF53C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827DF540: 4BBC21D1  bl 0x823a1710
	ctx.lr = 0x827DF544;
	sub_823A1710(ctx, base);
	// 827DF544: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827DF548: 48611F51  bl 0x82df1498
	ctx.lr = 0x827DF54C;
	sub_82DF1498(ctx, base);
	// 827DF54C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DF550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF560 size=112
    let mut pc: u32 = 0x827DF560;
    'dispatch: loop {
        match pc {
            0x827DF560 => {
    //   block [0x827DF560..0x827DF5D0)
	// 827DF560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF56C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF574: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF578: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DF57C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DF580: 4E800421  bctrl
	ctx.lr = 0x827DF584;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DF584: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 827DF588: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 827DF58C: 409A000C  bne cr6, 0x827df598
	if !ctx.cr[6].eq {
	pc = 0x827DF598; continue 'dispatch;
	}
	// 827DF590: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827DF594: 48000028  b 0x827df5bc
	pc = 0x827DF5BC; continue 'dispatch;
	// 827DF598: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DF59C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF5A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827DF5A4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF5A8: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DF5AC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF5B0: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 827DF5B4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DF5B8: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF5BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827DF5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF5C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF5D0 size=228
    let mut pc: u32 = 0x827DF5D0;
    'dispatch: loop {
        match pc {
            0x827DF5D0 => {
    //   block [0x827DF5D0..0x827DF6B4)
	// 827DF5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF5D4: 489C8B89  bl 0x831a815c
	ctx.lr = 0x827DF5D8;
	sub_831A8130(ctx, base);
	// 827DF5D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF5DC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 827DF5E0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827DF5E4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 827DF5E8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 827DF5EC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 827DF5F0: 480000B0  b 0x827df6a0
	pc = 0x827DF6A0; continue 'dispatch;
	// 827DF5F4: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DF5F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF5FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DF600: 419A0010  beq cr6, 0x827df610
	if ctx.cr[6].eq {
	pc = 0x827DF610; continue 'dispatch;
	}
	// 827DF604: 815B0030  lwz r10, 0x30(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DF608: 83EA0000  lwz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF60C: 48000008  b 0x827df614
	pc = 0x827DF614; continue 'dispatch;
	// 827DF610: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827DF614: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827DF618: 40990058  ble cr6, 0x827df670
	if !ctx.cr[6].gt {
	pc = 0x827DF670; continue 'dispatch;
	}
	// 827DF61C: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 827DF620: 40980008  bge cr6, 0x827df628
	if !ctx.cr[6].lt {
	pc = 0x827DF628; continue 'dispatch;
	}
	// 827DF624: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 827DF628: 57FE083C  slwi r30, r31, 1
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 827DF62C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF630: 5724083C  slwi r4, r25, 1
	ctx.r[4].u32 = ctx.r[25].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DF634: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827DF638: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827DF63C: 489C95F5  bl 0x831a8c30
	ctx.lr = 0x827DF640;
	sub_831A8C30(ctx, base);
	// 827DF640: 817B0030  lwz r11, 0x30(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DF644: 7F9EE214  add r28, r30, r28
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 827DF648: 7F5FD214  add r26, r31, r26
	ctx.r[26].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 827DF64C: 7FBFE850  subf r29, r31, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 827DF650: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF654: 7D5F5050  subf r10, r31, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 827DF658: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DF65C: 815B0020  lwz r10, 0x20(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DF660: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF664: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827DF668: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF66C: 48000034  b 0x827df6a0
	pc = 0x827DF6A0; continue 'dispatch;
	// 827DF670: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF674: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DF678: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DF67C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DF680: 4E800421  bctrl
	ctx.lr = 0x827DF684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DF684: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 827DF688: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 827DF68C: 419A001C  beq cr6, 0x827df6a8
	if ctx.cr[6].eq {
	pc = 0x827DF6A8; continue 'dispatch;
	}
	// 827DF690: B07C0000  sth r3, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 827DF694: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 827DF698: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 827DF69C: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 827DF6A0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827DF6A4: 4199FF50  bgt cr6, 0x827df5f4
	if ctx.cr[6].gt {
	pc = 0x827DF5F4; continue 'dispatch;
	}
	// 827DF6A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DF6AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DF6B0: 489C8AFC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF6B8 size=228
    let mut pc: u32 = 0x827DF6B8;
    'dispatch: loop {
        match pc {
            0x827DF6B8 => {
    //   block [0x827DF6B8..0x827DF79C)
	// 827DF6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF6BC: 489C8AA5  bl 0x831a8160
	ctx.lr = 0x827DF6C0;
	sub_831A8130(ctx, base);
	// 827DF6C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF6C4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 827DF6C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827DF6CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827DF6D0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 827DF6D4: 480000B4  b 0x827df788
	pc = 0x827DF788; continue 'dispatch;
	// 827DF6D8: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DF6DC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF6E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DF6E4: 419A0010  beq cr6, 0x827df6f4
	if ctx.cr[6].eq {
	pc = 0x827DF6F4; continue 'dispatch;
	}
	// 827DF6E8: 817B0034  lwz r11, 0x34(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF6EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF6F0: 48000008  b 0x827df6f8
	pc = 0x827DF6F8; continue 'dispatch;
	// 827DF6F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DF6F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DF6FC: 4099005C  ble cr6, 0x827df758
	if !ctx.cr[6].gt {
	pc = 0x827DF758; continue 'dispatch;
	}
	// 827DF700: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 827DF704: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 827DF708: 40980008  bge cr6, 0x827df710
	if !ctx.cr[6].lt {
	pc = 0x827DF710; continue 'dispatch;
	}
	// 827DF70C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 827DF710: 57FE083C  slwi r30, r31, 1
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 827DF714: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF718: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827DF71C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827DF720: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DF724: 489C950D  bl 0x831a8c30
	ctx.lr = 0x827DF728;
	sub_831A8C30(ctx, base);
	// 827DF728: 817B0034  lwz r11, 0x34(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF72C: 7F9EE214  add r28, r30, r28
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 827DF730: 7F5FD214  add r26, r31, r26
	ctx.r[26].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 827DF734: 7FBFE850  subf r29, r31, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 827DF738: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF73C: 7D5F5050  subf r10, r31, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 827DF740: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DF744: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DF748: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF74C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827DF750: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF754: 48000034  b 0x827df788
	pc = 0x827DF788; continue 'dispatch;
	// 827DF758: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF75C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827DF760: A09C0000  lhz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF764: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF768: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DF76C: 4E800421  bctrl
	ctx.lr = 0x827DF770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827DF770: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 827DF774: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 827DF778: 419A0018  beq cr6, 0x827df790
	if ctx.cr[6].eq {
	pc = 0x827DF790; continue 'dispatch;
	}
	// 827DF77C: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 827DF780: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 827DF784: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 827DF788: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827DF78C: 4199FF4C  bgt cr6, 0x827df6d8
	if ctx.cr[6].gt {
	pc = 0x827DF6D8; continue 'dispatch;
	}
	// 827DF790: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827DF794: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DF798: 489C8A18  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF7A0 size=116
    let mut pc: u32 = 0x827DF7A0;
    'dispatch: loop {
        match pc {
            0x827DF7A0 => {
    //   block [0x827DF7A0..0x827DF814)
	// 827DF7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF7A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DF7AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF7B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF7B4: 3BE3FFFC  addi r31, r3, -4
	ctx.r[31].s64 = ctx.r[3].s64 + -4;
	// 827DF7B8: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DF7BC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827DF7C0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 827DF7C4: 394A5630  addi r10, r10, 0x5630
	ctx.r[10].s64 = ctx.r[10].s64 + 22064;
	// 827DF7C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF7CC: 3929BF8C  addi r9, r9, -0x4074
	ctx.r[9].s64 = ctx.r[9].s64 + -16500;
	// 827DF7D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DF7D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF7D8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 827DF7DC: 914BFFFC  stw r10, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 827DF7E0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827DF7E4: 48611BFD  bl 0x82df13e0
	ctx.lr = 0x827DF7E8;
	sub_82DF13E0(ctx, base);
	// 827DF7E8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DF7EC: 4182000C  beq 0x827df7f8
	if ctx.cr[0].eq {
	pc = 0x827DF7F8; continue 'dispatch;
	}
	// 827DF7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF7F4: 4BAE0A75  bl 0x822c0268
	ctx.lr = 0x827DF7F8;
	sub_822C0268(ctx, base);
	// 827DF7F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF7FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF808: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DF80C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF818 size=116
    let mut pc: u32 = 0x827DF818;
    'dispatch: loop {
        match pc {
            0x827DF818 => {
    //   block [0x827DF818..0x827DF88C)
	// 827DF818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DF824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF82C: 3BE3FFF8  addi r31, r3, -8
	ctx.r[31].s64 = ctx.r[3].s64 + -8;
	// 827DF830: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827DF834: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 827DF838: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 827DF83C: 394A5638  addi r10, r10, 0x5638
	ctx.r[10].s64 = ctx.r[10].s64 + 22072;
	// 827DF840: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF844: 3929BF8C  addi r9, r9, -0x4074
	ctx.r[9].s64 = ctx.r[9].s64 + -16500;
	// 827DF848: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DF84C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF850: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 827DF854: 914BFFF8  stw r10, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 827DF858: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 827DF85C: 48611B85  bl 0x82df13e0
	ctx.lr = 0x827DF860;
	sub_82DF13E0(ctx, base);
	// 827DF860: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DF864: 4182000C  beq 0x827df870
	if ctx.cr[0].eq {
	pc = 0x827DF870; continue 'dispatch;
	}
	// 827DF868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF86C: 4BAE09FD  bl 0x822c0268
	ctx.lr = 0x827DF870;
	sub_822C0268(ctx, base);
	// 827DF870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF880: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DF884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF890 size=76
    let mut pc: u32 = 0x827DF890;
    'dispatch: loop {
        match pc {
            0x827DF890 => {
    //   block [0x827DF890..0x827DF8DC)
	// 827DF890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827DF898: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827DF89C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827DF8A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF8A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF8A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827DF8AC: 4BFFFC65  bl 0x827df510
	ctx.lr = 0x827DF8B0;
	sub_827DF510(ctx, base);
	// 827DF8B0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DF8B4: 4182000C  beq 0x827df8c0
	if ctx.cr[0].eq {
	pc = 0x827DF8C0; continue 'dispatch;
	}
	// 827DF8B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF8BC: 4BAE09AD  bl 0x822c0268
	ctx.lr = 0x827DF8C0;
	sub_822C0268(ctx, base);
	// 827DF8C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827DF8C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827DF8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827DF8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827DF8D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827DF8D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827DF8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DF8E0 size=32
    let mut pc: u32 = 0x827DF8E0;
    'dispatch: loop {
        match pc {
            0x827DF8E0 => {
    //   block [0x827DF8E0..0x827DF900)
	// 827DF8E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827DF8E4: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DF8E8: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF8EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827DF8F0: 419A0010  beq cr6, 0x827df900
	if ctx.cr[6].eq {
		sub_827DF900(ctx, base);
		return;
	}
	// 827DF8F4: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF8F8: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF8FC: 48000008  b 0x827df904
	sub_827DF900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DF900 size=56
    let mut pc: u32 = 0x827DF900;
    'dispatch: loop {
        match pc {
            0x827DF900 => {
    //   block [0x827DF900..0x827DF938)
	// 827DF900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827DF904: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DF908: 40990030  ble cr6, 0x827df938
	if !ctx.cr[6].gt {
		sub_827DF938(ctx, base);
		return;
	}
	// 827DF90C: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF910: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827DF914: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF918: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 827DF91C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DF920: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DF924: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF928: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 827DF92C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DF930: B08B0000  sth r4, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 827DF934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DF938 size=20
    let mut pc: u32 = 0x827DF938;
    'dispatch: loop {
        match pc {
            0x827DF938 => {
    //   block [0x827DF938..0x827DF94C)
	// 827DF938: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF93C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827DF940: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827DF944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827DF948: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DF950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827DF950 size=780
    let mut pc: u32 = 0x827DF950;
    'dispatch: loop {
        match pc {
            0x827DF950 => {
    //   block [0x827DF950..0x827DFC5C)
	// 827DF950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827DF954: 489C8809  bl 0x831a815c
	ctx.lr = 0x827DF958;
	sub_831A8130(ctx, base);
	// 827DF958: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827DF95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827DF960: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 827DF964: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DF968: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DF96C: 41820044  beq 0x827df9b0
	if ctx.cr[0].eq {
	pc = 0x827DF9B0; continue 'dispatch;
	}
	// 827DF970: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DF974: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF978: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DF97C: 419A0034  beq cr6, 0x827df9b0
	if ctx.cr[6].eq {
	pc = 0x827DF9B0; continue 'dispatch;
	}
	// 827DF980: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DF984: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DF988: 40980028  bge cr6, 0x827df9b0
	if !ctx.cr[6].lt {
	pc = 0x827DF9B0; continue 'dispatch;
	}
	// 827DF98C: 811F0034  lwz r8, 0x34(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF990: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF994: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF998: 5509083C  slwi r9, r8, 1
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827DF99C: 811F0034  lwz r8, 0x34(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF9A0: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 827DF9A4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827DF9A8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DF9AC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DF9B0: 572B043E  clrlwi r11, r25, 0x10
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 827DF9B4: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 827DF9B8: 409A000C  bne cr6, 0x827df9c4
	if !ctx.cr[6].eq {
	pc = 0x827DF9C4; continue 'dispatch;
	}
	// 827DF9BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827DF9C0: 48000294  b 0x827dfc54
	pc = 0x827DFC54; continue 'dispatch;
	// 827DF9C4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DF9C8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF9CC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 827DF9D0: 419A0030  beq cr6, 0x827dfa00
	if ctx.cr[6].eq {
	pc = 0x827DFA00; continue 'dispatch;
	}
	// 827DF9D4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DF9D8: 550A003E  slwi r10, r8, 0
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DF9DC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF9E0: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827DF9E4: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 827DF9E8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 827DF9EC: 40980014  bge cr6, 0x827dfa00
	if !ctx.cr[6].lt {
	pc = 0x827DFA00; continue 'dispatch;
	}
	// 827DF9F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DF9F4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 827DF9F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DF9FC: 48000240  b 0x827dfc3c
	pc = 0x827DFC3C; continue 'dispatch;
	// 827DFA00: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DFA04: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFA08: 4182000C  beq 0x827dfa14
	if ctx.cr[0].eq {
	pc = 0x827DFA14; continue 'dispatch;
	}
	// 827DFA0C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827DFA10: 48000244  b 0x827dfc54
	pc = 0x827DFC54; continue 'dispatch;
	// 827DFA14: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 827DFA18: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 827DFA1C: 409A000C  bne cr6, 0x827dfa28
	if !ctx.cr[6].eq {
	pc = 0x827DFA28; continue 'dispatch;
	}
	// 827DFA20: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 827DFA24: 48000024  b 0x827dfa48
	pc = 0x827DFA48; continue 'dispatch;
	// 827DFA28: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DFA2C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFA30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFA34: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFA38: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DFA3C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827DFA40: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 827DFA44: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFA48: 57ABF87E  srwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DFA4C: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 827DFA50: 2B0B0020  cmplwi cr6, r11, 0x20
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32 as u32, &mut ctx.xer);
	// 827DFA54: 40980008  bge cr6, 0x827dfa5c
	if !ctx.cr[6].lt {
	pc = 0x827DFA5C; continue 'dispatch;
	}
	// 827DFA58: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 827DFA5C: 3F608335  lis r27, -0x7ccb
	ctx.r[27].s64 = -2093678592;
	// 827DFA60: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 827DFA64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DFA68: 419A0048  beq cr6, 0x827dfab0
	if ctx.cr[6].eq {
	pc = 0x827DFAB0; continue 'dispatch;
	}
	// 827DFA6C: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 827DFA70: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 827DFA74: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827DFA78: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 827DFA7C: 4098000C  bge cr6, 0x827dfa88
	if !ctx.cr[6].lt {
	pc = 0x827DFA88; continue 'dispatch;
	}
	// 827DFA80: 556BF87F  rlwinm. r11, r11, 0x1f, 1, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFA84: 4082FFE8  bne 0x827dfa6c
	if !ctx.cr[0].eq {
	pc = 0x827DFA6C; continue 'dispatch;
	}
	// 827DFA88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827DFA8C: 419A0024  beq cr6, 0x827dfab0
	if ctx.cr[6].eq {
	pc = 0x827DFAB0; continue 'dispatch;
	}
	// 827DFA90: 7F8BEA14  add r28, r11, r29
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 827DFA94: 807B110C  lwz r3, 0x110c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827DFA98: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827DFA9C: 5786083C  slwi r6, r28, 1
	ctx.r[6].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 827DFAA0: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 827DFAA4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 827DFAA8: 48612621  bl 0x82df20c8
	ctx.lr = 0x827DFAAC;
	sub_82DF20C8(ctx, base);
	// 827DFAAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827DFAB0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827DFAB4: 419A001C  beq cr6, 0x827dfad0
	if ctx.cr[6].eq {
	pc = 0x827DFAD0; continue 'dispatch;
	}
	// 827DFAB8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFABC: 57A6083C  slwi r6, r29, 1
	ctx.r[6].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 827DFAC0: 5784083C  slwi r4, r28, 1
	ctx.r[4].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 827DFAC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827DFAC8: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFACC: 489C9165  bl 0x831a8c30
	ctx.lr = 0x827DFAD0;
	sub_831A8C30(ctx, base);
	// 827DFAD0: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DFAD4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFAD8: 41820014  beq 0x827dfaec
	if ctx.cr[0].eq {
	pc = 0x827DFAEC; continue 'dispatch;
	}
	// 827DFADC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFAE0: 807B110C  lwz r3, 0x110c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827DFAE4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFAE8: 486126A1  bl 0x82df2188
	ctx.lr = 0x827DFAEC;
	sub_82DF2188(ctx, base);
	// 827DFAEC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DFAF0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827DFAF4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827DFAF8: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 827DFAFC: 409A0058  bne cr6, 0x827dfb54
	if !ctx.cr[6].eq {
	pc = 0x827DFB54; continue 'dispatch;
	}
	// 827DFB00: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DFB04: 578A083C  slwi r10, r28, 1
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DFB08: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 827DFB0C: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 827DFB10: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DFB14: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFB18: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DFB1C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DFB20: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DFB24: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DFB28: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFB2C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFB30: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DFB34: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFB38: 41820010  beq 0x827dfb48
	if ctx.cr[0].eq {
	pc = 0x827DFB48; continue 'dispatch;
	}
	// 827DFB3C: 7FCA0E70  srawi r10, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 827DFB40: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 827DFB44: 480000E0  b 0x827dfc24
	pc = 0x827DFC24; continue 'dispatch;
	// 827DFB48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827DFB4C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DFB50: 480000D4  b 0x827dfc24
	pc = 0x827DFC24; continue 'dispatch;
	// 827DFB54: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFB58: 5789083C  slwi r9, r28, 1
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827DFB5C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFB60: 811F0024  lwz r8, 0x24(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFB64: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827DFB68: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFB6C: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 827DFB70: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFB74: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DFB78: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827DFB7C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 827DFB80: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFB84: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFB88: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFB8C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827DFB90: 7D0A4050  subf r8, r10, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 827DFB94: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFB98: 7D0A0E70  srawi r10, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 827DFB9C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DFBA0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DFBA4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827DFBA8: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 827DFBAC: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 827DFBB0: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DFBB4: 7D49F214  add r10, r9, r30
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 827DFBB8: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFBBC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFBC0: 7D4B0E70  srawi r11, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 827DFBC4: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DFBC8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFBCC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DFBD0: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFBD4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFBD8: 41820010  beq 0x827dfbe8
	if ctx.cr[0].eq {
	pc = 0x827DFBE8; continue 'dispatch;
	}
	// 827DFBDC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DFBE0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFBE4: 4BFFFF58  b 0x827dfb3c
	pc = 0x827DFB3C; continue 'dispatch;
	// 827DFBE8: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFBEC: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFBF0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFBF4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFBF8: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFBFC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827DFC00: 7D685050  subf r11, r8, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 827DFC04: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFC08: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DFC0C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 827DFC10: 7D4B4850  subf r10, r11, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 827DFC14: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFC18: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFC1C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 827DFC20: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 827DFC24: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DFC28: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DFC2C: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DFC30: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFC34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827DFC38: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFC3C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFC40: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 827DFC44: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFC48: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 827DFC4C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827DFC50: B32B0000  sth r25, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u16 ) };
	// 827DFC54: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827DFC58: 489C8554  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFC60 size=136
    let mut pc: u32 = 0x827DFC60;
    'dispatch: loop {
        match pc {
            0x827DFC60 => {
    //   block [0x827DFC60..0x827DFCE8)
	// 827DFC60: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFC64: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFC68: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DFC6C: 419A007C  beq cr6, 0x827dfce8
	if ctx.cr[6].eq {
		sub_827DFCE8(ctx, base);
		return;
	}
	// 827DFC70: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFC74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFC78: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DFC7C: 4099006C  ble cr6, 0x827dfce8
	if !ctx.cr[6].gt {
		sub_827DFCE8(ctx, base);
		return;
	}
	// 827DFC80: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 827DFC84: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 827DFC88: 419A001C  beq cr6, 0x827dfca4
	if ctx.cr[6].eq {
	pc = 0x827DFCA4; continue 'dispatch;
	}
	// 827DFC8C: A14AFFFE  lhz r10, -2(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-2 as u32) ) } as u64;
	// 827DFC90: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DFC94: 419A0010  beq cr6, 0x827dfca4
	if ctx.cr[6].eq {
	pc = 0x827DFCA4; continue 'dispatch;
	}
	// 827DFC98: 81430040  lwz r10, 0x40(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DFC9C: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DFCA0: 40820048  bne 0x827dfce8
	if !ctx.cr[0].eq {
		sub_827DFCE8(ctx, base);
		return;
	}
	// 827DFCA4: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DFCA8: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 827DFCAC: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFCB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827DFCB4: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DFCB8: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFCBC: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFCC0: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 827DFCC4: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DFCC8: 419A0014  beq cr6, 0x827dfcdc
	if ctx.cr[6].eq {
	pc = 0x827DFCDC; continue 'dispatch;
	}
	// 827DFCCC: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFCD0: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFCD4: B08A0000  sth r4, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u16 ) };
	// 827DFCD8: 409A0008  bne cr6, 0x827dfce0
	if !ctx.cr[6].eq {
	pc = 0x827DFCE0; continue 'dispatch;
	}
	// 827DFCDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DFCE0: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 827DFCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFCE8 size=8
    let mut pc: u32 = 0x827DFCE8;
    'dispatch: loop {
        match pc {
            0x827DFCE8 => {
    //   block [0x827DFCE8..0x827DFCF0)
	// 827DFCE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827DFCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFCF0 size=52
    let mut pc: u32 = 0x827DFCF0;
    'dispatch: loop {
        match pc {
            0x827DFCF0 => {
    //   block [0x827DFCF0..0x827DFD24)
	// 827DFCF0: 81230020  lwz r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFCF4: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFCF8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 827DFCFC: 419A008C  beq cr6, 0x827dfd88
	if ctx.cr[6].eq {
		sub_827DFD88(ctx, base);
		return;
	}
	// 827DFD00: 80E30030  lwz r7, 0x30(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DFD04: 550B003E  slwi r11, r8, 0
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827DFD08: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFD0C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DFD10: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827DFD14: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827DFD18: 4098000C  bge cr6, 0x827dfd24
	if !ctx.cr[6].lt {
		sub_827DFD24(ctx, base);
		return;
	}
	// 827DFD1C: A0680000  lhz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFD24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFD24 size=100
    let mut pc: u32 = 0x827DFD24;
    'dispatch: loop {
        match pc {
            0x827DFD24 => {
    //   block [0x827DFD24..0x827DFD88)
	// 827DFD24: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 827DFD28: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFD2C: 4082005C  bne 0x827dfd88
	if !ctx.cr[0].eq {
		sub_827DFD88(ctx, base);
		return;
	}
	// 827DFD30: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFD34: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFD38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFD3C: 419A004C  beq cr6, 0x827dfd88
	if ctx.cr[6].eq {
		sub_827DFD88(ctx, base);
		return;
	}
	// 827DFD40: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 827DFD44: 41990010  bgt cr6, 0x827dfd54
	if ctx.cr[6].gt {
	pc = 0x827DFD54; continue 'dispatch;
	}
	// 827DFD48: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFD4C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 827DFD50: 40990038  ble cr6, 0x827dfd88
	if !ctx.cr[6].gt {
		sub_827DFD88(ctx, base);
		return;
	}
	// 827DFD54: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFD58: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DFD5C: 40980008  bge cr6, 0x827dfd64
	if !ctx.cr[6].lt {
	pc = 0x827DFD64; continue 'dispatch;
	}
	// 827DFD60: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 827DFD64: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFD68: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFD6C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827DFD70: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFD74: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFD78: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFD7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFD80: A06B0000  lhz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFD88 size=8
    let mut pc: u32 = 0x827DFD88;
    'dispatch: loop {
        match pc {
            0x827DFD88 => {
    //   block [0x827DFD88..0x827DFD90)
	// 827DFD88: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827DFD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFD90 size=92
    let mut pc: u32 = 0x827DFD90;
    'dispatch: loop {
        match pc {
            0x827DFD90 => {
    //   block [0x827DFD90..0x827DFDEC)
	// 827DFD90: 81440024  lwz r10, 0x24(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFD94: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFD98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFD9C: 419A0014  beq cr6, 0x827dfdb0
	if ctx.cr[6].eq {
	pc = 0x827DFDB0; continue 'dispatch;
	}
	// 827DFDA0: 8124003C  lwz r9, 0x3c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFDA4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DFDA8: 40980008  bge cr6, 0x827dfdb0
	if !ctx.cr[6].lt {
	pc = 0x827DFDB0; continue 'dispatch;
	}
	// 827DFDAC: 9164003C  stw r11, 0x3c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 827DFDB0: 54EB07FF  clrlwi. r11, r7, 0x1f
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFDB4: 41820110  beq 0x827dfec4
	if ctx.cr[0].eq {
		sub_827DFEC4(ctx, base);
		return;
	}
	// 827DFDB8: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFDBC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFDC0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827DFDC4: 419A0100  beq cr6, 0x827dfec4
	if ctx.cr[6].eq {
		sub_827DFEC4(ctx, base);
		return;
	}
	// 827DFDC8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 827DFDCC: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 827DFDD0: 814BB060  lwz r10, -0x4fa0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20384 as u32) ) } as u64;
	// 827DFDD4: 409A0018  bne cr6, 0x827dfdec
	if !ctx.cr[6].eq {
		sub_827DFDEC(ctx, base);
		return;
	}
	// 827DFDD8: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFDDC: 8104003C  lwz r8, 0x3c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFDE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFDE4: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 827DFDE8: 48000020  b 0x827dfe08
	sub_827DFDEC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFDEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFDEC size=40
    let mut pc: u32 = 0x827DFDEC;
    'dispatch: loop {
        match pc {
            0x827DFDEC => {
    //   block [0x827DFDEC..0x827DFE14)
	// 827DFDEC: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 827DFDF0: 409A0024  bne cr6, 0x827dfe14
	if !ctx.cr[6].eq {
		sub_827DFE14(ctx, base);
		return;
	}
	// 827DFDF4: 54EB07BD  rlwinm. r11, r7, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFDF8: 40820024  bne 0x827dfe1c
	if !ctx.cr[0].eq {
		sub_827DFE14(ctx, base);
		return;
	}
	// 827DFDFC: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFE00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFE04: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 827DFE08: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFE0C: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 827DFE10: 48000010  b 0x827dfe20
	sub_827DFE14(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFE14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFE14 size=168
    let mut pc: u32 = 0x827DFE14;
    'dispatch: loop {
        match pc {
            0x827DFE14 => {
    //   block [0x827DFE14..0x827DFEBC)
	// 827DFE14: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 827DFE18: 419A0008  beq cr6, 0x827dfe20
	if ctx.cr[6].eq {
	pc = 0x827DFE20; continue 'dispatch;
	}
	// 827DFE1C: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 827DFE20: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827DFE24: 41980098  blt cr6, 0x827dfebc
	if ctx.cr[6].lt {
		sub_827DFEBC(ctx, base);
		return;
	}
	// 827DFE28: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFE2C: 8104003C  lwz r8, 0x3c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFE30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFE34: 7D0B4050  subf r8, r11, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 827DFE38: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 827DFE3C: 7F054000  cmpw cr6, r5, r8
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[8].s32, &mut ctx.xer);
	// 827DFE40: 4199007C  bgt cr6, 0x827dfebc
	if ctx.cr[6].gt {
		sub_827DFEBC(ctx, base);
		return;
	}
	// 827DFE44: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 827DFE48: 81040030  lwz r8, 0x30(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 827DFE4C: 54EA07BD  rlwinm. r10, r7, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DFE50: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFE54: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFE58: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 827DFE5C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DFE60: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 827DFE64: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFE68: 81240020  lwz r9, 0x20(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFE6C: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFE70: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827DFE74: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFE78: 41820118  beq 0x827dff90
	if ctx.cr[0].eq {
		sub_827DFF88(ctx, base);
		return;
	}
	// 827DFE7C: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFE80: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFE84: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DFE88: 419A0108  beq cr6, 0x827dff90
	if ctx.cr[6].eq {
		sub_827DFF88(ctx, base);
		return;
	}
	// 827DFE8C: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DFE90: 81040020  lwz r8, 0x20(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFE94: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFE98: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFE9C: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827DFEA0: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 827DFEA4: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 827DFEA8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827DFEAC: 81640034  lwz r11, 0x34(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DFEB0: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 827DFEB4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827DFEB8: 480000D8  b 0x827dff90
	sub_827DFF88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFEBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFEBC size=8
    let mut pc: u32 = 0x827DFEBC;
    'dispatch: loop {
        match pc {
            0x827DFEBC => {
    //   block [0x827DFEBC..0x827DFEC4)
	// 827DFEBC: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 827DFEC0: 480000D0  b 0x827dff90
	sub_827DFF88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFEC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFEC4 size=56
    let mut pc: u32 = 0x827DFEC4;
    'dispatch: loop {
        match pc {
            0x827DFEC4 => {
    //   block [0x827DFEC4..0x827DFEFC)
	// 827DFEC4: 54EB07BD  rlwinm. r11, r7, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFEC8: 418200C0  beq 0x827dff88
	if ctx.cr[0].eq {
		sub_827DFF88(ctx, base);
		return;
	}
	// 827DFECC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFED0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827DFED4: 419A00B4  beq cr6, 0x827dff88
	if ctx.cr[6].eq {
		sub_827DFF88(ctx, base);
		return;
	}
	// 827DFED8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 827DFEDC: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 827DFEE0: 812BB060  lwz r9, -0x4fa0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20384 as u32) ) } as u64;
	// 827DFEE4: 409A0018  bne cr6, 0x827dfefc
	if !ctx.cr[6].eq {
		sub_827DFEFC(ctx, base);
		return;
	}
	// 827DFEE8: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFEEC: 8104003C  lwz r8, 0x3c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFEF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFEF4: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 827DFEF8: 48000018  b 0x827dff10
	sub_827DFEFC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFEFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFEFC size=32
    let mut pc: u32 = 0x827DFEFC;
    'dispatch: loop {
        match pc {
            0x827DFEFC => {
    //   block [0x827DFEFC..0x827DFF1C)
	// 827DFEFC: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 827DFF00: 409A001C  bne cr6, 0x827dff1c
	if !ctx.cr[6].eq {
		sub_827DFF1C(ctx, base);
		return;
	}
	// 827DFF04: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFF08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFF0C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827DFF10: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFF14: 7CAB2A14  add r5, r11, r5
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 827DFF18: 48000010  b 0x827dff28
	sub_827DFF1C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFF1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFF1C size=100
    let mut pc: u32 = 0x827DFF1C;
    'dispatch: loop {
        match pc {
            0x827DFF1C => {
    //   block [0x827DFF1C..0x827DFF80)
	// 827DFF1C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 827DFF20: 419A0008  beq cr6, 0x827dff28
	if ctx.cr[6].eq {
	pc = 0x827DFF28; continue 'dispatch;
	}
	// 827DFF24: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 827DFF28: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827DFF2C: 41980054  blt cr6, 0x827dff80
	if ctx.cr[6].lt {
		sub_827DFF80(ctx, base);
		return;
	}
	// 827DFF30: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827DFF34: 8104003C  lwz r8, 0x3c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFF38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFF3C: 7D0B4050  subf r8, r11, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 827DFF40: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 827DFF44: 7F054000  cmpw cr6, r5, r8
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[8].s32, &mut ctx.xer);
	// 827DFF48: 41990038  bgt cr6, 0x827dff80
	if ctx.cr[6].gt {
		sub_827DFF80(ctx, base);
		return;
	}
	// 827DFF4C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827DFF50: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 827DFF54: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827DFF58: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 827DFF5C: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFF60: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 827DFF64: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 827DFF68: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFF6C: 81240024  lwz r9, 0x24(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFF70: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFF74: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827DFF78: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827DFF7C: 48000014  b 0x827dff90
	sub_827DFF88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFF80 size=8
    let mut pc: u32 = 0x827DFF80;
    'dispatch: loop {
        match pc {
            0x827DFF80 => {
    //   block [0x827DFF80..0x827DFF88)
	// 827DFF80: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 827DFF84: 4800000C  b 0x827dff90
	sub_827DFF88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFF88 size=28
    let mut pc: u32 = 0x827DFF88;
    'dispatch: loop {
        match pc {
            0x827DFF88 => {
    //   block [0x827DFF88..0x827DFFA4)
	// 827DFF88: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 827DFF8C: 80ABB060  lwz r5, -0x4fa0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20384 as u32) ) } as u64;
	// 827DFF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827DFF94: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 827DFF98: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 827DFF9C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827DFFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827DFFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827DFFA8 size=248
    let mut pc: u32 = 0x827DFFA8;
    'dispatch: loop {
        match pc {
            0x827DFFA8 => {
    //   block [0x827DFFA8..0x827E00A0)
	// 827DFFA8: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 827DFFAC: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 827DFFB0: 80E40024  lwz r7, 0x24(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 827DFFB4: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 827DFFB8: 81610020  lwz r11, 0x20(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFFBC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827DFFC0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFFC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFFC8: 419A0014  beq cr6, 0x827dffdc
	if ctx.cr[6].eq {
	pc = 0x827DFFDC; continue 'dispatch;
	}
	// 827DFFCC: 8124003C  lwz r9, 0x3c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827DFFD0: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827DFFD4: 40980008  bge cr6, 0x827dffdc
	if !ctx.cr[6].lt {
	pc = 0x827DFFDC; continue 'dispatch;
	}
	// 827DFFD8: 9164003C  stw r11, 0x3c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 827DFFDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 827DFFE0: 80CBB060  lwz r6, -0x4fa0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20384 as u32) ) } as u64;
	// 827DFFE4: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 827DFFE8: 419A0128  beq cr6, 0x827e0110
	if ctx.cr[6].eq {
		sub_827E010C(ctx, base);
		return;
	}
	// 827DFFEC: 550B07FF  clrlwi. r11, r8, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827DFFF0: 418200B0  beq 0x827e00a0
	if ctx.cr[0].eq {
		sub_827E00A0(ctx, base);
		return;
	}
	// 827DFFF4: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827DFFF8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827DFFFC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827E0000: 419A00A0  beq cr6, 0x827e00a0
	if ctx.cr[6].eq {
		sub_827E00A0(ctx, base);
		return;
	}
	// 827E0004: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E0008: 41980104  blt cr6, 0x827e010c
	if ctx.cr[6].lt {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E000C: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827E0010: 80E4003C  lwz r7, 0x3c(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827E0014: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0018: 7CEB3850  subf r7, r11, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 827E001C: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 827E0020: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 827E0024: 419900E8  bgt cr6, 0x827e010c
	if ctx.cr[6].gt {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E0028: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 827E002C: 80E40030  lwz r7, 0x30(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 827E0030: 550907BD  rlwinm. r9, r8, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827E0034: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827E0038: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E003C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827E0040: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827E0044: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 827E0048: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E004C: 81040020  lwz r8, 0x20(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827E0050: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0054: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 827E0058: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E005C: 418200B4  beq 0x827e0110
	if ctx.cr[0].eq {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E0060: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 827E0064: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0068: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827E006C: 419A00A4  beq cr6, 0x827e0110
	if ctx.cr[6].eq {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E0070: 81040034  lwz r8, 0x34(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 827E0074: 80E40020  lwz r7, 0x20(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827E0078: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E007C: 80E70000  lwz r7, 0(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0080: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 827E0084: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 827E0088: 7D274850  subf r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 827E008C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 827E0090: 81640034  lwz r11, 0x34(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 827E0094: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 827E0098: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827E009C: 48000074  b 0x827e0110
	sub_827E010C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E00A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E00A0 size=108
    let mut pc: u32 = 0x827E00A0;
    'dispatch: loop {
        match pc {
            0x827E00A0 => {
    //   block [0x827E00A0..0x827E010C)
	// 827E00A0: 550B07BD  rlwinm. r11, r8, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E00A4: 41820068  beq 0x827e010c
	if ctx.cr[0].eq {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E00A8: 81270000  lwz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E00AC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827E00B0: 419A005C  beq cr6, 0x827e010c
	if ctx.cr[6].eq {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E00B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E00B8: 41980054  blt cr6, 0x827e010c
	if ctx.cr[6].lt {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E00BC: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827E00C0: 8104003C  lwz r8, 0x3c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827E00C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E00C8: 7D0B4050  subf r8, r11, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 827E00CC: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 827E00D0: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 827E00D4: 41990038  bgt cr6, 0x827e010c
	if ctx.cr[6].gt {
		sub_827E010C(ctx, base);
		return;
	}
	// 827E00D8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 827E00DC: 81040034  lwz r8, 0x34(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 827E00E0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827E00E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827E00E8: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E00EC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827E00F0: 7D6B3850  subf r11, r11, r7
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[11].s64;
	// 827E00F4: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E00F8: 81040024  lwz r8, 0x24(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 827E00FC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0100: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 827E0104: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0108: 48000008  b 0x827e0110
	sub_827E010C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E010C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E010C size=24
    let mut pc: u32 = 0x827E010C;
    'dispatch: loop {
        match pc {
            0x827E010C => {
    //   block [0x827E010C..0x827E0124)
	// 827E010C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 827E0110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827E0114: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827E0118: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 827E011C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827E0120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0128 size=80
    let mut pc: u32 = 0x827E0128;
    'dispatch: loop {
        match pc {
            0x827E0128 => {
    //   block [0x827E0128..0x827E0178)
	// 827E0128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E012C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827E0134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E013C: 3BE3FFF4  addi r31, r3, -0xc
	ctx.r[31].s64 = ctx.r[3].s64 + -12;
	// 827E0140: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827E0144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0148: 4BFFF2B1  bl 0x827df3f8
	ctx.lr = 0x827E014C;
	sub_827DF3F8(ctx, base);
	// 827E014C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E0150: 4182000C  beq 0x827e015c
	if ctx.cr[0].eq {
	pc = 0x827E015C; continue 'dispatch;
	}
	// 827E0154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0158: 4BAE0111  bl 0x822c0268
	ctx.lr = 0x827E015C;
	sub_822C0268(ctx, base);
	// 827E015C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0160: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E0164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E016C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827E0170: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0178 size=96
    let mut pc: u32 = 0x827E0178;
    'dispatch: loop {
        match pc {
            0x827E0178 => {
    //   block [0x827E0178..0x827E01D8)
	// 827E0178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E017C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827E0184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E018C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E0190: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827E0194: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827E0198: 396B5684  addi r11, r11, 0x5684
	ctx.r[11].s64 = ctx.r[11].s64 + 22148;
	// 827E019C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E01A0: 4BD20379  bl 0x82500518
	ctx.lr = 0x827E01A4;
	sub_82500518(ctx, base);
	// 827E01A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E01A8: 4BFFF369  bl 0x827df510
	ctx.lr = 0x827E01AC;
	sub_827DF510(ctx, base);
	// 827E01AC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E01B0: 4182000C  beq 0x827e01bc
	if ctx.cr[0].eq {
	pc = 0x827E01BC; continue 'dispatch;
	}
	// 827E01B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E01B8: 4BAE00B1  bl 0x822c0268
	ctx.lr = 0x827E01BC;
	sub_822C0268(ctx, base);
	// 827E01BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E01C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E01C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E01C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E01CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827E01D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E01D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E01D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E01D8 size=136
    let mut pc: u32 = 0x827E01D8;
    'dispatch: loop {
        match pc {
            0x827E01D8 => {
    //   block [0x827E01D8..0x827E0260)
	// 827E01D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E01DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E01E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827E01E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E01E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E01EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E01F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827E01F4: 4BFFF255  bl 0x827df448
	ctx.lr = 0x827E01F8;
	sub_827DF448(ctx, base);
	// 827E01F8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827E01FC: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E0200: 392B5684  addi r9, r11, 0x5684
	ctx.r[9].s64 = ctx.r[11].s64 + 22148;
	// 827E0204: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827E0208: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827E020C: 40820008  bne 0x827e0214
	if !ctx.cr[0].eq {
	pc = 0x827E0214; continue 'dispatch;
	}
	// 827E0210: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 827E0214: 57CA07BD  rlwinm. r10, r30, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E0218: 40820008  bne 0x827e0220
	if !ctx.cr[0].eq {
	pc = 0x827E0220; continue 'dispatch;
	}
	// 827E021C: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 827E0220: 57CA0739  rlwinm. r10, r30, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E0224: 41820008  beq 0x827e022c
	if ctx.cr[0].eq {
	pc = 0x827E022C; continue 'dispatch;
	}
	// 827E0228: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 827E022C: 57CA077B  rlwinm. r10, r30, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E0230: 41820008  beq 0x827e0238
	if ctx.cr[0].eq {
	pc = 0x827E0238; continue 'dispatch;
	}
	// 827E0234: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 827E0238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827E023C: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 827E0240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0244: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 827E0248: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E024C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0254: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827E0258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E025C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0260 size=108
    let mut pc: u32 = 0x827E0260;
    'dispatch: loop {
        match pc {
            0x827E0260 => {
    //   block [0x827E0260..0x827E02CC)
	// 827E0260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0264: 489C7F09  bl 0x831a816c
	ctx.lr = 0x827E0268;
	sub_831A8130(ctx, base);
	// 827E0268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E026C: 83C40018  lwz r30, 0x18(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 827E0270: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827E0274: 83E40034  lwz r31, 0x34(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 827E0278: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E027C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827E0280: 48000030  b 0x827e02b0
	pc = 0x827E02B0; continue 'dispatch;
	// 827E0284: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827E0288: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827E028C: 4BFFFFD5  bl 0x827e0260
	ctx.lr = 0x827E0290;
	sub_827E0260(ctx, base);
	// 827E0290: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 827E0294: 40980008  bge cr6, 0x827e029c
	if !ctx.cr[6].lt {
	pc = 0x827E029C; continue 'dispatch;
	}
	// 827E0298: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E029C: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 827E02A0: 419A0024  beq cr6, 0x827e02c4
	if ctx.cr[6].eq {
	pc = 0x827E02C4; continue 'dispatch;
	}
	// 827E02A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E02A8: 4BBC13E1  bl 0x823a1688
	ctx.lr = 0x827E02AC;
	sub_823A1688(ctx, base);
	// 827E02AC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827E02B0: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827E02B4: 409AFFD0  bne cr6, 0x827e0284
	if !ctx.cr[6].eq {
	pc = 0x827E0284; continue 'dispatch;
	}
	// 827E02B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E02BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827E02C0: 489C7EFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 827E02C4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 827E02C8: 4BFFFFF4  b 0x827e02bc
	pc = 0x827E02BC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E02D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E02D0 size=104
    let mut pc: u32 = 0x827E02D0;
    'dispatch: loop {
        match pc {
            0x827E02D0 => {
    //   block [0x827E02D0..0x827E0338)
	// 827E02D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E02D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E02D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E02DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E02E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827E02E4: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827E02E8: 3D008207  lis r8, -0x7df9
	ctx.r[8].s64 = -2113470464;
	// 827E02EC: 392956C0  addi r9, r9, 0x56c0
	ctx.r[9].s64 = ctx.r[9].s64 + 22208;
	// 827E02F0: 39085684  addi r8, r8, 0x5684
	ctx.r[8].s64 = ctx.r[8].s64 + 22148;
	// 827E02F4: 814BFFAC  lwz r10, -0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-84 as u32) ) } as u64;
	// 827E02F8: 3BEBFFB8  addi r31, r11, -0x48
	ctx.r[31].s64 = ctx.r[11].s64 + -72;
	// 827E02FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0300: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827E0304: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827E0308: 912AFFAC  stw r9, -0x54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-84 as u32), ctx.r[9].u32 ) };
	// 827E030C: 910BFFB8  stw r8, -0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-72 as u32), ctx.r[8].u32 ) };
	// 827E0310: 4BD20209  bl 0x82500518
	ctx.lr = 0x827E0314;
	sub_82500518(ctx, base);
	// 827E0314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0318: 4BFFF1F9  bl 0x827df510
	ctx.lr = 0x827E031C;
	sub_827DF510(ctx, base);
	// 827E031C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0320: 4BFFF071  bl 0x827df390
	ctx.lr = 0x827E0324;
	sub_827DF390(ctx, base);
	// 827E0324: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827E0328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E032C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0330: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0338 size=80
    let mut pc: u32 = 0x827E0338;
    'dispatch: loop {
        match pc {
            0x827E0338 => {
    //   block [0x827E0338..0x827E0388)
	// 827E0338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E033C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827E0344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E034C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827E0350: 3BFE0054  addi r31, r30, 0x54
	ctx.r[31].s64 = ctx.r[30].s64 + 84;
	// 827E0354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0358: 4BFFFF79  bl 0x827e02d0
	ctx.lr = 0x827E035C;
	sub_827E02D0(ctx, base);
	// 827E035C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827E0360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0364: 396BBF8C  addi r11, r11, -0x4074
	ctx.r[11].s64 = ctx.r[11].s64 + -16500;
	// 827E0368: 917E0054  stw r11, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827E036C: 48611075  bl 0x82df13e0
	ctx.lr = 0x827E0370;
	sub_82DF13E0(ctx, base);
	// 827E0370: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E0374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E037C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827E0380: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0388 size=80
    let mut pc: u32 = 0x827E0388;
    'dispatch: loop {
        match pc {
            0x827E0388 => {
    //   block [0x827E0388..0x827E03D8)
	// 827E0388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827E0394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E039C: 3BE3FFAC  addi r31, r3, -0x54
	ctx.r[31].s64 = ctx.r[3].s64 + -84;
	// 827E03A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827E03A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E03A8: 4BFFFF91  bl 0x827e0338
	ctx.lr = 0x827E03AC;
	sub_827E0338(ctx, base);
	// 827E03AC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E03B0: 4182000C  beq 0x827e03bc
	if ctx.cr[0].eq {
	pc = 0x827E03BC; continue 'dispatch;
	}
	// 827E03B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E03B8: 4BADFEB1  bl 0x822c0268
	ctx.lr = 0x827E03BC;
	sub_822C0268(ctx, base);
	// 827E03BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E03C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E03C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E03C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E03CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827E03D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E03D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E03D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E03D8 size=96
    let mut pc: u32 = 0x827E03D8;
    'dispatch: loop {
        match pc {
            0x827E03D8 => {
    //   block [0x827E03D8..0x827E0438)
	// 827E03D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E03DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E03E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E03E4: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E03E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E03EC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827E03F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827E03F4: 396B56CC  addi r11, r11, 0x56cc
	ctx.r[11].s64 = ctx.r[11].s64 + 22220;
	// 827E03F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E03FC: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 827E0400: 388AD080  addi r4, r10, -0x2f80
	ctx.r[4].s64 = ctx.r[10].s64 + -12160;
	// 827E0404: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0408: 4BBCD6B9  bl 0x823adac0
	ctx.lr = 0x827E040C;
	sub_823ADAC0(ctx, base);
	// 827E040C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0410: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827E0414: 4BFFEEC5  bl 0x827df2d8
	ctx.lr = 0x827E0418;
	sub_827DF2D8(ctx, base);
	// 827E0418: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E041C: 4BBC7B95  bl 0x823a7fb0
	ctx.lr = 0x827E0420;
	sub_823A7FB0(ctx, base);
	// 827E0420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0424: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827E0428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E042C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0430: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0438 size=84
    let mut pc: u32 = 0x827E0438;
    'dispatch: loop {
        match pc {
            0x827E0438 => {
    //   block [0x827E0438..0x827E048C)
	// 827E0438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E043C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0440: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0444: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E044C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827E0450: 396B56CC  addi r11, r11, 0x56cc
	ctx.r[11].s64 = ctx.r[11].s64 + 22220;
	// 827E0454: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827E0458: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E045C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E0460: 419A000C  beq cr6, 0x827e046c
	if ctx.cr[6].eq {
	pc = 0x827E046C; continue 'dispatch;
	}
	// 827E0464: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827E0468: 489CB671  bl 0x831abad8
	ctx.lr = 0x827E046C;
	sub_831ABAD8(ctx, base);
	// 827E046C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827E0470: 396BBF7C  addi r11, r11, -0x4084
	ctx.r[11].s64 = ctx.r[11].s64 + -16516;
	// 827E0474: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827E047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0490 size=72
    let mut pc: u32 = 0x827E0490;
    'dispatch: loop {
        match pc {
            0x827E0490 => {
    //   block [0x827E0490..0x827E04D8)
	// 827E0490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0498: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E049C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E04A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827E04A4: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 827E04A8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 827E04AC: 486114E5  bl 0x82df1990
	ctx.lr = 0x827E04B0;
	sub_82DF1990(ctx, base);
	// 827E04B0: 7C6BF838  and r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[31].u64;
	// 827E04B4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 827E04B8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827E04BC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827E04C0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 827E04C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827E04C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E04CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E04D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E04D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E04D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E04D8 size=24
    let mut pc: u32 = 0x827E04D8;
    'dispatch: loop {
        match pc {
            0x827E04D8 => {
    //   block [0x827E04D8..0x827E04F0)
	// 827E04D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827E04DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827E04E0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827E04E4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 827E04E8: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 827E04EC: 486114CC  b 0x82df19b8
	sub_82DF19B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E04F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E04F0 size=92
    let mut pc: u32 = 0x827E04F0;
    'dispatch: loop {
        match pc {
            0x827E04F0 => {
    //   block [0x827E04F0..0x827E054C)
	// 827E04F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E04F4: 489C7C75  bl 0x831a8168
	ctx.lr = 0x827E04F8;
	sub_831A8130(ctx, base);
	// 827E04F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E04FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827E0500: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827E0504: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827E0508: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 827E050C: 4800002C  b 0x827e0538
	pc = 0x827E0538; continue 'dispatch;
	// 827E0510: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0514: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827E0518: A0BF0000  lhz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E051C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827E0520: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827E0524: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827E0528: 4E800421  bctrl
	ctx.lr = 0x827E052C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827E052C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E0530: 40820010  bne 0x827e0540
	if !ctx.cr[0].eq {
	pc = 0x827E0540; continue 'dispatch;
	}
	// 827E0534: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 827E0538: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 827E053C: 409AFFD4  bne cr6, 0x827e0510
	if !ctx.cr[6].eq {
	pc = 0x827E0510; continue 'dispatch;
	}
	// 827E0540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827E0548: 489C7C70  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0550 size=92
    let mut pc: u32 = 0x827E0550;
    'dispatch: loop {
        match pc {
            0x827E0550 => {
    //   block [0x827E0550..0x827E05AC)
	// 827E0550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0554: 489C7C15  bl 0x831a8168
	ctx.lr = 0x827E0558;
	sub_831A8130(ctx, base);
	// 827E0558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E055C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827E0560: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827E0564: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827E0568: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 827E056C: 4800002C  b 0x827e0598
	pc = 0x827E0598; continue 'dispatch;
	// 827E0570: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0574: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827E0578: A0BF0000  lhz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E057C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827E0580: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827E0584: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827E0588: 4E800421  bctrl
	ctx.lr = 0x827E058C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827E058C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E0590: 41820010  beq 0x827e05a0
	if ctx.cr[0].eq {
	pc = 0x827E05A0; continue 'dispatch;
	}
	// 827E0594: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 827E0598: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 827E059C: 409AFFD4  bne cr6, 0x827e0570
	if !ctx.cr[6].eq {
	pc = 0x827E0570; continue 'dispatch;
	}
	// 827E05A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E05A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827E05A8: 489C7C10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E05B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E05B0 size=16
    let mut pc: u32 = 0x827E05B0;
    'dispatch: loop {
        match pc {
            0x827E05B0 => {
    //   block [0x827E05B0..0x827E05C0)
	// 827E05B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827E05B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827E05B8: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 827E05BC: 48611444  b 0x82df1a00
	sub_82DF1A00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E05C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E05C0 size=72
    let mut pc: u32 = 0x827E05C0;
    'dispatch: loop {
        match pc {
            0x827E05C0 => {
    //   block [0x827E05C0..0x827E0608)
	// 827E05C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E05C4: 489C7BA9  bl 0x831a816c
	ctx.lr = 0x827E05C8;
	sub_831A8130(ctx, base);
	// 827E05C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E05CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827E05D0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827E05D4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827E05D8: 419A0024  beq cr6, 0x827e05fc
	if ctx.cr[6].eq {
	pc = 0x827E05FC; continue 'dispatch;
	}
	// 827E05DC: 3BA30008  addi r29, r3, 8
	ctx.r[29].s64 = ctx.r[3].s64 + 8;
	// 827E05E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827E05E4: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E05E8: 48611419  bl 0x82df1a00
	ctx.lr = 0x827E05EC;
	sub_82DF1A00(ctx, base);
	// 827E05EC: B07F0000  sth r3, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 827E05F0: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 827E05F4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827E05F8: 409AFFE8  bne cr6, 0x827e05e0
	if !ctx.cr[6].eq {
	pc = 0x827E05E0; continue 'dispatch;
	}
	// 827E05FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E0604: 489C7BB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E0608 size=16
    let mut pc: u32 = 0x827E0608;
    'dispatch: loop {
        match pc {
            0x827E0608 => {
    //   block [0x827E0608..0x827E0618)
	// 827E0608: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827E060C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827E0610: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 827E0614: 4861142C  b 0x82df1a40
	sub_82DF1A40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0618 size=72
    let mut pc: u32 = 0x827E0618;
    'dispatch: loop {
        match pc {
            0x827E0618 => {
    //   block [0x827E0618..0x827E0660)
	// 827E0618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E061C: 489C7B51  bl 0x831a816c
	ctx.lr = 0x827E0620;
	sub_831A8130(ctx, base);
	// 827E0620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0624: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827E0628: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827E062C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827E0630: 419A0024  beq cr6, 0x827e0654
	if ctx.cr[6].eq {
	pc = 0x827E0654; continue 'dispatch;
	}
	// 827E0634: 3BA30008  addi r29, r3, 8
	ctx.r[29].s64 = ctx.r[3].s64 + 8;
	// 827E0638: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827E063C: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0640: 48611401  bl 0x82df1a40
	ctx.lr = 0x827E0644;
	sub_82DF1A40(ctx, base);
	// 827E0644: B07F0000  sth r3, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 827E0648: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 827E064C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827E0650: 409AFFE8  bne cr6, 0x827e0638
	if !ctx.cr[6].eq {
	pc = 0x827E0638; continue 'dispatch;
	}
	// 827E0654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0658: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E065C: 489C7B60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0660 size=80
    let mut pc: u32 = 0x827E0660;
    'dispatch: loop {
        match pc {
            0x827E0660 => {
    //   block [0x827E0660..0x827E06B0)
	// 827E0660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E066C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827E0670: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 827E0674: 38E30018  addi r7, r3, 0x18
	ctx.r[7].s64 = ctx.r[3].s64 + 24;
	// 827E0678: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827E067C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 827E0680: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827E0684: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 827E0688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E068C: 486113F5  bl 0x82df1a80
	ctx.lr = 0x827E0690;
	sub_82DF1A80(ctx, base);
	// 827E0690: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 827E0694: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827E0698: 41800008  blt 0x827e06a0
	if ctx.cr[0].lt {
	pc = 0x827E06A0; continue 'dispatch;
	}
	// 827E069C: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827E06A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827E06A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E06A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E06AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E06B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E06B0 size=4
    let mut pc: u32 = 0x827E06B0;
    'dispatch: loop {
        match pc {
            0x827E06B0 => {
    //   block [0x827E06B0..0x827E06B4)
	// 827E06B0: 4BFFFFB0  b 0x827e0660
	sub_827E0660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E06B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E06B8 size=20
    let mut pc: u32 = 0x827E06B8;
    'dispatch: loop {
        match pc {
            0x827E06B8 => {
    //   block [0x827E06B8..0x827E06CC)
	// 827E06B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E06BC: 7CE42850  subf r7, r4, r5
	ctx.r[7].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 827E06C0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 827E06C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827E06C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E06D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E06D0 size=92
    let mut pc: u32 = 0x827E06D0;
    'dispatch: loop {
        match pc {
            0x827E06D0 => {
    //   block [0x827E06D0..0x827E072C)
	// 827E06D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E06D4: 489C7A95  bl 0x831a8168
	ctx.lr = 0x827E06D8;
	sub_831A8130(ctx, base);
	// 827E06D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E06DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827E06E0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827E06E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827E06E8: 7D7FE850  subf r11, r31, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 827E06EC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827E06F0: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827E06F4: 40980024  bge cr6, 0x827e0718
	if !ctx.cr[6].lt {
	pc = 0x827E0718; continue 'dispatch;
	}
	// 827E06F8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 827E06FC: 4800001C  b 0x827e0718
	pc = 0x827E0718; continue 'dispatch;
	// 827E0700: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827E0704: 889F0000  lbz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0708: 4BFFFF59  bl 0x827e0660
	ctx.lr = 0x827E070C;
	sub_827E0660(ctx, base);
	// 827E070C: B07E0000  sth r3, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 827E0710: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 827E0714: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 827E0718: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 827E071C: 409AFFE4  bne cr6, 0x827e0700
	if !ctx.cr[6].eq {
	pc = 0x827E0700; continue 'dispatch;
	}
	// 827E0720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827E0728: 489C7A90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0730 size=80
    let mut pc: u32 = 0x827E0730;
    'dispatch: loop {
        match pc {
            0x827E0730 => {
    //   block [0x827E0730..0x827E0780)
	// 827E0730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E073C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827E0744: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827E0748: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827E074C: 38C30018  addi r6, r3, 0x18
	ctx.r[6].s64 = ctx.r[3].s64 + 24;
	// 827E0750: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827E0754: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827E0758: 486102B1  bl 0x82df0a08
	ctx.lr = 0x827E075C;
	sub_82DF0A08(ctx, base);
	// 827E075C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 827E0760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0764: 409A0008  bne cr6, 0x827e076c
	if !ctx.cr[6].eq {
	pc = 0x827E076C; continue 'dispatch;
	}
	// 827E0768: 88610054  lbz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827E076C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E0770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E077C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E0780 size=24
    let mut pc: u32 = 0x827E0780;
    'dispatch: loop {
        match pc {
            0x827E0780 => {
    //   block [0x827E0780..0x827E0798)
	// 827E0780: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0784: 7D442850  subf r10, r4, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 827E0788: 7D480E70  srawi r8, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 827E078C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 827E0790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827E0794: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0798 size=136
    let mut pc: u32 = 0x827E0798;
    'dispatch: loop {
        match pc {
            0x827E0798 => {
    //   block [0x827E0798..0x827E0820)
	// 827E0798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E079C: 489C79C9  bl 0x831a8164
	ctx.lr = 0x827E07A0;
	sub_831A8130(ctx, base);
	// 827E07A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E07A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827E07A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827E07AC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 827E07B0: 7D7FF050  subf r11, r31, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 827E07B4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 827E07B8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827E07BC: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827E07C0: 40980008  bge cr6, 0x827e07c8
	if !ctx.cr[6].lt {
	pc = 0x827E07C8; continue 'dispatch;
	}
	// 827E07C4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 827E07C8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827E07CC: 419A0048  beq cr6, 0x827e0814
	if ctx.cr[6].eq {
	pc = 0x827E0814; continue 'dispatch;
	}
	// 827E07D0: 3B830018  addi r28, r3, 0x18
	ctx.r[28].s64 = ctx.r[3].s64 + 24;
	// 827E07D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827E07D8: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E07DC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 827E07E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827E07E4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827E07E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827E07EC: 4861021D  bl 0x82df0a08
	ctx.lr = 0x827E07F0;
	sub_82DF0A08(ctx, base);
	// 827E07F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 827E07F4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 827E07F8: 409A0008  bne cr6, 0x827e0800
	if !ctx.cr[6].eq {
	pc = 0x827E0800; continue 'dispatch;
	}
	// 827E07FC: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827E0800: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 827E0804: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 827E0808: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 827E080C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827E0810: 409AFFC4  bne cr6, 0x827e07d4
	if !ctx.cr[6].eq {
	pc = 0x827E07D4; continue 'dispatch;
	}
	// 827E0814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0818: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827E081C: 489C7998  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0820 size=76
    let mut pc: u32 = 0x827E0820;
    'dispatch: loop {
        match pc {
            0x827E0820 => {
    //   block [0x827E0820..0x827E086C)
	// 827E0820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827E082C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E0838: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827E083C: 4BFFFBFD  bl 0x827e0438
	ctx.lr = 0x827E0840;
	sub_827E0438(ctx, base);
	// 827E0840: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E0844: 4182000C  beq 0x827e0850
	if ctx.cr[0].eq {
	pc = 0x827E0850; continue 'dispatch;
	}
	// 827E0848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E084C: 4BADFA1D  bl 0x822c0268
	ctx.lr = 0x827E0850;
	sub_822C0268(ctx, base);
	// 827E0850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E0858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E085C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0860: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827E0864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0870 size=100
    let mut pc: u32 = 0x827E0870;
    'dispatch: loop {
        match pc {
            0x827E0870 => {
    //   block [0x827E0870..0x827E08D4)
	// 827E0870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E087C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E0884: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827E0888: 419A0034  beq cr6, 0x827e08bc
	if ctx.cr[6].eq {
	pc = 0x827E08BC; continue 'dispatch;
	}
	// 827E088C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0890: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827E0894: 409A0028  bne cr6, 0x827e08bc
	if !ctx.cr[6].eq {
	pc = 0x827E08BC; continue 'dispatch;
	}
	// 827E0898: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 827E089C: 4BAE009D  bl 0x822c0938
	ctx.lr = 0x827E08A0;
	sub_822C0938(ctx, base);
	// 827E08A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827E08A4: 41820010  beq 0x827e08b4
	if ctx.cr[0].eq {
	pc = 0x827E08B4; continue 'dispatch;
	}
	// 827E08A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E08AC: 4BFFFB2D  bl 0x827e03d8
	ctx.lr = 0x827E08B0;
	sub_827E03D8(ctx, base);
	// 827E08B0: 48000008  b 0x827e08b8
	pc = 0x827E08B8; continue 'dispatch;
	// 827E08B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827E08B8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 827E08BC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 827E08C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827E08C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E08C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E08CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E08D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E08D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E08D8 size=292
    let mut pc: u32 = 0x827E08D8;
    'dispatch: loop {
        match pc {
            0x827E08D8 => {
    //   block [0x827E08D8..0x827E09FC)
	// 827E08D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E08DC: 489C7891  bl 0x831a816c
	ctx.lr = 0x827E08E0;
	sub_831A8130(ctx, base);
	// 827E08E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E08E4: 81440040  lwz r10, 0x40(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 827E08E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827E08EC: 554B07BD  rlwinm. r11, r10, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E08F0: 40820068  bne 0x827e0958
	if !ctx.cr[0].eq {
	pc = 0x827E0958; continue 'dispatch;
	}
	// 827E08F4: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 827E08F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E08FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E0900: 419A0058  beq cr6, 0x827e0958
	if ctx.cr[6].eq {
	pc = 0x827E0958; continue 'dispatch;
	}
	// 827E0904: 8144003C  lwz r10, 0x3c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 827E0908: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 827E090C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827E0910: 41980008  blt cr6, 0x827e0918
	if ctx.cr[6].lt {
	pc = 0x827E0918; continue 'dispatch;
	}
	// 827E0914: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 827E0918: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 827E091C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827E0920: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E0924: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E0928: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E092C: 4BAE8505  bl 0x822c8e30
	ctx.lr = 0x827E0930;
	sub_822C8E30(ctx, base);
	// 827E0930: 7D7DF850  subf r11, r29, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 827E0934: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827E0938: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827E093C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E0940: 4BAE88E9  bl 0x822c9228
	ctx.lr = 0x827E0944;
	sub_822C9228(ctx, base);
	// 827E0944: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827E0948: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827E094C: 4BAE950D  bl 0x822c9e58
	ctx.lr = 0x827E0950;
	sub_822C9E58(ctx, base);
	// 827E0950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E0954: 48000090  b 0x827e09e4
	pc = 0x827E09E4; continue 'dispatch;
	// 827E0958: 554B077B  rlwinm. r11, r10, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827E095C: 40820068  bne 0x827e09c4
	if !ctx.cr[0].eq {
	pc = 0x827E09C4; continue 'dispatch;
	}
	// 827E0960: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 827E0964: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0968: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E096C: 419A0058  beq cr6, 0x827e09c4
	if ctx.cr[6].eq {
	pc = 0x827E09C4; continue 'dispatch;
	}
	// 827E0970: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 827E0974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827E0978: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827E097C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E0980: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827E0984: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0988: 83A90000  lwz r29, 0(r9)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E098C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827E0990: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 827E0994: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827E0998: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 827E099C: 4BAE8495  bl 0x822c8e30
	ctx.lr = 0x827E09A0;
	sub_822C8E30(ctx, base);
	// 827E09A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827E09A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827E09A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827E09AC: 4BAE887D  bl 0x822c9228
	ctx.lr = 0x827E09B0;
	sub_822C9228(ctx, base);
	// 827E09B0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827E09B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827E09B8: 4BAE94A1  bl 0x822c9e58
	ctx.lr = 0x827E09BC;
	sub_822C9E58(ctx, base);
	// 827E09BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827E09C0: 48000024  b 0x827e09e4
	pc = 0x827E09E4; continue 'dispatch;
	// 827E09C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827E09C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E09CC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 827E09D0: 4BAE8461  bl 0x822c8e30
	ctx.lr = 0x827E09D4;
	sub_822C8E30(ctx, base);
	// 827E09D4: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 827E09D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827E09DC: 4BAE947D  bl 0x822c9e58
	ctx.lr = 0x827E09E0;
	sub_822C9E58(ctx, base);
	// 827E09E0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 827E09E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827E09E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827E09EC: 4BAE8445  bl 0x822c8e30
	ctx.lr = 0x827E09F0;
	sub_822C8E30(ctx, base);
	// 827E09F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827E09F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827E09F8: 489C77C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0A00 size=96
    let mut pc: u32 = 0x827E0A00;
    'dispatch: loop {
        match pc {
            0x827E0A00 => {
    //   block [0x827E0A00..0x827E0A60)
	// 827E0A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0A0C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0A10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E0A14: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827E0A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827E0A1C: 396B570C  addi r11, r11, 0x570c
	ctx.r[11].s64 = ctx.r[11].s64 + 22284;
	// 827E0A20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E0A24: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 827E0A28: 388AD080  addi r4, r10, -0x2f80
	ctx.r[4].s64 = ctx.r[10].s64 + -12160;
	// 827E0A2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0A30: 4BBCD091  bl 0x823adac0
	ctx.lr = 0x827E0A34;
	sub_823ADAC0(ctx, base);
	// 827E0A34: 48610025  bl 0x82df0a58
	ctx.lr = 0x827E0A38;
	sub_82DF0A58(ctx, base);
	// 827E0A38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827E0A3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E0A40: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 827E0A44: 4BBC756D  bl 0x823a7fb0
	ctx.lr = 0x827E0A48;
	sub_823A7FB0(ctx, base);
	// 827E0A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0A4C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827E0A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0A58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0A60 size=244
    let mut pc: u32 = 0x827E0A60;
    'dispatch: loop {
        match pc {
            0x827E0A60 => {
    //   block [0x827E0A60..0x827E0B54)
	// 827E0A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0A64: 489C76F9  bl 0x831a815c
	ctx.lr = 0x827E0A68;
	sub_831A8130(ctx, base);
	// 827E0A68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0A6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827E0A70: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 827E0A74: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 827E0A78: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 827E0A7C: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 827E0A80: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0A84: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 827E0A88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827E0A8C: 409AFFF4  bne cr6, 0x827e0a80
	if !ctx.cr[6].eq {
	pc = 0x827E0A80; continue 'dispatch;
	}
	// 827E0A90: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 827E0A94: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 827E0A98: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827E0A9C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 827E0AA0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827E0AA4: 376B0001  addic. r27, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 827E0AA8: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 827E0AAC: 41820034  beq 0x827e0ae0
	if ctx.cr[0].eq {
	pc = 0x827E0AE0; continue 'dispatch;
	}
	// 827E0AB0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 827E0AB4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 827E0AB8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827E0ABC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827E0AC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E0AC4: 48610FBD  bl 0x82df1a80
	ctx.lr = 0x827E0AC8;
	sub_82DF1A80(ctx, base);
	// 827E0AC8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 827E0ACC: 40810014  ble 0x827e0ae0
	if !ctx.cr[0].gt {
	pc = 0x827E0AE0; continue 'dispatch;
	}
	// 827E0AD0: 7FE3F851  subf. r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827E0AD4: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 827E0AD8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 827E0ADC: 4082FFD4  bne 0x827e0ab0
	if !ctx.cr[0].eq {
	pc = 0x827E0AB0; continue 'dispatch;
	}
	// 827E0AE0: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 827E0AE4: 3BDC0001  addi r30, r28, 1
	ctx.r[30].s64 = ctx.r[28].s64 + 1;
	// 827E0AE8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 827E0AEC: 57C3083C  slwi r3, r30, 1
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 827E0AF0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827E0AF4: 40990008  ble cr6, 0x827e0afc
	if !ctx.cr[6].gt {
	pc = 0x827E0AFC; continue 'dispatch;
	}
	// 827E0AF8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 827E0AFC: 4860FA85  bl 0x82df0580
	ctx.lr = 0x827E0B00;
	sub_82DF0580(ctx, base);
	// 827E0B00: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827E0B04: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 827E0B08: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827E0B0C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 827E0B10: 419A0034  beq cr6, 0x827e0b44
	if ctx.cr[6].eq {
	pc = 0x827E0B44; continue 'dispatch;
	}
	// 827E0B14: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 827E0B18: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 827E0B1C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 827E0B20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827E0B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0B28: 48610F59  bl 0x82df1a80
	ctx.lr = 0x827E0B2C;
	sub_82DF1A80(ctx, base);
	// 827E0B2C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 827E0B30: 40810014  ble 0x827e0b44
	if !ctx.cr[0].gt {
	pc = 0x827E0B44; continue 'dispatch;
	}
	// 827E0B34: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827E0B38: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 827E0B3C: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 827E0B40: 4082FFD4  bne 0x827e0b14
	if !ctx.cr[0].eq {
	pc = 0x827E0B14; continue 'dispatch;
	}
	// 827E0B44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827E0B48: B35F0000  sth r26, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 827E0B4C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827E0B50: 489C765C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0B58 size=264
    let mut pc: u32 = 0x827E0B58;
    'dispatch: loop {
        match pc {
            0x827E0B58 => {
    //   block [0x827E0B58..0x827E0C60)
	// 827E0B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0B5C: 489C7611  bl 0x831a816c
	ctx.lr = 0x827E0B60;
	sub_831A8130(ctx, base);
	// 827E0B60: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0B64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827E0B68: 489CAFD1  bl 0x831abb38
	ctx.lr = 0x827E0B6C;
	sub_831ABB38(ctx, base);
	// 827E0B6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827E0B70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827E0B74: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 827E0B78: 93FE0010  stw r31, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 827E0B7C: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 827E0B80: 4860FED9  bl 0x82df0a58
	ctx.lr = 0x827E0B84;
	sub_82DF0A58(ctx, base);
	// 827E0B84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827E0B88: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827E0B8C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 827E0B90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E0B94: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 827E0B98: 4BBCD7A1  bl 0x823ae338
	ctx.lr = 0x827E0B9C;
	sub_823AE338(ctx, base);
	// 827E0B9C: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 827E0BA0: 4860FEB9  bl 0x82df0a58
	ctx.lr = 0x827E0BA4;
	sub_82DF0A58(ctx, base);
	// 827E0BA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827E0BA8: F8610060  std r3, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u64 ) };
	// 827E0BAC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827E0BB0: 386B9FF4  addi r3, r11, -0x600c
	ctx.r[3].s64 = ctx.r[11].s64 + -24588;
	// 827E0BB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E0BB8: 4BFFFEA9  bl 0x827e0a60
	ctx.lr = 0x827E0BBC;
	sub_827E0A60(ctx, base);
	// 827E0BBC: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 827E0BC0: 4860FE99  bl 0x82df0a58
	ctx.lr = 0x827E0BC4;
	sub_82DF0A58(ctx, base);
	// 827E0BC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827E0BC8: F8610068  std r3, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u64 ) };
	// 827E0BCC: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 827E0BD0: 386B9FFC  addi r3, r11, -0x6004
	ctx.r[3].s64 = ctx.r[11].s64 + -24580;
	// 827E0BD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E0BD8: 4BFFFE89  bl 0x827e0a60
	ctx.lr = 0x827E0BDC;
	sub_827E0A60(ctx, base);
	// 827E0BDC: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 827E0BE0: 4860FE79  bl 0x82df0a58
	ctx.lr = 0x827E0BE4;
	sub_82DF0A58(ctx, base);
	// 827E0BE4: F8610070  std r3, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u64 ) };
	// 827E0BE8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0BEC: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 827E0BF0: B3E10052  sth r31, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[31].u16 ) };
	// 827E0BF4: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 827E0BF8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 827E0BFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827E0C00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827E0C04: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 827E0C08: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0C0C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 827E0C10: 48610E71  bl 0x82df1a80
	ctx.lr = 0x827E0C14;
	sub_82DF1A80(ctx, base);
	// 827E0C14: A1610052  lhz r11, 0x52(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 827E0C18: B17E000C  sth r11, 0xc(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 827E0C1C: 4860FE3D  bl 0x82df0a58
	ctx.lr = 0x827E0C20;
	sub_82DF0A58(ctx, base);
	// 827E0C20: F8610078  std r3, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[3].u64 ) };
	// 827E0C24: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827E0C28: 38E10078  addi r7, r1, 0x78
	ctx.r[7].s64 = ctx.r[1].s64 + 120;
	// 827E0C2C: B3E10052  sth r31, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[31].u16 ) };
	// 827E0C30: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 827E0C34: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 827E0C38: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827E0C3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827E0C40: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 827E0C44: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0C48: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 827E0C4C: 48610E35  bl 0x82df1a80
	ctx.lr = 0x827E0C50;
	sub_82DF1A80(ctx, base);
	// 827E0C50: A1610052  lhz r11, 0x52(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 827E0C54: B17E000E  sth r11, 0xe(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 827E0C58: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827E0C5C: 489C7560  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0C60 size=80
    let mut pc: u32 = 0x827E0C60;
    'dispatch: loop {
        match pc {
            0x827E0C60 => {
    //   block [0x827E0C60..0x827E0CB0)
	// 827E0C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0C70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E0C74: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827E0C78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827E0C7C: 419A0014  beq cr6, 0x827e0c90
	if ctx.cr[6].eq {
	pc = 0x827E0C90; continue 'dispatch;
	}
	// 827E0C80: 4BFFEC61  bl 0x827df8e0
	ctx.lr = 0x827E0C84;
	sub_827DF8E0(ctx, base);
	// 827E0C84: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 827E0C88: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 827E0C8C: 409A000C  bne cr6, 0x827e0c98
	if !ctx.cr[6].eq {
	pc = 0x827E0C98; continue 'dispatch;
	}
	// 827E0C90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827E0C94: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 827E0C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827E0CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0CB0 size=100
    let mut pc: u32 = 0x827E0CB0;
    'dispatch: loop {
        match pc {
            0x827E0CB0 => {
    //   block [0x827E0CB0..0x827E0D14)
	// 827E0CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0CB4: 489C74B9  bl 0x831a816c
	ctx.lr = 0x827E0CB8;
	sub_831A8130(ctx, base);
	// 827E0CB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0CBC: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 827E0CC0: F8A10090  std r5, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[5].u64 ) };
	// 827E0CC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827E0CC8: F8C10098  std r6, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[6].u64 ) };
	// 827E0CCC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 827E0CD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827E0CD4: 419A001C  beq cr6, 0x827e0cf0
	if ctx.cr[6].eq {
	pc = 0x827E0CF0; continue 'dispatch;
	}
	// 827E0CD8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 827E0CDC: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0CE0: 4BFFFF81  bl 0x827e0c60
	ctx.lr = 0x827E0CE4;
	sub_827E0C60(ctx, base);
	// 827E0CE4: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827E0CE8: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 827E0CEC: 4082FFEC  bne 0x827e0cd8
	if !ctx.cr[0].eq {
	pc = 0x827E0CD8; continue 'dispatch;
	}
	// 827E0CF0: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 827E0CF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827E0CF8: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 827E0CFC: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 827E0D00: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0D04: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827E0D08: 913D0008  stw r9, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 827E0D0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E0D10: 489C74AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0D18 size=96
    let mut pc: u32 = 0x827E0D18;
    'dispatch: loop {
        match pc {
            0x827E0D18 => {
    //   block [0x827E0D18..0x827E0D78)
	// 827E0D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0D1C: 489C7451  bl 0x831a816c
	ctx.lr = 0x827E0D20;
	sub_831A8130(ctx, base);
	// 827E0D20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0D24: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 827E0D28: F8A10090  std r5, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[5].u64 ) };
	// 827E0D2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827E0D30: F8C10098  std r6, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[6].u64 ) };
	// 827E0D34: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 827E0D38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827E0D3C: 419A0018  beq cr6, 0x827e0d54
	if ctx.cr[6].eq {
	pc = 0x827E0D54; continue 'dispatch;
	}
	// 827E0D40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827E0D44: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 827E0D48: 4BFFFF19  bl 0x827e0c60
	ctx.lr = 0x827E0D4C;
	sub_827E0C60(ctx, base);
	// 827E0D4C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827E0D50: 4082FFF0  bne 0x827e0d40
	if !ctx.cr[0].eq {
	pc = 0x827E0D40; continue 'dispatch;
	}
	// 827E0D54: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 827E0D58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827E0D5C: 81410094  lwz r10, 0x94(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 827E0D60: 81210098  lwz r9, 0x98(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 827E0D64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0D68: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827E0D6C: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 827E0D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827E0D74: 489C7448  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0D78 size=148
    let mut pc: u32 = 0x827E0D78;
    'dispatch: loop {
        match pc {
            0x827E0D78 => {
    //   block [0x827E0D78..0x827E0E0C)
	// 827E0D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0D7C: 489C73E9  bl 0x831a8164
	ctx.lr = 0x827E0D80;
	sub_831A8130(ctx, base);
	// 827E0D80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0D84: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 827E0D88: F8A100B0  std r5, 0xb0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u64 ) };
	// 827E0D8C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 827E0D90: F8C100B8  std r6, 0xb8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[6].u64 ) };
	// 827E0D94: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 827E0D98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827E0D9C: 419A004C  beq cr6, 0x827e0de8
	if ctx.cr[6].eq {
	pc = 0x827E0DE8; continue 'dispatch;
	}
	// 827E0DA0: 3B840008  addi r28, r4, 8
	ctx.r[28].s64 = ctx.r[4].s64 + 8;
	// 827E0DA4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827E0DA8: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0DAC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 827E0DB0: B3A10052  sth r29, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[29].u16 ) };
	// 827E0DB4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 827E0DB8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 827E0DBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827E0DC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827E0DC4: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 827E0DC8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 827E0DCC: 48610CB5  bl 0x82df1a80
	ctx.lr = 0x827E0DD0;
	sub_82DF1A80(ctx, base);
	// 827E0DD0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 827E0DD4: A0810052  lhz r4, 0x52(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 827E0DD8: 4BFFFE89  bl 0x827e0c60
	ctx.lr = 0x827E0DDC;
	sub_827E0C60(ctx, base);
	// 827E0DDC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827E0DE0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 827E0DE4: 4082FFC4  bne 0x827e0da8
	if !ctx.cr[0].eq {
	pc = 0x827E0DA8; continue 'dispatch;
	}
	// 827E0DE8: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 827E0DEC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827E0DF0: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 827E0DF4: 812100B8  lwz r9, 0xb8(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 827E0DF8: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0DFC: 915B0004  stw r10, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827E0E00: 913B0008  stw r9, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 827E0E04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827E0E08: 489C73AC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0E10 size=252
    let mut pc: u32 = 0x827E0E10;
    'dispatch: loop {
        match pc {
            0x827E0E10 => {
    //   block [0x827E0E10..0x827E0F0C)
	// 827E0E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0E14: 489C7349  bl 0x831a815c
	ctx.lr = 0x827E0E18;
	sub_831A8130(ctx, base);
	// 827E0E18: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0E1C: F8A100D0  std r5, 0xd0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[5].u64 ) };
	// 827E0E20: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 827E0E24: F8C100D8  std r6, 0xd8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u64 ) };
	// 827E0E28: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 827E0E2C: 838100D8  lwz r28, 0xd8(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(216 as u32) ) } as u64;
	// 827E0E30: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 827E0E34: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 827E0E38: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 827E0E3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827E0E40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827E0E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0E48: 489C92D9  bl 0x831aa120
	ctx.lr = 0x827E0E4C;
	sub_831AA120(ctx, base);
	// 827E0E4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827E0E50: 7FDF1850  subf r30, r31, r3
	ctx.r[30].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 827E0E54: 40820008  bne 0x827e0e5c
	if !ctx.cr[0].eq {
	pc = 0x827E0E5C; continue 'dispatch;
	}
	// 827E0E58: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 827E0E5C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 827E0E60: E8A100D0  ld r5, 0xd0(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(208 as u32) ) };
	// 827E0E64: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 827E0E68: 7B8607C6  sldi r6, r28, 0x20
	ctx.r[6].u64 = ctx.r[28].u64.wrapping_shl(32);
	ctx.r[6].u32 = ctx.r[6].u64 as u32;
	// 827E0E6C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827E0E70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827E0E74: 4BFFFF05  bl 0x827e0d78
	ctx.lr = 0x827E0E78;
	sub_827E0D78(ctx, base);
	// 827E0E78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827E0E7C: 7FBEE851  subf. r29, r30, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827E0E80: 7FFEFA14  add r31, r30, r31
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 827E0E84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0E88: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827E0E8C: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827E0E90: 914100D0  stw r10, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[10].u32 ) };
	// 827E0E94: 912100D4  stw r9, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[9].u32 ) };
	// 827E0E98: 41820050  beq 0x827e0ee8
	if ctx.cr[0].eq {
	pc = 0x827E0EE8; continue 'dispatch;
	}
	// 827E0E9C: 572A043F  clrlwi. r10, r25, 0x10
	ctx.r[10].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827E0EA0: 4182003C  beq 0x827e0edc
	if ctx.cr[0].eq {
	pc = 0x827E0EDC; continue 'dispatch;
	}
	// 827E0EA4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0EA8: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827E0EAC: 578B003E  slwi r11, r28, 0
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827E0EB0: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 827E0EB4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 827E0EB8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827E0EBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827E0EC0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827E0EC4: 4BFFFD9D  bl 0x827e0c60
	ctx.lr = 0x827E0EC8;
	sub_827E0C60(ctx, base);
	// 827E0EC8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827E0ECC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827E0ED0: 916100D0  stw r11, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 827E0ED4: 914100D4  stw r10, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 827E0ED8: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827E0EDC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 827E0EE0: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 827E0EE4: 4BFFFF58  b 0x827e0e3c
	pc = 0x827E0E3C; continue 'dispatch;
	// 827E0EE8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827E0EEC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827E0EF0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827E0EF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827E0EF8: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827E0EFC: 913A0004  stw r9, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827E0F00: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827E0F04: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827E0F08: 489C72A4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0F10 size=168
    let mut pc: u32 = 0x827E0F10;
    'dispatch: loop {
        match pc {
            0x827E0F10 => {
    //   block [0x827E0F10..0x827E0FB8)
	// 827E0F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0F1C: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E0F24: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827E0F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827E0F2C: 396B5734  addi r11, r11, 0x5734
	ctx.r[11].s64 = ctx.r[11].s64 + 22324;
	// 827E0F30: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827E0F34: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 827E0F38: 388AD080  addi r4, r10, -0x2f80
	ctx.r[4].s64 = ctx.r[10].s64 + -12160;
	// 827E0F3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0F40: 4BBCCB81  bl 0x823adac0
	ctx.lr = 0x827E0F44;
	sub_823ADAC0(ctx, base);
	// 827E0F44: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827E0F48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0F4C: 4BFFFC0D  bl 0x827e0b58
	ctx.lr = 0x827E0F50;
	sub_827E0B58(ctx, base);
	// 827E0F50: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 827E0F54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827E0F58: 40820040  bne 0x827e0f98
	if !ctx.cr[0].eq {
	pc = 0x827E0F98; continue 'dispatch;
	}
	// 827E0F5C: 4860FAFD  bl 0x82df0a58
	ctx.lr = 0x827E0F60;
	sub_82DF0A58(ctx, base);
	// 827E0F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827E0F64: 3940002C  li r10, 0x2c
	ctx.r[10].s64 = 44;
	// 827E0F68: F8610058  std r3, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u64 ) };
	// 827E0F6C: B1610052  sth r11, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 827E0F70: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 827E0F74: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 827E0F78: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 827E0F7C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827E0F80: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827E0F84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827E0F88: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 827E0F8C: 48610AF5  bl 0x82df1a80
	ctx.lr = 0x827E0F90;
	sub_82DF1A80(ctx, base);
	// 827E0F90: A1610052  lhz r11, 0x52(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 827E0F94: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 827E0F98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827E0F9C: 4BBC7015  bl 0x823a7fb0
	ctx.lr = 0x827E0FA0;
	sub_823A7FB0(ctx, base);
	// 827E0FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827E0FA4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 827E0FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E0FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E0FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E0FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E0FB8 size=8
    let mut pc: u32 = 0x827E0FB8;
    'dispatch: loop {
        match pc {
            0x827E0FB8 => {
    //   block [0x827E0FB8..0x827E0FC0)
	// 827E0FB8: A063000C  lhz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827E0FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827E0FC0 size=8
    let mut pc: u32 = 0x827E0FC0;
    'dispatch: loop {
        match pc {
            0x827E0FC0 => {
    //   block [0x827E0FC0..0x827E0FC8)
	// 827E0FC0: A063000E  lhz r3, 0xe(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 827E0FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827E0FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827E0FC8 size=88
    let mut pc: u32 = 0x827E0FC8;
    'dispatch: loop {
        match pc {
            0x827E0FC8 => {
    //   block [0x827E0FC8..0x827E1020)
	// 827E0FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827E0FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827E0FD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827E0FD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827E0FD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827E0FDC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827E0FE0: 396B5734  addi r11, r11, 0x5734
	ctx.r[11].s64 = ctx.r[11].s64 + 22324;
	// 827E0FE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E0FE8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827E0FEC: 489C7C1D  bl 0x831a8c08
	ctx.lr = 0x827E0FF0;
	sub_831A8C08(ctx, base);
	// 827E0FF0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827E0FF4: 489C7C15  bl 0x831a8c08
	ctx.lr = 0x827E0FF8;
	sub_831A8C08(ctx, base);
	// 827E0FF8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827E0FFC: 489C7C0D  bl 0x831a8c08
	ctx.lr = 0x827E1000;
	sub_831A8C08(ctx, base);
	// 827E1000: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827E1004: 396BBF7C  addi r11, r11, -0x4084
	ctx.r[11].s64 = ctx.r[11].s64 + -16516;
	// 827E1008: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827E100C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827E1010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827E1014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827E1018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827E101C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


