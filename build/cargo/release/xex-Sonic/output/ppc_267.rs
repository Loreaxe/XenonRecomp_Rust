pub fn sub_8310BAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310BAD8 size=288
    let mut pc: u32 = 0x8310BAD8;
    'dispatch: loop {
        match pc {
            0x8310BAD8 => {
    //   block [0x8310BAD8..0x8310BBF8)
	// 8310BAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310BADC: 4809C681  bl 0x831a815c
	ctx.lr = 0x8310BAE0;
	sub_831A8130(ctx, base);
	// 8310BAE0: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8310BAE4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310BAE8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8310BAEC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8310BAF0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8310BAF4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8310BAF8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8310BAFC: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 8310BB00: 4B1B4501  bl 0x822c0000
	ctx.lr = 0x8310BB04;
	sub_822C0000(ctx, base);
	// 8310BB04: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8310BB08: 418200DC  beq 0x8310bbe4
	if ctx.cr[0].eq {
	pc = 0x8310BBE4; continue 'dispatch;
	}
	// 8310BB0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8310BB10: 419A00D4  beq cr6, 0x8310bbe4
	if ctx.cr[6].eq {
	pc = 0x8310BBE4; continue 'dispatch;
	}
	// 8310BB14: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8310BB18: 409A000C  bne cr6, 0x8310bb24
	if !ctx.cr[6].eq {
	pc = 0x8310BB24; continue 'dispatch;
	}
	// 8310BB1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310BB20: 480000CC  b 0x8310bbec
	pc = 0x8310BBEC; continue 'dispatch;
	// 8310BB24: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310BB28: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310BB2C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8310BB30: C19F000C  lfs f12, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310BB34: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 8310BB38: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8310BB3C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310BB40: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 8310BB44: D9A10058  stfd f13, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[13].u64 ) };
	// 8310BB48: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 8310BB4C: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8310BB50: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310BB54: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310BB58: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8310BB5C: D8010058  stfd f0, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[0].u64 ) };
	// 8310BB60: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8310BB64: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 8310BB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8310BB6C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 8310BB70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8310BB74: 83FD0098  lwz r31, 0x98(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(152 as u32) ) } as u64;
	// 8310BB78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8310BB7C: 419A0068  beq cr6, 0x8310bbe4
	if ctx.cr[6].eq {
	pc = 0x8310BBE4; continue 'dispatch;
	}
	// 8310BB80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8310BB84: 3B8BDB30  addi r28, r11, -0x24d0
	ctx.r[28].s64 = ctx.r[11].s64 + -9424;
	// 8310BB88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8310BB8C: 48136DE1  bl 0x8324296c
	ctx.lr = 0x8310BB90;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8310BB90: 48003F79  bl 0x8310fb08
	ctx.lr = 0x8310BB94;
	sub_8310FB08(ctx, base);
	// 8310BB94: 57CB029F  rlwinm. r11, r30, 0, 0xa, 0xf
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310BB98: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8310BB9C: 40820010  bne 0x8310bbac
	if !ctx.cr[0].eq {
	pc = 0x8310BBAC; continue 'dispatch;
	}
	// 8310BBA0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8310BBA4: 816B5734  lwz r11, 0x5734(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22324 as u32) ) } as u64;
	// 8310BBA8: 7D7EF378  or r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 | ctx.r[30].u64;
	// 8310BBAC: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8310BBB0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8310BBB4: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8310BBB8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8310BBBC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8310BBC0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8310BBC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8310BBC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310BBCC: 4BFFF55D  bl 0x8310b128
	ctx.lr = 0x8310BBD0;
	sub_8310B128(ctx, base);
	// 8310BBD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310BBD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8310BBD8: 48136D85  bl 0x8324295c
	ctx.lr = 0x8310BBDC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8310BBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310BBE0: 4800000C  b 0x8310bbec
	pc = 0x8310BBEC; continue 'dispatch;
	// 8310BBE4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310BBE8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310BBEC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8310BBF0: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8310BBF4: 4809C5B8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310BBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310BBF8 size=208
    let mut pc: u32 = 0x8310BBF8;
    'dispatch: loop {
        match pc {
            0x8310BBF8 => {
    //   block [0x8310BBF8..0x8310BCC8)
	// 8310BBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310BBFC: 4809C565  bl 0x831a8160
	ctx.lr = 0x8310BC00;
	sub_831A8130(ctx, base);
	// 8310BC00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310BC04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310BC08: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8310BC0C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8310BC10: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8310BC14: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 8310BC18: 897E0014  lbz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310BC1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310BC20: 40820010  bne 0x8310bc30
	if !ctx.cr[0].eq {
	pc = 0x8310BC30; continue 'dispatch;
	}
	// 8310BC24: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310BC28: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310BC2C: 4800008C  b 0x8310bcb8
	pc = 0x8310BCB8; continue 'dispatch;
	// 8310BC30: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8310BC34: 419A0080  beq cr6, 0x8310bcb4
	if ctx.cr[6].eq {
	pc = 0x8310BCB4; continue 'dispatch;
	}
	// 8310BC38: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8310BC3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310BC40: 4BFFC379  bl 0x83107fb8
	ctx.lr = 0x8310BC44;
	sub_83107FB8(ctx, base);
	// 8310BC44: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310BC48: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310BC4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310BC50: 57A804A5  rlwinm. r8, r29, 0, 0x12, 0x12
	ctx.r[8].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8310BC54: 7CCA4850  subf r6, r10, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 8310BC58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8310BC5C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8310BC60: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8310BC64: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8310BC68: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8310BC6C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8310BC70: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8310BC74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310BC78: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8310BC7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310BC80: 41820040  beq 0x8310bcc0
	if ctx.cr[0].eq {
	pc = 0x8310BCC0; continue 'dispatch;
	}
	// 8310BC84: 4BFFF33D  bl 0x8310afc0
	ctx.lr = 0x8310BC88;
	sub_8310AFC0(ctx, base);
	// 8310BC88: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310BC8C: 4180002C  blt 0x8310bcb8
	if ctx.cr[0].lt {
	pc = 0x8310BCB8; continue 'dispatch;
	}
	// 8310BC90: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8310BC94: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310BC98: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310BC9C: 7D6BD1D6  mullw r11, r11, r26
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[26].s32 as i64);
	// 8310BCA0: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310BCA4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8310BCA8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8310BCAC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8310BCB0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8310BCB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310BCB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8310BCBC: 4809C4F4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8310BCC0: 4BFFF171  bl 0x8310ae30
	ctx.lr = 0x8310BCC4;
	sub_8310AE30(ctx, base);
	// 8310BCC4: 4BFFFFC4  b 0x8310bc88
	pc = 0x8310BC88; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310BCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310BCC8 size=508
    let mut pc: u32 = 0x8310BCC8;
    'dispatch: loop {
        match pc {
            0x8310BCC8 => {
    //   block [0x8310BCC8..0x8310BEC4)
	// 8310BCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310BCCC: 4809C47D  bl 0x831a8148
	ctx.lr = 0x8310BCD0;
	sub_831A8130(ctx, base);
	// 8310BCD0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310BCD4: 7D555378  mr r21, r10
	ctx.r[21].u64 = ctx.r[10].u64;
	// 8310BCD8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8310BCDC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8310BCE0: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8310BCE4: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8310BCE8: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 8310BCEC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310BCF0: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 8310BCF4: 419A0008  beq cr6, 0x8310bcfc
	if ctx.cr[6].eq {
	pc = 0x8310BCFC; continue 'dispatch;
	}
	// 8310BCF8: 93D50000  stw r30, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8310BCFC: 82810134  lwz r20, 0x134(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 8310BD00: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 8310BD04: 419A0008  beq cr6, 0x8310bd0c
	if ctx.cr[6].eq {
	pc = 0x8310BD0C; continue 'dispatch;
	}
	// 8310BD08: 93D40000  stw r30, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8310BD0C: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310BD10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310BD14: 40820010  bne 0x8310bd24
	if !ctx.cr[0].eq {
	pc = 0x8310BD24; continue 'dispatch;
	}
	// 8310BD18: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310BD1C: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310BD20: 4800018C  b 0x8310beac
	pc = 0x8310BEAC; continue 'dispatch;
	// 8310BD24: 2F05FFFF  cmpwi cr6, r5, -1
	ctx.cr[6].compare_i32(ctx.r[5].s32, -1, &mut ctx.xer);
	// 8310BD28: 409A0010  bne cr6, 0x8310bd38
	if !ctx.cr[6].eq {
	pc = 0x8310BD38; continue 'dispatch;
	}
	// 8310BD2C: 3F007FFF  lis r24, 0x7fff
	ctx.r[24].s64 = 2147418112;
	// 8310BD30: 6318FFFF  ori r24, r24, 0xffff
	ctx.r[24].u64 = ctx.r[24].u64 | 65535;
	// 8310BD34: 48000014  b 0x8310bd48
	pc = 0x8310BD48; continue 'dispatch;
	// 8310BD38: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8310BD3C: 4198016C  blt cr6, 0x8310bea8
	if ctx.cr[6].lt {
	pc = 0x8310BEA8; continue 'dispatch;
	}
	// 8310BD40: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8310BD44: 419A0164  beq cr6, 0x8310bea8
	if ctx.cr[6].eq {
	pc = 0x8310BEA8; continue 'dispatch;
	}
	// 8310BD48: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8310BD4C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8310BD50: 4BFFC269  bl 0x83107fb8
	ctx.lr = 0x8310BD54;
	sub_83107FB8(ctx, base);
	// 8310BD54: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310BD58: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310BD5C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310BD60: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310BD64: 7F895050  subf r28, r9, r10
	ctx.r[28].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8310BD68: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310BD6C: 7D474050  subf r10, r7, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 8310BD70: 4182000C  beq 0x8310bd7c
	if ctx.cr[0].eq {
	pc = 0x8310BD7C; continue 'dispatch;
	}
	// 8310BD74: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8310BD78: 41990130  bgt cr6, 0x8310bea8
	if ctx.cr[6].gt {
	pc = 0x8310BEA8; continue 'dispatch;
	}
	// 8310BD7C: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 8310BD80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310BD84: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 8310BD88: 419A0024  beq cr6, 0x8310bdac
	if ctx.cr[6].eq {
	pc = 0x8310BDAC; continue 'dispatch;
	}
	// 8310BD8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310BD90: 419A001C  beq cr6, 0x8310bdac
	if ctx.cr[6].eq {
	pc = 0x8310BDAC; continue 'dispatch;
	}
	// 8310BD94: 554B083E  rotlwi r11, r10, 1
	ctx.r[11].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 8310BD98: 7CEA1BD6  divw r7, r10, r3
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[3].s32;
	// 8310BD9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8310BDA0: 0CC30000  twi 6, r3, 0
	// 8310BDA4: 7C6B5878  andc r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 & !ctx.r[11].u64;
	// 8310BDA8: 0CABFFFF  twi 5, r11, -1
	// 8310BDAC: 39410064  addi r10, r1, 0x64
	ctx.r[10].s64 = ctx.r[1].s64 + 100;
	// 8310BDB0: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8310BDB4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8310BDB8: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8310BDBC: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 8310BDC0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8310BDC4: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8310BDC8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8310BDCC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8310BDD0: 4BFFF1F1  bl 0x8310afc0
	ctx.lr = 0x8310BDD4;
	sub_8310AFC0(ctx, base);
	// 8310BDD4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310BDD8: 418000D4  blt 0x8310beac
	if ctx.cr[0].lt {
	pc = 0x8310BEAC; continue 'dispatch;
	}
	// 8310BDDC: 83610060  lwz r27, 0x60(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8310BDE0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8310BDE4: 409900C4  ble cr6, 0x8310bea8
	if !ctx.cr[6].gt {
	pc = 0x8310BEA8; continue 'dispatch;
	}
	// 8310BDE8: 5763103A  slwi r3, r27, 2
	ctx.r[3].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8310BDEC: 4BFD16C5  bl 0x830dd4b0
	ctx.lr = 0x8310BDF0;
	sub_830DD4B0(ctx, base);
	// 8310BDF0: 7C771B79  or. r23, r3, r3
	ctx.r[23].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8310BDF4: 40820010  bne 0x8310be04
	if !ctx.cr[0].eq {
	pc = 0x8310BE04; continue 'dispatch;
	}
	// 8310BDF8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310BDFC: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 8310BE00: 480000AC  b 0x8310beac
	pc = 0x8310BEAC; continue 'dispatch;
	// 8310BE04: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 8310BE08: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8310BE0C: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8310BE10: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8310BE14: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8310BE18: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8310BE1C: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8310BE20: 40990070  ble cr6, 0x8310be90
	if !ctx.cr[6].gt {
	pc = 0x8310BE90; continue 'dispatch;
	}
	// 8310BE24: 7EFEBB78  mr r30, r23
	ctx.r[30].u64 = ctx.r[23].u64;
	// 8310BE28: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 8310BE2C: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8310BE30: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8310BE34: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8310BE38: 39410064  addi r10, r1, 0x64
	ctx.r[10].s64 = ctx.r[1].s64 + 100;
	// 8310BE3C: 7CCBC050  subf r6, r11, r24
	ctx.r[6].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 8310BE40: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310BE44: 39210068  addi r9, r1, 0x68
	ctx.r[9].s64 = ctx.r[1].s64 + 104;
	// 8310BE48: 3901006C  addi r8, r1, 0x6c
	ctx.r[8].s64 = ctx.r[1].s64 + 108;
	// 8310BE4C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8310BE50: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8310BE54: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8310BE58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8310BE5C: 4BFFEA25  bl 0x8310a880
	ctx.lr = 0x8310BE60;
	sub_8310A880(ctx, base);
	// 8310BE60: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8310BE64: 41800050  blt 0x8310beb4
	if ctx.cr[0].lt {
	pc = 0x8310BEB4; continue 'dispatch;
	}
	// 8310BE68: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8310BE6C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8310BE70: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8310BE74: 409A0008  bne cr6, 0x8310be7c
	if !ctx.cr[6].eq {
	pc = 0x8310BE7C; continue 'dispatch;
	}
	// 8310BE78: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 8310BE7C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8310BE80: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8310BE84: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8310BE88: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8310BE8C: 4198FF9C  blt cr6, 0x8310be28
	if ctx.cr[6].lt {
	pc = 0x8310BE28; continue 'dispatch;
	}
	// 8310BE90: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 8310BE94: 419A0008  beq cr6, 0x8310be9c
	if ctx.cr[6].eq {
	pc = 0x8310BE9C; continue 'dispatch;
	}
	// 8310BE98: 93750000  stw r27, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8310BE9C: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 8310BEA0: 419A0008  beq cr6, 0x8310bea8
	if ctx.cr[6].eq {
	pc = 0x8310BEA8; continue 'dispatch;
	}
	// 8310BEA4: 92F40000  stw r23, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8310BEA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310BEAC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8310BEB0: 4809C2E8  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 8310BEB4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8310BEB8: 4BFD1621  bl 0x830dd4d8
	ctx.lr = 0x8310BEBC;
	sub_830DD4D8(ctx, base);
	// 8310BEBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310BEC0: 4BFFFFEC  b 0x8310beac
	pc = 0x8310BEAC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310BEC8 size=384
    let mut pc: u32 = 0x8310BEC8;
    'dispatch: loop {
        match pc {
            0x8310BEC8 => {
    //   block [0x8310BEC8..0x8310C048)
	// 8310BEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310BECC: 4809C291  bl 0x831a815c
	ctx.lr = 0x8310BED0;
	sub_831A8130(ctx, base);
	// 8310BED0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310BED4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 8310BED8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310BEDC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8310BEE0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8310BEE4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8310BEE8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8310BEEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8310BEF0: 409A0010  bne cr6, 0x8310bf00
	if !ctx.cr[6].eq {
	pc = 0x8310BF00; continue 'dispatch;
	}
	// 8310BEF4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310BEF8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310BEFC: 48000144  b 0x8310c040
	pc = 0x8310C040; continue 'dispatch;
	// 8310BF00: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8310BF04: 419A0128  beq cr6, 0x8310c02c
	if ctx.cr[6].eq {
	pc = 0x8310C02C; continue 'dispatch;
	}
	// 8310BF08: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310BF0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310BF10: 4182011C  beq 0x8310c02c
	if ctx.cr[0].eq {
	pc = 0x8310C02C; continue 'dispatch;
	}
	// 8310BF14: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310BF18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8310BF1C: C1BF000C  lfs f13, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310BF20: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8310BF24: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8310BF28: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 8310BF2C: C19F0004  lfs f12, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310BF30: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310BF34: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310BF38: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 8310BF3C: D9A10060  stfd f13, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[13].u64 ) };
	// 8310BF40: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8310BF44: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8310BF48: D8010060  stfd f0, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[0].u64 ) };
	// 8310BF4C: D9810050  stfd f12, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[12].u64 ) };
	// 8310BF50: 81010064  lwz r8, 0x64(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8310BF54: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310BF58: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8310BF5C: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 8310BF60: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 8310BF64: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 8310BF68: 419AFF8C  beq cr6, 0x8310bef4
	if ctx.cr[6].eq {
	pc = 0x8310BEF4; continue 'dispatch;
	}
	// 8310BF6C: 57AB029F  rlwinm. r11, r29, 0, 0xa, 0xf
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310BF70: 40820010  bne 0x8310bf80
	if !ctx.cr[0].eq {
	pc = 0x8310BF80; continue 'dispatch;
	}
	// 8310BF74: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8310BF78: 816B5734  lwz r11, 0x5734(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22324 as u32) ) } as u64;
	// 8310BF7C: 7D7DEB78  or r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 | ctx.r[29].u64;
	// 8310BF80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8310BF84: 3B2BDB30  addi r25, r11, -0x24d0
	ctx.r[25].s64 = ctx.r[11].s64 + -9424;
	// 8310BF88: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8310BF8C: 481369E1  bl 0x8324296c
	ctx.lr = 0x8310BF90;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8310BF90: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8310BF94: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8310BF98: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8310BF9C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8310BFA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310BFA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310BFA8: 4BFFFC51  bl 0x8310bbf8
	ctx.lr = 0x8310BFAC;
	sub_8310BBF8(ctx, base);
	// 8310BFAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8310BFB0: 48003B59  bl 0x8310fb08
	ctx.lr = 0x8310BFB4;
	sub_8310FB08(ctx, base);
	// 8310BFB4: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8310BFB8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8310BFBC: 481369A1  bl 0x8324295c
	ctx.lr = 0x8310BFC0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8310BFC0: E961006E  lwa r11, 0x6c(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as i32) as i64;
	// 8310BFC4: E9410062  lwa r10, 0x60(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as i32) as i64;
	// 8310BFC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8310BFCC: E9210066  lwa r9, 0x64(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as i32) as i64;
	// 8310BFD0: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8310BFD4: E901006A  lwa r8, 0x68(r1)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as i32) as i64;
	// 8310BFD8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8310BFDC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8310BFE0: C9A10050  lfd f13, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8310BFE4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8310BFE8: C8010060  lfd f0, 0x60(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8310BFEC: F9210060  std r9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 8310BFF0: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8310BFF4: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8310BFF8: C9610060  lfd f11, 0x60(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8310BFFC: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8310C000: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8310C004: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8310C008: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8310C00C: D19F0004  stfs f12, 4(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310C010: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8310C014: D1BF0000  stfs f13, 0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310C018: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8310C01C: D17F0008  stfs f11, 8(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8310C020: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8310C024: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8310C028: 48000018  b 0x8310c040
	pc = 0x8310C040; continue 'dispatch;
	// 8310C02C: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C030: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C034: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310C038: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8310C03C: D1BF000C  stfs f13, 0xc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8310C040: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8310C044: 4809C168  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8310C048 size=28
    let mut pc: u32 = 0x8310C048;
    'dispatch: loop {
        match pc {
            0x8310C048 => {
    //   block [0x8310C048..0x8310C064)
	// 8310C048: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8310C04C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8310C050: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8310C054: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8310C058: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8310C05C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8310C060: 4BFFFA78  b 0x8310bad8
	sub_8310BAD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310C068 size=308
    let mut pc: u32 = 0x8310C068;
    'dispatch: loop {
        match pc {
            0x8310C068 => {
    //   block [0x8310C068..0x8310C19C)
	// 8310C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C06C: 4809C0E9  bl 0x831a8154
	ctx.lr = 0x8310C070;
	sub_831A8130(ctx, base);
	// 8310C070: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C074: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 8310C078: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8310C07C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8310C080: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8310C084: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8310C088: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8310C08C: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 8310C090: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 8310C094: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8310C098: 409A0010  bne cr6, 0x8310c0a8
	if !ctx.cr[6].eq {
	pc = 0x8310C0A8; continue 'dispatch;
	}
	// 8310C09C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310C0A0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310C0A4: 480000F0  b 0x8310c194
	pc = 0x8310C194; continue 'dispatch;
	// 8310C0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C0AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8310C0B0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310C0B4: 419AFFE8  beq cr6, 0x8310c09c
	if ctx.cr[6].eq {
	pc = 0x8310C09C; continue 'dispatch;
	}
	// 8310C0B8: 2F19FFFF  cmpwi cr6, r25, -1
	ctx.cr[6].compare_i32(ctx.r[25].s32, -1, &mut ctx.xer);
	// 8310C0BC: 4198FFE0  blt cr6, 0x8310c09c
	if ctx.cr[6].lt {
	pc = 0x8310C09C; continue 'dispatch;
	}
	// 8310C0C0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8310C0C4: 419AFFD8  beq cr6, 0x8310c09c
	if ctx.cr[6].eq {
	pc = 0x8310C09C; continue 'dispatch;
	}
	// 8310C0C8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8310C0CC: 409A000C  bne cr6, 0x8310c0d8
	if !ctx.cr[6].eq {
	pc = 0x8310C0D8; continue 'dispatch;
	}
	// 8310C0D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C0D4: 480000C0  b 0x8310c194
	pc = 0x8310C194; continue 'dispatch;
	// 8310C0D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8310C0DC: 3BABDB30  addi r29, r11, -0x24d0
	ctx.r[29].s64 = ctx.r[11].s64 + -9424;
	// 8310C0E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8310C0E4: 48136889  bl 0x8324296c
	ctx.lr = 0x8310C0E8;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8310C0E8: 48003A21  bl 0x8310fb08
	ctx.lr = 0x8310C0EC;
	sub_8310FB08(ctx, base);
	// 8310C0EC: 907C0010  stw r3, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8310C0F0: 57CB029F  rlwinm. r11, r30, 0, 0xa, 0xf
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C0F4: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C0F8: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310C0FC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8310C100: D8010068  stfd f0, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.f[0].u64 ) };
	// 8310C104: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 8310C108: C19F0008  lfs f12, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310C10C: 8121006C  lwz r9, 0x6c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8310C110: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C114: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 8310C118: D9A10070  stfd f13, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.f[13].u64 ) };
	// 8310C11C: 81010074  lwz r8, 0x74(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8310C120: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8310C124: D8010070  stfd f0, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.f[0].u64 ) };
	// 8310C128: D9810060  stfd f12, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[12].u64 ) };
	// 8310C12C: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8310C130: 80E10074  lwz r7, 0x74(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8310C134: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 8310C138: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8310C13C: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8310C140: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8310C144: 40820010  bne 0x8310c154
	if !ctx.cr[0].eq {
	pc = 0x8310C154; continue 'dispatch;
	}
	// 8310C148: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8310C14C: 816B5734  lwz r11, 0x5734(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22324 as u32) ) } as u64;
	// 8310C150: 7D7EF378  or r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 | ctx.r[30].u64;
	// 8310C154: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8310C158: 7EE80034  cntlzw r8, r23
	ctx.r[8].u64 = if ctx.r[23].u32 == 0 { 32 } else { ctx.r[23].u32.leading_zeros() as u64 };
	// 8310C15C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8310C160: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8310C164: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8310C168: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 8310C16C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8310C170: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8310C174: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8310C178: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8310C17C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8310C180: 4BFFFB49  bl 0x8310bcc8
	ctx.lr = 0x8310C184;
	sub_8310BCC8(ctx, base);
	// 8310C184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310C188: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8310C18C: 481367D1  bl 0x8324295c
	ctx.lr = 0x8310C190;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8310C190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C194: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8310C198: 4809C00C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C1A0 size=88
    let mut pc: u32 = 0x8310C1A0;
    'dispatch: loop {
        match pc {
            0x8310C1A0 => {
    //   block [0x8310C1A0..0x8310C1F8)
	// 8310C1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310C1A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310C1AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310C1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C1B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8310C1B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310C1BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310C1C0: 4BAD2DA1  bl 0x82bdef60
	ctx.lr = 0x8310C1C4;
	sub_82BDEF60(ctx, base);
	// 8310C1C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310C1C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310C1CC: 419A0010  beq cr6, 0x8310c1dc
	if ctx.cr[6].eq {
	pc = 0x8310C1DC; continue 'dispatch;
	}
	// 8310C1D0: 4BAD2E09  bl 0x82bdefd8
	ctx.lr = 0x8310C1D4;
	sub_82BDEFD8(ctx, base);
	// 8310C1D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C1D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8310C1DC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8310C1E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310C1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310C1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310C1EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310C1F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310C1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310C1F8 size=16
    let mut pc: u32 = 0x8310C1F8;
    'dispatch: loop {
        match pc {
            0x8310C1F8 => {
    //   block [0x8310C1F8..0x8310C208)
	// 8310C1F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C1FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310C200: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310C204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C208 size=92
    let mut pc: u32 = 0x8310C208;
    'dispatch: loop {
        match pc {
            0x8310C208 => {
    //   block [0x8310C208..0x8310C264)
	// 8310C208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310C210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310C214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310C218: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C21C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310C220: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C224: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310C228: 419A001C  beq cr6, 0x8310c244
	if ctx.cr[6].eq {
	pc = 0x8310C244; continue 'dispatch;
	}
	// 8310C22C: 83E30010  lwz r31, 0x10(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C230: 4BFC20A9  bl 0x830ce2d8
	ctx.lr = 0x8310C234;
	sub_830CE2D8(ctx, base);
	// 8310C234: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C23C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8310C240: 409AFFEC  bne cr6, 0x8310c22c
	if !ctx.cr[6].eq {
	pc = 0x8310C22C; continue 'dispatch;
	}
	// 8310C244: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C248: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8310C24C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310C250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310C254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310C258: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310C25C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310C260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310C268 size=280
    let mut pc: u32 = 0x8310C268;
    'dispatch: loop {
        match pc {
            0x8310C268 => {
    //   block [0x8310C268..0x8310C380)
	// 8310C268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310C270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310C274: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 8310C278: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8310C27C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310C284: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8310C288: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8310C28C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8310C290: 4BFC1FB1  bl 0x830ce240
	ctx.lr = 0x8310C294;
	sub_830CE240(ctx, base);
	// 8310C294: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C298: 41820018  beq 0x8310c2b0
	if ctx.cr[0].eq {
	pc = 0x8310C2B0; continue 'dispatch;
	}
	// 8310C29C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8310C2A0: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 8310C2A4: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8310C2A8: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8310C2AC: 48000008  b 0x8310c2b4
	pc = 0x8310C2B4; continue 'dispatch;
	// 8310C2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C2B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310C2B8: 409A0010  bne cr6, 0x8310c2c8
	if !ctx.cr[6].eq {
	pc = 0x8310C2C8; continue 'dispatch;
	}
	// 8310C2BC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310C2C0: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 8310C2C4: 480000A0  b 0x8310c364
	pc = 0x8310C364; continue 'dispatch;
	// 8310C2C8: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C2CC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8310C2D0: 40980008  bge cr6, 0x8310c2d8
	if !ctx.cr[6].lt {
	pc = 0x8310C2D8; continue 'dispatch;
	}
	// 8310C2D4: D3FF0000  stfs f31, 0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310C2D8: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C2DC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8310C2E0: 40990008  ble cr6, 0x8310c2e8
	if !ctx.cr[6].gt {
	pc = 0x8310C2E8; continue 'dispatch;
	}
	// 8310C2E4: D3FF0008  stfs f31, 8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8310C2E8: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C2EC: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8310C2F0: 40980008  bge cr6, 0x8310c2f8
	if !ctx.cr[6].lt {
	pc = 0x8310C2F8; continue 'dispatch;
	}
	// 8310C2F4: D3DF0004  stfs f30, 4(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310C2F8: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C2FC: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8310C300: 40990008  ble cr6, 0x8310c308
	if !ctx.cr[6].gt {
	pc = 0x8310C308; continue 'dispatch;
	}
	// 8310C304: D3DF000C  stfs f30, 0xc(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8310C308: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310C30C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8310C310: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310C314: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8310C318: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8310C31C: D3EB0004  stfs f31, 4(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310C320: D3CB0008  stfs f30, 8(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8310C324: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C328: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8310C32C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C330: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8310C334: 419A001C  beq cr6, 0x8310c350
	if ctx.cr[6].eq {
	pc = 0x8310C350; continue 'dispatch;
	}
	// 8310C338: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310C33C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8310C340: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8310C344: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C348: 916A0014  stw r11, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8310C34C: 48000014  b 0x8310c360
	pc = 0x8310C360; continue 'dispatch;
	// 8310C350: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8310C354: 916B0014  stw r11, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8310C358: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C35C: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8310C360: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C364: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310C368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310C36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310C370: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8310C374: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310C378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310C37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8310C380 size=84
    let mut pc: u32 = 0x8310C380;
    'dispatch: loop {
        match pc {
            0x8310C380 => {
    //   block [0x8310C380..0x8310C3D4)
	// 8310C380: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 8310C384: C1A10018  lfs f13, 0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310C388: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 8310C38C: C1210020  lfs f9, 0x20(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8310C390: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 8310C394: C1610028  lfs f11, 0x28(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310C398: C141002C  lfs f10, 0x2c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(44 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8310C39C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8310C3A0: C001001C  lfs f0, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C3A4: C1810024  lfs f12, 0x24(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310C3A8: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8310C3AC: ED6B6828  fsubs f11, f11, f13
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310C3B0: EC0A0028  fsubs f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 8310C3B4: ED496828  fsubs f10, f9, f13
	ctx.f[10].f64 = (((ctx.f[9].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310C3B8: C1AB9450  lfs f13, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310C3BC: ED8C02F2  fmuls f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 8310C3C0: EC0062B8  fmsubs f0, f0, f10, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 8310C3C4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8310C3C8: 4099000C  ble cr6, 0x8310c3d4
	if !ctx.cr[6].gt {
		sub_8310C3D4(ctx, base);
		return;
	}
	// 8310C3CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8310C3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C3D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8310C3D4 size=20
    let mut pc: u32 = 0x8310C3D4;
    'dispatch: loop {
        match pc {
            0x8310C3D4 => {
    //   block [0x8310C3D4..0x8310C3E8)
	// 8310C3D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8310C3D8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8310C3DC: C1AB9530  lfs f13, -0x6ad0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27344 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310C3E0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8310C3E4: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310C3E8 size=8
    let mut pc: u32 = 0x8310C3E8;
    'dispatch: loop {
        match pc {
            0x8310C3E8 => {
    //   block [0x8310C3E8..0x8310C3F0)
	// 8310C3E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310C3F0 size=144
    let mut pc: u32 = 0x8310C3F0;
    'dispatch: loop {
        match pc {
            0x8310C3F0 => {
    //   block [0x8310C3F0..0x8310C480)
	// 8310C3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310C3F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C3FC: F8810078  std r4, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u64 ) };
	// 8310C400: F8A10080  std r5, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[5].u64 ) };
	// 8310C404: F8C10088  std r6, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[6].u64 ) };
	// 8310C408: 4BFFFF79  bl 0x8310c380
	ctx.lr = 0x8310C40C;
	sub_8310C380(ctx, base);
	// 8310C40C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310C410: 4182000C  beq 0x8310c41c
	if ctx.cr[0].eq {
	pc = 0x8310C41C; continue 'dispatch;
	}
	// 8310C414: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C418: 48000058  b 0x8310c470
	pc = 0x8310C470; continue 'dispatch;
	// 8310C41C: C1A10078  lfs f13, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310C420: C1810080  lfs f12, 0x80(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310C424: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 8310C428: 419A000C  beq cr6, 0x8310c434
	if ctx.cr[6].eq {
	pc = 0x8310C434; continue 'dispatch;
	}
	// 8310C42C: C0010088  lfs f0, 0x88(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C430: 48000010  b 0x8310c440
	pc = 0x8310C440; continue 'dispatch;
	// 8310C434: C1A1007C  lfs f13, 0x7c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310C438: C001008C  lfs f0, 0x8c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310C43C: C1810084  lfs f12, 0x84(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310C440: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8310C444: 4199000C  bgt cr6, 0x8310c450
	if ctx.cr[6].gt {
	pc = 0x8310C450; continue 'dispatch;
	}
	// 8310C448: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8310C44C: 40990014  ble cr6, 0x8310c460
	if !ctx.cr[6].gt {
	pc = 0x8310C460; continue 'dispatch;
	}
	// 8310C450: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8310C454: 41980014  blt cr6, 0x8310c468
	if ctx.cr[6].lt {
	pc = 0x8310C468; continue 'dispatch;
	}
	// 8310C458: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8310C45C: 4198000C  blt cr6, 0x8310c468
	if ctx.cr[6].lt {
	pc = 0x8310C468; continue 'dispatch;
	}
	// 8310C460: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C464: 48000008  b 0x8310c46c
	pc = 0x8310C46C; continue 'dispatch;
	// 8310C468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C46C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310C470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310C474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310C478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310C47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C480 size=432
    let mut pc: u32 = 0x8310C480;
    'dispatch: loop {
        match pc {
            0x8310C480 => {
    //   block [0x8310C480..0x8310C630)
	// 8310C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C484: 4809BCCD  bl 0x831a8150
	ctx.lr = 0x8310C488;
	sub_831A8130(ctx, base);
	// 8310C488: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C48C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310C490: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8310C494: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8310C498: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8310C49C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8310C4A0: 4BFFFEE1  bl 0x8310c380
	ctx.lr = 0x8310C4A4;
	sub_8310C380(ctx, base);
	// 8310C4A4: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8310C4A8: 4182017C  beq 0x8310c624
	if ctx.cr[0].eq {
	pc = 0x8310C624; continue 'dispatch;
	}
	// 8310C4AC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8310C4B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8310C4B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8310C4B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C4BC: 4BFFFEC5  bl 0x8310c380
	ctx.lr = 0x8310C4C0;
	sub_8310C380(ctx, base);
	// 8310C4C0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8310C4C4: 41820160  beq 0x8310c624
	if ctx.cr[0].eq {
	pc = 0x8310C624; continue 'dispatch;
	}
	// 8310C4C8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8310C4CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310C4D0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8310C4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C4D8: 4BFFFEA9  bl 0x8310c380
	ctx.lr = 0x8310C4DC;
	sub_8310C380(ctx, base);
	// 8310C4DC: 7C761B79  or. r22, r3, r3
	ctx.r[22].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 8310C4E0: 41820144  beq 0x8310c624
	if ctx.cr[0].eq {
	pc = 0x8310C624; continue 'dispatch;
	}
	// 8310C4E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8310C4E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310C4EC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8310C4F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C4F4: 4BFFFE8D  bl 0x8310c380
	ctx.lr = 0x8310C4F8;
	sub_8310C380(ctx, base);
	// 8310C4F8: 7C781B79  or. r24, r3, r3
	ctx.r[24].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8310C4FC: 41820128  beq 0x8310c624
	if ctx.cr[0].eq {
	pc = 0x8310C624; continue 'dispatch;
	}
	// 8310C500: 82FF0018  lwz r23, 0x18(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310C504: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8310C508: 419A001C  beq cr6, 0x8310c524
	if ctx.cr[6].eq {
	pc = 0x8310C524; continue 'dispatch;
	}
	// 8310C50C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8310C510: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8310C514: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8310C518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C51C: 4BFFFE65  bl 0x8310c380
	ctx.lr = 0x8310C520;
	sub_8310C380(ctx, base);
	// 8310C520: 48000008  b 0x8310c528
	pc = 0x8310C528; continue 'dispatch;
	// 8310C524: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8310C528: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C52C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8310C530: 41990008  bgt cr6, 0x8310c538
	if ctx.cr[6].gt {
	pc = 0x8310C538; continue 'dispatch;
	}
	// 8310C534: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8310C538: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8310C53C: 419A001C  beq cr6, 0x8310c558
	if ctx.cr[6].eq {
	pc = 0x8310C558; continue 'dispatch;
	}
	// 8310C540: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8310C544: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8310C548: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8310C54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C550: 4BFFFE31  bl 0x8310c380
	ctx.lr = 0x8310C554;
	sub_8310C380(ctx, base);
	// 8310C554: 48000008  b 0x8310c55c
	pc = 0x8310C55C; continue 'dispatch;
	// 8310C558: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8310C55C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C560: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C564: 41990008  bgt cr6, 0x8310c56c
	if ctx.cr[6].gt {
	pc = 0x8310C56C; continue 'dispatch;
	}
	// 8310C568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C56C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310C570: 578A063E  clrlwi r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8310C574: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8310C578: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8310C57C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8310C580: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8310C584: 7D6B5279  xor. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C588: 41820090  beq 0x8310c618
	if ctx.cr[0].eq {
	pc = 0x8310C618; continue 'dispatch;
	}
	// 8310C58C: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8310C590: 419A001C  beq cr6, 0x8310c5ac
	if ctx.cr[6].eq {
	pc = 0x8310C5AC; continue 'dispatch;
	}
	// 8310C594: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8310C598: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310C59C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310C5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C5A4: 4BFFFDDD  bl 0x8310c380
	ctx.lr = 0x8310C5A8;
	sub_8310C380(ctx, base);
	// 8310C5A8: 48000008  b 0x8310c5b0
	pc = 0x8310C5B0; continue 'dispatch;
	// 8310C5AC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8310C5B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C5B4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8310C5B8: 41990008  bgt cr6, 0x8310c5c0
	if ctx.cr[6].gt {
	pc = 0x8310C5C0; continue 'dispatch;
	}
	// 8310C5BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310C5C0: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8310C5C4: 419A001C  beq cr6, 0x8310c5e0
	if ctx.cr[6].eq {
	pc = 0x8310C5E0; continue 'dispatch;
	}
	// 8310C5C8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8310C5CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310C5D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8310C5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C5D8: 4BFFFDA9  bl 0x8310c380
	ctx.lr = 0x8310C5DC;
	sub_8310C380(ctx, base);
	// 8310C5DC: 48000008  b 0x8310c5e4
	pc = 0x8310C5E4; continue 'dispatch;
	// 8310C5E0: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8310C5E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C5E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C5EC: 41990008  bgt cr6, 0x8310c5f4
	if ctx.cr[6].gt {
	pc = 0x8310C5F4; continue 'dispatch;
	}
	// 8310C5F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C5F4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310C5F8: 57CA063E  clrlwi r10, r30, 0x18
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8310C5FC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8310C600: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8310C604: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8310C608: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8310C60C: 7D6B5279  xor. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C610: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C614: 40820008  bne 0x8310c61c
	if !ctx.cr[0].eq {
	pc = 0x8310C61C; continue 'dispatch;
	}
	// 8310C618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C61C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310C620: 48000008  b 0x8310c628
	pc = 0x8310C628; continue 'dispatch;
	// 8310C624: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C628: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8310C62C: 4809BB74  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C630 size=172
    let mut pc: u32 = 0x8310C630;
    'dispatch: loop {
        match pc {
            0x8310C630 => {
    //   block [0x8310C630..0x8310C6DC)
	// 8310C630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C634: 4809BB31  bl 0x831a8164
	ctx.lr = 0x8310C638;
	sub_831A8130(ctx, base);
	// 8310C638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C63C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310C640: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8310C644: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8310C648: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8310C64C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8310C650: 4BFFFE31  bl 0x8310c480
	ctx.lr = 0x8310C654;
	sub_8310C480(ctx, base);
	// 8310C654: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C658: 40820078  bne 0x8310c6d0
	if !ctx.cr[0].eq {
	pc = 0x8310C6D0; continue 'dispatch;
	}
	// 8310C65C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8310C660: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310C664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310C668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C66C: 4BFFFD85  bl 0x8310c3f0
	ctx.lr = 0x8310C670;
	sub_8310C3F0(ctx, base);
	// 8310C670: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C674: 4082005C  bne 0x8310c6d0
	if !ctx.cr[0].eq {
	pc = 0x8310C6D0; continue 'dispatch;
	}
	// 8310C678: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8310C67C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310C680: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310C684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C688: 4BFFFD69  bl 0x8310c3f0
	ctx.lr = 0x8310C68C;
	sub_8310C3F0(ctx, base);
	// 8310C68C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C690: 40820040  bne 0x8310c6d0
	if !ctx.cr[0].eq {
	pc = 0x8310C6D0; continue 'dispatch;
	}
	// 8310C694: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8310C698: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8310C69C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310C6A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C6A4: 4BFFFD4D  bl 0x8310c3f0
	ctx.lr = 0x8310C6A8;
	sub_8310C3F0(ctx, base);
	// 8310C6A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C6AC: 40820024  bne 0x8310c6d0
	if !ctx.cr[0].eq {
	pc = 0x8310C6D0; continue 'dispatch;
	}
	// 8310C6B0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8310C6B4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8310C6B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310C6BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C6C0: 4BFFFD31  bl 0x8310c3f0
	ctx.lr = 0x8310C6C4;
	sub_8310C3F0(ctx, base);
	// 8310C6C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C6C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C6CC: 41820008  beq 0x8310c6d4
	if ctx.cr[0].eq {
	pc = 0x8310C6D4; continue 'dispatch;
	}
	// 8310C6D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8310C6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310C6D8: 4809BADC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C6E0 size=328
    let mut pc: u32 = 0x8310C6E0;
    'dispatch: loop {
        match pc {
            0x8310C6E0 => {
    //   block [0x8310C6E0..0x8310C828)
	// 8310C6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C6E4: 4809BA7D  bl 0x831a8160
	ctx.lr = 0x8310C6E8;
	sub_831A8130(ctx, base);
	// 8310C6E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C6EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310C6F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310C6F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8310C6F8: 837F0010  lwz r27, 0x10(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C6FC: 835E0018  lwz r26, 0x18(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310C700: 839F0014  lwz r28, 0x14(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310C704: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8310C708: E8BB0004  ld r5, 4(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	// 8310C70C: 419A0010  beq cr6, 0x8310c71c
	if ctx.cr[6].eq {
	pc = 0x8310C71C; continue 'dispatch;
	}
	// 8310C710: E8DF0004  ld r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C714: E89C0004  ld r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	// 8310C718: 4800000C  b 0x8310c724
	pc = 0x8310C724; continue 'dispatch;
	// 8310C71C: E8DC0004  ld r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	// 8310C720: E89F0004  ld r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C724: 4BFFFC5D  bl 0x8310c380
	ctx.lr = 0x8310C728;
	sub_8310C380(ctx, base);
	// 8310C728: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C72C: E8BD0004  ld r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	// 8310C730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310C734: 41980088  blt cr6, 0x8310c7bc
	if ctx.cr[6].lt {
	pc = 0x8310C7BC; continue 'dispatch;
	}
	// 8310C738: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8310C73C: 419A0010  beq cr6, 0x8310c74c
	if ctx.cr[6].eq {
	pc = 0x8310C74C; continue 'dispatch;
	}
	// 8310C740: E8DF0004  ld r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C744: E89C0004  ld r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	// 8310C748: 4800000C  b 0x8310c754
	pc = 0x8310C754; continue 'dispatch;
	// 8310C74C: E8DC0004  ld r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	// 8310C750: E89F0004  ld r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C754: 4BFFFC2D  bl 0x8310c380
	ctx.lr = 0x8310C758;
	sub_8310C380(ctx, base);
	// 8310C758: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C75C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C760: 41990008  bgt cr6, 0x8310c768
	if ctx.cr[6].gt {
	pc = 0x8310C768; continue 'dispatch;
	}
	// 8310C764: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C768: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C76C: 41820048  beq 0x8310c7b4
	if ctx.cr[0].eq {
	pc = 0x8310C7B4; continue 'dispatch;
	}
	// 8310C770: E8BF0004  ld r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C774: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8310C778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310C77C: 419A0010  beq cr6, 0x8310c78c
	if ctx.cr[6].eq {
	pc = 0x8310C78C; continue 'dispatch;
	}
	// 8310C780: E8DD0004  ld r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	// 8310C784: E89B0004  ld r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	// 8310C788: 4800000C  b 0x8310c794
	pc = 0x8310C794; continue 'dispatch;
	// 8310C78C: E8DB0004  ld r6, 4(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	// 8310C790: E89D0004  ld r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	// 8310C794: 4BFFFBED  bl 0x8310c380
	ctx.lr = 0x8310C798;
	sub_8310C380(ctx, base);
	// 8310C798: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C79C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C7A0: 41990008  bgt cr6, 0x8310c7a8
	if ctx.cr[6].gt {
	pc = 0x8310C7A8; continue 'dispatch;
	}
	// 8310C7A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C7A8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C7AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C7B0: 4082006C  bne 0x8310c81c
	if !ctx.cr[0].eq {
	pc = 0x8310C81C; continue 'dispatch;
	}
	// 8310C7B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C7B8: 48000064  b 0x8310c81c
	pc = 0x8310C81C; continue 'dispatch;
	// 8310C7BC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8310C7C0: 419A0010  beq cr6, 0x8310c7d0
	if ctx.cr[6].eq {
	pc = 0x8310C7D0; continue 'dispatch;
	}
	// 8310C7C4: E8DF0004  ld r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C7C8: E89B0004  ld r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	// 8310C7CC: 4800000C  b 0x8310c7d8
	pc = 0x8310C7D8; continue 'dispatch;
	// 8310C7D0: E8DB0004  ld r6, 4(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	// 8310C7D4: E89F0004  ld r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C7D8: 4BFFFBA9  bl 0x8310c380
	ctx.lr = 0x8310C7DC;
	sub_8310C380(ctx, base);
	// 8310C7DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C7E0: 41980038  blt cr6, 0x8310c818
	if ctx.cr[6].lt {
	pc = 0x8310C818; continue 'dispatch;
	}
	// 8310C7E4: E8BF0004  ld r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C7E8: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8310C7EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310C7F0: 419A0010  beq cr6, 0x8310c800
	if ctx.cr[6].eq {
	pc = 0x8310C800; continue 'dispatch;
	}
	// 8310C7F4: E8DD0004  ld r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	// 8310C7F8: E89C0004  ld r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	// 8310C7FC: 4800000C  b 0x8310c808
	pc = 0x8310C808; continue 'dispatch;
	// 8310C800: E8DC0004  ld r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	// 8310C804: E89D0004  ld r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	// 8310C808: 4BFFFB79  bl 0x8310c380
	ctx.lr = 0x8310C80C;
	sub_8310C380(ctx, base);
	// 8310C80C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310C810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C814: 40980008  bge cr6, 0x8310c81c
	if !ctx.cr[6].lt {
	pc = 0x8310C81C; continue 'dispatch;
	}
	// 8310C818: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C81C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310C820: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8310C824: 4809B98C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C828 size=132
    let mut pc: u32 = 0x8310C828;
    'dispatch: loop {
        match pc {
            0x8310C828 => {
    //   block [0x8310C828..0x8310C8AC)
	// 8310C828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C82C: 4809B939  bl 0x831a8164
	ctx.lr = 0x8310C830;
	sub_831A8130(ctx, base);
	// 8310C830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C834: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8310C838: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8310C83C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8310C840: 837C0010  lwz r27, 0x10(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C844: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8310C848: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310C84C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8310C850: 419A003C  beq cr6, 0x8310c88c
	if ctx.cr[6].eq {
	pc = 0x8310C88C; continue 'dispatch;
	}
	// 8310C854: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8310C858: 419A0034  beq cr6, 0x8310c88c
	if ctx.cr[6].eq {
	pc = 0x8310C88C; continue 'dispatch;
	}
	// 8310C85C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8310C860: 419A002C  beq cr6, 0x8310c88c
	if ctx.cr[6].eq {
	pc = 0x8310C88C; continue 'dispatch;
	}
	// 8310C864: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8310C868: 419A0024  beq cr6, 0x8310c88c
	if ctx.cr[6].eq {
	pc = 0x8310C88C; continue 'dispatch;
	}
	// 8310C86C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8310C870: E8FF0004  ld r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	// 8310C874: E8CB0004  ld r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	// 8310C878: E8BD0004  ld r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	// 8310C87C: E89E0004  ld r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	// 8310C880: 4BFFFDB1  bl 0x8310c630
	ctx.lr = 0x8310C884;
	sub_8310C630(ctx, base);
	// 8310C884: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C888: 4082001C  bne 0x8310c8a4
	if !ctx.cr[0].eq {
	pc = 0x8310C8A4; continue 'dispatch;
	}
	// 8310C88C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8310C890: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8310C894: 409AFFB4  bne cr6, 0x8310c848
	if !ctx.cr[6].eq {
	pc = 0x8310C848; continue 'dispatch;
	}
	// 8310C898: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8310C89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310C8A0: 4809B914  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8310C8A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C8A8: 4BFFFFF4  b 0x8310c89c
	pc = 0x8310C89C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C8B0 size=104
    let mut pc: u32 = 0x8310C8B0;
    'dispatch: loop {
        match pc {
            0x8310C8B0 => {
    //   block [0x8310C8B0..0x8310C918)
	// 8310C8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C8B4: 4809B8B9  bl 0x831a816c
	ctx.lr = 0x8310C8B8;
	sub_831A8130(ctx, base);
	// 8310C8B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C8BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310C8C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8310C8C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8310C8C8: 4BFFFE19  bl 0x8310c6e0
	ctx.lr = 0x8310C8CC;
	sub_8310C6E0(ctx, base);
	// 8310C8CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C8D0: 41820038  beq 0x8310c908
	if ctx.cr[0].eq {
	pc = 0x8310C908; continue 'dispatch;
	}
	// 8310C8D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8310C8D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8310C8DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C8E0: 4BFFFE01  bl 0x8310c6e0
	ctx.lr = 0x8310C8E4;
	sub_8310C6E0(ctx, base);
	// 8310C8E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C8E8: 41820020  beq 0x8310c908
	if ctx.cr[0].eq {
	pc = 0x8310C908; continue 'dispatch;
	}
	// 8310C8EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310C8F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310C8F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310C8F8: 4BFFFF31  bl 0x8310c828
	ctx.lr = 0x8310C8FC;
	sub_8310C828(ctx, base);
	// 8310C8FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310C900: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310C904: 40820008  bne 0x8310c90c
	if !ctx.cr[0].eq {
	pc = 0x8310C90C; continue 'dispatch;
	}
	// 8310C908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C90C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310C910: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310C914: 4809B8A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C918 size=96
    let mut pc: u32 = 0x8310C918;
    'dispatch: loop {
        match pc {
            0x8310C918 => {
    //   block [0x8310C918..0x8310C978)
	// 8310C918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310C920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310C924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310C92C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310C930: 396BAEAC  addi r11, r11, -0x5154
	ctx.r[11].s64 = ctx.r[11].s64 + -20820;
	// 8310C934: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310C938: 4BFFDCC1  bl 0x8310a5f8
	ctx.lr = 0x8310C93C;
	sub_8310A5F8(ctx, base);
	// 8310C93C: 48009355  bl 0x83115c90
	ctx.lr = 0x8310C940;
	sub_83115C90(ctx, base);
	// 8310C940: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310C944: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310C948: 419A0014  beq cr6, 0x8310c95c
	if ctx.cr[6].eq {
	pc = 0x8310C95C; continue 'dispatch;
	}
	// 8310C94C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310C950: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310C954: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310C958: 4E800421  bctrl
	ctx.lr = 0x8310C95C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310C95C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310C960: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8310C964: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310C968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310C96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310C970: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310C974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310C978 size=12
    let mut pc: u32 = 0x8310C978;
    'dispatch: loop {
        match pc {
            0x8310C978 => {
    //   block [0x8310C978..0x8310C984)
	// 8310C978: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310C97C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310C980: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310C984 size=12
    let mut pc: u32 = 0x8310C984;
    'dispatch: loop {
        match pc {
            0x8310C984 => {
    //   block [0x8310C984..0x8310C990)
	// 8310C984: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8310C988: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310C98C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310C990 size=8
    let mut pc: u32 = 0x8310C990;
    'dispatch: loop {
        match pc {
            0x8310C990 => {
    //   block [0x8310C990..0x8310C998)
	// 8310C990: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8310C994: 4800D89C  b 0x8311a230
	sub_8311A230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310C998 size=4
    let mut pc: u32 = 0x8310C998;
    'dispatch: loop {
        match pc {
            0x8310C998 => {
    //   block [0x8310C998..0x8310C99C)
	// 8310C998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310C9A0 size=12
    let mut pc: u32 = 0x8310C9A0;
    'dispatch: loop {
        match pc {
            0x8310C9A0 => {
    //   block [0x8310C9A0..0x8310C9AC)
	// 8310C9A0: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 8310C9A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310C9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310C9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310C9B0 size=112
    let mut pc: u32 = 0x8310C9B0;
    'dispatch: loop {
        match pc {
            0x8310C9B0 => {
    //   block [0x8310C9B0..0x8310CA20)
	// 8310C9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310C9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310C9B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310C9BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310C9C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310C9C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310C9C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8310C9CC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8310C9D0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310C9D4: 4BFD0B05  bl 0x830dd4d8
	ctx.lr = 0x8310C9D8;
	sub_830DD4D8(ctx, base);
	// 8310C9D8: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8310C9DC: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8310C9E0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310C9E4: 4BFD0AF5  bl 0x830dd4d8
	ctx.lr = 0x8310C9E8;
	sub_830DD4D8(ctx, base);
	// 8310C9E8: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8310C9EC: 93FE0010  stw r31, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 8310C9F0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310C9F4: 4BFD0AE5  bl 0x830dd4d8
	ctx.lr = 0x8310C9F8;
	sub_830DD4D8(ctx, base);
	// 8310C9F8: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 8310C9FC: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 8310CA00: 93FE001C  stw r31, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 8310CA04: 93FE0024  stw r31, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8310CA08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310CA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310CA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310CA14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310CA18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310CA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310CA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310CA20 size=92
    let mut pc: u32 = 0x8310CA20;
    'dispatch: loop {
        match pc {
            0x8310CA20 => {
    //   block [0x8310CA20..0x8310CA7C)
	// 8310CA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310CA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310CA28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310CA2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310CA30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310CA34: 38600190  li r3, 0x190
	ctx.r[3].s64 = 400;
	// 8310CA38: 4BFC1809  bl 0x830ce240
	ctx.lr = 0x8310CA3C;
	sub_830CE240(ctx, base);
	// 8310CA3C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8310CA40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310CA44: 40820010  bne 0x8310ca54
	if !ctx.cr[0].eq {
	pc = 0x8310CA54; continue 'dispatch;
	}
	// 8310CA48: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310CA4C: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 8310CA50: 48000018  b 0x8310ca68
	pc = 0x8310CA68; continue 'dispatch;
	// 8310CA54: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310CA58: 39400032  li r10, 0x32
	ctx.r[10].s64 = 50;
	// 8310CA5C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8310CA60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310CA64: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8310CA68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310CA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310CA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310CA74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310CA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310CA80 size=188
    let mut pc: u32 = 0x8310CA80;
    'dispatch: loop {
        match pc {
            0x8310CA80 => {
    //   block [0x8310CA80..0x8310CB3C)
	// 8310CA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310CA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310CA88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310CA8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310CA90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310CA94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310CA98: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310CA9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310CAA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310CAA4: 419A0058  beq cr6, 0x8310cafc
	if ctx.cr[6].eq {
	pc = 0x8310CAFC; continue 'dispatch;
	}
	// 8310CAA8: 55632036  slwi r3, r11, 4
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8310CAAC: 4BFD0A05  bl 0x830dd4b0
	ctx.lr = 0x8310CAB0;
	sub_830DD4B0(ctx, base);
	// 8310CAB0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8310CAB4: 40820010  bne 0x8310cac4
	if !ctx.cr[0].eq {
	pc = 0x8310CAC4; continue 'dispatch;
	}
	// 8310CAB8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310CABC: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 8310CAC0: 48000064  b 0x8310cb24
	pc = 0x8310CB24; continue 'dispatch;
	// 8310CAC4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310CAC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310CACC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310CAD0: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8310CAD4: 4809BA3D  bl 0x831a8510
	ctx.lr = 0x8310CAD8;
	sub_831A8510(ctx, base);
	// 8310CAD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310CADC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310CAE0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8310CAE4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8310CAE8: 4BFC17F1  bl 0x830ce2d8
	ctx.lr = 0x8310CAEC;
	sub_830CE2D8(ctx, base);
	// 8310CAEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310CAF0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8310CAF4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8310CAF8: 48000028  b 0x8310cb20
	pc = 0x8310CB20; continue 'dispatch;
	// 8310CAFC: 55642036  slwi r4, r11, 4
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8310CB00: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310CB04: 4BFD09ED  bl 0x830dd4f0
	ctx.lr = 0x8310CB08;
	sub_830DD4F0(ctx, base);
	// 8310CB08: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310CB0C: 4182FFAC  beq 0x8310cab8
	if ctx.cr[0].eq {
	pc = 0x8310CAB8; continue 'dispatch;
	}
	// 8310CB10: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310CB14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8310CB18: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8310CB1C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8310CB20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310CB24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310CB28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310CB2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310CB30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310CB34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310CB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310CB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310CB40 size=448
    let mut pc: u32 = 0x8310CB40;
    'dispatch: loop {
        match pc {
            0x8310CB40 => {
    //   block [0x8310CB40..0x8310CD00)
	// 8310CB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310CB44: 4809B629  bl 0x831a816c
	ctx.lr = 0x8310CB48;
	sub_831A8130(ctx, base);
	// 8310CB48: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 8310CB4C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8310CB50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310CB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310CB58: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8310CB5C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8310CB60: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310CB64: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310CB68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8310CB6C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8310CB70: 40990010  ble cr6, 0x8310cb80
	if !ctx.cr[6].gt {
	pc = 0x8310CB80; continue 'dispatch;
	}
	// 8310CB74: 4BFFFF0D  bl 0x8310ca80
	ctx.lr = 0x8310CB78;
	sub_8310CA80(ctx, base);
	// 8310CB78: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310CB7C: 41800174  blt 0x8310ccf0
	if ctx.cr[0].lt {
	pc = 0x8310CCF0; continue 'dispatch;
	}
	// 8310CB80: C01F0014  lfs f0, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CB84: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8310CB88: 40980008  bge cr6, 0x8310cb90
	if !ctx.cr[6].lt {
	pc = 0x8310CB90; continue 'dispatch;
	}
	// 8310CB8C: D3FF0014  stfs f31, 0x14(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8310CB90: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CB94: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8310CB98: 40990008  ble cr6, 0x8310cba0
	if !ctx.cr[6].gt {
	pc = 0x8310CBA0; continue 'dispatch;
	}
	// 8310CB9C: D3FF001C  stfs f31, 0x1c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8310CBA0: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CBA4: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8310CBA8: 40980008  bge cr6, 0x8310cbb0
	if !ctx.cr[6].lt {
	pc = 0x8310CBB0; continue 'dispatch;
	}
	// 8310CBAC: D3DF0018  stfs f30, 0x18(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8310CBB0: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CBB4: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8310CBB8: 40990008  ble cr6, 0x8310cbc0
	if !ctx.cr[6].gt {
	pc = 0x8310CBC0; continue 'dispatch;
	}
	// 8310CBBC: D3DF0020  stfs f30, 0x20(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8310CBC0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310CBC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310CBC8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8310CBCC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8310CBD0: D3EB0000  stfs f31, 0(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310CBD4: D3CB0004  stfs f30, 4(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310CBD8: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310CBDC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310CBE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8310CBE4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310CBE8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8310CBEC: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 8310CBF0: 419A00FC  beq cr6, 0x8310ccec
	if ctx.cr[6].eq {
	pc = 0x8310CCEC; continue 'dispatch;
	}
	// 8310CBF4: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8310CBF8: 409900F4  ble cr6, 0x8310ccec
	if !ctx.cr[6].gt {
	pc = 0x8310CCEC; continue 'dispatch;
	}
	// 8310CBFC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310CC00: 57DD1838  slwi r29, r30, 3
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8310CC04: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 8310CC08: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8310CC0C: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310CC10: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CC14: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310CC18: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310CC1C: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310CC20: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310CC24: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310CC28: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310CC2C: 419800C0  blt cr6, 0x8310ccec
	if ctx.cr[6].lt {
	pc = 0x8310CCEC; continue 'dispatch;
	}
	// 8310CC30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8310CC34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8310CC38: 4800B2D1  bl 0x83117f08
	ctx.lr = 0x8310CC3C;
	sub_83117F08(ctx, base);
	// 8310CC3C: 397EFFFE  addi r11, r30, -2
	ctx.r[11].s64 = ctx.r[30].s64 + -2;
	// 8310CC40: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310CC44: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8310CC48: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8310CC4C: 7D7D5214  add r11, r29, r10
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 8310CC50: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8310CC54: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8310CC58: 392BFFF8  addi r9, r11, -8
	ctx.r[9].s64 = ctx.r[11].s64 + -8;
	// 8310CC5C: C00BFFFC  lfs f0, -4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CC60: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310CC64: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310CC68: C18A0000  lfs f12, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310CC6C: C00BFFF8  lfs f0, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CC70: EC006028  fsubs f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8310CC74: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8310CC78: FC006850  fneg f0, f13
	ctx.f[0].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 8310CC7C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8310CC80: 4800B289  bl 0x83117f08
	ctx.lr = 0x8310CC84;
	sub_83117F08(ctx, base);
	// 8310CC84: C1810058  lfs f12, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310CC88: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CC8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8310CC90: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8310CC94: C161005C  lfs f11, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310CC98: C1ABCC2C  lfs f13, -0x33d4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13268 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310CC9C: C1810054  lfs f12, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310CCA0: EC0C02FA  fmadds f0, f12, f11, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64);
	// 8310CCA4: FD800210  fabs f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 8310CCA8: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 8310CCAC: 41980040  blt cr6, 0x8310ccec
	if ctx.cr[6].lt {
	pc = 0x8310CCEC; continue 'dispatch;
	}
	// 8310CCB0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310CCB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310CCB8: 409A0014  bne cr6, 0x8310cccc
	if !ctx.cr[6].eq {
	pc = 0x8310CCCC; continue 'dispatch;
	}
	// 8310CCBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310CCC0: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8310CCC4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8310CCC8: 48000024  b 0x8310ccec
	pc = 0x8310CCEC; continue 'dispatch;
	// 8310CCCC: C1BF002C  lfs f13, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310CCD0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8310CCD4: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310CCD8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310CCDC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8310CCE0: 4098000C  bge cr6, 0x8310ccec
	if !ctx.cr[6].lt {
	pc = 0x8310CCEC; continue 'dispatch;
	}
	// 8310CCE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310CCE8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8310CCEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310CCF0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8310CCF4: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8310CCF8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8310CCFC: 4809B4C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310CD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310CD00 size=756
    let mut pc: u32 = 0x8310CD00;
    'dispatch: loop {
        match pc {
            0x8310CD00 => {
    //   block [0x8310CD00..0x8310CFF4)
	// 8310CD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310CD04: 4809B42D  bl 0x831a8130
	ctx.lr = 0x8310CD08;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310CFF8 size=112
    let mut pc: u32 = 0x8310CFF8;
    'dispatch: loop {
        match pc {
            0x8310CFF8 => {
    //   block [0x8310CFF8..0x8310D068)
	// 8310CFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310CFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310D004: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D008: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8310D00C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8310D010: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8310D014: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8310D018: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8310D01C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8310D020: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8310D024: 4082FFE8  bne 0x8310d00c
	if !ctx.cr[0].eq {
	pc = 0x8310D00C; continue 'dispatch;
	}
	// 8310D028: 7D5F5378  mr r31, r10
	ctx.r[31].u64 = ctx.r[10].u64;
	// 8310D02C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8310D030: 409A0020  bne cr6, 0x8310d050
	if !ctx.cr[6].eq {
	pc = 0x8310D050; continue 'dispatch;
	}
	// 8310D034: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310D038: 419A0018  beq cr6, 0x8310d050
	if ctx.cr[6].eq {
	pc = 0x8310D050; continue 'dispatch;
	}
	// 8310D03C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D040: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310D044: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D048: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D04C: 4E800421  bctrl
	ctx.lr = 0x8310D050;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310D054: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310D058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310D064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310D068 size=200
    let mut pc: u32 = 0x8310D068;
    'dispatch: loop {
        match pc {
            0x8310D068 => {
    //   block [0x8310D068..0x8310D130)
	// 8310D068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D06C: 4809B101  bl 0x831a816c
	ctx.lr = 0x8310D070;
	sub_831A8130(ctx, base);
	// 8310D070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310D078: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8310D07C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8310D080: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310D084: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310D088: 419A0010  beq cr6, 0x8310d098
	if ctx.cr[6].eq {
	pc = 0x8310D098; continue 'dispatch;
	}
	// 8310D08C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310D090: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310D094: 48000094  b 0x8310d128
	pc = 0x8310D128; continue 'dispatch;
	// 8310D098: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D09C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310D0A0: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 8310D0A4: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8310D0A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D0AC: 4E800421  bctrl
	ctx.lr = 0x8310D0B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310D0B4: 917F00BC  stw r11, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 8310D0B8: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8310D0BC: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 8310D0C0: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8310D0C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310D0C8: 409A0024  bne cr6, 0x8310d0ec
	if !ctx.cr[6].eq {
	pc = 0x8310D0EC; continue 'dispatch;
	}
	// 8310D0CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D0D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8310D0D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310D0D8: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8310D0DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D0E0: 4E800421  bctrl
	ctx.lr = 0x8310D0E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D0E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310D0E8: 41800040  blt 0x8310d128
	if ctx.cr[0].lt {
	pc = 0x8310D128; continue 'dispatch;
	}
	// 8310D0EC: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310D0F0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8310D0F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8310D0F8: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8310D0FC: 409A0008  bne cr6, 0x8310d104
	if !ctx.cr[6].eq {
	pc = 0x8310D104; continue 'dispatch;
	}
	// 8310D100: 4BFFB259  bl 0x83108358
	ctx.lr = 0x8310D104;
	sub_83108358(ctx, base);
	// 8310D104: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D108: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310D10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310D110: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310D114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D118: 4E800421  bctrl
	ctx.lr = 0x8310D11C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D11C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310D120: 41800008  blt 0x8310d128
	if ctx.cr[0].lt {
	pc = 0x8310D128; continue 'dispatch;
	}
	// 8310D124: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D128: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310D12C: 4809B090  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310D130 size=104
    let mut pc: u32 = 0x8310D130;
    'dispatch: loop {
        match pc {
            0x8310D130 => {
    //   block [0x8310D130..0x8310D198)
	// 8310D130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310D13C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310D140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310D148: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310D14C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8310D150: 419A0010  beq cr6, 0x8310d160
	if ctx.cr[6].eq {
	pc = 0x8310D160; continue 'dispatch;
	}
	// 8310D154: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310D158: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310D15C: 48000024  b 0x8310d180
	pc = 0x8310D180; continue 'dispatch;
	// 8310D160: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8310D164: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310D168: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 8310D16C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310D170: 409A0008  bne cr6, 0x8310d178
	if !ctx.cr[6].eq {
	pc = 0x8310D178; continue 'dispatch;
	}
	// 8310D174: 4BFFC22D  bl 0x831093a0
	ctx.lr = 0x8310D178;
	sub_831093A0(ctx, base);
	// 8310D178: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D17C: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 8310D180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310D184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D18C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310D190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310D194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310D198 size=200
    let mut pc: u32 = 0x8310D198;
    'dispatch: loop {
        match pc {
            0x8310D198 => {
    //   block [0x8310D198..0x8310D260)
	// 8310D198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310D1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310D1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D1AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310D1B0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8310D1B4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8310D1B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310D1BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D1C0: 93FE008C  stw r31, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 8310D1C4: 915E0090  stw r10, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8310D1C8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310D1CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D1D0: 4E800421  bctrl
	ctx.lr = 0x8310D1D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D1D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D1D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310D1DC: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310D1E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D1E4: 4E800421  bctrl
	ctx.lr = 0x8310D1E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D1E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8310D1EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310D1F0: 917E00AC  stw r11, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 8310D1F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D1F8: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8310D1FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D200: 4E800421  bctrl
	ctx.lr = 0x8310D204;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D204: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D208: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310D20C: 93FE00B0  stw r31, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 8310D210: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8310D214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D218: 4E800421  bctrl
	ctx.lr = 0x8310D21C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D21C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D220: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310D224: 93FE0088  stw r31, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 8310D228: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8310D22C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D230: 4E800421  bctrl
	ctx.lr = 0x8310D234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D234: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310D23C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8310D240: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D244: 4E800421  bctrl
	ctx.lr = 0x8310D248;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D248: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310D24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D254: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310D258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310D25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310D260 size=188
    let mut pc: u32 = 0x8310D260;
    'dispatch: loop {
        match pc {
            0x8310D260 => {
    //   block [0x8310D260..0x8310D31C)
	// 8310D260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D26C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D270: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8310D274: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D278: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8310D27C: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D280: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8310D284: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D288: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8310D28C: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D290: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8310D294: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D298: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8310D29C: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2A0: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8310D2A4: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2A8: D0030024  stfs f0, 0x24(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8310D2AC: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2B0: D0030028  stfs f0, 0x28(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8310D2B4: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2B8: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8310D2BC: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2C0: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8310D2C4: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2C8: D0030034  stfs f0, 0x34(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8310D2CC: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2D0: D0030038  stfs f0, 0x38(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8310D2D4: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2D8: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8310D2DC: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2E0: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8310D2E4: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D2E8: D0030044  stfs f0, 0x44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8310D2EC: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310D2F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310D2F4: 40990014  ble cr6, 0x8310d308
	if !ctx.cr[6].gt {
	pc = 0x8310D308; continue 'dispatch;
	}
	// 8310D2F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D2FC: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 8310D300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D304: 4E800421  bctrl
	ctx.lr = 0x8310D308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D308: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D30C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310D310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8310D320 size=140
    let mut pc: u32 = 0x8310D320;
    'dispatch: loop {
        match pc {
            0x8310D320 => {
    //   block [0x8310D320..0x8310D3AC)
	// 8310D320: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8310D324: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D328: C00B0008  lfs f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D32C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310D330: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D334: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310D338: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D33C: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8310D340: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D344: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8310D348: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D34C: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8310D350: C00B001C  lfs f0, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D354: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8310D358: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D35C: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8310D360: C00B0024  lfs f0, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D364: D004001C  stfs f0, 0x1c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8310D368: C00B0028  lfs f0, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D36C: D0040020  stfs f0, 0x20(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8310D370: C00B002C  lfs f0, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D374: D0040024  stfs f0, 0x24(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8310D378: C00B0030  lfs f0, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D37C: D0040028  stfs f0, 0x28(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8310D380: C00B0034  lfs f0, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D384: D004002C  stfs f0, 0x2c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8310D388: C00B0038  lfs f0, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D38C: D0040030  stfs f0, 0x30(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8310D390: C00B003C  lfs f0, 0x3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D394: D0040034  stfs f0, 0x34(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8310D398: C00B0040  lfs f0, 0x40(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D39C: D0040038  stfs f0, 0x38(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8310D3A0: C00B0044  lfs f0, 0x44(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3A4: D004003C  stfs f0, 0x3c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8310D3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310D3B0 size=188
    let mut pc: u32 = 0x8310D3B0;
    'dispatch: loop {
        match pc {
            0x8310D3B0 => {
    //   block [0x8310D3B0..0x8310D46C)
	// 8310D3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D3BC: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3C0: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 8310D3C4: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3C8: D003004C  stfs f0, 0x4c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 8310D3CC: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3D0: D0030050  stfs f0, 0x50(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310D3D4: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3D8: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310D3DC: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3E0: D0030058  stfs f0, 0x58(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8310D3E4: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3E8: D003005C  stfs f0, 0x5c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8310D3EC: C0040018  lfs f0, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3F0: D0030060  stfs f0, 0x60(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8310D3F4: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D3F8: D0030064  stfs f0, 0x64(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8310D3FC: C0040020  lfs f0, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D400: D0030068  stfs f0, 0x68(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8310D404: C0040024  lfs f0, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D408: D003006C  stfs f0, 0x6c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8310D40C: C0040028  lfs f0, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D410: D0030070  stfs f0, 0x70(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8310D414: C004002C  lfs f0, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D418: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8310D41C: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D420: D0030078  stfs f0, 0x78(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8310D424: C0040034  lfs f0, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D428: D003007C  stfs f0, 0x7c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8310D42C: C0040038  lfs f0, 0x38(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D430: D0030080  stfs f0, 0x80(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8310D434: C004003C  lfs f0, 0x3c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D438: D0030084  stfs f0, 0x84(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8310D43C: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310D440: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310D444: 40990014  ble cr6, 0x8310d458
	if !ctx.cr[6].gt {
	pc = 0x8310D458; continue 'dispatch;
	}
	// 8310D448: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D44C: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 8310D450: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D454: 4E800421  bctrl
	ctx.lr = 0x8310D458;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D458: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D45C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310D460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8310D470 size=140
    let mut pc: u32 = 0x8310D470;
    'dispatch: loop {
        match pc {
            0x8310D470 => {
    //   block [0x8310D470..0x8310D4FC)
	// 8310D470: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8310D474: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D478: C00B0048  lfs f0, 0x48(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D47C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310D480: C00B004C  lfs f0, 0x4c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D484: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310D488: C00B0050  lfs f0, 0x50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D48C: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8310D490: C00B0054  lfs f0, 0x54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D494: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8310D498: C00B0058  lfs f0, 0x58(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D49C: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8310D4A0: C00B005C  lfs f0, 0x5c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4A4: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8310D4A8: C00B0060  lfs f0, 0x60(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4AC: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8310D4B0: C00B0064  lfs f0, 0x64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4B4: D004001C  stfs f0, 0x1c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8310D4B8: C00B0068  lfs f0, 0x68(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4BC: D0040020  stfs f0, 0x20(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8310D4C0: C00B006C  lfs f0, 0x6c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4C4: D0040024  stfs f0, 0x24(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8310D4C8: C00B0070  lfs f0, 0x70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4CC: D0040028  stfs f0, 0x28(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8310D4D0: C00B0074  lfs f0, 0x74(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4D4: D004002C  stfs f0, 0x2c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8310D4D8: C00B0078  lfs f0, 0x78(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4DC: D0040030  stfs f0, 0x30(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8310D4E0: C00B007C  lfs f0, 0x7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4E4: D0040034  stfs f0, 0x34(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8310D4E8: C00B0080  lfs f0, 0x80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4EC: D0040038  stfs f0, 0x38(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8310D4F0: C00B0084  lfs f0, 0x84(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D4F4: D004003C  stfs f0, 0x3c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8310D4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310D500 size=416
    let mut pc: u32 = 0x8310D500;
    'dispatch: loop {
        match pc {
            0x8310D500 => {
    //   block [0x8310D500..0x8310D6A0)
	// 8310D500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D504: 4809AC59  bl 0x831a815c
	ctx.lr = 0x8310D508;
	sub_831A8130(ctx, base);
	// 8310D508: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D50C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310D6A0 size=332
    let mut pc: u32 = 0x8310D6A0;
    'dispatch: loop {
        match pc {
            0x8310D6A0 => {
    //   block [0x8310D6A0..0x8310D7EC)
	// 8310D6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D6A4: 4809AAA9  bl 0x831a814c
	ctx.lr = 0x8310D6A8;
	sub_831A8130(ctx, base);
	// 8310D6A8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310D7F0 size=188
    let mut pc: u32 = 0x8310D7F0;
    'dispatch: loop {
        match pc {
            0x8310D7F0 => {
    //   block [0x8310D7F0..0x8310D8AC)
	// 8310D7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D7FC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8310D800: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8310D804: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D808: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8310D80C: 5567863E  rlwinm r7, r11, 0x10, 0x18, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8310D810: 5565C63E  rlwinm r5, r11, 0x18, 0x18, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310D814: 5568463E  srwi r8, r11, 0x18
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(24);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8310D818: C00A94AC  lfs f0, -0x6b54(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27476 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D81C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8310D820: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 8310D824: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8310D828: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310D82C: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 8310D830: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8310D834: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8310D838: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8310D83C: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8310D840: F8E10050  std r7, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u64 ) };
	// 8310D844: C9410060  lfd f10, 0x60(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8310D848: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8310D84C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8310D850: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 8310D854: 81290040  lwz r9, 0x40(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(64 as u32) ) } as u64;
	// 8310D858: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8310D85C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8310D860: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8310D864: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310D868: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8310D86C: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 8310D870: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8310D874: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310D878: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8310D87C: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310D880: D1410068  stfs f10, 0x68(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8310D884: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310D888: D1610064  stfs f11, 0x64(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8310D88C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310D890: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8310D894: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8310D898: 4E800421  bctrl
	ctx.lr = 0x8310D89C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310D8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310D8B0 size=92
    let mut pc: u32 = 0x8310D8B0;
    'dispatch: loop {
        match pc {
            0x8310D8B0 => {
    //   block [0x8310D8B0..0x8310D90C)
	// 8310D8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D8B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D8BC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8310D8C0: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8310D8C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310D8C8: 409A001C  bne cr6, 0x8310d8e4
	if !ctx.cr[6].eq {
	pc = 0x8310D8E4; continue 'dispatch;
	}
	// 8310D8CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D8D0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310D8D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D8D8: 4E800421  bctrl
	ctx.lr = 0x8310D8DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D8DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D8E0: 4800001C  b 0x8310d8fc
	pc = 0x8310D8FC; continue 'dispatch;
	// 8310D8E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D8E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8310D8EC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8310D8F0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310D8F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D8F8: 4E800421  bctrl
	ctx.lr = 0x8310D8FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D8FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310D900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310D910 size=92
    let mut pc: u32 = 0x8310D910;
    'dispatch: loop {
        match pc {
            0x8310D910 => {
    //   block [0x8310D910..0x8310D96C)
	// 8310D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D91C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8310D920: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8310D924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310D928: 409A001C  bne cr6, 0x8310d944
	if !ctx.cr[6].eq {
	pc = 0x8310D944; continue 'dispatch;
	}
	// 8310D92C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D930: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310D934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D938: 4E800421  bctrl
	ctx.lr = 0x8310D93C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D93C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310D940: 4800001C  b 0x8310d95c
	pc = 0x8310D95C; continue 'dispatch;
	// 8310D944: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D948: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8310D94C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8310D950: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310D954: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D958: 4E800421  bctrl
	ctx.lr = 0x8310D95C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D95C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310D960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310D964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310D968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310D970 size=188
    let mut pc: u32 = 0x8310D970;
    'dispatch: loop {
        match pc {
            0x8310D970 => {
    //   block [0x8310D970..0x8310DA2C)
	// 8310D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310D974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310D978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310D97C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310D980: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310D984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310D988: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8310D98C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310D990: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D994: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310D998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310D99C: 4E800421  bctrl
	ctx.lr = 0x8310D9A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310D9A0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310D9A4: 41800070  blt 0x8310da14
	if ctx.cr[0].lt {
	pc = 0x8310DA14; continue 'dispatch;
	}
	// 8310D9A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310D9AC: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310D9B0: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310D9B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8310D9B8: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310D9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8310D9C0: C17E000C  lfs f11, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310D9C4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8310D9C8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310D9CC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8310D9D0: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8310D9D4: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310D9D8: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8310D9DC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8310D9E0: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8310D9E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310D9E8: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8310D9EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310D9F0: D1610064  stfs f11, 0x64(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8310D9F4: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8310D9F8: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8310D9FC: D1810070  stfs f12, 0x70(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8310DA00: D1610074  stfs f11, 0x74(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8310DA04: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8310DA08: D161007C  stfs f11, 0x7c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8310DA0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310DA10: 4E800421  bctrl
	ctx.lr = 0x8310DA14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310DA14: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8310DA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310DA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310DA20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310DA24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310DA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310DA30 size=80
    let mut pc: u32 = 0x8310DA30;
    'dispatch: loop {
        match pc {
            0x8310DA30 => {
    //   block [0x8310DA30..0x8310DA80)
	// 8310DA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310DA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310DA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310DA3C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8310DA40: 409A0010  bne cr6, 0x8310da50
	if !ctx.cr[6].eq {
	pc = 0x8310DA50; continue 'dispatch;
	}
	// 8310DA44: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310DA48: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310DA4C: 48000024  b 0x8310da70
	pc = 0x8310DA70; continue 'dispatch;
	// 8310DA50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310DA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310DA58: 90830098  stw r4, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[4].u32 ) };
	// 8310DA5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310DA60: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310DA64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310DA68: 4E800421  bctrl
	ctx.lr = 0x8310DA6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310DA6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310DA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310DA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310DA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310DA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310DA80 size=40
    let mut pc: u32 = 0x8310DA80;
    'dispatch: loop {
        match pc {
            0x8310DA80 => {
    //   block [0x8310DA80..0x8310DAA8)
	// 8310DA80: 814300BC  lwz r10, 0xbc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8310DA84: 812300C0  lwz r9, 0xc0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310DA88: 816300C4  lwz r11, 0xc4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 8310DA8C: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 8310DA90: 7D254A14  add r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 8310DA94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8310DA98: 914300BC  stw r10, 0xbc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 8310DA9C: 912300C0  stw r9, 0xc0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), ctx.r[9].u32 ) };
	// 8310DAA0: 916300C4  stw r11, 0xc4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 8310DAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310DAA8 size=12
    let mut pc: u32 = 0x8310DAA8;
    'dispatch: loop {
        match pc {
            0x8310DAA8 => {
    //   block [0x8310DAA8..0x8310DAB4)
	// 8310DAA8: 908300A8  stw r4, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[4].u32 ) };
	// 8310DAAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310DAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310DAB8 size=20
    let mut pc: u32 = 0x8310DAB8;
    'dispatch: loop {
        match pc {
            0x8310DAB8 => {
    //   block [0x8310DAB8..0x8310DACC)
	// 8310DAB8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8310DABC: 409A0010  bne cr6, 0x8310dacc
	if !ctx.cr[6].eq {
		sub_8310DACC(ctx, base);
		return;
	}
	// 8310DAC0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310DAC4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310DAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310DACC size=16
    let mut pc: u32 = 0x8310DACC;
    'dispatch: loop {
        match pc {
            0x8310DACC => {
    //   block [0x8310DACC..0x8310DADC)
	// 8310DACC: 816300A8  lwz r11, 0xa8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 8310DAD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310DAD4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310DAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310DAE0 size=76
    let mut pc: u32 = 0x8310DAE0;
    'dispatch: loop {
        match pc {
            0x8310DAE0 => {
    //   block [0x8310DAE0..0x8310DB2C)
	// 8310DAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310DAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310DAE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310DAEC: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8310DAF0: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8310DAF4: 419A0024  beq cr6, 0x8310db18
	if ctx.cr[6].eq {
	pc = 0x8310DB18; continue 'dispatch;
	}
	// 8310DAF8: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310DAFC: 908300A4  stw r4, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 8310DB00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310DB04: 40990014  ble cr6, 0x8310db18
	if !ctx.cr[6].gt {
	pc = 0x8310DB18; continue 'dispatch;
	}
	// 8310DB08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310DB0C: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 8310DB10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310DB14: 4E800421  bctrl
	ctx.lr = 0x8310DB18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310DB18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310DB1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310DB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310DB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310DB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310DB30 size=76
    let mut pc: u32 = 0x8310DB30;
    'dispatch: loop {
        match pc {
            0x8310DB30 => {
    //   block [0x8310DB30..0x8310DB7C)
	// 8310DB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310DB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310DB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310DB3C: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 8310DB40: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8310DB44: 419A0024  beq cr6, 0x8310db68
	if ctx.cr[6].eq {
	pc = 0x8310DB68; continue 'dispatch;
	}
	// 8310DB48: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310DB4C: 908300AC  stw r4, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[4].u32 ) };
	// 8310DB50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310DB54: 40990014  ble cr6, 0x8310db68
	if !ctx.cr[6].gt {
	pc = 0x8310DB68; continue 'dispatch;
	}
	// 8310DB58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310DB5C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8310DB60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310DB64: 4E800421  bctrl
	ctx.lr = 0x8310DB68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310DB68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310DB6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310DB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310DB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310DB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310DB80 size=76
    let mut pc: u32 = 0x8310DB80;
    'dispatch: loop {
        match pc {
            0x8310DB80 => {
    //   block [0x8310DB80..0x8310DBCC)
	// 8310DB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310DB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310DB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310DB8C: 816300B0  lwz r11, 0xb0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) } as u64;
	// 8310DB90: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8310DB94: 419A0024  beq cr6, 0x8310dbb8
	if ctx.cr[6].eq {
	pc = 0x8310DBB8; continue 'dispatch;
	}
	// 8310DB98: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310DB9C: 908300B0  stw r4, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 8310DBA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310DBA4: 40990014  ble cr6, 0x8310dbb8
	if !ctx.cr[6].gt {
	pc = 0x8310DBB8; continue 'dispatch;
	}
	// 8310DBA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310DBAC: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8310DBB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310DBB4: 4E800421  bctrl
	ctx.lr = 0x8310DBB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310DBB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310DBBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310DBC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310DBC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310DBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310DBD0 size=12
    let mut pc: u32 = 0x8310DBD0;
    'dispatch: loop {
        match pc {
            0x8310DBD0 => {
    //   block [0x8310DBD0..0x8310DBDC)
	// 8310DBD0: 816300B8  lwz r11, 0xb8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 8310DBD4: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8310DBD8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DBDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310DBDC size=16
    let mut pc: u32 = 0x8310DBDC;
    'dispatch: loop {
        match pc {
            0x8310DBDC => {
    //   block [0x8310DBDC..0x8310DBEC)
	// 8310DBDC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8310DBE0: 908300B8  stw r4, 0xb8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), ctx.r[4].u32 ) };
	// 8310DBE4: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8310DBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8310DBF0 size=744
    let mut pc: u32 = 0x8310DBF0;
    'dispatch: loop {
        match pc {
            0x8310DBF0 => {
    //   block [0x8310DBF0..0x8310DED8)
	// 8310DBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310DBF4: 4809A551  bl 0x831a8144
	ctx.lr = 0x8310DBF8;
	sub_831A8130(ctx, base);
	// 8310DBF8: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 8310DBFC: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 8310DC00: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8310DC04: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8310DC08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310DC0C: F961FF00  std r11, -0x100(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-256 as u32), ctx.r[11].u64 ) };
	// 8310DC10: C801FF00  lfd f0, -0x100(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-256 as u32) ) };
	// 8310DC14: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8310DC18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8310DC1C: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8310DC20: C12B08A8  lfs f9, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8310DC24: C14A08A4  lfs f10, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8310DC28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8310DC2C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8310DC30: ECE96824  fdivs f7, f9, f13
	ctx.f[7].f64 = ((ctx.f[9].f64 / ctx.f[13].f64) as f32) as f64;
	// 8310DC34: 40990298  ble cr6, 0x8310decc
	if !ctx.cr[6].gt {
	pc = 0x8310DECC; continue 'dispatch;
	}
	// 8310DC38: 3F808201  lis r28, -0x7dff
	ctx.r[28].s64 = -2113863680;
	// 8310DC3C: 3F608201  lis r27, -0x7dff
	ctx.r[27].s64 = -2113863680;
	// 8310DC40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8310DC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8310DC48: 3967000C  addi r11, r7, 0xc
	ctx.r[11].s64 = ctx.r[7].s64 + 12;
	// 8310DC4C: C01C94AC  lfs f0, -0x6b54(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-27476 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310DC50: 23E7FFFC  subfic r31, r7, -4
	ctx.xer.ca = ctx.r[7].u32 <= -4 as u32;
	ctx.r[31].s64 = (-4 as i64) - ctx.r[7].s64;
	// 8310DC54: C11BCC2C  lfs f8, -0x33d4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-13268 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8310DC58: 3BAA9CD0  addi r29, r10, -0x6330
	ctx.r[29].s64 = ctx.r[10].s64 + -25392;
	// 8310DC5C: EDAA01F2  fmuls f13, f10, f7
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[7].f64) as f32) as f64);
	// 8310DC60: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310DC64: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 8310DC68: 40990020  ble cr6, 0x8310dc88
	if !ctx.cr[6].gt {
	pc = 0x8310DC88; continue 'dispatch;
	}
	// 8310DC6C: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 8310DC70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310DC74: 7D3F5A14  add r9, r31, r11
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 8310DC78: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 8310DC7C: 40980218  bge cr6, 0x8310de94
	if !ctx.cr[6].lt {
	pc = 0x8310DE94; continue 'dispatch;
	}
	// 8310DC80: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8310DC84: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8310DC88: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8310DC8C: 419A0208  beq cr6, 0x8310de94
	if ctx.cr[6].eq {
	pc = 0x8310DE94; continue 'dispatch;
	}
	// 8310DC90: 7D493A14  add r10, r9, r7
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8310DC94: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310DC98: C16A0004  lfs f11, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310DC9C: ECCC5828  fsubs f6, f12, f11
	ctx.f[6].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8310DCA0: FF064000  fcmpu cr6, f6, f8
	ctx.cr[6].compare_f64(ctx.f[6].f64, ctx.f[8].f64);
	// 8310DCA4: 419801F0  blt cr6, 0x8310de94
	if ctx.cr[6].lt {
	pc = 0x8310DE94; continue 'dispatch;
	}
	// 8310DCA8: FF0D5800  fcmpu cr6, f13, f11
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[11].f64);
	// 8310DCAC: 4098000C  bge cr6, 0x8310dcb8
	if !ctx.cr[6].lt {
	pc = 0x8310DCB8; continue 'dispatch;
	}
	// 8310DCB0: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310DCB4: 480001E4  b 0x8310de98
	pc = 0x8310DE98; continue 'dispatch;
	// 8310DCB8: 838A0000  lwz r28, 0(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310DCBC: C16A0004  lfs f11, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310DCC0: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8310DCC4: EDAD5828  fsubs f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[11].f64) as f32) as f64);
	// 8310DCC8: 579B463E  srwi r27, r28, 0x18
	ctx.r[27].u32 = ctx.r[28].u32.wrapping_shr(24);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 8310DCCC: ED8C5828  fsubs f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8310DCD0: 5558063E  clrlwi r24, r10, 0x18
	ctx.r[24].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310DED8 size=136
    let mut pc: u32 = 0x8310DED8;
    'dispatch: loop {
        match pc {
            0x8310DED8 => {
    //   block [0x8310DED8..0x8310DF60)
	// 8310DED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310DEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310DEE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310DEE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310DEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310DEEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310DEF0: 4BFFEE11  bl 0x8310cd00
	ctx.lr = 0x8310DEF4;
	sub_8310CD00(ctx, base);
	// 8310DEF4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310DEF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310DEFC: 396BAFC0  addi r11, r11, -0x5040
	ctx.r[11].s64 = ctx.r[11].s64 + -20544;
	// 8310DF00: 387F0160  addi r3, r31, 0x160
	ctx.r[3].s64 = ctx.r[31].s64 + 352;
	// 8310DF04: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 8310DF08: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 8310DF0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310DF10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310DF14: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 8310DF18: 93DF0150  stw r30, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[30].u32 ) };
	// 8310DF1C: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 8310DF20: 93DF0158  stw r30, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[30].u32 ) };
	// 8310DF24: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 8310DF28: 4809A2B9  bl 0x831a81e0
	ctx.lr = 0x8310DF2C;
	sub_831A81E0(ctx, base);
	// 8310DF2C: 387F0190  addi r3, r31, 0x190
	ctx.r[3].s64 = ctx.r[31].s64 + 400;
	// 8310DF30: 93DF0188  stw r30, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[30].u32 ) };
	// 8310DF34: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8310DF38: 93DF018C  stw r30, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[30].u32 ) };
	// 8310DF3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310DF40: 4809A2A1  bl 0x831a81e0
	ctx.lr = 0x8310DF44;
	sub_831A81E0(ctx, base);
	// 8310DF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310DF48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310DF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310DF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310DF54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310DF58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310DF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310DF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310DF60 size=188
    let mut pc: u32 = 0x8310DF60;
    'dispatch: loop {
        match pc {
            0x8310DF60 => {
    //   block [0x8310DF60..0x8310E01C)
	// 8310DF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310DF64: 4809A205  bl 0x831a8168
	ctx.lr = 0x8310DF68;
	sub_831A8130(ctx, base);
	// 8310DF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310DF6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310DF70: 3B80000A  li r28, 0xa
	ctx.r[28].s64 = 10;
	// 8310DF74: 3BBF0160  addi r29, r31, 0x160
	ctx.r[29].s64 = ctx.r[31].s64 + 352;
	// 8310DF78: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310DF7C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310DF80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310DF84: 419A0008  beq cr6, 0x8310df8c
	if ctx.cr[6].eq {
	pc = 0x8310DF8C; continue 'dispatch;
	}
	// 8310DF88: 4BAD1051  bl 0x82bdefd8
	ctx.lr = 0x8310DF8C;
	sub_82BDEFD8(ctx, base);
	// 8310DF8C: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8310DF90: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8310DF94: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8310DF98: 4082FFE4  bne 0x8310df7c
	if !ctx.cr[0].eq {
	pc = 0x8310DF7C; continue 'dispatch;
	}
	// 8310DF9C: 807F0154  lwz r3, 0x154(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8310DFA0: 93DF0188  stw r30, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[30].u32 ) };
	// 8310DFA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310DFA8: 419A0008  beq cr6, 0x8310dfb0
	if ctx.cr[6].eq {
	pc = 0x8310DFB0; continue 'dispatch;
	}
	// 8310DFAC: 4BAD102D  bl 0x82bdefd8
	ctx.lr = 0x8310DFB0;
	sub_82BDEFD8(ctx, base);
	// 8310DFB0: 807F0150  lwz r3, 0x150(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 8310DFB4: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 8310DFB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310DFBC: 419A0008  beq cr6, 0x8310dfc4
	if ctx.cr[6].eq {
	pc = 0x8310DFC4; continue 'dispatch;
	}
	// 8310DFC0: 4BAD1019  bl 0x82bdefd8
	ctx.lr = 0x8310DFC4;
	sub_82BDEFD8(ctx, base);
	// 8310DFC4: 807F015C  lwz r3, 0x15c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8310DFC8: 93DF0150  stw r30, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[30].u32 ) };
	// 8310DFCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310DFD0: 419A0008  beq cr6, 0x8310dfd8
	if ctx.cr[6].eq {
	pc = 0x8310DFD8; continue 'dispatch;
	}
	// 8310DFD4: 4BAD1005  bl 0x82bdefd8
	ctx.lr = 0x8310DFD8;
	sub_82BDEFD8(ctx, base);
	// 8310DFD8: 807F0158  lwz r3, 0x158(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 8310DFDC: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 8310DFE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310DFE4: 419A0008  beq cr6, 0x8310dfec
	if ctx.cr[6].eq {
	pc = 0x8310DFEC; continue 'dispatch;
	}
	// 8310DFE8: 4BAD0FF1  bl 0x82bdefd8
	ctx.lr = 0x8310DFEC;
	sub_82BDEFD8(ctx, base);
	// 8310DFEC: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310DFF0: 93DF0158  stw r30, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[30].u32 ) };
	// 8310DFF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310DFF8: 419A0014  beq cr6, 0x8310e00c
	if ctx.cr[6].eq {
	pc = 0x8310E00C; continue 'dispatch;
	}
	// 8310DFFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E000: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310E004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E008: 4E800421  bctrl
	ctx.lr = 0x8310E00C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E00C: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 8310E010: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 8310E014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310E018: 4809A1A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E020 size=68
    let mut pc: u32 = 0x8310E020;
    'dispatch: loop {
        match pc {
            0x8310E020 => {
    //   block [0x8310E020..0x8310E064)
	// 8310E020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310E02C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E034: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310E038: 396BAFC0  addi r11, r11, -0x5040
	ctx.r[11].s64 = ctx.r[11].s64 + -20544;
	// 8310E03C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310E040: 4BFFFF21  bl 0x8310df60
	ctx.lr = 0x8310E044;
	sub_8310DF60(ctx, base);
	// 8310E044: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310E048: 396BAEC8  addi r11, r11, -0x5138
	ctx.r[11].s64 = ctx.r[11].s64 + -20792;
	// 8310E04C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310E050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310E054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E05C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310E060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E068 size=272
    let mut pc: u32 = 0x8310E068;
    'dispatch: loop {
        match pc {
            0x8310E068 => {
    //   block [0x8310E068..0x8310E178)
	// 8310E068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310E074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310E078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E07C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E080: 4BFFF0B1  bl 0x8310d130
	ctx.lr = 0x8310E084;
	sub_8310D130(ctx, base);
	// 8310E084: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E088: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310E08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310E090: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8310E094: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E098: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E09C: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8310E0A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E0A4: 4E800421  bctrl
	ctx.lr = 0x8310E0A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E0A8: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E0AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310E0B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310E0B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E0B8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8310E0BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E0C0: 4E800421  bctrl
	ctx.lr = 0x8310E0C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E0C4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E0C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310E0CC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8310E0D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E0D4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8310E0D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E0DC: 4E800421  bctrl
	ctx.lr = 0x8310E0E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E0E0: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E0E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E0E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E0EC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8310E0F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E0F4: 4E800421  bctrl
	ctx.lr = 0x8310E0F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E0F8: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E0FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E104: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310E108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E10C: 4E800421  bctrl
	ctx.lr = 0x8310E110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E110: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E114: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E118: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E11C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8310E120: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E124: 4E800421  bctrl
	ctx.lr = 0x8310E128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E128: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8310E12C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310E130: 409A002C  bne cr6, 0x8310e15c
	if !ctx.cr[6].eq {
	pc = 0x8310E15C; continue 'dispatch;
	}
	// 8310E134: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E138: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E13C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310E140: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E144: 4E800421  bctrl
	ctx.lr = 0x8310E148;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E148: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8310E14C: 41980010  blt cr6, 0x8310e15c
	if ctx.cr[6].lt {
	pc = 0x8310E15C; continue 'dispatch;
	}
	// 8310E150: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8310E154: 40980008  bge cr6, 0x8310e15c
	if !ctx.cr[6].lt {
	pc = 0x8310E15C; continue 'dispatch;
	}
	// 8310E158: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310E15C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310E160: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310E164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E16C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310E170: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310E178 size=196
    let mut pc: u32 = 0x8310E178;
    'dispatch: loop {
        match pc {
            0x8310E178 => {
    //   block [0x8310E178..0x8310E23C)
	// 8310E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E180: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310E184: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E18C: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 8310E190: 817F0148  lwz r11, 0x148(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 8310E194: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E198: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310E19C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E1A0: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 8310E1A4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8310E1A8: 814A0030  lwz r10, 0x30(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E1AC: 71650025  andi. r5, r11, 0x25
	ctx.r[5].u64 = ctx.r[11].u64 & 37;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8310E1B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8310E1B4: 4E800421  bctrl
	ctx.lr = 0x8310E1B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E1B8: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E1BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310E1C0: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 8310E1C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E1C8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E1CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E1D0: 4E800421  bctrl
	ctx.lr = 0x8310E1D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E1D4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E1D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8310E1DC: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 8310E1E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E1E4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E1E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E1EC: 4E800421  bctrl
	ctx.lr = 0x8310E1F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E1F0: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E1F4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8310E1F8: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8310E1FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E200: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E208: 4E800421  bctrl
	ctx.lr = 0x8310E20C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E20C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E210: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8310E214: 38800130  li r4, 0x130
	ctx.r[4].s64 = 304;
	// 8310E218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E21C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E224: 4E800421  bctrl
	ctx.lr = 0x8310E228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310E238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E240 size=216
    let mut pc: u32 = 0x8310E240;
    'dispatch: loop {
        match pc {
            0x8310E240 => {
    //   block [0x8310E240..0x8310E318)
	// 8310E240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310E24C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E254: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8310E258: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E25C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E260: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8310E264: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 8310E268: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E26C: 5566E7FE  rlwinm r6, r11, 0x1c, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8310E270: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310E274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E278: 4E800421  bctrl
	ctx.lr = 0x8310E27C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E27C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E280: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8310E284: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8310E288: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E28C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 8310E290: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E294: 5566DFFE  rlwinm r6, r11, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8310E298: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310E29C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E2A0: 4E800421  bctrl
	ctx.lr = 0x8310E2A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E2A4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E2A8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8310E2AC: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8310E2B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E2B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E2B8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310E2BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E2C0: 4E800421  bctrl
	ctx.lr = 0x8310E2C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E2C4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E2C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8310E2CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310E2D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E2D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E2D8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310E2DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E2E0: 4E800421  bctrl
	ctx.lr = 0x8310E2E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E2E4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E2E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8310E2EC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8310E2F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E2F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E2F8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310E2FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E300: 4E800421  bctrl
	ctx.lr = 0x8310E304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310E308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310E314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E318 size=208
    let mut pc: u32 = 0x8310E318;
    'dispatch: loop {
        match pc {
            0x8310E318 => {
    //   block [0x8310E318..0x8310E3E8)
	// 8310E318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E31C: 48099E51  bl 0x831a816c
	ctx.lr = 0x8310E320;
	sub_831A8130(ctx, base);
	// 8310E320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E324: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8310E328: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310E32C: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 8310E330: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
	// 8310E334: 817D00AC  lwz r11, 0xac(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(172 as u32) ) } as u64;
	// 8310E338: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8310E33C: 419A0054  beq cr6, 0x8310e390
	if ctx.cr[6].eq {
	pc = 0x8310E390; continue 'dispatch;
	}
	// 8310E340: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8310E344: 419A0044  beq cr6, 0x8310e388
	if ctx.cr[6].eq {
	pc = 0x8310E388; continue 'dispatch;
	}
	// 8310E348: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8310E34C: 419A002C  beq cr6, 0x8310e378
	if ctx.cr[6].eq {
	pc = 0x8310E378; continue 'dispatch;
	}
	// 8310E350: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8310E354: 419A001C  beq cr6, 0x8310e370
	if ctx.cr[6].eq {
	pc = 0x8310E370; continue 'dispatch;
	}
	// 8310E358: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8310E35C: 419A0024  beq cr6, 0x8310e380
	if ctx.cr[6].eq {
	pc = 0x8310E380; continue 'dispatch;
	}
	// 8310E360: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8310E364: 409A007C  bne cr6, 0x8310e3e0
	if !ctx.cr[6].eq {
	pc = 0x8310E3E0; continue 'dispatch;
	}
	// 8310E368: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8310E36C: 48000014  b 0x8310e380
	pc = 0x8310E380; continue 'dispatch;
	// 8310E370: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8310E374: 48000008  b 0x8310e37c
	pc = 0x8310E37C; continue 'dispatch;
	// 8310E378: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8310E37C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8310E380: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8310E384: 4800000C  b 0x8310e390
	pc = 0x8310E390; continue 'dispatch;
	// 8310E388: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310E38C: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8310E390: 807D014C  lwz r3, 0x14c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E394: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 8310E398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E39C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E3A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E3A4: 4E800421  bctrl
	ctx.lr = 0x8310E3A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E3A8: 807D014C  lwz r3, 0x14c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E3AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8310E3B0: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 8310E3B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E3B8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E3BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E3C0: 4E800421  bctrl
	ctx.lr = 0x8310E3C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E3C4: 807D014C  lwz r3, 0x14c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E3C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8310E3CC: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 8310E3D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E3D4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E3D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E3DC: 4E800421  bctrl
	ctx.lr = 0x8310E3E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E3E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310E3E4: 48099DD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310E3E8 size=88
    let mut pc: u32 = 0x8310E3E8;
    'dispatch: loop {
        match pc {
            0x8310E3E8 => {
    //   block [0x8310E3E8..0x8310E440)
	// 8310E3E8: 816300B0  lwz r11, 0xb0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) } as u64;
	// 8310E3EC: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8310E3F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310E3F4: 419A0034  beq cr6, 0x8310e428
	if ctx.cr[6].eq {
	pc = 0x8310E428; continue 'dispatch;
	}
	// 8310E3F8: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310E3FC: 40820008  bne 0x8310e404
	if !ctx.cr[0].eq {
	pc = 0x8310E404; continue 'dispatch;
	}
	// 8310E400: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8310E404: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310E408: 40820008  bne 0x8310e410
	if !ctx.cr[0].eq {
	pc = 0x8310E410; continue 'dispatch;
	}
	// 8310E40C: 54A5003C  rlwinm r5, r5, 0, 0, 0x1e
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 8310E410: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310E414: 40820008  bne 0x8310e41c
	if !ctx.cr[0].eq {
	pc = 0x8310E41C; continue 'dispatch;
	}
	// 8310E418: 54A507FA  rlwinm r5, r5, 0, 0x1f, 0x1d
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 8310E41C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310E420: 40820008  bne 0x8310e428
	if !ctx.cr[0].eq {
	pc = 0x8310E428; continue 'dispatch;
	}
	// 8310E424: 54A507B8  rlwinm r5, r5, 0, 0x1e, 0x1c
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 8310E428: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E42C: 388000D4  li r4, 0xd4
	ctx.r[4].s64 = 212;
	// 8310E430: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E434: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310E438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E43C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E440 size=196
    let mut pc: u32 = 0x8310E440;
    'dispatch: loop {
        match pc {
            0x8310E440 => {
    //   block [0x8310E440..0x8310E504)
	// 8310E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E444: 48099D29  bl 0x831a816c
	ctx.lr = 0x8310E448;
	sub_831A8130(ctx, base);
	// 8310E448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E44C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8310E450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E454: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8310E458: 409A0010  bne cr6, 0x8310e468
	if !ctx.cr[6].eq {
	pc = 0x8310E468; continue 'dispatch;
	}
	// 8310E45C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310E460: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8310E464: 48000094  b 0x8310e4f8
	pc = 0x8310E4F8; continue 'dispatch;
	// 8310E468: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8310E46C: 419A0030  beq cr6, 0x8310e49c
	if ctx.cr[6].eq {
	pc = 0x8310E49C; continue 'dispatch;
	}
	// 8310E470: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8310E474: 419A0010  beq cr6, 0x8310e484
	if ctx.cr[6].eq {
	pc = 0x8310E484; continue 'dispatch;
	}
	// 8310E478: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310E47C: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310E480: 4800007C  b 0x8310e4fc
	pc = 0x8310E4FC; continue 'dispatch;
	// 8310E484: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8310E488: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8310E48C: 419A006C  beq cr6, 0x8310e4f8
	if ctx.cr[6].eq {
	pc = 0x8310E4F8; continue 'dispatch;
	}
	// 8310E490: 809F0158  lwz r4, 0x158(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 8310E494: 83DF015C  lwz r30, 0x15c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8310E498: 48000018  b 0x8310e4b0
	pc = 0x8310E4B0; continue 'dispatch;
	// 8310E49C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8310E4A0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8310E4A4: 419A0054  beq cr6, 0x8310e4f8
	if ctx.cr[6].eq {
	pc = 0x8310E4F8; continue 'dispatch;
	}
	// 8310E4A8: 809F0150  lwz r4, 0x150(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 8310E4AC: 83DF0154  lwz r30, 0x154(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8310E4B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8310E4B4: 419A0044  beq cr6, 0x8310e4f8
	if ctx.cr[6].eq {
	pc = 0x8310E4F8; continue 'dispatch;
	}
	// 8310E4B8: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E4BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E4C0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8310E4C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E4C8: 4E800421  bctrl
	ctx.lr = 0x8310E4CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E4CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E4D0: 4180002C  blt 0x8310e4fc
	if ctx.cr[0].lt {
	pc = 0x8310E4FC; continue 'dispatch;
	}
	// 8310E4D4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E4D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310E4DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E4E0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310E4E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E4E8: 4E800421  bctrl
	ctx.lr = 0x8310E4EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E4EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E4F0: 4180000C  blt 0x8310e4fc
	if ctx.cr[0].lt {
	pc = 0x8310E4FC; continue 'dispatch;
	}
	// 8310E4F4: 93BF008C  stw r29, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 8310E4F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310E4FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310E500: 48099CBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E508 size=272
    let mut pc: u32 = 0x8310E508;
    'dispatch: loop {
        match pc {
            0x8310E508 => {
    //   block [0x8310E508..0x8310E618)
	// 8310E508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E510: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310E514: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310E518: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E51C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E520: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8310E524: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8310E528: 419A00D4  beq cr6, 0x8310e5fc
	if ctx.cr[6].eq {
	pc = 0x8310E5FC; continue 'dispatch;
	}
	// 8310E52C: 909F0090  stw r4, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 8310E530: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8310E534: 419800C8  blt cr6, 0x8310e5fc
	if ctx.cr[6].lt {
	pc = 0x8310E5FC; continue 'dispatch;
	}
	// 8310E538: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 8310E53C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310E540: 419A0084  beq cr6, 0x8310e5c4
	if ctx.cr[6].eq {
	pc = 0x8310E5C4; continue 'dispatch;
	}
	// 8310E544: 39640005  addi r11, r4, 5
	ctx.r[11].s64 = ctx.r[4].s64 + 5;
	// 8310E548: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8310E54C: 41980010  blt cr6, 0x8310e55c
	if ctx.cr[6].lt {
	pc = 0x8310E55C; continue 'dispatch;
	}
	// 8310E550: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310E554: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310E558: 480000A8  b 0x8310e600
	pc = 0x8310E600; continue 'dispatch;
	// 8310E55C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E560: 3964005D  addi r11, r4, 0x5d
	ctx.r[11].s64 = ctx.r[4].s64 + 93;
	// 8310E564: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8310E568: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E56C: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8310E570: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8310E574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E578: 4E800421  bctrl
	ctx.lr = 0x8310E57C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E57C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8310E580: 4080000C  bge 0x8310e58c
	if !ctx.cr[0].lt {
	pc = 0x8310E58C; continue 'dispatch;
	}
	// 8310E584: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310E588: 48000078  b 0x8310e600
	pc = 0x8310E600; continue 'dispatch;
	// 8310E58C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E590: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310E594: 815F00B8  lwz r10, 0xb8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 8310E598: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8310E59C: 396BAE00  addi r11, r11, -0x5200
	ctx.r[11].s64 = ctx.r[11].s64 + -20992;
	// 8310E5A0: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8310E5A4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8310E5A8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E5AC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8310E5B0: 38ABFFC0  addi r5, r11, -0x40
	ctx.r[5].s64 = ctx.r[11].s64 + -64;
	// 8310E5B4: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310E5B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E5BC: 4E800421  bctrl
	ctx.lr = 0x8310E5C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E5C0: 48000030  b 0x8310e5f0
	pc = 0x8310E5F0; continue 'dispatch;
	// 8310E5C4: 2F04000A  cmpwi cr6, r4, 0xa
	ctx.cr[6].compare_i32(ctx.r[4].s32, 10, &mut ctx.xer);
	// 8310E5C8: 4098FF88  bge cr6, 0x8310e550
	if !ctx.cr[6].lt {
	pc = 0x8310E550; continue 'dispatch;
	}
	// 8310E5CC: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E5D0: 39640058  addi r11, r4, 0x58
	ctx.r[11].s64 = ctx.r[4].s64 + 88;
	// 8310E5D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8310E5D8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E5DC: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8310E5E0: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8310E5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E5E8: 4E800421  bctrl
	ctx.lr = 0x8310E5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E5EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310E5F0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8310E5F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310E5F8: 41980008  blt cr6, 0x8310e600
	if ctx.cr[6].lt {
	pc = 0x8310E600; continue 'dispatch;
	}
	// 8310E5FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310E600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310E604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E60C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310E610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310E614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310E618 size=36
    let mut pc: u32 = 0x8310E618;
    'dispatch: loop {
        match pc {
            0x8310E618 => {
    //   block [0x8310E618..0x8310E63C)
	// 8310E618: 8143009C  lwz r10, 0x9c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 8310E61C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8310E620: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8310E624: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8310E628: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310E62C: 419A0010  beq cr6, 0x8310e63c
	if ctx.cr[6].eq {
		sub_8310E63C(ctx, base);
		return;
	}
	// 8310E630: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310E634: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310E638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E63C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310E63C size=40
    let mut pc: u32 = 0x8310E63C;
    'dispatch: loop {
        match pc {
            0x8310E63C => {
    //   block [0x8310E63C..0x8310E664)
	// 8310E63C: 814300B4  lwz r10, 0xb4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 8310E640: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310E644: 409A0020  bne cr6, 0x8310e664
	if !ctx.cr[6].eq {
		sub_8310E664(ctx, base);
		return;
	}
	// 8310E648: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E64C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8310E650: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8310E654: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E658: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8310E65C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E660: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310E664 size=8
    let mut pc: u32 = 0x8310E664;
    'dispatch: loop {
        match pc {
            0x8310E664 => {
    //   block [0x8310E664..0x8310E66C)
	// 8310E664: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310E668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E670 size=88
    let mut pc: u32 = 0x8310E670;
    'dispatch: loop {
        match pc {
            0x8310E670 => {
    //   block [0x8310E670..0x8310E6C8)
	// 8310E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E67C: 8163014C  lwz r11, 0x14c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E680: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310E684: 409A0010  bne cr6, 0x8310e694
	if !ctx.cr[6].eq {
	pc = 0x8310E694; continue 'dispatch;
	}
	// 8310E688: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310E68C: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310E690: 48000028  b 0x8310e6b8
	pc = 0x8310E6B8; continue 'dispatch;
	// 8310E694: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E698: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8310E69C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E6A0: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8310E6A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E6A8: 4E800421  bctrl
	ctx.lr = 0x8310E6AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E6AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E6B0: 41800008  blt 0x8310e6b8
	if ctx.cr[0].lt {
	pc = 0x8310E6B8; continue 'dispatch;
	}
	// 8310E6B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310E6B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310E6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E6C8 size=88
    let mut pc: u32 = 0x8310E6C8;
    'dispatch: loop {
        match pc {
            0x8310E6C8 => {
    //   block [0x8310E6C8..0x8310E720)
	// 8310E6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E6D4: 8163014C  lwz r11, 0x14c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E6D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310E6DC: 409A0010  bne cr6, 0x8310e6ec
	if !ctx.cr[6].eq {
	pc = 0x8310E6EC; continue 'dispatch;
	}
	// 8310E6E0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310E6E4: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310E6E8: 48000028  b 0x8310e710
	pc = 0x8310E710; continue 'dispatch;
	// 8310E6EC: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E6F0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8310E6F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E6F8: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8310E6FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E700: 4E800421  bctrl
	ctx.lr = 0x8310E704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E704: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E708: 41800008  blt 0x8310e710
	if ctx.cr[0].lt {
	pc = 0x8310E710; continue 'dispatch;
	}
	// 8310E70C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310E710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310E714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E720 size=460
    let mut pc: u32 = 0x8310E720;
    'dispatch: loop {
        match pc {
            0x8310E720 => {
    //   block [0x8310E720..0x8310E8EC)
	// 8310E720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E724: 48099A3D  bl 0x831a8160
	ctx.lr = 0x8310E728;
	sub_831A8130(ctx, base);
	// 8310E728: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E72C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310E730: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8310E734: 4BFFF82D  bl 0x8310df60
	ctx.lr = 0x8310E738;
	sub_8310DF60(ctx, base);
	// 8310E738: 935E0148  stw r26, 0x148(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(328 as u32), ctx.r[26].u32 ) };
	// 8310E73C: 807A0008  lwz r3, 8(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310E740: 907E014C  stw r3, 0x14c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(332 as u32), ctx.r[3].u32 ) };
	// 8310E744: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E748: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E74C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E750: 4E800421  bctrl
	ctx.lr = 0x8310E754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E754: 3D60002C  lis r11, 0x2c
	ctx.r[11].s64 = 2883584;
	// 8310E758: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E75C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8310E760: 617D23A5  ori r29, r11, 0x23a5
	ctx.r[29].u64 = ctx.r[11].u64 | 9125;
	// 8310E764: 3B6000FF  li r27, 0xff
	ctx.r[27].s64 = 255;
	// 8310E768: B3E10050  sth r31, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u16 ) };
	// 8310E76C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 8310E770: B3E10052  sth r31, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[31].u16 ) };
	// 8310E774: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8310E778: 38BE0150  addi r5, r30, 0x150
	ctx.r[5].s64 = ctx.r[30].s64 + 336;
	// 8310E77C: 9BE10058  stb r31, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u8 ) };
	// 8310E780: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8310E784: 9BE10059  stb r31, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[31].u8 ) };
	// 8310E788: 9BE1005A  stb r31, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[31].u8 ) };
	// 8310E78C: B361005C  sth r27, 0x5c(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u16 ) };
	// 8310E790: B3E1005E  sth r31, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[31].u16 ) };
	// 8310E794: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 8310E798: 9BE10064  stb r31, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u8 ) };
	// 8310E79C: 9BE10065  stb r31, 0x65(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(101 as u32), ctx.r[31].u8 ) };
	// 8310E7A0: 9BE10066  stb r31, 0x66(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[31].u8 ) };
	// 8310E7A4: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310E7A8: 917E00B4  stw r11, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 8310E7AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E7B0: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8310E7B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E7B8: 4E800421  bctrl
	ctx.lr = 0x8310E7BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E7BC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E7C0: 41800124  blt 0x8310e8e4
	if ctx.cr[0].lt {
	pc = 0x8310E8E4; continue 'dispatch;
	}
	// 8310E7C4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E7C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310E7CC: 38DE0154  addi r6, r30, 0x154
	ctx.r[6].s64 = ctx.r[30].s64 + 340;
	// 8310E7D0: 3B4B98D8  addi r26, r11, -0x6728
	ctx.r[26].s64 = ctx.r[11].s64 + -26408;
	// 8310E7D4: 38A00258  li r5, 0x258
	ctx.r[5].s64 = 600;
	// 8310E7D8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8310E7DC: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310E7E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310E7E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E7E8: 4E800421  bctrl
	ctx.lr = 0x8310E7EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E7EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E7F0: 418000F4  blt 0x8310e8e4
	if ctx.cr[0].lt {
	pc = 0x8310E8E4; continue 'dispatch;
	}
	// 8310E7F4: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E7F8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8310E7FC: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8310E800: B3E10070  sth r31, 0x70(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u16 ) };
	// 8310E804: B161007E  sth r11, 0x7e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[11].u16 ) };
	// 8310E808: 38BE0158  addi r5, r30, 0x158
	ctx.r[5].s64 = ctx.r[30].s64 + 344;
	// 8310E80C: B3E10072  sth r31, 0x72(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(114 as u32), ctx.r[31].u16 ) };
	// 8310E810: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8310E814: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 8310E818: 9BE10078  stb r31, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u8 ) };
	// 8310E81C: 9BE10079  stb r31, 0x79(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(121 as u32), ctx.r[31].u8 ) };
	// 8310E820: 9BE1007A  stb r31, 0x7a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(122 as u32), ctx.r[31].u8 ) };
	// 8310E824: B3E1007C  sth r31, 0x7c(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u16 ) };
	// 8310E828: 93A10080  stw r29, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 8310E82C: 9BE10084  stb r31, 0x84(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[31].u8 ) };
	// 8310E830: 99410085  stb r10, 0x85(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 8310E834: 9BE10086  stb r31, 0x86(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(134 as u32), ctx.r[31].u8 ) };
	// 8310E838: B3610088  sth r27, 0x88(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[27].u16 ) };
	// 8310E83C: B3E1008A  sth r31, 0x8a(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(138 as u32), ctx.r[31].u16 ) };
	// 8310E840: 9381008C  stw r28, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 8310E844: 9BE10090  stb r31, 0x90(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u8 ) };
	// 8310E848: 9BE10091  stb r31, 0x91(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(145 as u32), ctx.r[31].u8 ) };
	// 8310E84C: 9BE10092  stb r31, 0x92(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(146 as u32), ctx.r[31].u8 ) };
	// 8310E850: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E854: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8310E858: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E85C: 4E800421  bctrl
	ctx.lr = 0x8310E860;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E860: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E864: 41800080  blt 0x8310e8e4
	if ctx.cr[0].lt {
	pc = 0x8310E8E4; continue 'dispatch;
	}
	// 8310E868: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E86C: 389A0258  addi r4, r26, 0x258
	ctx.r[4].s64 = ctx.r[26].s64 + 600;
	// 8310E870: 38DE015C  addi r6, r30, 0x15c
	ctx.r[6].s64 = ctx.r[30].s64 + 348;
	// 8310E874: 38A001D0  li r5, 0x1d0
	ctx.r[5].s64 = 464;
	// 8310E878: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310E87C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310E880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E884: 4E800421  bctrl
	ctx.lr = 0x8310E888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E888: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E88C: 41800058  blt 0x8310e8e4
	if ctx.cr[0].lt {
	pc = 0x8310E8E4; continue 'dispatch;
	}
	// 8310E890: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310E894: 3B9E0160  addi r28, r30, 0x160
	ctx.r[28].s64 = ctx.r[30].s64 + 352;
	// 8310E898: 3BABB0B8  addi r29, r11, -0x4f48
	ctx.r[29].s64 = ctx.r[11].s64 + -20296;
	// 8310E89C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E8A0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8310E8A4: 419A0028  beq cr6, 0x8310e8cc
	if ctx.cr[6].eq {
	pc = 0x8310E8CC; continue 'dispatch;
	}
	// 8310E8A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E8AC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8310E8B0: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310E8B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310E8B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310E8BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E8C0: 4E800421  bctrl
	ctx.lr = 0x8310E8C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E8C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310E8C8: 4180001C  blt 0x8310e8e4
	if ctx.cr[0].lt {
	pc = 0x8310E8E4; continue 'dispatch;
	}
	// 8310E8CC: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8310E8D0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8310E8D4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 8310E8D8: 2B1F0050  cmplwi cr6, r31, 0x50
	ctx.cr[6].compare_u32(ctx.r[31].u32, 80 as u32, &mut ctx.xer);
	// 8310E8DC: 4198FFC0  blt cr6, 0x8310e89c
	if ctx.cr[6].lt {
	pc = 0x8310E89C; continue 'dispatch;
	}
	// 8310E8E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310E8E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8310E8E8: 480998C8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E8F0 size=120
    let mut pc: u32 = 0x8310E8F0;
    'dispatch: loop {
        match pc {
            0x8310E8F0 => {
    //   block [0x8310E8F0..0x8310E968)
	// 8310E8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E8F4: 48099879  bl 0x831a816c
	ctx.lr = 0x8310E8F8;
	sub_831A8130(ctx, base);
	// 8310E8F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E8FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310E900: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8310E904: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8310E908: 419A0020  beq cr6, 0x8310e928
	if ctx.cr[6].eq {
	pc = 0x8310E928; continue 'dispatch;
	}
	// 8310E90C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E910: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8310E914: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310E918: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E91C: 4E800421  bctrl
	ctx.lr = 0x8310E920;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E924: 48000008  b 0x8310e92c
	pc = 0x8310E92C; continue 'dispatch;
	// 8310E928: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8310E92C: 817E0188  lwz r11, 0x188(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(392 as u32) ) } as u64;
	// 8310E930: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8310E934: 419A0028  beq cr6, 0x8310e95c
	if ctx.cr[6].eq {
	pc = 0x8310E95C; continue 'dispatch;
	}
	// 8310E938: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E93C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8310E940: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E944: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E948: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8310E94C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E950: 4E800421  bctrl
	ctx.lr = 0x8310E954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E954: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8310E958: 93FE0188  stw r31, 0x188(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(392 as u32), ctx.r[31].u32 ) };
	// 8310E95C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8310E960: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310E964: 48099858  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E968 size=76
    let mut pc: u32 = 0x8310E968;
    'dispatch: loop {
        match pc {
            0x8310E968 => {
    //   block [0x8310E968..0x8310E9B4)
	// 8310E968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310E974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E97C: 4BFFEB85  bl 0x8310d500
	ctx.lr = 0x8310E980;
	sub_8310D500(ctx, base);
	// 8310E980: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E984: 38BF00C8  addi r5, r31, 0xc8
	ctx.r[5].s64 = ctx.r[31].s64 + 200;
	// 8310E988: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8310E98C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310E990: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E994: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8310E998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E99C: 4E800421  bctrl
	ctx.lr = 0x8310E9A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310E9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310E9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310E9AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310E9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310E9B8 size=116
    let mut pc: u32 = 0x8310E9B8;
    'dispatch: loop {
        match pc {
            0x8310E9B8 => {
    //   block [0x8310E9B8..0x8310EA2C)
	// 8310E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310E9C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310E9C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310E9C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310E9CC: 4BFFECD5  bl 0x8310d6a0
	ctx.lr = 0x8310E9D0;
	sub_8310D6A0(ctx, base);
	// 8310E9D0: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310E9D4: 38BF0108  addi r5, r31, 0x108
	ctx.r[5].s64 = ctx.r[31].s64 + 264;
	// 8310E9D8: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8310E9DC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8310E9E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E9E4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8310E9E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310E9EC: 4E800421  bctrl
	ctx.lr = 0x8310E9F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310E9F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310E9F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310E9F8: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 8310E9FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EA00: 4E800421  bctrl
	ctx.lr = 0x8310EA04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EA04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310EA0C: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 8310EA10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EA14: 4E800421  bctrl
	ctx.lr = 0x8310EA18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310EA24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310EA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EA30 size=32
    let mut pc: u32 = 0x8310EA30;
    'dispatch: loop {
        match pc {
            0x8310EA30 => {
    //   block [0x8310EA30..0x8310EA50)
	// 8310EA30: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EA34: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8310EA38: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8310EA3C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 8310EA40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EA44: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8310EA48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EA4C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EA50 size=32
    let mut pc: u32 = 0x8310EA50;
    'dispatch: loop {
        match pc {
            0x8310EA50 => {
    //   block [0x8310EA50..0x8310EA70)
	// 8310EA50: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EA54: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8310EA58: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8310EA5C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8310EA60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EA64: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8310EA68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EA6C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310EA70 size=184
    let mut pc: u32 = 0x8310EA70;
    'dispatch: loop {
        match pc {
            0x8310EA70 => {
    //   block [0x8310EA70..0x8310EB28)
	// 8310EA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310EA74: 480996F9  bl 0x831a816c
	ctx.lr = 0x8310EA78;
	sub_831A8130(ctx, base);
	// 8310EA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310EA7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310EA80: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8310EA84: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8310EA88: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8310EA8C: 409A0020  bne cr6, 0x8310eaac
	if !ctx.cr[6].eq {
	pc = 0x8310EAAC; continue 'dispatch;
	}
	// 8310EA90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8310EA94: 409A0040  bne cr6, 0x8310ead4
	if !ctx.cr[6].eq {
	pc = 0x8310EAD4; continue 'dispatch;
	}
	// 8310EA98: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8310EA9C: 409A0060  bne cr6, 0x8310eafc
	if !ctx.cr[6].eq {
	pc = 0x8310EAFC; continue 'dispatch;
	}
	// 8310EAA0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310EAA4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310EAA8: 48000078  b 0x8310eb20
	pc = 0x8310EB20; continue 'dispatch;
	// 8310EAAC: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EAB0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8310EAB4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8310EAB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310EABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EAC0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310EAC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EAC8: 4E800421  bctrl
	ctx.lr = 0x8310EACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EACC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8310EAD0: 419A0024  beq cr6, 0x8310eaf4
	if ctx.cr[6].eq {
	pc = 0x8310EAF4; continue 'dispatch;
	}
	// 8310EAD4: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EAD8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8310EADC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8310EAE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310EAE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EAE8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310EAEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EAF0: 4E800421  bctrl
	ctx.lr = 0x8310EAF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EAF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8310EAF8: 419A0024  beq cr6, 0x8310eb1c
	if ctx.cr[6].eq {
	pc = 0x8310EB1C; continue 'dispatch;
	}
	// 8310EAFC: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EB00: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8310EB04: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310EB08: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8310EB0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EB10: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310EB14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EB18: 4E800421  bctrl
	ctx.lr = 0x8310EB1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EB1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310EB20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310EB24: 48099698  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310EB28 size=144
    let mut pc: u32 = 0x8310EB28;
    'dispatch: loop {
        match pc {
            0x8310EB28 => {
    //   block [0x8310EB28..0x8310EBB8)
	// 8310EB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310EB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310EB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310EB34: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310EB38: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 8310EB3C: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310EB44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310EB48: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8310EB4C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8310EB50: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310EB54: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8310EB58: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8310EB5C: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8310EB60: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8310EB64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EB68: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 8310EB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EB70: 4E800421  bctrl
	ctx.lr = 0x8310EB74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EB74: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8310EB78: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310EB7C: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310EB80: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8310EB84: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310EB88: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8310EB8C: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8310EB90: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8310EB94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310EB98: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8310EB9C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8310EBA0: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8310EBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310EBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310EBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310EBB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310EBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310EBB8 size=208
    let mut pc: u32 = 0x8310EBB8;
    'dispatch: loop {
        match pc {
            0x8310EBB8 => {
    //   block [0x8310EBB8..0x8310EC88)
	// 8310EBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310EBBC: 480995AD  bl 0x831a8168
	ctx.lr = 0x8310EBC0;
	sub_831A8130(ctx, base);
	// 8310EBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310EBC4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8310EBC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310EBCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8310EBD0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8310EBD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310EBD8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8310EBDC: 419A0008  beq cr6, 0x8310ebe4
	if ctx.cr[6].eq {
	pc = 0x8310EBE4; continue 'dispatch;
	}
	// 8310EBE0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310EBE4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EBE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8310EBEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8310EBF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310EBF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EBF8: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8310EBFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EC00: 4E800421  bctrl
	ctx.lr = 0x8310EC04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EC04: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EC08: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8310EC0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310EC10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EC14: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310EC18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EC1C: 4E800421  bctrl
	ctx.lr = 0x8310EC20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EC20: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8310EC24: 4180002C  blt 0x8310ec50
	if ctx.cr[0].lt {
	pc = 0x8310EC50; continue 'dispatch;
	}
	// 8310EC28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EC2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310EC30: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8310EC34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EC38: 4E800421  bctrl
	ctx.lr = 0x8310EC3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EC3C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8310EC40: 419A0034  beq cr6, 0x8310ec74
	if ctx.cr[6].eq {
	pc = 0x8310EC74; continue 'dispatch;
	}
	// 8310EC44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8310EC48: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310EC4C: 48000030  b 0x8310ec7c
	pc = 0x8310EC7C; continue 'dispatch;
	// 8310EC50: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8310EC54: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8310EC58: 419A0024  beq cr6, 0x8310ec7c
	if ctx.cr[6].eq {
	pc = 0x8310EC7C; continue 'dispatch;
	}
	// 8310EC5C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EC60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310EC64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EC68: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310EC6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EC70: 4E800421  bctrl
	ctx.lr = 0x8310EC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EC74: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8310EC78: 4BAD0361  bl 0x82bdefd8
	ctx.lr = 0x8310EC7C;
	sub_82BDEFD8(ctx, base);
	// 8310EC7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8310EC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310EC84: 48099534  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EC88 size=20
    let mut pc: u32 = 0x8310EC88;
    'dispatch: loop {
        match pc {
            0x8310EC88 => {
    //   block [0x8310EC88..0x8310EC9C)
	// 8310EC88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310EC8C: 546A173A  rlwinm r10, r3, 2, 0x1c, 0x1d
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x3FFFFFFFu64;
	// 8310EC90: 396BAE80  addi r11, r11, -0x5180
	ctx.r[11].s64 = ctx.r[11].s64 + -20864;
	// 8310EC94: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8310EC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310ECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310ECA0 size=20
    let mut pc: u32 = 0x8310ECA0;
    'dispatch: loop {
        match pc {
            0x8310ECA0 => {
    //   block [0x8310ECA0..0x8310ECB4)
	// 8310ECA0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310ECA4: 546A073A  rlwinm r10, r3, 0, 0x1c, 0x1d
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8310ECA8: 396BAE80  addi r11, r11, -0x5180
	ctx.r[11].s64 = ctx.r[11].s64 + -20864;
	// 8310ECAC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8310ECB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310ECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310ECB8 size=12
    let mut pc: u32 = 0x8310ECB8;
    'dispatch: loop {
        match pc {
            0x8310ECB8 => {
    //   block [0x8310ECB8..0x8310ECC4)
	// 8310ECB8: 7C6B18F8  nor r11, r3, r3
	ctx.r[11].u64 = !(ctx.r[3].u64 | ctx.r[3].u64);
	// 8310ECBC: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8310ECC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310ECC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310ECC8 size=12
    let mut pc: u32 = 0x8310ECC8;
    'dispatch: loop {
        match pc {
            0x8310ECC8 => {
    //   block [0x8310ECC8..0x8310ECD4)
	// 8310ECC8: 7C6B18F8  nor r11, r3, r3
	ctx.r[11].u64 = !(ctx.r[3].u64 | ctx.r[3].u64);
	// 8310ECCC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8310ECD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310ECD8 size=252
    let mut pc: u32 = 0x8310ECD8;
    'dispatch: loop {
        match pc {
            0x8310ECD8 => {
    //   block [0x8310ECD8..0x8310EDD4)
	// 8310ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310ECDC: 4809948D  bl 0x831a8168
	ctx.lr = 0x8310ECE0;
	sub_831A8130(ctx, base);
	// 8310ECE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310ECE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310ECE8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8310ECEC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8310ECF0: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8310ECF4: 7FCB5A78  xor r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 ^ ctx.r[11].u64;
	// 8310ECF8: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310ECFC: 41820024  beq 0x8310ed20
	if ctx.cr[0].eq {
	pc = 0x8310ED20; continue 'dispatch;
	}
	// 8310ED00: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310ED04: 7FCBF0F8  nor r11, r30, r30
	ctx.r[11].u64 = !(ctx.r[30].u64 | ctx.r[30].u64);
	// 8310ED08: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8310ED0C: 5566E7FE  rlwinm r6, r11, 0x1c, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8310ED10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310ED14: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310ED18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310ED1C: 4E800421  bctrl
	ctx.lr = 0x8310ED20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310ED20: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8310ED24: 7FCB5A78  xor r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 ^ ctx.r[11].u64;
	// 8310ED28: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310ED2C: 41820028  beq 0x8310ed54
	if ctx.cr[0].eq {
	pc = 0x8310ED54; continue 'dispatch;
	}
	// 8310ED30: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310ED34: 7FCBF0F8  nor r11, r30, r30
	ctx.r[11].u64 = !(ctx.r[30].u64 | ctx.r[30].u64);
	// 8310ED38: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8310ED3C: 5566DFFE  rlwinm r6, r11, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8310ED40: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310ED44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310ED48: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310ED4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310ED50: 4E800421  bctrl
	ctx.lr = 0x8310ED54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310ED54: 813F0088  lwz r9, 0x88(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8310ED58: 57CB07BE  clrlwi r11, r30, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000003u64;
	// 8310ED5C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 8310ED60: 552907BE  clrlwi r9, r9, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 8310ED64: 3BAAAE80  addi r29, r10, -0x5180
	ctx.r[29].s64 = ctx.r[10].s64 + -20864;
	// 8310ED68: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8310ED6C: 419A0028  beq cr6, 0x8310ed94
	if ctx.cr[6].eq {
	pc = 0x8310ED94; continue 'dispatch;
	}
	// 8310ED70: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310ED74: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8310ED78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310ED7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310ED80: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310ED84: 7CCBE82E  lwzx r6, r11, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8310ED88: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310ED8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310ED90: 4E800421  bctrl
	ctx.lr = 0x8310ED94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310ED94: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8310ED98: 7FCB5A78  xor r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 ^ ctx.r[11].u64;
	// 8310ED9C: 556B073B  rlwinm. r11, r11, 0, 0x1c, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310EDA0: 41820028  beq 0x8310edc8
	if ctx.cr[0].eq {
	pc = 0x8310EDC8; continue 'dispatch;
	}
	// 8310EDA4: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EDA8: 57CB073A  rlwinm r11, r30, 0, 0x1c, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 8310EDAC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8310EDB0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310EDB4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EDB8: 7CCBE82E  lwzx r6, r11, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8310EDBC: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310EDC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EDC4: 4E800421  bctrl
	ctx.lr = 0x8310EDC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EDC8: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 8310EDCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310EDD0: 480993E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EDD8 size=20
    let mut pc: u32 = 0x8310EDD8;
    'dispatch: loop {
        match pc {
            0x8310EDD8 => {
    //   block [0x8310EDD8..0x8310EDEC)
	// 8310EDD8: 81630170  lwz r11, 0x170(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(368 as u32) ) } as u64;
	// 8310EDDC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8310EDE0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8310EDE4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8310EDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310EDF0 size=144
    let mut pc: u32 = 0x8310EDF0;
    'dispatch: loop {
        match pc {
            0x8310EDF0 => {
    //   block [0x8310EDF0..0x8310EE80)
	// 8310EDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310EDF4: 48099379  bl 0x831a816c
	ctx.lr = 0x8310EDF8;
	sub_831A8130(ctx, base);
	// 8310EDF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310EDFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8310EE00: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8310EE04: 409A0010  bne cr6, 0x8310ee14
	if !ctx.cr[6].eq {
	pc = 0x8310EE14; continue 'dispatch;
	}
	// 8310EE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310EE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310EE10: 4800004C  b 0x8310ee5c
	pc = 0x8310EE5C; continue 'dispatch;
	// 8310EE14: 3BFD0190  addi r31, r29, 0x190
	ctx.r[31].s64 = ctx.r[29].s64 + 400;
	// 8310EE18: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8310EE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310EE20: 480996F1  bl 0x831a8510
	ctx.lr = 0x8310EE24;
	sub_831A8510(ctx, base);
	// 8310EE24: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310EE28: 807D014C  lwz r3, 0x14c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EE2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8310EE30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310EE34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EE38: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 8310EE3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EE40: 4E800421  bctrl
	ctx.lr = 0x8310EE44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EE44: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8310EE48: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 8310EE4C: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 8310EE50: 4198FFD8  blt cr6, 0x8310ee28
	if ctx.cr[6].lt {
	pc = 0x8310EE28; continue 'dispatch;
	}
	// 8310EE54: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 8310EE58: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8310EE5C: 807D014C  lwz r3, 0x14c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EE60: 388000AC  li r4, 0xac
	ctx.r[4].s64 = 172;
	// 8310EE64: 917D018C  stw r11, 0x18c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(396 as u32), ctx.r[11].u32 ) };
	// 8310EE68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EE6C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310EE70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EE74: 4E800421  bctrl
	ctx.lr = 0x8310EE78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EE78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310EE7C: 48099340  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310EE80 size=64
    let mut pc: u32 = 0x8310EE80;
    'dispatch: loop {
        match pc {
            0x8310EE80 => {
    //   block [0x8310EE80..0x8310EEC0)
	// 8310EE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310EE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310EE88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310EE8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8310EE90: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8310EE94: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8310EE98: 388B0190  addi r4, r11, 0x190
	ctx.r[4].s64 = ctx.r[11].s64 + 400;
	// 8310EE9C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8310EEA0: 812B018C  lwz r9, 0x18c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(396 as u32) ) } as u64;
	// 8310EEA4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8310EEA8: 48099669  bl 0x831a8510
	ctx.lr = 0x8310EEAC;
	sub_831A8510(ctx, base);
	// 8310EEAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310EEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310EEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310EEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310EEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EEC0 size=20
    let mut pc: u32 = 0x8310EEC0;
    'dispatch: loop {
        match pc {
            0x8310EEC0 => {
    //   block [0x8310EEC0..0x8310EED4)
	// 8310EEC0: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EEC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EEC8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310EECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EED0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EED8 size=8
    let mut pc: u32 = 0x8310EED8;
    'dispatch: loop {
        match pc {
            0x8310EED8 => {
    //   block [0x8310EED8..0x8310EEE0)
	// 8310EED8: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310EEE0 size=172
    let mut pc: u32 = 0x8310EEE0;
    'dispatch: loop {
        match pc {
            0x8310EEE0 => {
    //   block [0x8310EEE0..0x8310EF8C)
	// 8310EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310EEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310EEE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310EEEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310EEF0: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8310EEF4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8310EEF8: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 8310EEFC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 8310EF00: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8310EF04: 4198006C  blt cr6, 0x8310ef70
	if ctx.cr[6].lt {
	pc = 0x8310EF70; continue 'dispatch;
	}
	// 8310EF08: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 8310EF0C: 40980064  bge cr6, 0x8310ef70
	if !ctx.cr[6].lt {
	pc = 0x8310EF70; continue 'dispatch;
	}
	// 8310EF10: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8310EF14: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EF18: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8310EF1C: 396BB108  addi r11, r11, -0x4ef8
	ctx.r[11].s64 = ctx.r[11].s64 + -20216;
	// 8310EF20: 409A0024  bne cr6, 0x8310ef44
	if !ctx.cr[6].eq {
	pc = 0x8310EF44; continue 'dispatch;
	}
	// 8310EF24: 5489103A  slwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8310EF28: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8310EF2C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EF30: 7C89582E  lwzx r4, r9, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8310EF34: 816A0070  lwz r11, 0x70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(112 as u32) ) } as u64;
	// 8310EF38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EF3C: 4E800421  bctrl
	ctx.lr = 0x8310EF40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EF40: 48000038  b 0x8310ef78
	pc = 0x8310EF78; continue 'dispatch;
	// 8310EF44: 5484103A  slwi r4, r4, 2
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8310EF48: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EF4C: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8310EF50: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 8310EF54: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8310EF58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310EF5C: 7C84582E  lwzx r4, r4, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8310EF60: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8310EF64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EF68: 4E800421  bctrl
	ctx.lr = 0x8310EF6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310EF6C: 4800000C  b 0x8310ef78
	pc = 0x8310EF78; continue 'dispatch;
	// 8310EF70: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310EF74: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310EF78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310EF84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310EF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EF90 size=16
    let mut pc: u32 = 0x8310EF90;
    'dispatch: loop {
        match pc {
            0x8310EF90 => {
    //   block [0x8310EF90..0x8310EFA0)
	// 8310EF90: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8310EF94: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8310EF98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8310EF9C: 4BFFFF44  b 0x8310eee0
	sub_8310EEE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EFA0 size=16
    let mut pc: u32 = 0x8310EFA0;
    'dispatch: loop {
        match pc {
            0x8310EFA0 => {
    //   block [0x8310EFA0..0x8310EFB0)
	// 8310EFA0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8310EFA4: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8310EFA8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8310EFAC: 4BFFFF34  b 0x8310eee0
	sub_8310EEE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310EFB0 size=40
    let mut pc: u32 = 0x8310EFB0;
    'dispatch: loop {
        match pc {
            0x8310EFB0 => {
    //   block [0x8310EFB0..0x8310EFD8)
	// 8310EFB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8310EFB4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8310EFB8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8310EFBC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8310EFC0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8310EFC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EFC8: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 8310EFCC: 816B00D8  lwz r11, 0xd8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 8310EFD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310EFD4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310EFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310EFD8 size=112
    let mut pc: u32 = 0x8310EFD8;
    'dispatch: loop {
        match pc {
            0x8310EFD8 => {
    //   block [0x8310EFD8..0x8310F048)
	// 8310EFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310EFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310EFE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310EFE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310EFE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310EFEC: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310EFF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310EFF4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8310EFF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310EFFC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F004: 4E800421  bctrl
	ctx.lr = 0x8310F008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F008: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8310F00C: 40820010  bne 0x8310f01c
	if !ctx.cr[0].eq {
	pc = 0x8310F01C; continue 'dispatch;
	}
	// 8310F010: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310F014: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310F018: 48000018  b 0x8310f030
	pc = 0x8310F030; continue 'dispatch;
	// 8310F01C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F020: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310F024: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8310F028: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F02C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8310F030: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310F034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310F038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310F03C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310F040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310F044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8310F048 size=12
    let mut pc: u32 = 0x8310F048;
    'dispatch: loop {
        match pc {
            0x8310F048 => {
    //   block [0x8310F048..0x8310F054)
	// 8310F048: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8310F04C: C02B9450  lfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8310F050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310F058 size=100
    let mut pc: u32 = 0x8310F058;
    'dispatch: loop {
        match pc {
            0x8310F058 => {
    //   block [0x8310F058..0x8310F0BC)
	// 8310F058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310F060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310F064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310F068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F06C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310F070: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F074: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8310F078: 815E014C  lwz r10, 0x14c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310F07C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310F080: 83EA0000  lwz r31, 0(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F088: 4E800421  bctrl
	ctx.lr = 0x8310F08C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F08C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8310F090: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 8310F094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310F098: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8310F09C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F0A0: 4E800421  bctrl
	ctx.lr = 0x8310F0A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F0A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310F0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310F0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310F0B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310F0B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310F0B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310F0C0 size=248
    let mut pc: u32 = 0x8310F0C0;
    'dispatch: loop {
        match pc {
            0x8310F0C0 => {
    //   block [0x8310F0C0..0x8310F1B8)
	// 8310F0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310F0C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310F0CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F0D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8310F0D4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F0D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310F0DC: 5567863E  rlwinm r7, r11, 0x10, 0x18, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8310F0E0: 5566C63E  rlwinm r6, r11, 0x18, 0x18, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310F0E4: 5568463E  srwi r8, r11, 0x18
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(24);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8310F0E8: C00A94AC  lfs f0, -0x6b54(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27476 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310F0EC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8310F0F0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8310F0F4: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8310F0F8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8310F0FC: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 8310F100: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8310F104: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8310F108: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8310F10C: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8310F110: F8E10050  std r7, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u64 ) };
	// 8310F114: C9410060  lfd f10, 0x60(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8310F118: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8310F11C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8310F120: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 8310F124: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F128: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8310F12C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310F130: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8310F134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F138: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8310F13C: 81290018  lwz r9, 0x18(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310F140: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 8310F144: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8310F148: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310F14C: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8310F150: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310F154: D1410068  stfs f10, 0x68(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8310F158: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310F15C: D1610064  stfs f11, 0x64(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8310F160: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8310F164: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8310F168: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8310F16C: 4E800421  bctrl
	ctx.lr = 0x8310F170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F170: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F174: 41800030  blt 0x8310f1a4
	if ctx.cr[0].lt {
	pc = 0x8310F1A4; continue 'dispatch;
	}
	// 8310F178: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8310F180: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8310F184: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8310F188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F18C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8310F190: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F194: 4E800421  bctrl
	ctx.lr = 0x8310F198;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F198: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F19C: 41800008  blt 0x8310f1a4
	if ctx.cr[0].lt {
	pc = 0x8310F1A4; continue 'dispatch;
	}
	// 8310F1A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310F1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310F1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310F1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310F1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310F1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310F1B8 size=184
    let mut pc: u32 = 0x8310F1B8;
    'dispatch: loop {
        match pc {
            0x8310F1B8 => {
    //   block [0x8310F1B8..0x8310F270)
	// 8310F1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310F1C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310F1C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310F1C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F1CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310F1D0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F1D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310F1D8: 419A0014  beq cr6, 0x8310f1ec
	if ctx.cr[6].eq {
	pc = 0x8310F1EC; continue 'dispatch;
	}
	// 8310F1DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F1E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F1E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F1E8: 4E800421  bctrl
	ctx.lr = 0x8310F1EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F1EC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F1F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310F1F4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8310F1F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310F1FC: 419A0034  beq cr6, 0x8310f230
	if ctx.cr[6].eq {
	pc = 0x8310F230; continue 'dispatch;
	}
	// 8310F200: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F208: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F20C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F210: 4E800421  bctrl
	ctx.lr = 0x8310F214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F214: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F218: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F21C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8310F220: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F228: 4E800421  bctrl
	ctx.lr = 0x8310F22C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F22C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8310F230: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F234: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310F238: 419A0014  beq cr6, 0x8310f24c
	if ctx.cr[6].eq {
	pc = 0x8310F24C; continue 'dispatch;
	}
	// 8310F23C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F240: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310F244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F248: 4E800421  bctrl
	ctx.lr = 0x8310F24C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F24C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8310F250: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8310F254: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8310F258: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310F25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310F260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310F264: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310F268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310F26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310F270 size=332
    let mut pc: u32 = 0x8310F270;
    'dispatch: loop {
        match pc {
            0x8310F270 => {
    //   block [0x8310F270..0x8310F3BC)
	// 8310F270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F274: 48098EF5  bl 0x831a8168
	ctx.lr = 0x8310F278;
	sub_831A8130(ctx, base);
	// 8310F278: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F27C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310F280: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8310F284: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8310F288: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8310F28C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F290: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F294: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F29C: 4E800421  bctrl
	ctx.lr = 0x8310F2A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F2A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F2A4: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F2A8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310F2AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F2B0: 4E800421  bctrl
	ctx.lr = 0x8310F2B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F2B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F2B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F2BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8310F2C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310F2C4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310F2C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F2CC: 4E800421  bctrl
	ctx.lr = 0x8310F2D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F2D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F2D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310F2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F2DC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310F2E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F2E4: 4E800421  bctrl
	ctx.lr = 0x8310F2E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F2E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F2EC: 418000C8  blt 0x8310f3b4
	if ctx.cr[0].lt {
	pc = 0x8310F3B4; continue 'dispatch;
	}
	// 8310F2F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8310F2F4: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8310F2F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310F2FC: 48099215  bl 0x831a8510
	ctx.lr = 0x8310F300;
	sub_831A8510(ctx, base);
	// 8310F300: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8310F304: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8310F308: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F30C: C181007C  lfs f12, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310F310: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310F314: C161008C  lfs f11, 0x8c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310F318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F31C: D1810054  stfs f12, 0x54(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310F320: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310F324: D1A1009C  stfs f13, 0x9c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8310F328: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310F32C: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F330: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8310F334: D1610058  stfs f11, 0x58(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8310F338: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8310F33C: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8310F340: C1A1006C  lfs f13, 0x6c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310F344: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310F348: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8310F34C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F350: 4E800421  bctrl
	ctx.lr = 0x8310F354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F354: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F358: 4180005C  blt 0x8310f3b4
	if ctx.cr[0].lt {
	pc = 0x8310F3B4; continue 'dispatch;
	}
	// 8310F35C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F360: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8310F364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F368: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8310F36C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F370: 4E800421  bctrl
	ctx.lr = 0x8310F374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F374: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F378: 4180003C  blt 0x8310f3b4
	if ctx.cr[0].lt {
	pc = 0x8310F3B4; continue 'dispatch;
	}
	// 8310F37C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F380: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8310F384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F388: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 8310F38C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F390: 4E800421  bctrl
	ctx.lr = 0x8310F394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F394: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F398: 4180001C  blt 0x8310f3b4
	if ctx.cr[0].lt {
	pc = 0x8310F3B4; continue 'dispatch;
	}
	// 8310F39C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F3A4: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F3A8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310F3AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F3B0: 4E800421  bctrl
	ctx.lr = 0x8310F3B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F3B4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8310F3B8: 48098E00  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310F3C0 size=364
    let mut pc: u32 = 0x8310F3C0;
    'dispatch: loop {
        match pc {
            0x8310F3C0 => {
    //   block [0x8310F3C0..0x8310F52C)
	// 8310F3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F3C4: 48098DA1  bl 0x831a8164
	ctx.lr = 0x8310F3C8;
	sub_831A8130(ctx, base);
	// 8310F3C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F3CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310F3D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310F3D4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8310F3D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F3DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F3E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F3E4: 4E800421  bctrl
	ctx.lr = 0x8310F3E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F3E8: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8310F3EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F3F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F3F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F3FC: 4E800421  bctrl
	ctx.lr = 0x8310F400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F400: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8310F404: 814B5984  lwz r10, 0x5984(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22916 as u32) ) } as u64;
	// 8310F408: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8310F40C: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310F410: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 8310F414: 4082000C  bne 0x8310f420
	if !ctx.cr[0].eq {
	pc = 0x8310F420; continue 'dispatch;
	}
	// 8310F418: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8310F41C: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 8310F420: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8310F424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F428: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F42C: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F434: 4E800421  bctrl
	ctx.lr = 0x8310F438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F43C: 3BBE0014  addi r29, r30, 0x14
	ctx.r[29].s64 = ctx.r[30].s64 + 20;
	// 8310F440: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8310F444: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310F448: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310F44C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310F450: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F454: 4E800421  bctrl
	ctx.lr = 0x8310F458;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F458: 3B7E0010  addi r27, r30, 0x10
	ctx.r[27].s64 = ctx.r[30].s64 + 16;
	// 8310F45C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310F460: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8310F464: 408200AC  bne 0x8310f510
	if !ctx.cr[0].eq {
	pc = 0x8310F510; continue 'dispatch;
	}
	// 8310F468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F46C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8310F470: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8310F474: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310F478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F47C: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 8310F480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F484: 4E800421  bctrl
	ctx.lr = 0x8310F488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F488: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F48C: 41800098  blt 0x8310f524
	if ctx.cr[0].lt {
	pc = 0x8310F524; continue 'dispatch;
	}
	// 8310F490: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F498: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F49C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F4A0: 4E800421  bctrl
	ctx.lr = 0x8310F4A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F4A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F4A8: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F4AC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8310F4B0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8310F4B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310F4B8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310F4BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F4C0: 4E800421  bctrl
	ctx.lr = 0x8310F4C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F4C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8310F4C8: 409A004C  bne cr6, 0x8310f514
	if !ctx.cr[6].eq {
	pc = 0x8310F514; continue 'dispatch;
	}
	// 8310F4CC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F4D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F4D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F4D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F4DC: 4E800421  bctrl
	ctx.lr = 0x8310F4E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F4E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F4E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F4E8: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F4EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F4F0: 4E800421  bctrl
	ctx.lr = 0x8310F4F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F4F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F4F8: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F4FC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8310F500: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310F504: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F50C: 4E800421  bctrl
	ctx.lr = 0x8310F510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F510: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310F514: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8310F518: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8310F51C: 915E005C  stw r10, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8310F520: 917E0058  stw r11, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8310F524: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8310F528: 48098C8C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310F530 size=356
    let mut pc: u32 = 0x8310F530;
    'dispatch: loop {
        match pc {
            0x8310F530 => {
    //   block [0x8310F530..0x8310F694)
	// 8310F530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F534: 48098C31  bl 0x831a8164
	ctx.lr = 0x8310F538;
	sub_831A8130(ctx, base);
	// 8310F538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F53C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310F540: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310F544: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8310F548: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8310F54C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F550: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F558: 4E800421  bctrl
	ctx.lr = 0x8310F55C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F55C: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8310F560: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F568: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F56C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F570: 4E800421  bctrl
	ctx.lr = 0x8310F574;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F574: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8310F578: 814B5984  lwz r10, 0x5984(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22916 as u32) ) } as u64;
	// 8310F57C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8310F580: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310F584: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 8310F588: 4082000C  bne 0x8310f594
	if !ctx.cr[0].eq {
	pc = 0x8310F594; continue 'dispatch;
	}
	// 8310F58C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8310F590: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 8310F594: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8310F598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F59C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F5A0: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F5A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F5A8: 4E800421  bctrl
	ctx.lr = 0x8310F5AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F5AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F5B0: 3BBE0014  addi r29, r30, 0x14
	ctx.r[29].s64 = ctx.r[30].s64 + 20;
	// 8310F5B4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8310F5B8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8310F5BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310F5C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F5C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F5C8: 4E800421  bctrl
	ctx.lr = 0x8310F5CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F5CC: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 8310F5D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310F5D4: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8310F5D8: 408200B0  bne 0x8310f688
	if !ctx.cr[0].eq {
	pc = 0x8310F688; continue 'dispatch;
	}
	// 8310F5DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F5E0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8310F5E4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8310F5E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8310F5EC: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 8310F5F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F5F4: 816B00D8  lwz r11, 0xd8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 8310F5F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F5FC: 4E800421  bctrl
	ctx.lr = 0x8310F600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F600: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F604: 41800088  blt 0x8310f68c
	if ctx.cr[0].lt {
	pc = 0x8310F68C; continue 'dispatch;
	}
	// 8310F608: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F60C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F610: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F614: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F618: 4E800421  bctrl
	ctx.lr = 0x8310F61C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F61C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F620: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F624: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8310F628: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8310F62C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8310F630: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8310F634: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F638: 4E800421  bctrl
	ctx.lr = 0x8310F63C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F63C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8310F640: 409A004C  bne cr6, 0x8310f68c
	if !ctx.cr[6].eq {
	pc = 0x8310F68C; continue 'dispatch;
	}
	// 8310F644: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F64C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F654: 4E800421  bctrl
	ctx.lr = 0x8310F658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F658: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F65C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F660: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F668: 4E800421  bctrl
	ctx.lr = 0x8310F66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F66C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F670: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8310F678: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8310F67C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F684: 4E800421  bctrl
	ctx.lr = 0x8310F688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F688: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310F68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310F690: 48098B24  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310F698 size=328
    let mut pc: u32 = 0x8310F698;
    'dispatch: loop {
        match pc {
            0x8310F698 => {
    //   block [0x8310F698..0x8310F7E0)
	// 8310F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F69C: 48098AD1  bl 0x831a816c
	ctx.lr = 0x8310F6A0;
	sub_831A8130(ctx, base);
	// 8310F6A0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F6A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310F6A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8310F6AC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8310F6B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F6B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F6B8: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F6BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F6C0: 4E800421  bctrl
	ctx.lr = 0x8310F6C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F6C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F6C8: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F6CC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310F6D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F6D4: 4E800421  bctrl
	ctx.lr = 0x8310F6D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F6D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F6DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F6E0: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 8310F6E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310F6E8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310F6EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F6F0: 4E800421  bctrl
	ctx.lr = 0x8310F6F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F6F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F6F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310F6FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F700: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310F704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F708: 4E800421  bctrl
	ctx.lr = 0x8310F70C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F70C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F710: 418000C8  blt 0x8310f7d8
	if ctx.cr[0].lt {
	pc = 0x8310F7D8; continue 'dispatch;
	}
	// 8310F714: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8310F718: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8310F71C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310F720: 48098DF1  bl 0x831a8510
	ctx.lr = 0x8310F724;
	sub_831A8510(ctx, base);
	// 8310F724: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8310F728: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8310F72C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F730: C181007C  lfs f12, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310F734: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310F738: C161008C  lfs f11, 0x8c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310F73C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F740: D1810054  stfs f12, 0x54(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310F744: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310F748: D1A1009C  stfs f13, 0x9c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8310F74C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310F750: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F754: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8310F758: D1610058  stfs f11, 0x58(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8310F75C: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8310F760: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8310F764: C1A1006C  lfs f13, 0x6c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310F768: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310F76C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8310F770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F774: 4E800421  bctrl
	ctx.lr = 0x8310F778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F778: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F77C: 4180005C  blt 0x8310f7d8
	if ctx.cr[0].lt {
	pc = 0x8310F7D8; continue 'dispatch;
	}
	// 8310F780: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F784: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8310F788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F78C: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8310F790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F794: 4E800421  bctrl
	ctx.lr = 0x8310F798;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F798: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F79C: 4180003C  blt 0x8310f7d8
	if ctx.cr[0].lt {
	pc = 0x8310F7D8; continue 'dispatch;
	}
	// 8310F7A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F7A4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8310F7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F7AC: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 8310F7B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F7B4: 4E800421  bctrl
	ctx.lr = 0x8310F7B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F7B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F7BC: 4180001C  blt 0x8310f7d8
	if ctx.cr[0].lt {
	pc = 0x8310F7D8; continue 'dispatch;
	}
	// 8310F7C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F7C8: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F7CC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310F7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F7D4: 4E800421  bctrl
	ctx.lr = 0x8310F7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F7D8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8310F7DC: 480989E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310F7E0 size=176
    let mut pc: u32 = 0x8310F7E0;
    'dispatch: loop {
        match pc {
            0x8310F7E0 => {
    //   block [0x8310F7E0..0x8310F890)
	// 8310F7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310F7E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310F7EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310F7F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F7F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310F7F8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F7FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310F800: 419A0014  beq cr6, 0x8310f814
	if ctx.cr[6].eq {
	pc = 0x8310F814; continue 'dispatch;
	}
	// 8310F804: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F808: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F80C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F810: 4E800421  bctrl
	ctx.lr = 0x8310F814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F814: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F818: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8310F81C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8310F820: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310F824: 419A0034  beq cr6, 0x8310f858
	if ctx.cr[6].eq {
	pc = 0x8310F858; continue 'dispatch;
	}
	// 8310F828: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F82C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F830: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F838: 4E800421  bctrl
	ctx.lr = 0x8310F83C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F83C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F840: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F844: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8310F848: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310F84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F850: 4E800421  bctrl
	ctx.lr = 0x8310F854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F854: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8310F858: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F85C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310F860: 419A0014  beq cr6, 0x8310f874
	if ctx.cr[6].eq {
	pc = 0x8310F874; continue 'dispatch;
	}
	// 8310F864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F868: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310F86C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F870: 4E800421  bctrl
	ctx.lr = 0x8310F874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F874: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8310F878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310F87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310F880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310F884: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310F888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310F88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310F890 size=256
    let mut pc: u32 = 0x8310F890;
    'dispatch: loop {
        match pc {
            0x8310F890 => {
    //   block [0x8310F890..0x8310F990)
	// 8310F890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F894: 480988D1  bl 0x831a8164
	ctx.lr = 0x8310F898;
	sub_831A8130(ctx, base);
	// 8310F898: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F89C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310F8A0: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 8310F8A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310F8A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8310F8AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8310F8B0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8310F8B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F8B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F8BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F8C0: 4E800421  bctrl
	ctx.lr = 0x8310F8C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F8C4: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8310F8C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F8CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F8D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310F8D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F8D8: 4E800421  bctrl
	ctx.lr = 0x8310F8DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F8DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F8E4: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F8E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F8EC: 4E800421  bctrl
	ctx.lr = 0x8310F8F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F8F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F8F4: 3B7E0014  addi r27, r30, 0x14
	ctx.r[27].s64 = ctx.r[30].s64 + 20;
	// 8310F8F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8310F8FC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8310F900: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8310F904: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310F908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F90C: 4E800421  bctrl
	ctx.lr = 0x8310F910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F910: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 8310F914: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310F918: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8310F91C: 40820068  bne 0x8310f984
	if !ctx.cr[0].eq {
	pc = 0x8310F984; continue 'dispatch;
	}
	// 8310F920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F924: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 8310F928: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8310F92C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8310F930: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8310F934: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 8310F938: 816B00DC  lwz r11, 0xdc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(220 as u32) ) } as u64;
	// 8310F93C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F940: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F944: 4E800421  bctrl
	ctx.lr = 0x8310F948;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F948: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310F94C: 4180003C  blt 0x8310f988
	if ctx.cr[0].lt {
	pc = 0x8310F988; continue 'dispatch;
	}
	// 8310F950: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F958: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F95C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F960: 4E800421  bctrl
	ctx.lr = 0x8310F964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F964: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F968: 80DB0000  lwz r6, 0(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F96C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8310F970: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8310F974: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8310F978: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8310F97C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F980: 4E800421  bctrl
	ctx.lr = 0x8310F984;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F984: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310F988: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8310F98C: 48098828  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310F990 size=8
    let mut pc: u32 = 0x8310F990;
    'dispatch: loop {
        match pc {
            0x8310F990 => {
    //   block [0x8310F990..0x8310F998)
	// 8310F990: 38630018  addi r3, r3, 0x18
	ctx.r[3].s64 = ctx.r[3].s64 + 24;
	// 8310F994: 48008864  b 0x831181f8
	sub_831181F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310F998 size=20
    let mut pc: u32 = 0x8310F998;
    'dispatch: loop {
        match pc {
            0x8310F998 => {
    //   block [0x8310F998..0x8310F9AC)
	// 8310F998: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F99C: 38C30018  addi r6, r3, 0x18
	ctx.r[6].s64 = ctx.r[3].s64 + 24;
	// 8310F9A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310F9A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F9A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310F9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310F9B0 size=344
    let mut pc: u32 = 0x8310F9B0;
    'dispatch: loop {
        match pc {
            0x8310F9B0 => {
    //   block [0x8310F9B0..0x8310FB08)
	// 8310F9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310F9B4: 480987B9  bl 0x831a816c
	ctx.lr = 0x8310F9B8;
	sub_831A8130(ctx, base);
	// 8310F9B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310F9BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8310F9C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8310F9C4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8310F9C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F9CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F9D0: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8310F9D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F9D8: 4E800421  bctrl
	ctx.lr = 0x8310F9DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F9DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F9E0: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310F9E4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8310F9E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310F9EC: 4E800421  bctrl
	ctx.lr = 0x8310F9F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310F9F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310F9F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310F9F8: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 8310F9FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310FA00: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8310FA04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FA08: 4E800421  bctrl
	ctx.lr = 0x8310FA0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FA0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FA10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310FA14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310FA18: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FA1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FA20: 4E800421  bctrl
	ctx.lr = 0x8310FA24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FA24: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310FA28: 418000D8  blt 0x8310fb00
	if ctx.cr[0].lt {
	pc = 0x8310FB00; continue 'dispatch;
	}
	// 8310FA2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FA30: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8310FA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310FA38: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310FA3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FA40: 4E800421  bctrl
	ctx.lr = 0x8310FA44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FA44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310FA48: 418000B8  blt 0x8310fb00
	if ctx.cr[0].lt {
	pc = 0x8310FB00; continue 'dispatch;
	}
	// 8310FA4C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8310FA50: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8310FA54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8310FA58: 48098AB9  bl 0x831a8510
	ctx.lr = 0x8310FA5C;
	sub_831A8510(ctx, base);
	// 8310FA5C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8310FA60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8310FA64: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FA68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8310FA6C: C161007C  lfs f11, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310FA70: C141008C  lfs f10, 0x8c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8310FA74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8310FA78: D1410058  stfs f10, 0x58(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8310FA7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310FA80: C18808A8  lfs f12, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FA84: D181009C  stfs f12, 0x9c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8310FA88: C1AB9450  lfs f13, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310FA8C: 816900B0  lwz r11, 0xb0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(176 as u32) ) } as u64;
	// 8310FA90: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FA94: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8310FA98: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8310FA9C: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8310FAA0: C181006C  lfs f12, 0x6c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FAA4: ED8C6828  fsubs f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310FAA8: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310FAAC: EDAB6828  fsubs f13, f11, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310FAB0: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310FAB4: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8310FAB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FABC: 4E800421  bctrl
	ctx.lr = 0x8310FAC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FAC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310FAC4: 4180003C  blt 0x8310fb00
	if ctx.cr[0].lt {
	pc = 0x8310FB00; continue 'dispatch;
	}
	// 8310FAC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FACC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8310FAD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310FAD4: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 8310FAD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FADC: 4E800421  bctrl
	ctx.lr = 0x8310FAE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FAE0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310FAE4: 4180001C  blt 0x8310fb00
	if ctx.cr[0].lt {
	pc = 0x8310FB00; continue 'dispatch;
	}
	// 8310FAE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FAEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8310FAF0: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FAF4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8310FAF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FAFC: 4E800421  bctrl
	ctx.lr = 0x8310FB00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FB00: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8310FB04: 480986B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FB08 size=12
    let mut pc: u32 = 0x8310FB08;
    'dispatch: loop {
        match pc {
            0x8310FB08 => {
    //   block [0x8310FB08..0x8310FB14)
	// 8310FB08: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8310FB0C: 806BDEBC  lwz r3, -0x2144(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8516 as u32) ) } as u64;
	// 8310FB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FB18 size=16
    let mut pc: u32 = 0x8310FB18;
    'dispatch: loop {
        match pc {
            0x8310FB18 => {
    //   block [0x8310FB18..0x8310FB28)
	// 8310FB18: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8310FB1C: 816BDEB8  lwz r11, -0x2148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8520 as u32) ) } as u64;
	// 8310FB20: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8310FB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FB28 size=20
    let mut pc: u32 = 0x8310FB28;
    'dispatch: loop {
        match pc {
            0x8310FB28 => {
    //   block [0x8310FB28..0x8310FB3C)
	// 8310FB28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310FB2C: 409A0010  bne cr6, 0x8310fb3c
	if !ctx.cr[6].eq {
		sub_8310FB3C(ctx, base);
		return;
	}
	// 8310FB30: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310FB34: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310FB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FB3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FB3C size=16
    let mut pc: u32 = 0x8310FB3C;
    'dispatch: loop {
        match pc {
            0x8310FB3C => {
    //   block [0x8310FB3C..0x8310FB4C)
	// 8310FB3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FB40: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8310FB44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FB48: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310FB50 size=168
    let mut pc: u32 = 0x8310FB50;
    'dispatch: loop {
        match pc {
            0x8310FB50 => {
    //   block [0x8310FB50..0x8310FBF8)
	// 8310FB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310FB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310FB58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8310FB5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310FB60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310FB64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310FB68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8310FB6C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310FB70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310FB74: 409A000C  bne cr6, 0x8310fb80
	if !ctx.cr[6].eq {
	pc = 0x8310FB80; continue 'dispatch;
	}
	// 8310FB78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310FB7C: 48000064  b 0x8310fbe0
	pc = 0x8310FBE0; continue 'dispatch;
	// 8310FB80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FB84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310FB88: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310FB8C: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8310FB90: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8310FB94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FB98: 4E800421  bctrl
	ctx.lr = 0x8310FB9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FB9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FBA0: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8310FBA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8310FBA8: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310FBAC: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310FBB0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8310FBB4: 80BF0024  lwz r5, 0x24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8310FBB8: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8310FBBC: 409A0014  bne cr6, 0x8310fbd0
	if !ctx.cr[6].eq {
	pc = 0x8310FBD0; continue 'dispatch;
	}
	// 8310FBC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8310FBC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8310FBC8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8310FBCC: 4800000C  b 0x8310fbd8
	pc = 0x8310FBD8; continue 'dispatch;
	// 8310FBD0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8310FBD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8310FBD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FBDC: 4E800421  bctrl
	ctx.lr = 0x8310FBE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FBE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8310FBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310FBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310FBEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8310FBF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310FBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310FBF8 size=612
    let mut pc: u32 = 0x8310FBF8;
    'dispatch: loop {
        match pc {
            0x8310FBF8 => {
    //   block [0x8310FBF8..0x8310FE5C)
	// 8310FBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310FBFC: 4809855D  bl 0x831a8158
	ctx.lr = 0x8310FC00;
	sub_831A8130(ctx, base);
	// 8310FC00: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8310FC04: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310FC08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8310FC0C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8310FC10: 831E0008  lwz r24, 8(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310FC14: 837E0004  lwz r27, 4(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310FC18: 2F180002  cmpwi cr6, r24, 2
	ctx.cr[6].compare_i32(ctx.r[24].s32, 2, &mut ctx.xer);
	// 8310FC1C: 41980230  blt cr6, 0x8310fe4c
	if ctx.cr[6].lt {
	pc = 0x8310FE4C; continue 'dispatch;
	}
	// 8310FC20: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FC24: 1D780005  mulli r11, r24, 5
	ctx.r[11].s64 = ctx.r[24].s64 * 5;
	// 8310FC28: 3BEBFFFB  addi r31, r11, -5
	ctx.r[31].s64 = ctx.r[11].s64 + -5;
	// 8310FC2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8310FC30: 409A0030  bne cr6, 0x8310fc60
	if !ctx.cr[6].eq {
	pc = 0x8310FC60; continue 'dispatch;
	}
	// 8310FC34: 57E31838  slwi r3, r31, 3
	ctx.r[3].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8310FC38: 4BFCD879  bl 0x830dd4b0
	ctx.lr = 0x8310FC3C;
	sub_830DD4B0(ctx, base);
	// 8310FC3C: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8310FC40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8310FC44: 40820010  bne 0x8310fc54
	if !ctx.cr[0].eq {
	pc = 0x8310FC54; continue 'dispatch;
	}
	// 8310FC48: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310FC4C: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 8310FC50: 48000200  b 0x8310fe50
	pc = 0x8310FE50; continue 'dispatch;
	// 8310FC54: 397FFFFE  addi r11, r31, -2
	ctx.r[11].s64 = ctx.r[31].s64 + -2;
	// 8310FC58: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 8310FC5C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8310FC60: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8310FC64: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8310FC68: 409901E4  ble cr6, 0x8310fe4c
	if !ctx.cr[6].gt {
	pc = 0x8310FE4C; continue 'dispatch;
	}
	// 8310FC6C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8310FC70: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310FC74: 3B3A0001  addi r25, r26, 1
	ctx.r[25].s64 = ctx.r[26].s64 + 1;
	// 8310FC78: 574A1838  slwi r10, r26, 3
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8310FC7C: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 8310FC80: 572B083E  rotlwi r11, r25, 1
	ctx.r[11].u64 = ((ctx.r[25].u32).rotate_left(1)) as u64;
	// 8310FC84: 7D194BD6  divw r8, r25, r9
	ctx.r[8].s32 = ctx.r[25].s32 / ctx.r[9].s32;
	// 8310FC88: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 8310FC8C: 7D6849D6  mullw r11, r8, r9
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8310FC90: 7C0ADC2E  lfsx f0, r10, r27
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FC94: 7D6BC850  subf r11, r11, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[11].s64;
	// 8310FC98: 7FAADA14  add r29, r10, r27
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 8310FC9C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8310FCA0: 7D2A3878  andc r10, r9, r7
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[7].u64;
	// 8310FCA4: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8310FCA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8310FCAC: C1BD0004  lfs f13, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310FCB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8310FCB4: 7D8BDC2E  lfsx f12, r11, r27
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FCB8: 0CC90000  twi 6, r9, 0
	// 8310FCBC: EC0C0028  fsubs f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8310FCC0: 0CAAFFFF  twi 5, r10, -1
	// 8310FCC4: C19F0004  lfs f12, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FCC8: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8310FCCC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310FCD0: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310FCD4: 48008235  bl 0x83117f08
	ctx.lr = 0x8310FCD8;
	sub_83117F08(ctx, base);
	// 8310FCD8: C1A10050  lfs f13, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310FCDC: EDAD07F2  fmuls f13, f13, f31
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 8310FCE0: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FCE4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FCE8: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8310FCEC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8310FCF0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8310FCF4: 395C0008  addi r10, r28, 8
	ctx.r[10].s64 = ctx.r[28].s64 + 8;
	// 8310FCF8: 7D7C5A14  add r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 8310FCFC: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 8310FD00: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8310FD04: 419A00B8  beq cr6, 0x8310fdbc
	if ctx.cr[6].eq {
	pc = 0x8310FDBC; continue 'dispatch;
	}
	// 8310FD08: C19D0000  lfs f12, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FD0C: 3938FFFF  addi r9, r24, -1
	ctx.r[9].s64 = ctx.r[24].s64 + -1;
	// 8310FD10: C17D0004  lfs f11, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310FD14: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 8310FD18: EDAB682A  fadds f13, f11, f13
	ctx.f[13].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 8310FD1C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FD20: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FD24: 7F1A4800  cmpw cr6, r26, r9
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8310FD28: 419A0124  beq cr6, 0x8310fe4c
	if ctx.cr[6].eq {
	pc = 0x8310FE4C; continue 'dispatch;
	}
	// 8310FD2C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FD30: C1810050  lfs f12, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FD34: C1610054  lfs f11, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310FD38: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8310FD3C: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FD40: C1BD0004  lfs f13, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310FD44: EC00602A  fadds f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8310FD48: EDAD582A  fadds f13, f13, f11
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64;
	// 8310FD4C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8310FD50: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FD54: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FD58: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FD5C: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FD60: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8310FD64: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FD68: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8310FD6C: C01D0004  lfs f0, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FD70: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FD74: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FD78: C19F0004  lfs f12, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FD7C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8310FD80: C1610054  lfs f11, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310FD84: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FD88: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310FD8C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8310FD90: ED8C582A  fadds f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 8310FD94: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FD98: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FD9C: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 8310FDA0: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FDA4: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FDA8: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8310FDAC: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 8310FDB0: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FDB4: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FDB8: 48000084  b 0x8310fe3c
	pc = 0x8310FE3C; continue 'dispatch;
	// 8310FDBC: C01B0000  lfs f0, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FDC0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FDC4: C01B0004  lfs f0, 4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FDC8: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FDCC: C1610054  lfs f11, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310FDD0: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FDD4: C19B0004  lfs f12, 4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FDD8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8310FDDC: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310FDE0: ED8C582A  fadds f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 8310FDE4: C01B0000  lfs f0, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FDE8: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8310FDEC: D18B0004  stfs f12, 4(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FDF0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FDF4: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 8310FDF8: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FDFC: 7D4B4A14  add r10, r11, r9
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8310FE00: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FE04: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8310FE08: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FE0C: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FE10: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FE14: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FE18: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FE1C: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8310FE20: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8310FE24: C19F0004  lfs f12, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8310FE28: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8310FE2C: C1610054  lfs f11, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8310FE30: ED8C582A  fadds f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 8310FE34: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8310FE38: D18A0004  stfs f12, 4(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8310FE3C: 3B8B0008  addi r28, r11, 8
	ctx.r[28].s64 = ctx.r[11].s64 + 8;
	// 8310FE40: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 8310FE44: 7F19C000  cmpw cr6, r25, r24
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[24].s32, &mut ctx.xer);
	// 8310FE48: 4198FE28  blt cr6, 0x8310fc70
	if ctx.cr[6].lt {
	pc = 0x8310FC70; continue 'dispatch;
	}
	// 8310FE4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310FE50: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8310FE54: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8310FE58: 48098350  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8310FE60 size=172
    let mut pc: u32 = 0x8310FE60;
    'dispatch: loop {
        match pc {
            0x8310FE60 => {
    //   block [0x8310FE60..0x8310FF0C)
	// 8310FE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310FE64: 48098309  bl 0x831a816c
	ctx.lr = 0x8310FE68;
	sub_831A8130(ctx, base);
	// 8310FE68: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8310FE6C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310FE70: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8310FE74: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8310FE78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8310FE7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8310FE80: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8310FE84: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FE88: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8310FE8C: 41980070  blt cr6, 0x8310fefc
	if ctx.cr[6].lt {
	pc = 0x8310FEFC; continue 'dispatch;
	}
	// 8310FE90: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8310FE94: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8310FE98: 41980064  blt cr6, 0x8310fefc
	if ctx.cr[6].lt {
	pc = 0x8310FEFC; continue 'dispatch;
	}
	// 8310FE9C: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8310FEA0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8310FEA4: 419A0014  beq cr6, 0x8310feb8
	if ctx.cr[6].eq {
	pc = 0x8310FEB8; continue 'dispatch;
	}
	// 8310FEA8: 4BFFFD51  bl 0x8310fbf8
	ctx.lr = 0x8310FEAC;
	sub_8310FBF8(ctx, base);
	// 8310FEAC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8310FEB0: 41800050  blt 0x8310ff00
	if ctx.cr[0].lt {
	pc = 0x8310FF00; continue 'dispatch;
	}
	// 8310FEB4: D3FF0020  stfs f31, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8310FEB8: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8310FEBC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8310FEC0: 409A0010  bne cr6, 0x8310fed0
	if !ctx.cr[6].eq {
	pc = 0x8310FED0; continue 'dispatch;
	}
	// 8310FEC4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8310FEC8: 1D6B0005  mulli r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 * 5;
	// 8310FECC: 38ABFFF8  addi r5, r11, -8
	ctx.r[5].s64 = ctx.r[11].s64 + -8;
	// 8310FED0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8310FED8: 80FF0018  lwz r7, 0x18(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8310FEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8310FEE0: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8310FEE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8310FEE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8310FEEC: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8310FEF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FEF4: 4E800421  bctrl
	ctx.lr = 0x8310FEF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FEF8: 48000008  b 0x8310ff00
	pc = 0x8310FF00; continue 'dispatch;
	// 8310FEFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310FF00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8310FF04: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8310FF08: 480982B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FF10 size=20
    let mut pc: u32 = 0x8310FF10;
    'dispatch: loop {
        match pc {
            0x8310FF10 => {
    //   block [0x8310FF10..0x8310FF24)
	// 8310FF10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310FF14: 409A0010  bne cr6, 0x8310ff24
	if !ctx.cr[6].eq {
		sub_8310FF24(ctx, base);
		return;
	}
	// 8310FF18: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310FF1C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310FF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FF24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FF24 size=16
    let mut pc: u32 = 0x8310FF24;
    'dispatch: loop {
        match pc {
            0x8310FF24 => {
    //   block [0x8310FF24..0x8310FF34)
	// 8310FF24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FF28: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8310FF2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FF30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FF38 size=20
    let mut pc: u32 = 0x8310FF38;
    'dispatch: loop {
        match pc {
            0x8310FF38 => {
    //   block [0x8310FF38..0x8310FF4C)
	// 8310FF38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310FF3C: 409A0010  bne cr6, 0x8310ff4c
	if !ctx.cr[6].eq {
		sub_8310FF4C(ctx, base);
		return;
	}
	// 8310FF40: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8310FF44: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8310FF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FF4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FF4C size=16
    let mut pc: u32 = 0x8310FF4C;
    'dispatch: loop {
        match pc {
            0x8310FF4C => {
    //   block [0x8310FF4C..0x8310FF5C)
	// 8310FF4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FF50: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8310FF54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FF58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8310FF60 size=80
    let mut pc: u32 = 0x8310FF60;
    'dispatch: loop {
        match pc {
            0x8310FF60 => {
    //   block [0x8310FF60..0x8310FFB0)
	// 8310FF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8310FF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8310FF68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8310FF6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8310FF70: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8310FF74: 807FDEC0  lwz r3, -0x2140(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 8310FF78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310FF7C: 419A0014  beq cr6, 0x8310ff90
	if ctx.cr[6].eq {
	pc = 0x8310FF90; continue 'dispatch;
	}
	// 8310FF80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8310FF84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8310FF88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8310FF8C: 4E800421  bctrl
	ctx.lr = 0x8310FF90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8310FF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8310FF94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310FF98: 917FDEC0  stw r11, -0x2140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8512 as u32), ctx.r[11].u32 ) };
	// 8310FF9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8310FFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8310FFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8310FFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8310FFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FFB0 size=28
    let mut pc: u32 = 0x8310FFB0;
    'dispatch: loop {
        match pc {
            0x8310FFB0 => {
    //   block [0x8310FFB0..0x8310FFCC)
	// 8310FFB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8310FFB4: 816BDEC0  lwz r11, -0x2140(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 8310FFB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8310FFBC: 409A0010  bne cr6, 0x8310ffcc
	if !ctx.cr[6].eq {
		sub_8310FFCC(ctx, base);
		return;
	}
	// 8310FFC0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8310FFC4: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8310FFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FFCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FFCC size=28
    let mut pc: u32 = 0x8310FFCC;
    'dispatch: loop {
        match pc {
            0x8310FFCC => {
    //   block [0x8310FFCC..0x8310FFE8)
	// 8310FFCC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8310FFD0: 814ADEC4  lwz r10, -0x213c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8508 as u32) ) } as u64;
	// 8310FFD4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8310FFD8: 409A0010  bne cr6, 0x8310ffe8
	if !ctx.cr[6].eq {
		sub_8310FFE8(ctx, base);
		return;
	}
	// 8310FFDC: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 8310FFE0: 60630030  ori r3, r3, 0x30
	ctx.r[3].u64 = ctx.r[3].u64 | 48;
	// 8310FFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FFE8 size=12
    let mut pc: u32 = 0x8310FFE8;
    'dispatch: loop {
        match pc {
            0x8310FFE8 => {
    //   block [0x8310FFE8..0x8310FFF4)
	// 8310FFE8: 906B000C  stw r3, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8310FFEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8310FFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8310FFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8310FFF8 size=20
    let mut pc: u32 = 0x8310FFF8;
    'dispatch: loop {
        match pc {
            0x8310FFF8 => {
    //   block [0x8310FFF8..0x8311000C)
	// 8310FFF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8310FFFC: 409A0010  bne cr6, 0x8311000c
	if !ctx.cr[6].eq {
		sub_8311000C(ctx, base);
		return;
	}
	// 83110000: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110004: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8311000C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8311000C size=20
    let mut pc: u32 = 0x8311000C;
    'dispatch: loop {
        match pc {
            0x8311000C => {
    //   block [0x8311000C..0x83110020)
	// 8311000C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110010: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83110014: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83110018: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311001C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110020 size=20
    let mut pc: u32 = 0x83110020;
    'dispatch: loop {
        match pc {
            0x83110020 => {
    //   block [0x83110020..0x83110034)
	// 83110020: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110024: 409A0010  bne cr6, 0x83110034
	if !ctx.cr[6].eq {
		sub_83110034(ctx, base);
		return;
	}
	// 83110028: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8311002C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110034 size=16
    let mut pc: u32 = 0x83110034;
    'dispatch: loop {
        match pc {
            0x83110034 => {
    //   block [0x83110034..0x83110044)
	// 83110034: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110038: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8311003C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110040: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110048 size=52
    let mut pc: u32 = 0x83110048;
    'dispatch: loop {
        match pc {
            0x83110048 => {
    //   block [0x83110048..0x8311007C)
	// 83110048: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8311004C: 419A0030  beq cr6, 0x8311007c
	if ctx.cr[6].eq {
		sub_8311007C(ctx, base);
		return;
	}
	// 83110050: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83110054: 419A0028  beq cr6, 0x8311007c
	if ctx.cr[6].eq {
		sub_8311007C(ctx, base);
		return;
	}
	// 83110058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8311005C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110060: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110064: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110068: 419A0014  beq cr6, 0x8311007c
	if ctx.cr[6].eq {
		sub_8311007C(ctx, base);
		return;
	}
	// 8311006C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110070: 816B00E0  lwz r11, 0xe0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(224 as u32) ) } as u64;
	// 83110074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110078: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8311007C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8311007C size=12
    let mut pc: u32 = 0x8311007C;
    'dispatch: loop {
        match pc {
            0x8311007C => {
    //   block [0x8311007C..0x83110088)
	// 8311007C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110080: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110088 size=20
    let mut pc: u32 = 0x83110088;
    'dispatch: loop {
        match pc {
            0x83110088 => {
    //   block [0x83110088..0x8311009C)
	// 83110088: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8311008C: 409A0010  bne cr6, 0x8311009c
	if !ctx.cr[6].eq {
		sub_8311009C(ctx, base);
		return;
	}
	// 83110090: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110094: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8311009C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8311009C size=16
    let mut pc: u32 = 0x8311009C;
    'dispatch: loop {
        match pc {
            0x8311009C => {
    //   block [0x8311009C..0x831100AC)
	// 8311009C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831100A0: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 831100A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831100A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831100B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831100B0 size=176
    let mut pc: u32 = 0x831100B0;
    'dispatch: loop {
        match pc {
            0x831100B0 => {
    //   block [0x831100B0..0x83110160)
	// 831100B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831100B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831100B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831100BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831100C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831100C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831100C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831100CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831100D0: 409A0010  bne cr6, 0x831100e0
	if !ctx.cr[6].eq {
	pc = 0x831100E0; continue 'dispatch;
	}
	// 831100D4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831100D8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831100DC: 48000070  b 0x8311014c
	pc = 0x8311014C; continue 'dispatch;
	// 831100E0: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831100E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831100E8: 806ADEC0  lwz r3, -0x2140(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 831100EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831100F0: 409A0010  bne cr6, 0x83110100
	if !ctx.cr[6].eq {
	pc = 0x83110100; continue 'dispatch;
	}
	// 831100F4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 831100F8: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 831100FC: 48000050  b 0x8311014c
	pc = 0x8311014C; continue 'dispatch;
	// 83110100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110104: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83110108: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8311010C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110110: 4E800421  bctrl
	ctx.lr = 0x83110114;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110114: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83110118: 41800034  blt 0x8311014c
	if ctx.cr[0].lt {
	pc = 0x8311014C; continue 'dispatch;
	}
	// 8311011C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83110120: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83110124: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83110128: 814BDEC8  lwz r10, -0x2138(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 8311012C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83110130: 409A0018  bne cr6, 0x83110148
	if !ctx.cr[6].eq {
	pc = 0x83110148; continue 'dispatch;
	}
	// 83110134: 906BDEC8  stw r3, -0x2138(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8504 as u32), ctx.r[3].u32 ) };
	// 83110138: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311013C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83110140: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110144: 4E800421  bctrl
	ctx.lr = 0x83110148;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8311014C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83110150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8311015C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110160 size=140
    let mut pc: u32 = 0x83110160;
    'dispatch: loop {
        match pc {
            0x83110160 => {
    //   block [0x83110160..0x831101EC)
	// 83110160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8311016C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110174: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83110178: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8311017C: 409A0010  bne cr6, 0x8311018c
	if !ctx.cr[6].eq {
	pc = 0x8311018C; continue 'dispatch;
	}
	// 83110180: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110184: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110188: 4800004C  b 0x831101d4
	pc = 0x831101D4; continue 'dispatch;
	// 8311018C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83110190: 807FDEC8  lwz r3, -0x2138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83110194: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83110198: 409A0024  bne cr6, 0x831101bc
	if !ctx.cr[6].eq {
	pc = 0x831101BC; continue 'dispatch;
	}
	// 8311019C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831101A0: 419A0014  beq cr6, 0x831101b4
	if ctx.cr[6].eq {
	pc = 0x831101B4; continue 'dispatch;
	}
	// 831101A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831101A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831101AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831101B0: 4E800421  bctrl
	ctx.lr = 0x831101B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831101B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831101B8: 917FDEC8  stw r11, -0x2138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8504 as u32), ctx.r[11].u32 ) };
	// 831101BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831101C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831101C4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831101C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831101CC: 4E800421  bctrl
	ctx.lr = 0x831101D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831101D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831101D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831101D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831101DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831101E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831101E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831101E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831101F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831101F0 size=68
    let mut pc: u32 = 0x831101F0;
    'dispatch: loop {
        match pc {
            0x831101F0 => {
    //   block [0x831101F0..0x83110234)
	// 831101F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831101F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831101F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831101FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110200: 409A0010  bne cr6, 0x83110210
	if !ctx.cr[6].eq {
	pc = 0x83110210; continue 'dispatch;
	}
	// 83110204: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110208: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8311020C: 48000018  b 0x83110224
	pc = 0x83110224; continue 'dispatch;
	// 83110210: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110214: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 83110218: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311021C: 4E800421  bctrl
	ctx.lr = 0x83110220;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110220: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83110224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83110228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8311022C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110238 size=128
    let mut pc: u32 = 0x83110238;
    'dispatch: loop {
        match pc {
            0x83110238 => {
    //   block [0x83110238..0x831102B8)
	// 83110238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311023C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110248: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8311024C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83110250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110254: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110258: 409A0010  bne cr6, 0x83110268
	if !ctx.cr[6].eq {
	pc = 0x83110268; continue 'dispatch;
	}
	// 8311025C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110260: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110264: 48000040  b 0x831102a4
	pc = 0x831102A4; continue 'dispatch;
	// 83110268: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311026C: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 83110270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110274: 4E800421  bctrl
	ctx.lr = 0x83110278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110278: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8311027C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83110280: 4182001C  beq 0x8311029c
	if ctx.cr[0].eq {
	pc = 0x8311029C; continue 'dispatch;
	}
	// 83110284: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110288: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311028C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110290: 4E800421  bctrl
	ctx.lr = 0x83110294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110294: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83110298: 4800000C  b 0x831102a4
	pc = 0x831102A4; continue 'dispatch;
	// 8311029C: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 831102A0: 60630030  ori r3, r3, 0x30
	ctx.r[3].u64 = ctx.r[3].u64 | 48;
	// 831102A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831102A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831102AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831102B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831102B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831102B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831102B8 size=20
    let mut pc: u32 = 0x831102B8;
    'dispatch: loop {
        match pc {
            0x831102B8 => {
    //   block [0x831102B8..0x831102CC)
	// 831102B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831102BC: 409A0010  bne cr6, 0x831102cc
	if !ctx.cr[6].eq {
		sub_831102CC(ctx, base);
		return;
	}
	// 831102C0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831102C4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831102C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831102CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831102CC size=16
    let mut pc: u32 = 0x831102CC;
    'dispatch: loop {
        match pc {
            0x831102CC => {
    //   block [0x831102CC..0x831102DC)
	// 831102CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831102D0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 831102D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831102D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831102E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831102E0 size=20
    let mut pc: u32 = 0x831102E0;
    'dispatch: loop {
        match pc {
            0x831102E0 => {
    //   block [0x831102E0..0x831102F4)
	// 831102E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831102E4: 409A0010  bne cr6, 0x831102f4
	if !ctx.cr[6].eq {
		sub_831102F4(ctx, base);
		return;
	}
	// 831102E8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831102EC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831102F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831102F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831102F4 size=16
    let mut pc: u32 = 0x831102F4;
    'dispatch: loop {
        match pc {
            0x831102F4 => {
    //   block [0x831102F4..0x83110304)
	// 831102F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831102F8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 831102FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110300: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110308 size=20
    let mut pc: u32 = 0x83110308;
    'dispatch: loop {
        match pc {
            0x83110308 => {
    //   block [0x83110308..0x8311031C)
	// 83110308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8311030C: 409A0010  bne cr6, 0x8311031c
	if !ctx.cr[6].eq {
		sub_8311031C(ctx, base);
		return;
	}
	// 83110310: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110314: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8311031C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8311031C size=16
    let mut pc: u32 = 0x8311031C;
    'dispatch: loop {
        match pc {
            0x8311031C => {
    //   block [0x8311031C..0x8311032C)
	// 8311031C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110320: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 83110324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110328: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110330 size=20
    let mut pc: u32 = 0x83110330;
    'dispatch: loop {
        match pc {
            0x83110330 => {
    //   block [0x83110330..0x83110344)
	// 83110330: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110334: 409A0010  bne cr6, 0x83110344
	if !ctx.cr[6].eq {
		sub_83110344(ctx, base);
		return;
	}
	// 83110338: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8311033C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110344 size=16
    let mut pc: u32 = 0x83110344;
    'dispatch: loop {
        match pc {
            0x83110344 => {
    //   block [0x83110344..0x83110354)
	// 83110344: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110348: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8311034C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110350: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110358 size=20
    let mut pc: u32 = 0x83110358;
    'dispatch: loop {
        match pc {
            0x83110358 => {
    //   block [0x83110358..0x8311036C)
	// 83110358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8311035C: 409A0010  bne cr6, 0x8311036c
	if !ctx.cr[6].eq {
		sub_8311036C(ctx, base);
		return;
	}
	// 83110360: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110364: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8311036C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8311036C size=16
    let mut pc: u32 = 0x8311036C;
    'dispatch: loop {
        match pc {
            0x8311036C => {
    //   block [0x8311036C..0x8311037C)
	// 8311036C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110370: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 83110374: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110378: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110380 size=20
    let mut pc: u32 = 0x83110380;
    'dispatch: loop {
        match pc {
            0x83110380 => {
    //   block [0x83110380..0x83110394)
	// 83110380: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110384: 409A0010  bne cr6, 0x83110394
	if !ctx.cr[6].eq {
		sub_83110394(ctx, base);
		return;
	}
	// 83110388: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8311038C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110394 size=16
    let mut pc: u32 = 0x83110394;
    'dispatch: loop {
        match pc {
            0x83110394 => {
    //   block [0x83110394..0x831103A4)
	// 83110394: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110398: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8311039C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831103A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831103A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831103A8 size=20
    let mut pc: u32 = 0x831103A8;
    'dispatch: loop {
        match pc {
            0x831103A8 => {
    //   block [0x831103A8..0x831103BC)
	// 831103A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831103AC: 409A0010  bne cr6, 0x831103bc
	if !ctx.cr[6].eq {
		sub_831103BC(ctx, base);
		return;
	}
	// 831103B0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831103B4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831103B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831103BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831103BC size=16
    let mut pc: u32 = 0x831103BC;
    'dispatch: loop {
        match pc {
            0x831103BC => {
    //   block [0x831103BC..0x831103CC)
	// 831103BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831103C0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 831103C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831103C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831103D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831103D0 size=68
    let mut pc: u32 = 0x831103D0;
    'dispatch: loop {
        match pc {
            0x831103D0 => {
    //   block [0x831103D0..0x83110414)
	// 831103D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831103D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831103D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831103DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831103E0: 409A0010  bne cr6, 0x831103f0
	if !ctx.cr[6].eq {
	pc = 0x831103F0; continue 'dispatch;
	}
	// 831103E4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831103E8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831103EC: 48000018  b 0x83110404
	pc = 0x83110404; continue 'dispatch;
	// 831103F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831103F4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831103F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831103FC: 4E800421  bctrl
	ctx.lr = 0x83110400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110400: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83110404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83110408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8311040C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110418 size=68
    let mut pc: u32 = 0x83110418;
    'dispatch: loop {
        match pc {
            0x83110418 => {
    //   block [0x83110418..0x8311045C)
	// 83110418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110424: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110428: 409A0010  bne cr6, 0x83110438
	if !ctx.cr[6].eq {
	pc = 0x83110438; continue 'dispatch;
	}
	// 8311042C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110430: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110434: 48000018  b 0x8311044c
	pc = 0x8311044C; continue 'dispatch;
	// 83110438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311043C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83110440: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110444: 4E800421  bctrl
	ctx.lr = 0x83110448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110448: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8311044C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83110450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110460 size=72
    let mut pc: u32 = 0x83110460;
    'dispatch: loop {
        match pc {
            0x83110460 => {
    //   block [0x83110460..0x831104A8)
	// 83110460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311046C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110470: 409A0010  bne cr6, 0x83110480
	if !ctx.cr[6].eq {
	pc = 0x83110480; continue 'dispatch;
	}
	// 83110474: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110478: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8311047C: 4800001C  b 0x83110498
	pc = 0x83110498; continue 'dispatch;
	// 83110480: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110484: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83110488: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311048C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110490: 4E800421  bctrl
	ctx.lr = 0x83110494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110494: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83110498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8311049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831104A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831104A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831104A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831104A8 size=20
    let mut pc: u32 = 0x831104A8;
    'dispatch: loop {
        match pc {
            0x831104A8 => {
    //   block [0x831104A8..0x831104BC)
	// 831104A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831104AC: 409A0010  bne cr6, 0x831104bc
	if !ctx.cr[6].eq {
		sub_831104BC(ctx, base);
		return;
	}
	// 831104B0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831104B4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831104B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831104BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831104BC size=16
    let mut pc: u32 = 0x831104BC;
    'dispatch: loop {
        match pc {
            0x831104BC => {
    //   block [0x831104BC..0x831104CC)
	// 831104BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831104C0: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 831104C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831104C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831104D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831104D0 size=20
    let mut pc: u32 = 0x831104D0;
    'dispatch: loop {
        match pc {
            0x831104D0 => {
    //   block [0x831104D0..0x831104E4)
	// 831104D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831104D4: 409A0010  bne cr6, 0x831104e4
	if !ctx.cr[6].eq {
		sub_831104E4(ctx, base);
		return;
	}
	// 831104D8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831104DC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831104E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831104E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831104E4 size=16
    let mut pc: u32 = 0x831104E4;
    'dispatch: loop {
        match pc {
            0x831104E4 => {
    //   block [0x831104E4..0x831104F4)
	// 831104E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831104E8: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 831104EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831104F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831104F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831104F8 size=20
    let mut pc: u32 = 0x831104F8;
    'dispatch: loop {
        match pc {
            0x831104F8 => {
    //   block [0x831104F8..0x8311050C)
	// 831104F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831104FC: 409A0010  bne cr6, 0x8311050c
	if !ctx.cr[6].eq {
		sub_8311050C(ctx, base);
		return;
	}
	// 83110500: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110504: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8311050C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8311050C size=16
    let mut pc: u32 = 0x8311050C;
    'dispatch: loop {
        match pc {
            0x8311050C => {
    //   block [0x8311050C..0x8311051C)
	// 8311050C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110510: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 83110514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110518: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110520 size=20
    let mut pc: u32 = 0x83110520;
    'dispatch: loop {
        match pc {
            0x83110520 => {
    //   block [0x83110520..0x83110534)
	// 83110520: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110524: 409A0010  bne cr6, 0x83110534
	if !ctx.cr[6].eq {
		sub_83110534(ctx, base);
		return;
	}
	// 83110528: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8311052C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110534 size=16
    let mut pc: u32 = 0x83110534;
    'dispatch: loop {
        match pc {
            0x83110534 => {
    //   block [0x83110534..0x83110544)
	// 83110534: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110538: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 8311053C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110540: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110548 size=92
    let mut pc: u32 = 0x83110548;
    'dispatch: loop {
        match pc {
            0x83110548 => {
    //   block [0x83110548..0x831105A4)
	// 83110548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311054C: 48097C21  bl 0x831a816c
	ctx.lr = 0x83110550;
	sub_831A8130(ctx, base);
	// 83110550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110554: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83110558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8311055C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83110560: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83110564: 806BDEC8  lwz r3, -0x2138(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83110568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8311056C: 419A0030  beq cr6, 0x8311059c
	if ctx.cr[6].eq {
	pc = 0x8311059C; continue 'dispatch;
	}
	// 83110570: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110574: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 83110578: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311057C: 4E800421  bctrl
	ctx.lr = 0x83110580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110584: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83110588: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8311058C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83110590: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83110594: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110598: 4E800421  bctrl
	ctx.lr = 0x8311059C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8311059C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831105A0: 48097C1C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831105A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831105A8 size=104
    let mut pc: u32 = 0x831105A8;
    'dispatch: loop {
        match pc {
            0x831105A8 => {
    //   block [0x831105A8..0x83110610)
	// 831105A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831105AC: 48097BC1  bl 0x831a816c
	ctx.lr = 0x831105B0;
	sub_831A8130(ctx, base);
	// 831105B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831105B4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831105B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831105BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831105C0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831105C4: 806BDEC8  lwz r3, -0x2138(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 831105C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831105CC: 409A0010  bne cr6, 0x831105dc
	if !ctx.cr[6].eq {
	pc = 0x831105DC; continue 'dispatch;
	}
	// 831105D0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 831105D4: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 831105D8: 48000030  b 0x83110608
	pc = 0x83110608; continue 'dispatch;
	// 831105DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831105E0: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 831105E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831105E8: 4E800421  bctrl
	ctx.lr = 0x831105EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831105EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831105F0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 831105F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831105F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831105FC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83110600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110604: 4E800421  bctrl
	ctx.lr = 0x83110608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8311060C: 48097BB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110610 size=132
    let mut pc: u32 = 0x83110610;
    'dispatch: loop {
        match pc {
            0x83110610 => {
    //   block [0x83110610..0x83110694)
	// 83110610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110614: 48097B59  bl 0x831a816c
	ctx.lr = 0x83110618;
	sub_831A8130(ctx, base);
	// 83110618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311061C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83110620: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83110624: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83110628: 807FDEC8  lwz r3, -0x2138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 8311062C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110630: 419A005C  beq cr6, 0x8311068c
	if ctx.cr[6].eq {
	pc = 0x8311068C; continue 'dispatch;
	}
	// 83110634: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110638: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8311063C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110640: 4E800421  bctrl
	ctx.lr = 0x83110644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110644: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110648: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8311064C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110650: 4E800421  bctrl
	ctx.lr = 0x83110654;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110654: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83110658: 41820008  beq 0x83110660
	if ctx.cr[0].eq {
	pc = 0x83110660; continue 'dispatch;
	}
	// 8311065C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83110660: 807FDEC8  lwz r3, -0x2138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83110664: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110668: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 8311066C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110670: 4E800421  bctrl
	ctx.lr = 0x83110674;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110678: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8311067C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83110680: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83110684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110688: 4E800421  bctrl
	ctx.lr = 0x8311068C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8311068C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83110690: 48097B2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110698 size=140
    let mut pc: u32 = 0x83110698;
    'dispatch: loop {
        match pc {
            0x83110698 => {
    //   block [0x83110698..0x83110724)
	// 83110698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831106A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831106A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831106A8: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 831106AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831106B0: 807FDEC8  lwz r3, -0x2138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 831106B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831106B8: 419A0058  beq cr6, 0x83110710
	if ctx.cr[6].eq {
	pc = 0x83110710; continue 'dispatch;
	}
	// 831106BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831106C0: 409A0030  bne cr6, 0x831106f0
	if !ctx.cr[6].eq {
	pc = 0x831106F0; continue 'dispatch;
	}
	// 831106C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831106C8: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 831106CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831106D0: 4E800421  bctrl
	ctx.lr = 0x831106D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831106D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831106D8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 831106DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831106E0: 4E800421  bctrl
	ctx.lr = 0x831106E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831106E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831106E8: 40820028  bne 0x83110710
	if !ctx.cr[0].eq {
	pc = 0x83110710; continue 'dispatch;
	}
	// 831106EC: 807FDEC8  lwz r3, -0x2138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 831106F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831106F4: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 831106F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831106FC: 4E800421  bctrl
	ctx.lr = 0x83110700;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110700: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110704: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83110708: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311070C: 4E800421  bctrl
	ctx.lr = 0x83110710;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83110714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8311071C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83110720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83110728 size=344
    let mut pc: u32 = 0x83110728;
    'dispatch: loop {
        match pc {
            0x83110728 => {
    //   block [0x83110728..0x83110880)
	// 83110728: DBC1FFF0  stfd f30, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[30].u64 ) };
	// 8311072C: DBE1FFF8  stfd f31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[31].u64 ) };
	// 83110730: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83110734: 419A0140  beq cr6, 0x83110874
	if ctx.cr[6].eq {
	pc = 0x83110874; continue 'dispatch;
	}
	// 83110738: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8311073C: 546A863E  rlwinm r10, r3, 0x10, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83110740: F961FFE0  std r11, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[11].u64 ) };
	// 83110744: C9A1FFE0  lfd f13, -0x20(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 83110748: F941FFE0  std r10, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[10].u64 ) };
	// 8311074C: C981FFE0  lfd f12, -0x20(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 83110750: 546BC63E  rlwinm r11, r3, 0x18, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83110754: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83110758: F961FFE0  std r11, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[11].u64 ) };
	// 8311075C: C801FFE0  lfd f0, -0x20(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 83110760: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83110764: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83110768: FD600018  frsp f11, f0
	ctx.f[11].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8311076C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110770: C00A94AC  lfs f0, -0x6b54(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27476 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83110774: FD006818  frsp f8, f13
	ctx.f[8].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83110778: 396BAE00  addi r11, r11, -0x5200
	ctx.r[11].s64 = ctx.r[11].s64 + -20992;
	// 8311077C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 83110780: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 83110784: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 83110788: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8311078C: 390B0014  addi r8, r11, 0x14
	ctx.r[8].s64 = ctx.r[11].s64 + 20;
	// 83110790: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 83110794: 38CB0018  addi r6, r11, 0x18
	ctx.r[6].s64 = ctx.r[11].s64 + 24;
	// 83110798: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 8311079C: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 831107A0: 7D4A4C2E  lfsx f10, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 831107A4: ED080032  fmuls f8, f8, f0
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 831107A8: 7D2A442E  lfsx f9, r10, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 831107AC: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 831107B0: 390B0024  addi r8, r11, 0x24
	ctx.r[8].s64 = ctx.r[11].s64 + 36;
	// 831107B4: 7CEA3C2E  lfsx f7, r10, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 831107B8: 392B000C  addi r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 + 12;
	// 831107BC: 7CCA342E  lfsx f6, r10, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 831107C0: 38CB0028  addi r6, r11, 0x28
	ctx.r[6].s64 = ctx.r[11].s64 + 40;
	// 831107C4: 7CAA5C2E  lfsx f5, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 831107C8: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 831107CC: 7C8A2C2E  lfsx f4, r10, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 831107D0: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 831107D4: 7C4A442E  lfsx f2, r10, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 831107D8: 396B002C  addi r11, r11, 0x2c
	ctx.r[11].s64 = ctx.r[11].s64 + 44;
	// 831107DC: 7C6A4C2E  lfsx f3, r10, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 831107E0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 831107E4: 7FEA342E  lfsx f31, r10, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 831107E8: 5468842E  rlwinm r8, r3, 0x10, 0x10, 0x17
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 831107EC: ED4A02F2  fmuls f10, f10, f11
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[11].f64) as f32) as f64);
	// 831107F0: 7C2A3C2E  lfsx f1, r10, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831107F4: ED2902F2  fmuls f9, f9, f11
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[11].f64) as f32) as f64);
	// 831107F8: 7FCA2C2E  lfsx f30, r10, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 831107FC: ED6202F2  fmuls f11, f2, f11
	ctx.f[11].f64 = (((ctx.f[2].f64 * ctx.f[11].f64) as f32) as f64);
	// 83110800: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 83110804: 7D8A5C2E  lfsx f12, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83110808: C1A9C3C8  lfs f13, -0x3c38(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8311080C: ED47523A  fmadds f10, f7, f8, f10
	ctx.f[10].f64 = (((ctx.f[7].f64 * ctx.f[8].f64 + ctx.f[10].f64) as f32) as f64);
	// 83110810: ED264A3A  fmadds f9, f6, f8, f9
	ctx.f[9].f64 = (((ctx.f[6].f64 * ctx.f[8].f64 + ctx.f[9].f64) as f32) as f64);
	// 83110814: ED7F5A3A  fmadds f11, f31, f8, f11
	ctx.f[11].f64 = (((ctx.f[31].f64 * ctx.f[8].f64 + ctx.f[11].f64) as f32) as f64);
	// 83110818: ED45503A  fmadds f10, f5, f0, f10
	ctx.f[10].f64 = (((ctx.f[5].f64 * ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64);
	// 8311081C: ED24483A  fmadds f9, f4, f0, f9
	ctx.f[9].f64 = (((ctx.f[4].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 83110820: EC1E583A  fmadds f0, f30, f0, f11
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 83110824: ED6A182A  fadds f11, f10, f3
	ctx.f[11].f64 = ((ctx.f[10].f64 + ctx.f[3].f64) as f32) as f64;
	// 83110828: ED49082A  fadds f10, f9, f1
	ctx.f[10].f64 = ((ctx.f[9].f64 + ctx.f[1].f64) as f32) as f64;
	// 8311082C: EC00602A  fadds f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 83110830: ED8B0372  fmuls f12, f11, f13
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 83110834: ED6A0372  fmuls f11, f10, f13
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 83110838: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8311083C: FDA0665E  fctidz f13, f12
	ctx.f[13].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 83110840: D9A1FFE0  stfd f13, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[13].u64 ) };
	// 83110844: 8961FFE7  lbz r11, -0x19(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-25 as u32) ) } as u64;
	// 83110848: FDA05E5E  fctidz f13, f11
	ctx.f[13].s64 = if ctx.f[11].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[11].f64.trunc() as i64 };
	// 8311084C: D9A1FFE0  stfd f13, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[13].u64 ) };
	// 83110850: 8941FFE7  lbz r10, -0x19(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-25 as u32) ) } as u64;
	// 83110854: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 83110858: D801FFE0  stfd f0, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[0].u64 ) };
	// 8311085C: 8921FFE7  lbz r9, -0x19(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-25 as u32) ) } as u64;
	// 83110860: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 83110864: 5529402E  slwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83110868: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 8311086C: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83110870: 7D635378  or r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83110874: CBC1FFF0  lfd f30, -0x10(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83110878: CBE1FFF8  lfd f31, -8(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8311087C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110880 size=104
    let mut pc: u32 = 0x83110880;
    'dispatch: loop {
        match pc {
            0x83110880 => {
    //   block [0x83110880..0x831108E8)
	// 83110880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110884: 480978E1  bl 0x831a8164
	ctx.lr = 0x83110888;
	sub_831A8130(ctx, base);
	// 83110888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311088C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83110890: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83110894: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83110898: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8311089C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 831108A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831108A4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831108A8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831108AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831108B0: 4E800421  bctrl
	ctx.lr = 0x831108B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831108B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831108B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831108BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831108C0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 831108C4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 831108C8: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 831108CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831108D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831108D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831108D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831108DC: 4E800421  bctrl
	ctx.lr = 0x831108E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831108E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831108E4: 480978D0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831108E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831108E8 size=16
    let mut pc: u32 = 0x831108E8;
    'dispatch: loop {
        match pc {
            0x831108E8 => {
    //   block [0x831108E8..0x831108F8)
	// 831108E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831108EC: 816B00E4  lwz r11, 0xe4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 831108F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831108F4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831108F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831108F8 size=16
    let mut pc: u32 = 0x831108F8;
    'dispatch: loop {
        match pc {
            0x831108F8 => {
    //   block [0x831108F8..0x83110908)
	// 831108F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831108FC: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 83110900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110904: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110908 size=40
    let mut pc: u32 = 0x83110908;
    'dispatch: loop {
        match pc {
            0x83110908 => {
    //   block [0x83110908..0x83110930)
	// 83110908: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8311090C: 419A000C  beq cr6, 0x83110918
	if ctx.cr[6].eq {
	pc = 0x83110918; continue 'dispatch;
	}
	// 83110910: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 83110914: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110918: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8311091C: 419A000C  beq cr6, 0x83110928
	if ctx.cr[6].eq {
	pc = 0x83110928; continue 'dispatch;
	}
	// 83110920: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 83110924: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110928: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8311092C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110930 size=12
    let mut pc: u32 = 0x83110930;
    'dispatch: loop {
        match pc {
            0x83110930 => {
    //   block [0x83110930..0x8311093C)
	// 83110930: 816300B0  lwz r11, 0xb0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) } as u64;
	// 83110934: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110940 size=8
    let mut pc: u32 = 0x83110940;
    'dispatch: loop {
        match pc {
            0x83110940 => {
    //   block [0x83110940..0x83110948)
	// 83110940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110944: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110948 size=8
    let mut pc: u32 = 0x83110948;
    'dispatch: loop {
        match pc {
            0x83110948 => {
    //   block [0x83110948..0x83110950)
	// 83110948: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8311094C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110950 size=8
    let mut pc: u32 = 0x83110950;
    'dispatch: loop {
        match pc {
            0x83110950 => {
    //   block [0x83110950..0x83110958)
	// 83110950: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110954: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110958 size=16
    let mut pc: u32 = 0x83110958;
    'dispatch: loop {
        match pc {
            0x83110958 => {
    //   block [0x83110958..0x83110968)
	// 83110958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311095C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83110960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110964: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110968 size=4
    let mut pc: u32 = 0x83110968;
    'dispatch: loop {
        match pc {
            0x83110968 => {
    //   block [0x83110968..0x8311096C)
	// 83110968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110970 size=16
    let mut pc: u32 = 0x83110970;
    'dispatch: loop {
        match pc {
            0x83110970 => {
    //   block [0x83110970..0x83110980)
	// 83110970: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83110974: 816BDEB8  lwz r11, -0x2148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8520 as u32) ) } as u64;
	// 83110978: 556307BC  rlwinm r3, r11, 0, 0x1e, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8311097C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110980 size=32
    let mut pc: u32 = 0x83110980;
    'dispatch: loop {
        match pc {
            0x83110980 => {
    //   block [0x83110980..0x831109A0)
	// 83110980: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83110984: 816BDEC4  lwz r11, -0x213c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8508 as u32) ) } as u64;
	// 83110988: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8311098C: 419A000C  beq cr6, 0x83110998
	if ctx.cr[6].eq {
	pc = 0x83110998; continue 'dispatch;
	}
	// 83110990: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83110994: 816BDEC0  lwz r11, -0x2140(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 83110998: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8311099C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831109A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831109A0 size=36
    let mut pc: u32 = 0x831109A0;
    'dispatch: loop {
        match pc {
            0x831109A0 => {
    //   block [0x831109A0..0x831109C4)
	// 831109A0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831109A4: 816BDEC4  lwz r11, -0x213c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8508 as u32) ) } as u64;
	// 831109A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831109AC: 419A0018  beq cr6, 0x831109c4
	if ctx.cr[6].eq {
		sub_831109C4(ctx, base);
		return;
	}
	// 831109B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831109B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831109B8: 816BDEC0  lwz r11, -0x2140(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 831109BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831109C0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831109C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831109C4 size=8
    let mut pc: u32 = 0x831109C4;
    'dispatch: loop {
        match pc {
            0x831109C4 => {
    //   block [0x831109C4..0x831109CC)
	// 831109C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831109C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831109D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831109D0 size=20
    let mut pc: u32 = 0x831109D0;
    'dispatch: loop {
        match pc {
            0x831109D0 => {
    //   block [0x831109D0..0x831109E4)
	// 831109D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831109D4: 409A0010  bne cr6, 0x831109e4
	if !ctx.cr[6].eq {
		sub_831109E4(ctx, base);
		return;
	}
	// 831109D8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831109DC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831109E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831109E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831109E4 size=16
    let mut pc: u32 = 0x831109E4;
    'dispatch: loop {
        match pc {
            0x831109E4 => {
    //   block [0x831109E4..0x831109F4)
	// 831109E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831109E8: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 831109EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831109F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831109F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831109F8 size=20
    let mut pc: u32 = 0x831109F8;
    'dispatch: loop {
        match pc {
            0x831109F8 => {
    //   block [0x831109F8..0x83110A0C)
	// 831109F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831109FC: 409A0010  bne cr6, 0x83110a0c
	if !ctx.cr[6].eq {
		sub_83110A0C(ctx, base);
		return;
	}
	// 83110A00: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83110A04: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83110A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110A0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110A0C size=16
    let mut pc: u32 = 0x83110A0C;
    'dispatch: loop {
        match pc {
            0x83110A0C => {
    //   block [0x83110A0C..0x83110A1C)
	// 83110A0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110A10: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 83110A14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110A18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110A20 size=76
    let mut pc: u32 = 0x83110A20;
    'dispatch: loop {
        match pc {
            0x83110A20 => {
    //   block [0x83110A20..0x83110A6C)
	// 83110A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110A28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110A2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110A30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83110A34: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110A38: 396BB120  addi r11, r11, -0x4ee0
	ctx.r[11].s64 = ctx.r[11].s64 + -20192;
	// 83110A3C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83110A40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110A44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83110A48: 419A0010  beq cr6, 0x83110a58
	if ctx.cr[6].eq {
	pc = 0x83110A58; continue 'dispatch;
	}
	// 83110A4C: 4BACE58D  bl 0x82bdefd8
	ctx.lr = 0x83110A50;
	sub_82BDEFD8(ctx, base);
	// 83110A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83110A54: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83110A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83110A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110A64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83110A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110A70 size=40
    let mut pc: u32 = 0x83110A70;
    'dispatch: loop {
        match pc {
            0x83110A70 => {
    //   block [0x83110A70..0x83110A98)
	// 83110A70: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 83110A74: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83110A78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83110A7C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83110A80: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83110A84: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83110A88: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83110A8C: 4082FFE8  bne 0x83110a74
	if !ctx.cr[0].eq {
	pc = 0x83110A74; continue 'dispatch;
	}
	// 83110A90: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83110A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110A98 size=112
    let mut pc: u32 = 0x83110A98;
    'dispatch: loop {
        match pc {
            0x83110A98 => {
    //   block [0x83110A98..0x83110B08)
	// 83110A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110A9C: 480976CD  bl 0x831a8168
	ctx.lr = 0x83110AA0;
	sub_831A8130(ctx, base);
	// 83110AA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110AA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83110AA8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83110AAC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83110AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83110AB4: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 83110AB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83110ABC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83110AC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83110AC4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83110AC8: 48097719  bl 0x831a81e0
	ctx.lr = 0x83110ACC;
	sub_831A81E0(ctx, base);
	// 83110ACC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83110AD0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83110AD4: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83110AD8: 4BAD0FA9  bl 0x82be1a80
	ctx.lr = 0x83110ADC;
	sub_82BE1A80(ctx, base);
	// 83110ADC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83110AE0: 419A000C  beq cr6, 0x83110aec
	if ctx.cr[6].eq {
	pc = 0x83110AEC; continue 'dispatch;
	}
	// 83110AE4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83110AE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110AEC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83110AF0: 419A000C  beq cr6, 0x83110afc
	if ctx.cr[6].eq {
	pc = 0x83110AFC; continue 'dispatch;
	}
	// 83110AF4: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83110AF8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110AFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83110B00: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83110B04: 480976B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110B08 size=120
    let mut pc: u32 = 0x83110B08;
    'dispatch: loop {
        match pc {
            0x83110B08 => {
    //   block [0x83110B08..0x83110B80)
	// 83110B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83110B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110B18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83110B20: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 83110B24: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83110B28: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83110B2C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83110B30: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83110B34: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83110B38: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83110B3C: 4082FFE8  bne 0x83110b24
	if !ctx.cr[0].eq {
	pc = 0x83110B24; continue 'dispatch;
	}
	// 83110B40: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 83110B44: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83110B48: 409A001C  bne cr6, 0x83110b64
	if !ctx.cr[6].eq {
	pc = 0x83110B64; continue 'dispatch;
	}
	// 83110B4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83110B50: 419A0014  beq cr6, 0x83110b64
	if ctx.cr[6].eq {
	pc = 0x83110B64; continue 'dispatch;
	}
	// 83110B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110B58: 4BFFFEC9  bl 0x83110a20
	ctx.lr = 0x83110B5C;
	sub_83110A20(ctx, base);
	// 83110B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110B60: 4BFCC979  bl 0x830dd4d8
	ctx.lr = 0x83110B64;
	sub_830DD4D8(ctx, base);
	// 83110B64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83110B68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83110B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110B74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83110B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83110B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110B80 size=20
    let mut pc: u32 = 0x83110B80;
    'dispatch: loop {
        match pc {
            0x83110B80 => {
    //   block [0x83110B80..0x83110B94)
	// 83110B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83110B84: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 83110B88: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110B8C: 60634001  ori r3, r3, 0x4001
	ctx.r[3].u64 = ctx.r[3].u64 | 16385;
	// 83110B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110B98 size=232
    let mut pc: u32 = 0x83110B98;
    'dispatch: loop {
        match pc {
            0x83110B98 => {
    //   block [0x83110B98..0x83110C80)
	// 83110B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110B9C: 480975D1  bl 0x831a816c
	ctx.lr = 0x83110BA0;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110C80 size=20
    let mut pc: u32 = 0x83110C80;
    'dispatch: loop {
        match pc {
            0x83110C80 => {
    //   block [0x83110C80..0x83110C94)
	// 83110C80: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 83110C84: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110C88: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 83110C8C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110C98 size=100
    let mut pc: u32 = 0x83110C98;
    'dispatch: loop {
        match pc {
            0x83110C98 => {
    //   block [0x83110C98..0x83110CFC)
	// 83110C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110CA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83110CA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110CAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83110CB0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110CB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83110CB8: 396BB154  addi r11, r11, -0x4eac
	ctx.r[11].s64 = ctx.r[11].s64 + -20140;
	// 83110CBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110CC0: 4BFFE4F9  bl 0x8310f1b8
	ctx.lr = 0x83110CC4;
	sub_8310F1B8(ctx, base);
	// 83110CC4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110CC8: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83110CCC: 396BAE90  addi r11, r11, -0x5170
	ctx.r[11].s64 = ctx.r[11].s64 + -20848;
	// 83110CD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110CD4: 4182000C  beq 0x83110ce0
	if ctx.cr[0].eq {
	pc = 0x83110CE0; continue 'dispatch;
	}
	// 83110CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110CDC: 4BFCC7FD  bl 0x830dd4d8
	ctx.lr = 0x83110CE0;
	sub_830DD4D8(ctx, base);
	// 83110CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110CE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83110CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110CF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83110CF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83110CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110D00 size=224
    let mut pc: u32 = 0x83110D00;
    'dispatch: loop {
        match pc {
            0x83110D00 => {
    //   block [0x83110D00..0x83110DE0)
	// 83110D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110D04: 48097461  bl 0x831a8164
	ctx.lr = 0x83110D08;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110DE0 size=20
    let mut pc: u32 = 0x83110DE0;
    'dispatch: loop {
        match pc {
            0x83110DE0 => {
    //   block [0x83110DE0..0x83110DF4)
	// 83110DE0: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 83110DE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83110DE8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110DEC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83110DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110DF8 size=100
    let mut pc: u32 = 0x83110DF8;
    'dispatch: loop {
        match pc {
            0x83110DF8 => {
    //   block [0x83110DF8..0x83110E5C)
	// 83110DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110E00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83110E04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83110E10: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110E14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83110E18: 396BB170  addi r11, r11, -0x4e90
	ctx.r[11].s64 = ctx.r[11].s64 + -20112;
	// 83110E1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110E20: 4BFFE9C1  bl 0x8310f7e0
	ctx.lr = 0x83110E24;
	sub_8310F7E0(ctx, base);
	// 83110E24: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110E28: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83110E2C: 396BAE90  addi r11, r11, -0x5170
	ctx.r[11].s64 = ctx.r[11].s64 + -20848;
	// 83110E30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110E34: 4182000C  beq 0x83110e40
	if ctx.cr[0].eq {
	pc = 0x83110E40; continue 'dispatch;
	}
	// 83110E38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110E3C: 4BFCC69D  bl 0x830dd4d8
	ctx.lr = 0x83110E40;
	sub_830DD4D8(ctx, base);
	// 83110E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110E44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83110E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110E50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83110E54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83110E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83110E60 size=220
    let mut pc: u32 = 0x83110E60;
    'dispatch: loop {
        match pc {
            0x83110E60 => {
    //   block [0x83110E60..0x83110F3C)
	// 83110E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110E64: 480972FD  bl 0x831a8160
	ctx.lr = 0x83110E68;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110F40 size=72
    let mut pc: u32 = 0x83110F40;
    'dispatch: loop {
        match pc {
            0x83110F40 => {
    //   block [0x83110F40..0x83110F88)
	// 83110F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110F4C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83110F50: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110F54: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83110F58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83110F5C: 419A0018  beq cr6, 0x83110f74
	if ctx.cr[6].eq {
	pc = 0x83110F74; continue 'dispatch;
	}
	// 83110F60: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83110F64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110F68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83110F6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83110F70: 4E800421  bctrl
	ctx.lr = 0x83110F74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83110F74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83110F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83110F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83110F88 size=100
    let mut pc: u32 = 0x83110F88;
    'dispatch: loop {
        match pc {
            0x83110F88 => {
    //   block [0x83110F88..0x83110FEC)
	// 83110F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110F90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83110F94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110F98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83110F9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83110FA0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110FA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83110FA8: 396BB18C  addi r11, r11, -0x4e74
	ctx.r[11].s64 = ctx.r[11].s64 + -20084;
	// 83110FAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110FB0: 4BFFE831  bl 0x8310f7e0
	ctx.lr = 0x83110FB4;
	sub_8310F7E0(ctx, base);
	// 83110FB4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83110FB8: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83110FBC: 396BAE90  addi r11, r11, -0x5170
	ctx.r[11].s64 = ctx.r[11].s64 + -20848;
	// 83110FC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83110FC4: 4182000C  beq 0x83110fd0
	if ctx.cr[0].eq {
	pc = 0x83110FD0; continue 'dispatch;
	}
	// 83110FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110FCC: 4BFCC50D  bl 0x830dd4d8
	ctx.lr = 0x83110FD0;
	sub_830DD4D8(ctx, base);
	// 83110FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83110FD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83110FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83110FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83110FE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83110FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83110FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83110FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83110FF0 size=184
    let mut pc: u32 = 0x83110FF0;
    'dispatch: loop {
        match pc {
            0x83110FF0 => {
    //   block [0x83110FF0..0x831110A8)
	// 83110FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83110FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83110FF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83110FFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83111004: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111008: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8311100C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83111010: C00A0008  lfs f0, 8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83111014: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83111018: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8311101C: 40980008  bge cr6, 0x83111024
	if !ctx.cr[6].lt {
	pc = 0x83111024; continue 'dispatch;
	}
	// 83111020: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83111024: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111028: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8311102C: 409AFFE4  bne cr6, 0x83111010
	if !ctx.cr[6].eq {
	pc = 0x83111010; continue 'dispatch;
	}
	// 83111030: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111034: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83111038: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8311103C: 409A0008  bne cr6, 0x83111044
	if !ctx.cr[6].eq {
	pc = 0x83111044; continue 'dispatch;
	}
	// 83111040: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111044: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83111048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311104C: E8A80004  ld r5, 4(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	// 83111050: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83111054: 419A0010  beq cr6, 0x83111064
	if ctx.cr[6].eq {
	pc = 0x83111064; continue 'dispatch;
	}
	// 83111058: E8CB0004  ld r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	// 8311105C: E88A0004  ld r4, 4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	// 83111060: 4800000C  b 0x8311106c
	pc = 0x8311106C; continue 'dispatch;
	// 83111064: E8CA0004  ld r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	// 83111068: E88B0004  ld r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	// 8311106C: 4BFFB315  bl 0x8310c380
	ctx.lr = 0x83111070;
	sub_8310C380(ctx, base);
	// 83111070: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83111074: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83111078: 41990008  bgt cr6, 0x83111080
	if ctx.cr[6].gt {
	pc = 0x83111080; continue 'dispatch;
	}
	// 8311107C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83111080: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83111084: 4082000C  bne 0x83111090
	if !ctx.cr[0].eq {
	pc = 0x83111090; continue 'dispatch;
	}
	// 83111088: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8311108C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83111090: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83111094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83111098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8311109C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831110A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831110A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831110A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831110A8 size=584
    let mut pc: u32 = 0x831110A8;
    'dispatch: loop {
        match pc {
            0x831110A8 => {
    //   block [0x831110A8..0x831112F0)
	// 831110A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831110AC: 480970A1  bl 0x831a814c
	ctx.lr = 0x831110B0;
	sub_831A8130(ctx, base);
	// 831110B0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831110B4: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 831110B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831110BC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 831110C0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831110C4: 409A0014  bne cr6, 0x831110d8
	if !ctx.cr[6].eq {
	pc = 0x831110D8; continue 'dispatch;
	}
	// 831110C8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 831110CC: 60634003  ori r3, r3, 0x4003
	ctx.r[3].u64 = ctx.r[3].u64 | 16387;
	// 831110D0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831110D4: 480970C8  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 831110D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831110DC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 831110E0: 40980010  bge cr6, 0x831110f0
	if !ctx.cr[6].lt {
	pc = 0x831110F0; continue 'dispatch;
	}
	// 831110E4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 831110E8: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 831110EC: 4BFFFFE4  b 0x831110d0
	pc = 0x831110D0; continue 'dispatch;
	// 831110F0: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 831110F4: 1D6B0006  mulli r11, r11, 6
	ctx.r[11].s64 = ctx.r[11].s64 * 6;
	// 831110F8: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831110FC: 40980010  bge cr6, 0x8311110c
	if !ctx.cr[6].lt {
	pc = 0x8311110C; continue 'dispatch;
	}
	// 83111100: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83111104: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83111108: 4BFFFFC8  b 0x831110d0
	pc = 0x831110D0; continue 'dispatch;
	// 8311110C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111110: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111114: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83111118: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8311111C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 83111120: 409A0018  bne cr6, 0x83111138
	if !ctx.cr[6].eq {
	pc = 0x83111138; continue 'dispatch;
	}
	// 83111124: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83111128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8311112C: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83111130: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83111134: 419A0008  beq cr6, 0x8311113c
	if ctx.cr[6].eq {
	pc = 0x8311113C; continue 'dispatch;
	}
	// 83111138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8311113C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83111140: 41820028  beq 0x83111168
	if ctx.cr[0].eq {
	pc = 0x83111168; continue 'dispatch;
	}
	// 83111144: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111148: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8311114C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111150: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111154: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83111158: 4BFBD181  bl 0x830ce2d8
	ctx.lr = 0x8311115C;
	sub_830CE2D8(ctx, base);
	// 8311115C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111160: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83111164: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83111168: 837F0014  lwz r27, 0x14(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8311116C: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111170: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111178: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8311117C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83111180: 4BFFB731  bl 0x8310c8b0
	ctx.lr = 0x83111184;
	sub_8310C8B0(ctx, base);
	// 83111184: 987E000C  stb r3, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u8 ) };
	// 83111188: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8311118C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 83111190: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83111194: 409AFFDC  bne cr6, 0x83111170
	if !ctx.cr[6].eq {
	pc = 0x83111170; continue 'dispatch;
	}
	// 83111198: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8311119C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831111A0: 2F1B0003  cmpwi cr6, r27, 3
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3, &mut ctx.xer);
	// 831111A4: 409900D4  ble cr6, 0x83111278
	if !ctx.cr[6].gt {
	pc = 0x83111278; continue 'dispatch;
	}
	// 831111A8: 3BB80004  addi r29, r24, 4
	ctx.r[29].s64 = ctx.r[24].s64 + 4;
	// 831111AC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831111B0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831111B4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 831111B8: 893E000C  lbz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831111BC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831111C0: 4082001C  bne 0x831111dc
	if !ctx.cr[0].eq {
	pc = 0x831111DC; continue 'dispatch;
	}
	// 831111C4: 5549063F  clrlwi. r9, r10, 0x18
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831111C8: 40820014  bne 0x831111dc
	if !ctx.cr[0].eq {
	pc = 0x831111DC; continue 'dispatch;
	}
	// 831111CC: 83DE0010  lwz r30, 0x10(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 831111D0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831111D4: 409AFFE4  bne cr6, 0x831111b8
	if !ctx.cr[6].eq {
	pc = 0x831111B8; continue 'dispatch;
	}
	// 831111D8: 48000088  b 0x83111260
	pc = 0x83111260; continue 'dispatch;
	// 831111DC: 82FE0010  lwz r23, 0x10(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 831111E0: 572B043E  clrlwi r11, r25, 0x10
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 831111E4: 82DE0014  lwz r22, 0x14(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 831111E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831111EC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831111F0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 831111F4: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 831111F8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831111FC: 82B70010  lwz r21, 0x10(r23)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111200: 3B9C0003  addi r28, r28, 3
	ctx.r[28].s64 = ctx.r[28].s64 + 3;
	// 83111204: 80960014  lwz r4, 0x14(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111208: B15DFFFC  sth r10, -4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 8311120C: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111210: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83111214: B15DFFFE  sth r10, -2(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 83111218: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311121C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83111220: B17D0000  sth r11, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83111224: 3BBD0006  addi r29, r29, 6
	ctx.r[29].s64 = ctx.r[29].s64 + 6;
	// 83111228: 4BFFB689  bl 0x8310c8b0
	ctx.lr = 0x8311122C;
	sub_8310C8B0(ctx, base);
	// 8311122C: 9876000C  stb r3, 0xc(r22)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[22].u32.wrapping_add(12 as u32), ctx.r[3].u8 ) };
	// 83111230: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 83111234: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83111238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311123C: 4BFFB675  bl 0x8310c8b0
	ctx.lr = 0x83111240;
	sub_8310C8B0(ctx, base);
	// 83111240: 9877000C  stb r3, 0xc(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(12 as u32), ctx.r[3].u8 ) };
	// 83111244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111248: 4BFBD091  bl 0x830ce2d8
	ctx.lr = 0x8311124C;
	sub_830CE2D8(ctx, base);
	// 8311124C: 92F60010  stw r23, 0x10(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(16 as u32), ctx.r[23].u32 ) };
	// 83111250: 92D70014  stw r22, 0x14(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(20 as u32), ctx.r[22].u32 ) };
	// 83111254: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 83111258: 92FF0010  stw r23, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[23].u32 ) };
	// 8311125C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83111260: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83111264: 40820008  bne 0x8311126c
	if !ctx.cr[0].eq {
	pc = 0x8311126C; continue 'dispatch;
	}
	// 83111268: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8311126C: 2F1B0003  cmpwi cr6, r27, 3
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3, &mut ctx.xer);
	// 83111270: 4199FF3C  bgt cr6, 0x831111ac
	if ctx.cr[6].gt {
	pc = 0x831111AC; continue 'dispatch;
	}
	// 83111274: 2F1B0003  cmpwi cr6, r27, 3
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3, &mut ctx.xer);
	// 83111278: 4098001C  bge cr6, 0x83111294
	if !ctx.cr[6].lt {
	pc = 0x83111294; continue 'dispatch;
	}
	// 8311127C: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 83111280: 63DEFFFF  ori r30, r30, 0xffff
	ctx.r[30].u64 = ctx.r[30].u64 | 65535;
	// 83111284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111288: 4BFFAF81  bl 0x8310c208
	ctx.lr = 0x8311128C;
	sub_8310C208(ctx, base);
	// 8311128C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111290: 4BFFFE40  b 0x831110d0
	pc = 0x831110D0; continue 'dispatch;
	// 83111294: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111298: 578A083C  slwi r10, r28, 1
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8311129C: 572B043E  clrlwi r11, r25, 0x10
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 831112A0: 7D4AC214  add r10, r10, r24
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 831112A4: 391C0002  addi r8, r28, 2
	ctx.r[8].s64 = ctx.r[28].s64 + 2;
	// 831112A8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831112AC: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831112B0: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 831112B4: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831112B8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 831112BC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831112C0: 81290010  lwz r9, 0x10(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 831112C4: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 831112C8: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 831112CC: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 831112D0: B12A0002  sth r9, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 831112D4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831112D8: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 831112DC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 831112E0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831112E4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831112E8: 7D68C32E  sthx r11, r8, r24
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[24].u32), ctx.r[11].u16) };
	// 831112EC: 4BFFFF98  b 0x83111284
	pc = 0x83111284; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831112F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831112F0 size=120
    let mut pc: u32 = 0x831112F0;
    'dispatch: loop {
        match pc {
            0x831112F0 => {
    //   block [0x831112F0..0x83111368)
	// 831112F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831112F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831112F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831112FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83111300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111304: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83111308: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8311130C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83111310: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83111314: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83111318: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8311131C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83111320: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83111324: 4082FFE8  bne 0x8311130c
	if !ctx.cr[0].eq {
	pc = 0x8311130C; continue 'dispatch;
	}
	// 83111328: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 8311132C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83111330: 409A001C  bne cr6, 0x8311134c
	if !ctx.cr[6].eq {
	pc = 0x8311134C; continue 'dispatch;
	}
	// 83111334: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83111338: 419A0014  beq cr6, 0x8311134c
	if ctx.cr[6].eq {
	pc = 0x8311134C; continue 'dispatch;
	}
	// 8311133C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111340: 4BFFB5D9  bl 0x8310c918
	ctx.lr = 0x83111344;
	sub_8310C918(ctx, base);
	// 83111344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111348: 4BFCC191  bl 0x830dd4d8
	ctx.lr = 0x8311134C;
	sub_830DD4D8(ctx, base);
	// 8311134C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111350: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83111354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83111358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8311135C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83111360: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83111364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111368 size=416
    let mut pc: u32 = 0x83111368;
    'dispatch: loop {
        match pc {
            0x83111368 => {
    //   block [0x83111368..0x83111508)
	// 83111368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311136C: 48096DF9  bl 0x831a8164
	ctx.lr = 0x83111370;
	sub_831A8130(ctx, base);
	// 83111370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111374: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83111378: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8311137C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83111380: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83111384: 419A0174  beq cr6, 0x831114f8
	if ctx.cr[6].eq {
	pc = 0x831114F8; continue 'dispatch;
	}
	// 83111388: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311138C: 2B0B001C  cmplwi cr6, r11, 0x1c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 28 as u32, &mut ctx.xer);
	// 83111390: 41980168  blt cr6, 0x831114f8
	if ctx.cr[6].lt {
	pc = 0x831114F8; continue 'dispatch;
	}
	// 83111394: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83111398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8311139C: 419A00A8  beq cr6, 0x83111444
	if ctx.cr[6].eq {
	pc = 0x83111444; continue 'dispatch;
	}
	// 831113A0: 38600094  li r3, 0x94
	ctx.r[3].s64 = 148;
	// 831113A4: 4BFCC10D  bl 0x830dd4b0
	ctx.lr = 0x831113A8;
	sub_830DD4B0(ctx, base);
	// 831113A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831113AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831113B0: 41820010  beq 0x831113c0
	if ctx.cr[0].eq {
	pc = 0x831113C0; continue 'dispatch;
	}
	// 831113B4: 48008F35  bl 0x8311a2e8
	ctx.lr = 0x831113B8;
	sub_8311A2E8(ctx, base);
	// 831113B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831113BC: 48000008  b 0x831113c4
	pc = 0x831113C4; continue 'dispatch;
	// 831113C0: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 831113C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831113C8: 409A0010  bne cr6, 0x831113d8
	if !ctx.cr[6].eq {
	pc = 0x831113D8; continue 'dispatch;
	}
	// 831113CC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831113D0: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 831113D4: 4800012C  b 0x83111500
	pc = 0x83111500; continue 'dispatch;
	// 831113D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831113DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831113E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831113E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831113E8: 4E800421  bctrl
	ctx.lr = 0x831113EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831113EC: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 831113F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831113F4: 409A0028  bne cr6, 0x8311141c
	if !ctx.cr[6].eq {
	pc = 0x8311141C; continue 'dispatch;
	}
	// 831113F8: 3FC08007  lis r30, -0x7ff9
	ctx.r[30].s64 = -2147024896;
	// 831113FC: 63DE0057  ori r30, r30, 0x57
	ctx.r[30].u64 = ctx.r[30].u64 | 87;
	// 83111400: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111408: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8311140C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111410: 4E800421  bctrl
	ctx.lr = 0x83111414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111414: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111418: 480000E8  b 0x83111500
	pc = 0x83111500; continue 'dispatch;
	// 8311141C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111420: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83111424: 48009015  bl 0x8311a438
	ctx.lr = 0x83111428;
	sub_8311A438(ctx, base);
	// 83111428: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8311142C: 4080000C  bge 0x83111438
	if !ctx.cr[0].lt {
	pc = 0x83111438; continue 'dispatch;
	}
	// 83111430: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83111434: 4BFFFFCC  b 0x83111400
	pc = 0x83111400; continue 'dispatch;
	// 83111438: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8311143C: 917D001C  stw r11, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83111440: 48000030  b 0x83111470
	pc = 0x83111470; continue 'dispatch;
	// 83111444: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83111448: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8311144C: 419A00AC  beq cr6, 0x831114f8
	if ctx.cr[6].eq {
	pc = 0x831114F8; continue 'dispatch;
	}
	// 83111450: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83111454: 939D001C  stw r28, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 83111458: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8311145C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111460: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111464: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311146C: 4E800421  bctrl
	ctx.lr = 0x83111470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111470: 937D0014  stw r27, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 83111474: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83111478: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8311147C: 4BFF7E85  bl 0x83109300
	ctx.lr = 0x83111480;
	sub_83109300(ctx, base);
	// 83111480: 7C691B79  or. r9, r3, r3
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83111484: 40800014  bge 0x83111498
	if !ctx.cr[0].lt {
	pc = 0x83111498; continue 'dispatch;
	}
	// 83111488: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8311148C: 4BFFEB25  bl 0x8310ffb0
	ctx.lr = 0x83111490;
	sub_8310FFB0(ctx, base);
	// 83111490: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 83111494: 4BFFFF6C  b 0x83111400
	pc = 0x83111400; continue 'dispatch;
	// 83111498: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8311149C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831114A0: 909D0010  stw r4, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 831114A4: 419A0024  beq cr6, 0x831114c8
	if ctx.cr[6].eq {
	pc = 0x831114C8; continue 'dispatch;
	}
	// 831114A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831114AC: 480029F5  bl 0x83113ea0
	ctx.lr = 0x831114B0;
	sub_83113EA0(ctx, base);
	// 831114B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831114B4: 40800014  bge 0x831114c8
	if !ctx.cr[0].lt {
	pc = 0x831114C8; continue 'dispatch;
	}
	// 831114B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831114BC: 939D0010  stw r28, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 831114C0: 939D000C  stw r28, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 831114C4: 4BFFFF3C  b 0x83111400
	pc = 0x83111400; continue 'dispatch;
	// 831114C8: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 831114CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831114D0: 419A0014  beq cr6, 0x831114e4
	if ctx.cr[6].eq {
	pc = 0x831114E4; continue 'dispatch;
	}
	// 831114D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831114D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831114DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831114E0: 4E800421  bctrl
	ctx.lr = 0x831114E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831114E4: 93FD0008  stw r31, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 831114E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831114EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831114F0: 917D0020  stw r11, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 831114F4: 4800000C  b 0x83111500
	pc = 0x83111500; continue 'dispatch;
	// 831114F8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831114FC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83111500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83111504: 48096CB0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111508 size=140
    let mut pc: u32 = 0x83111508;
    'dispatch: loop {
        match pc {
            0x83111508 => {
    //   block [0x83111508..0x83111594)
	// 83111508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311150C: 48096C61  bl 0x831a816c
	ctx.lr = 0x83111510;
	sub_831A8130(ctx, base);
	// 83111510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111514: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83111518: 386001D0  li r3, 0x1d0
	ctx.r[3].s64 = 464;
	// 8311151C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83111520: 4BFCBF91  bl 0x830dd4b0
	ctx.lr = 0x83111524;
	sub_830DD4B0(ctx, base);
	// 83111524: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83111528: 41820010  beq 0x83111538
	if ctx.cr[0].eq {
	pc = 0x83111538; continue 'dispatch;
	}
	// 8311152C: 4BFFC9AD  bl 0x8310ded8
	ctx.lr = 0x83111530;
	sub_8310DED8(ctx, base);
	// 83111530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83111534: 48000008  b 0x8311153c
	pc = 0x8311153C; continue 'dispatch;
	// 83111538: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8311153C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83111540: 409A0010  bne cr6, 0x83111550
	if !ctx.cr[6].eq {
	pc = 0x83111550; continue 'dispatch;
	}
	// 83111544: 3FC08007  lis r30, -0x7ff9
	ctx.r[30].s64 = -2147024896;
	// 83111548: 63DE000E  ori r30, r30, 0xe
	ctx.r[30].u64 = ctx.r[30].u64 | 14;
	// 8311154C: 4800003C  b 0x83111588
	pc = 0x83111588; continue 'dispatch;
	// 83111550: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83111554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111558: 4BFFD1C9  bl 0x8310e720
	ctx.lr = 0x8311155C;
	sub_8310E720(ctx, base);
	// 8311155C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83111560: 4180000C  blt 0x8311156c
	if ctx.cr[0].lt {
	pc = 0x8311156C; continue 'dispatch;
	}
	// 83111564: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83111568: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8311156C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83111570: 419A0018  beq cr6, 0x83111588
	if ctx.cr[6].eq {
	pc = 0x83111588; continue 'dispatch;
	}
	// 83111574: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311157C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83111580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111584: 4E800421  bctrl
	ctx.lr = 0x83111588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8311158C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83111590: 48096C2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111598 size=220
    let mut pc: u32 = 0x83111598;
    'dispatch: loop {
        match pc {
            0x83111598 => {
    //   block [0x83111598..0x83111674)
	// 83111598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311159C: 48096BC5  bl 0x831a8160
	ctx.lr = 0x831115A0;
	sub_831A8130(ctx, base);
	// 831115A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831115A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831115A8: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 831115AC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 831115B0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831115B4: 4BFCBEFD  bl 0x830dd4b0
	ctx.lr = 0x831115B8;
	sub_830DD4B0(ctx, base);
	// 831115B8: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 831115BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831115C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831115C4: 41820038  beq 0x831115fc
	if ctx.cr[0].eq {
	pc = 0x831115FC; continue 'dispatch;
	}
	// 831115C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831115CC: 93630004  stw r27, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 831115D0: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 831115D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831115D8: 396BAEAC  addi r11, r11, -0x5154
	ctx.r[11].s64 = ctx.r[11].s64 + -20820;
	// 831115DC: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 831115E0: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 831115E4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831115E8: 93C30010  stw r30, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 831115EC: 93C30018  stw r30, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 831115F0: 93C3001C  stw r30, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 831115F4: 93C30020  stw r30, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 831115F8: 48000008  b 0x83111600
	pc = 0x83111600; continue 'dispatch;
	// 831115FC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83111600: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83111604: 409A0010  bne cr6, 0x83111614
	if !ctx.cr[6].eq {
	pc = 0x83111614; continue 'dispatch;
	}
	// 83111608: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 8311160C: 63BD000E  ori r29, r29, 0xe
	ctx.r[29].u64 = ctx.r[29].u64 | 14;
	// 83111610: 48000058  b 0x83111668
	pc = 0x83111668; continue 'dispatch;
	// 83111614: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111618: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8311161C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83111620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111624: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83111628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311162C: 4E800421  bctrl
	ctx.lr = 0x83111630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111630: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83111634: 41800018  blt 0x8311164c
	if ctx.cr[0].lt {
	pc = 0x8311164C; continue 'dispatch;
	}
	// 83111638: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8311163C: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83111640: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 83111644: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83111648: 916ADEC4  stw r11, -0x213c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8508 as u32), ctx.r[11].u32 ) };
	// 8311164C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83111650: 419A0018  beq cr6, 0x83111668
	if ctx.cr[6].eq {
	pc = 0x83111668; continue 'dispatch;
	}
	// 83111654: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311165C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83111660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111664: 4E800421  bctrl
	ctx.lr = 0x83111668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111668: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8311166C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83111670: 48096B40  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111678 size=68
    let mut pc: u32 = 0x83111678;
    'dispatch: loop {
        match pc {
            0x83111678 => {
    //   block [0x83111678..0x831116BC)
	// 83111678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311167C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83111680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83111684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8311168C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83111690: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83111694: 396BAEC8  addi r11, r11, -0x5138
	ctx.r[11].s64 = ctx.r[11].s64 + -20792;
	// 83111698: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8311169C: 41820008  beq 0x831116a4
	if ctx.cr[0].eq {
	pc = 0x831116A4; continue 'dispatch;
	}
	// 831116A0: 4BFCBE39  bl 0x830dd4d8
	ctx.lr = 0x831116A4;
	sub_830DD4D8(ctx, base);
	// 831116A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831116A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831116AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831116B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831116B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831116B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831116C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831116C0 size=116
    let mut pc: u32 = 0x831116C0;
    'dispatch: loop {
        match pc {
            0x831116C0 => {
    //   block [0x831116C0..0x83111734)
	// 831116C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831116C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831116C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831116CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831116D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831116D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831116D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831116DC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831116E0: 409A0010  bne cr6, 0x831116f0
	if !ctx.cr[6].eq {
	pc = 0x831116F0; continue 'dispatch;
	}
	// 831116E4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831116E8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831116EC: 48000030  b 0x8311171c
	pc = 0x8311171C; continue 'dispatch;
	// 831116F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831116F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831116F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831116FC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111700: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111704: 4E800421  bctrl
	ctx.lr = 0x83111708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111708: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8311170C: 41800010  blt 0x8311171c
	if ctx.cr[0].lt {
	pc = 0x8311171C; continue 'dispatch;
	}
	// 83111710: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83111714: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111718: 4BFFE439  bl 0x8310fb50
	ctx.lr = 0x8311171C;
	sub_8310FB50(ctx, base);
	// 8311171C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83111720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83111724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83111728: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8311172C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83111730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111738 size=440
    let mut pc: u32 = 0x83111738;
    'dispatch: loop {
        match pc {
            0x83111738 => {
    //   block [0x83111738..0x831118F0)
	// 83111738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311173C: 48096A2D  bl 0x831a8168
	ctx.lr = 0x83111740;
	sub_831A8130(ctx, base);
	// 83111740: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 83111744: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8311174C: 908100EC  stw r4, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[4].u32 ) };
	// 83111750: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 83111754: 90C100FC  stw r6, 0xfc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), ctx.r[6].u32 ) };
	// 83111758: 93E100E4  stw r31, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[31].u32 ) };
	// 8311175C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83111760: 409A0010  bne cr6, 0x83111770
	if !ctx.cr[6].eq {
	pc = 0x83111770; continue 'dispatch;
	}
	// 83111764: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83111768: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8311176C: 48000178  b 0x831118e4
	pc = 0x831118E4; continue 'dispatch;
	// 83111770: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111774: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83111778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311177C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83111780: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111784: 4E800421  bctrl
	ctx.lr = 0x83111788;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111788: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8311178C: 41800158  blt 0x831118e4
	if ctx.cr[0].lt {
	pc = 0x831118E4; continue 'dispatch;
	}
	// 83111790: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831118F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831118F0 size=76
    let mut pc: u32 = 0x831118F0;
    'dispatch: loop {
        match pc {
            0x831118F0 => {
    //   block [0x831118F0..0x8311193C)
	// 831118F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831118F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831118F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831118FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83111900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83111908: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8311190C: 4BFFC715  bl 0x8310e020
	ctx.lr = 0x83111910;
	sub_8310E020(ctx, base);
	// 83111910: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83111914: 4182000C  beq 0x83111920
	if ctx.cr[0].eq {
	pc = 0x83111920; continue 'dispatch;
	}
	// 83111918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311191C: 4BFCBBBD  bl 0x830dd4d8
	ctx.lr = 0x83111920;
	sub_830DD4D8(ctx, base);
	// 83111920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83111928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8311192C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83111930: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83111934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83111938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83111940 size=132
    let mut pc: u32 = 0x83111940;
    'dispatch: loop {
        match pc {
            0x83111940 => {
    //   block [0x83111940..0x831119C4)
	// 83111940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83111948: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8311194C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83111954: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83111958: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 8311195C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83111960: 419A0008  beq cr6, 0x83111968
	if ctx.cr[6].eq {
	pc = 0x83111968; continue 'dispatch;
	}
	// 83111964: 4BFFEDC5  bl 0x83110728
	ctx.lr = 0x83111968;
	sub_83110728(ctx, base);
	// 83111968: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8311196C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83111970: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83111974: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83111978: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8311197C: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 83111980: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111984: C02A08A8  lfs f1, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83111988: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8311198C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83111990: 8168002C  lwz r11, 0x2c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(44 as u32) ) } as u64;
	// 83111994: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111998: 4E800421  bctrl
	ctx.lr = 0x8311199C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8311199C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 831119A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831119A4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831119A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831119AC: 4E800421  bctrl
	ctx.lr = 0x831119B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831119B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831119B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831119B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831119BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831119C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831119C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831119C8 size=188
    let mut pc: u32 = 0x831119C8;
    'dispatch: loop {
        match pc {
            0x831119C8 => {
    //   block [0x831119C8..0x83111A84)
	// 831119C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831119CC: 4809679D  bl 0x831a8168
	ctx.lr = 0x831119D0;
	sub_831A8130(ctx, base);
	// 831119D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831119D4: 81630148  lwz r11, 0x148(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(328 as u32) ) } as u64;
	// 831119D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831119DC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831119E0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831119E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831119E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831119EC: 409A0010  bne cr6, 0x831119fc
	if !ctx.cr[6].eq {
	pc = 0x831119FC; continue 'dispatch;
	}
	// 831119F0: 3FA08000  lis r29, -0x8000
	ctx.r[29].s64 = -2147483648;
	// 831119F4: 63BD4005  ori r29, r29, 0x4005
	ctx.r[29].u64 = ctx.r[29].u64 | 16389;
	// 831119F8: 48000080  b 0x83111a78
	pc = 0x83111A78; continue 'dispatch;
	// 831119FC: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 83111A00: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83111A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111A08: 4E800421  bctrl
	ctx.lr = 0x83111A0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111A0C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83111A10: 41800058  blt 0x83111a68
	if ctx.cr[0].lt {
	pc = 0x83111A68; continue 'dispatch;
	}
	// 83111A14: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83111A18: 4BFCBA99  bl 0x830dd4b0
	ctx.lr = 0x83111A1C;
	sub_830DD4B0(ctx, base);
	// 83111A1C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83111A20: 4182002C  beq 0x83111a4c
	if ctx.cr[0].eq {
	pc = 0x83111A4C; continue 'dispatch;
	}
	// 83111A24: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83111A28: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83111A2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83111A30: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83111A34: 396BB120  addi r11, r11, -0x4ee0
	ctx.r[11].s64 = ctx.r[11].s64 + -20192;
	// 83111A38: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83111A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111A40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83111A44: 4BFFA75D  bl 0x8310c1a0
	ctx.lr = 0x83111A48;
	sub_8310C1A0(ctx, base);
	// 83111A48: 48000008  b 0x83111a50
	pc = 0x83111A50; continue 'dispatch;
	// 83111A4C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83111A50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83111A54: 409A0010  bne cr6, 0x83111a64
	if !ctx.cr[6].eq {
	pc = 0x83111A64; continue 'dispatch;
	}
	// 83111A58: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 83111A5C: 63BD000E  ori r29, r29, 0xe
	ctx.r[29].u64 = ctx.r[29].u64 | 14;
	// 83111A60: 48000008  b 0x83111a68
	pc = 0x83111A68; continue 'dispatch;
	// 83111A64: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83111A68: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83111A6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83111A70: 419A0008  beq cr6, 0x83111a78
	if ctx.cr[6].eq {
	pc = 0x83111A78; continue 'dispatch;
	}
	// 83111A74: 4BACD565  bl 0x82bdefd8
	ctx.lr = 0x83111A78;
	sub_82BDEFD8(ctx, base);
	// 83111A78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83111A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83111A80: 48096738  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111A88 size=416
    let mut pc: u32 = 0x83111A88;
    'dispatch: loop {
        match pc {
            0x83111A88 => {
    //   block [0x83111A88..0x83111C28)
	// 83111A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111A8C: 480966CD  bl 0x831a8158
	ctx.lr = 0x83111A90;
	sub_831A8130(ctx, base);
	// 83111A90: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111A94: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 83111A98: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83111A9C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83111AA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83111AA4: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83111AA8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83111AAC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83111AB0: 93B80000  stw r29, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83111AB4: 2F1A0002  cmpwi cr6, r26, 2
	ctx.cr[6].compare_i32(ctx.r[26].s32, 2, &mut ctx.xer);
	// 83111AB8: 40980010  bge cr6, 0x83111ac8
	if !ctx.cr[6].lt {
	pc = 0x83111AC8; continue 'dispatch;
	}
	// 83111ABC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83111AC0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83111AC4: 4800015C  b 0x83111c20
	pc = 0x83111C20; continue 'dispatch;
	// 83111AC8: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 83111ACC: 3D001828  lis r8, 0x1828
	ctx.r[8].s64 = 405274624;
	// 83111AD0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 83111AD4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83111AD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83111ADC: 61080086  ori r8, r8, 0x86
	ctx.r[8].u64 = ctx.r[8].u64 | 134;
	// 83111AE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83111AE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111AE8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83111AEC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83111AF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83111AF4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83111AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111AFC: 4E800421  bctrl
	ctx.lr = 0x83111B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111B00: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83111B04: 41800108  blt 0x83111c0c
	if ctx.cr[0].lt {
	pc = 0x83111C0C; continue 'dispatch;
	}
	// 83111B08: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83111B0C: 4BFCB9A5  bl 0x830dd4b0
	ctx.lr = 0x83111B10;
	sub_830DD4B0(ctx, base);
	// 83111B10: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83111B14: 41820030  beq 0x83111b44
	if ctx.cr[0].eq {
	pc = 0x83111B44; continue 'dispatch;
	}
	// 83111B18: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83111B1C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83111B20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83111B24: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83111B28: 396BB120  addi r11, r11, -0x4ee0
	ctx.r[11].s64 = ctx.r[11].s64 + -20192;
	// 83111B2C: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83111B30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111B34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83111B38: 4BFFA669  bl 0x8310c1a0
	ctx.lr = 0x83111B3C;
	sub_8310C1A0(ctx, base);
	// 83111B3C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 83111B40: 48000008  b 0x83111b48
	pc = 0x83111B48; continue 'dispatch;
	// 83111B44: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 83111B48: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83111B4C: 409A0010  bne cr6, 0x83111b5c
	if !ctx.cr[6].eq {
	pc = 0x83111B5C; continue 'dispatch;
	}
	// 83111B50: 3FC08007  lis r30, -0x7ff9
	ctx.r[30].s64 = -2147024896;
	// 83111B54: 63DE000E  ori r30, r30, 0xe
	ctx.r[30].u64 = ctx.r[30].u64 | 14;
	// 83111B58: 480000B4  b 0x83111c0c
	pc = 0x83111C0C; continue 'dispatch;
	// 83111B5C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 83111B60: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83111B64: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83111B68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83111B6C: 38C10068  addi r6, r1, 0x68
	ctx.r[6].s64 = ctx.r[1].s64 + 104;
	// 83111B70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83111B74: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111B78: 816BDEB8  lwz r11, -0x2148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8520 as u32) ) } as u64;
	// 83111B7C: 556807BC  rlwinm r8, r11, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83111B80: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 83111B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111B88: 4E800421  bctrl
	ctx.lr = 0x83111B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111B8C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83111B90: 41800060  blt 0x83111bf0
	if ctx.cr[0].lt {
	pc = 0x83111BF0; continue 'dispatch;
	}
	// 83111B94: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111B98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83111B9C: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83111BA0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83111BA4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83111BA8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83111BAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111BB0: 816B00F4  lwz r11, 0xf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 83111BB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111BB8: 4E800421  bctrl
	ctx.lr = 0x83111BBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111BBC: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 83111BC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83111BC4: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83111BC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83111BCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83111BD0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111BD4: 816A0084  lwz r11, 0x84(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(132 as u32) ) } as u64;
	// 83111BD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111BDC: 4E800421  bctrl
	ctx.lr = 0x83111BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111BE0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83111BE4: 4198000C  blt cr6, 0x83111bf0
	if ctx.cr[6].lt {
	pc = 0x83111BF0; continue 'dispatch;
	}
	// 83111BE8: 93980000  stw r28, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83111BEC: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 83111BF0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83111BF4: 419A0018  beq cr6, 0x83111c0c
	if ctx.cr[6].eq {
	pc = 0x83111C0C; continue 'dispatch;
	}
	// 83111BF8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111BFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83111C00: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83111C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111C08: 4E800421  bctrl
	ctx.lr = 0x83111C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111C0C: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83111C10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83111C14: 419A0008  beq cr6, 0x83111c1c
	if ctx.cr[6].eq {
	pc = 0x83111C1C; continue 'dispatch;
	}
	// 83111C18: 4BACD3C1  bl 0x82bdefd8
	ctx.lr = 0x83111C1C;
	sub_82BDEFD8(ctx, base);
	// 83111C1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111C20: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83111C24: 48096584  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111C28 size=192
    let mut pc: u32 = 0x83111C28;
    'dispatch: loop {
        match pc {
            0x83111C28 => {
    //   block [0x83111C28..0x83111CE8)
	// 83111C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111C2C: 4809653D  bl 0x831a8168
	ctx.lr = 0x83111C30;
	sub_831A8130(ctx, base);
	// 83111C30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111C34: 8063014C  lwz r3, 0x14c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 83111C38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83111C3C: 3D000490  lis r8, 0x490
	ctx.r[8].s64 = 76546048;
	// 83111C40: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83111C44: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83111C48: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 83111C4C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83111C50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83111C54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111C58: 61080002  ori r8, r8, 2
	ctx.r[8].u64 = ctx.r[8].u64 | 2;
	// 83111C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83111C60: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83111C64: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83111C68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111C6C: 4E800421  bctrl
	ctx.lr = 0x83111C70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111C70: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83111C74: 41800058  blt 0x83111ccc
	if ctx.cr[0].lt {
	pc = 0x83111CCC; continue 'dispatch;
	}
	// 83111C78: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83111C7C: 4BFCB835  bl 0x830dd4b0
	ctx.lr = 0x83111C80;
	sub_830DD4B0(ctx, base);
	// 83111C80: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83111C84: 4182002C  beq 0x83111cb0
	if ctx.cr[0].eq {
	pc = 0x83111CB0; continue 'dispatch;
	}
	// 83111C88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83111C8C: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83111C90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83111C94: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83111C98: 396BB120  addi r11, r11, -0x4ee0
	ctx.r[11].s64 = ctx.r[11].s64 + -20192;
	// 83111C9C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83111CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111CA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83111CA8: 4BFFA4F9  bl 0x8310c1a0
	ctx.lr = 0x83111CAC;
	sub_8310C1A0(ctx, base);
	// 83111CAC: 48000008  b 0x83111cb4
	pc = 0x83111CB4; continue 'dispatch;
	// 83111CB0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83111CB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83111CB8: 409A0010  bne cr6, 0x83111cc8
	if !ctx.cr[6].eq {
	pc = 0x83111CC8; continue 'dispatch;
	}
	// 83111CBC: 3F808007  lis r28, -0x7ff9
	ctx.r[28].s64 = -2147024896;
	// 83111CC0: 639C000E  ori r28, r28, 0xe
	ctx.r[28].u64 = ctx.r[28].u64 | 14;
	// 83111CC4: 48000008  b 0x83111ccc
	pc = 0x83111CCC; continue 'dispatch;
	// 83111CC8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83111CCC: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83111CD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83111CD4: 419A0008  beq cr6, 0x83111cdc
	if ctx.cr[6].eq {
	pc = 0x83111CDC; continue 'dispatch;
	}
	// 83111CD8: 4BACD301  bl 0x82bdefd8
	ctx.lr = 0x83111CDC;
	sub_82BDEFD8(ctx, base);
	// 83111CDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83111CE0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83111CE4: 480964D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111CE8 size=112
    let mut pc: u32 = 0x83111CE8;
    'dispatch: loop {
        match pc {
            0x83111CE8 => {
    //   block [0x83111CE8..0x83111D58)
	// 83111CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111CEC: 48096479  bl 0x831a8164
	ctx.lr = 0x83111CF0;
	sub_831A8130(ctx, base);
	// 83111CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111CF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83111CF8: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111CFC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83111D00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83111D04: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83111D08: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83111D0C: 813E014C  lwz r9, 0x14c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 83111D10: 816BDEB8  lwz r11, -0x2148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8520 as u32) ) } as u64;
	// 83111D14: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83111D18: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83111D1C: 83E90000  lwz r31, 0(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111D20: 617B1000  ori r27, r11, 0x1000
	ctx.r[27].u64 = ctx.r[11].u64 | 4096;
	// 83111D24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83111D28: 4E800421  bctrl
	ctx.lr = 0x83111D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111D2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83111D30: 807E014C  lwz r3, 0x14c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 83111D34: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83111D38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83111D3C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83111D40: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83111D44: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 83111D48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111D4C: 4E800421  bctrl
	ctx.lr = 0x83111D50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111D50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83111D54: 48096460  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111D58 size=256
    let mut pc: u32 = 0x83111D58;
    'dispatch: loop {
        match pc {
            0x83111D58 => {
    //   block [0x83111D58..0x83111E58)
	// 83111D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111D5C: 48096411  bl 0x831a816c
	ctx.lr = 0x83111D60;
	sub_831A8130(ctx, base);
	// 83111D60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111D64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83111D68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83111D6C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83111D70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111D74: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83111D78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111D7C: 4E800421  bctrl
	ctx.lr = 0x83111D80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111D80: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83111D84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111D88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111D8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83111D90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111D94: 4E800421  bctrl
	ctx.lr = 0x83111D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111D98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83111D9C: 814B5984  lwz r10, 0x5984(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22916 as u32) ) } as u64;
	// 83111DA0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83111DA4: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83111DA8: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 83111DAC: 4082000C  bne 0x83111db8
	if !ctx.cr[0].eq {
	pc = 0x83111DB8; continue 'dispatch;
	}
	// 83111DB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83111DB4: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 83111DB8: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83111DBC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83111DC0: 409A0010  bne cr6, 0x83111dd0
	if !ctx.cr[6].eq {
	pc = 0x83111DD0; continue 'dispatch;
	}
	// 83111DC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83111DC8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83111DCC: 48000080  b 0x83111e4c
	pc = 0x83111E4C; continue 'dispatch;
	// 83111DD0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83111DD4: 4BFCB6DD  bl 0x830dd4b0
	ctx.lr = 0x83111DD8;
	sub_830DD4B0(ctx, base);
	// 83111DD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83111DDC: 41820028  beq 0x83111e04
	if ctx.cr[0].eq {
	pc = 0x83111E04; continue 'dispatch;
	}
	// 83111DE0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83111DE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83111DE8: 396BB120  addi r11, r11, -0x4ee0
	ctx.r[11].s64 = ctx.r[11].s64 + -20192;
	// 83111DEC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83111DF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83111DF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83111DF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83111DFC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83111E00: 48000008  b 0x83111e08
	pc = 0x83111E08; continue 'dispatch;
	// 83111E04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83111E08: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83111E0C: 409A0010  bne cr6, 0x83111e1c
	if !ctx.cr[6].eq {
	pc = 0x83111E1C; continue 'dispatch;
	}
	// 83111E10: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83111E14: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83111E18: 48000038  b 0x83111e50
	pc = 0x83111E50; continue 'dispatch;
	// 83111E1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83111E20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83111E24: 4BFFA37D  bl 0x8310c1a0
	ctx.lr = 0x83111E28;
	sub_8310C1A0(ctx, base);
	// 83111E28: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83111E2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83111E30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83111E34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83111E38: 4BACFC49  bl 0x82be1a80
	ctx.lr = 0x83111E3C;
	sub_82BE1A80(ctx, base);
	// 83111E3C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83111E40: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83111E44: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83111E48: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83111E4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83111E50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83111E54: 48096368  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111E58 size=216
    let mut pc: u32 = 0x83111E58;
    'dispatch: loop {
        match pc {
            0x83111E58 => {
    //   block [0x83111E58..0x83111F30)
	// 83111E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111E5C: 48096311  bl 0x831a816c
	ctx.lr = 0x83111E60;
	sub_831A8130(ctx, base);
	// 83111E60: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111E64: 39210068  addi r9, r1, 0x68
	ctx.r[9].s64 = ctx.r[1].s64 + 104;
	// 83111E68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83111E6C: 3900001C  li r8, 0x1c
	ctx.r[8].s64 = 28;
	// 83111E70: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 83111E74: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83111E78: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 83111E7C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 83111E80: FBE90000  std r31, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 83111E84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83111E88: FBE90008  std r31, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 83111E8C: 93E90010  stw r31, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 83111E90: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 83111E94: 409A0010  bne cr6, 0x83111ea4
	if !ctx.cr[6].eq {
	pc = 0x83111EA4; continue 'dispatch;
	}
	// 83111E98: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 83111E9C: 63BD0057  ori r29, r29, 0x57
	ctx.r[29].u64 = ctx.r[29].u64 | 87;
	// 83111EA0: 48000084  b 0x83111f24
	pc = 0x83111F24; continue 'dispatch;
	// 83111EA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83111EA8: 419AFFF0  beq cr6, 0x83111e98
	if ctx.cr[6].eq {
	pc = 0x83111E98; continue 'dispatch;
	}
	// 83111EAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83111EB0: 419AFFE8  beq cr6, 0x83111e98
	if ctx.cr[6].eq {
	pc = 0x83111E98; continue 'dispatch;
	}
	// 83111EB4: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 83111EB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83111EBC: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 83111EC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83111EC4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83111EC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83111ECC: 4BFFF6CD  bl 0x83111598
	ctx.lr = 0x83111ED0;
	sub_83111598(ctx, base);
	// 83111ED0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83111ED4: 41800034  blt 0x83111f08
	if ctx.cr[0].lt {
	pc = 0x83111F08; continue 'dispatch;
	}
	// 83111ED8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83111EDC: 807EDEC0  lwz r3, -0x2140(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8512 as u32) ) } as u64;
	// 83111EE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83111EE4: 419A0014  beq cr6, 0x83111ef8
	if ctx.cr[6].eq {
	pc = 0x83111EF8; continue 'dispatch;
	}
	// 83111EE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111EEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83111EF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111EF4: 4E800421  bctrl
	ctx.lr = 0x83111EF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111EF8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83111EFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83111F00: 917EDEC0  stw r11, -0x2140(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-8512 as u32), ctx.r[11].u32 ) };
	// 83111F04: 48000008  b 0x83111f0c
	pc = 0x83111F0C; continue 'dispatch;
	// 83111F08: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83111F0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83111F10: 419A0014  beq cr6, 0x83111f24
	if ctx.cr[6].eq {
	pc = 0x83111F24; continue 'dispatch;
	}
	// 83111F14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111F18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83111F1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111F20: 4E800421  bctrl
	ctx.lr = 0x83111F24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83111F24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83111F28: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83111F2C: 48096290  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83111F30 size=20
    let mut pc: u32 = 0x83111F30;
    'dispatch: loop {
        match pc {
            0x83111F30 => {
    //   block [0x83111F30..0x83111F44)
	// 83111F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83111F34: 409A0010  bne cr6, 0x83111f44
	if !ctx.cr[6].eq {
		sub_83111F44(ctx, base);
		return;
	}
	// 83111F38: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83111F3C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83111F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111F44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83111F44 size=20
    let mut pc: u32 = 0x83111F44;
    'dispatch: loop {
        match pc {
            0x83111F44 => {
    //   block [0x83111F44..0x83111F58)
	// 83111F44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83111F48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83111F4C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83111F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83111F54: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111F58 size=156
    let mut pc: u32 = 0x83111F58;
    'dispatch: loop {
        match pc {
            0x83111F58 => {
    //   block [0x83111F58..0x83111FF4)
	// 83111F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111F5C: 48096211  bl 0x831a816c
	ctx.lr = 0x83111F60;
	sub_831A8130(ctx, base);
	// 83111F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83111F64: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83111F68: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83111F6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83111F70: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83111F74: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83111F78: 4BFCB539  bl 0x830dd4b0
	ctx.lr = 0x83111F7C;
	sub_830DD4B0(ctx, base);
	// 83111F7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83111F80: 41820024  beq 0x83111fa4
	if ctx.cr[0].eq {
	pc = 0x83111FA4; continue 'dispatch;
	}
	// 83111F84: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83111F88: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 83111F8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83111F90: 396BB138  addi r11, r11, -0x4ec8
	ctx.r[11].s64 = ctx.r[11].s64 + -20168;
	// 83111F94: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83111F98: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83111F9C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83111FA0: 48000008  b 0x83111fa8
	pc = 0x83111FA8; continue 'dispatch;
	// 83111FA4: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 83111FA8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83111FAC: 409A0010  bne cr6, 0x83111fbc
	if !ctx.cr[6].eq {
	pc = 0x83111FBC; continue 'dispatch;
	}
	// 83111FB0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83111FB4: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83111FB8: 48000034  b 0x83111fec
	pc = 0x83111FEC; continue 'dispatch;
	// 83111FBC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83111FC0: 93C9000C  stw r30, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83111FC4: 814B5984  lwz r10, 0x5984(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22916 as u32) ) } as u64;
	// 83111FC8: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 83111FCC: 354A0001  addic. r10, r10, 1
	ctx.xer.ca = (ctx.r[10].u32 > (!(1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83111FD0: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 83111FD4: 4082000C  bne 0x83111fe0
	if !ctx.cr[0].eq {
	pc = 0x83111FE0; continue 'dispatch;
	}
	// 83111FD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83111FDC: 914B5984  stw r10, 0x5984(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(22916 as u32), ctx.r[10].u32 ) };
	// 83111FE0: 91090008  stw r8, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 83111FE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83111FE8: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83111FEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83111FF0: 480961CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83111FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83111FF8 size=172
    let mut pc: u32 = 0x83111FF8;
    'dispatch: loop {
        match pc {
            0x83111FF8 => {
    //   block [0x83111FF8..0x831120A4)
	// 83111FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83111FFC: 4809616D  bl 0x831a8168
	ctx.lr = 0x83112000;
	sub_831A8130(ctx, base);
	// 83112000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112004: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83112008: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8311200C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83112010: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83112014: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83112018: 817EDEC8  lwz r11, -0x2138(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 8311201C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112020: 409A0010  bne cr6, 0x83112030
	if !ctx.cr[6].eq {
	pc = 0x83112030; continue 'dispatch;
	}
	// 83112024: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 83112028: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8311202C: 48000070  b 0x8311209c
	pc = 0x8311209C; continue 'dispatch;
	// 83112030: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83112034: 4BFCB47D  bl 0x830dd4b0
	ctx.lr = 0x83112038;
	sub_830DD4B0(ctx, base);
	// 83112038: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8311203C: 4182000C  beq 0x83112048
	if ctx.cr[0].eq {
	pc = 0x83112048; continue 'dispatch;
	}
	// 83112040: 4BFFEB59  bl 0x83110b98
	ctx.lr = 0x83112044;
	sub_83110B98(ctx, base);
	// 83112044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112048: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8311204C: 409A0010  bne cr6, 0x8311205c
	if !ctx.cr[6].eq {
	pc = 0x8311205C; continue 'dispatch;
	}
	// 83112050: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83112054: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83112058: 48000044  b 0x8311209c
	pc = 0x8311209C; continue 'dispatch;
	// 8311205C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83112060: 809EDEC8  lwz r4, -0x2138(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83112064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112068: 4BFFD359  bl 0x8310f3c0
	ctx.lr = 0x8311206C;
	sub_8310F3C0(ctx, base);
	// 8311206C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83112070: 40800024  bge 0x83112094
	if !ctx.cr[0].lt {
	pc = 0x83112094; continue 'dispatch;
	}
	// 83112074: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112078: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8311207C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112080: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83112088: 4E800421  bctrl
	ctx.lr = 0x8311208C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8311208C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112090: 4800000C  b 0x8311209c
	pc = 0x8311209C; continue 'dispatch;
	// 83112094: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83112098: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8311209C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831120A0: 48096118  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831120A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831120A8 size=172
    let mut pc: u32 = 0x831120A8;
    'dispatch: loop {
        match pc {
            0x831120A8 => {
    //   block [0x831120A8..0x83112154)
	// 831120A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831120AC: 480960BD  bl 0x831a8168
	ctx.lr = 0x831120B0;
	sub_831A8130(ctx, base);
	// 831120B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831120B4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831120B8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 831120BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831120C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831120C4: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831120C8: 817EDEC8  lwz r11, -0x2138(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 831120CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831120D0: 409A0010  bne cr6, 0x831120e0
	if !ctx.cr[6].eq {
	pc = 0x831120E0; continue 'dispatch;
	}
	// 831120D4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 831120D8: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 831120DC: 48000070  b 0x8311214c
	pc = 0x8311214C; continue 'dispatch;
	// 831120E0: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 831120E4: 4BFCB3CD  bl 0x830dd4b0
	ctx.lr = 0x831120E8;
	sub_830DD4B0(ctx, base);
	// 831120E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831120EC: 4182000C  beq 0x831120f8
	if ctx.cr[0].eq {
	pc = 0x831120F8; continue 'dispatch;
	}
	// 831120F0: 4BFFEAA9  bl 0x83110b98
	ctx.lr = 0x831120F4;
	sub_83110B98(ctx, base);
	// 831120F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831120F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831120FC: 409A0010  bne cr6, 0x8311210c
	if !ctx.cr[6].eq {
	pc = 0x8311210C; continue 'dispatch;
	}
	// 83112100: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83112104: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83112108: 48000044  b 0x8311214c
	pc = 0x8311214C; continue 'dispatch;
	// 8311210C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83112110: 809EDEC8  lwz r4, -0x2138(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83112114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112118: 4BFFFC41  bl 0x83111d58
	ctx.lr = 0x8311211C;
	sub_83111D58(ctx, base);
	// 8311211C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83112120: 40800024  bge 0x83112144
	if !ctx.cr[0].lt {
	pc = 0x83112144; continue 'dispatch;
	}
	// 83112124: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112128: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8311212C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112130: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83112138: 4E800421  bctrl
	ctx.lr = 0x8311213C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8311213C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112140: 4800000C  b 0x8311214c
	pc = 0x8311214C; continue 'dispatch;
	// 83112144: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83112148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8311214C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83112150: 48096068  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112158 size=180
    let mut pc: u32 = 0x83112158;
    'dispatch: loop {
        match pc {
            0x83112158 => {
    //   block [0x83112158..0x8311220C)
	// 83112158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311215C: 48096009  bl 0x831a8164
	ctx.lr = 0x83112160;
	sub_831A8130(ctx, base);
	// 83112160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112164: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83112168: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8311216C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83112170: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83112174: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83112178: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8311217C: 817EDEC8  lwz r11, -0x2138(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83112180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112184: 409A0010  bne cr6, 0x83112194
	if !ctx.cr[6].eq {
	pc = 0x83112194; continue 'dispatch;
	}
	// 83112188: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8311218C: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 83112190: 48000074  b 0x83112204
	pc = 0x83112204; continue 'dispatch;
	// 83112194: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 83112198: 4BFCB319  bl 0x830dd4b0
	ctx.lr = 0x8311219C;
	sub_830DD4B0(ctx, base);
	// 8311219C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831121A0: 4182000C  beq 0x831121ac
	if ctx.cr[0].eq {
	pc = 0x831121AC; continue 'dispatch;
	}
	// 831121A4: 4BFFEB5D  bl 0x83110d00
	ctx.lr = 0x831121A8;
	sub_83110D00(ctx, base);
	// 831121A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831121AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831121B0: 409A0010  bne cr6, 0x831121c0
	if !ctx.cr[6].eq {
	pc = 0x831121C0; continue 'dispatch;
	}
	// 831121B4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831121B8: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 831121BC: 48000048  b 0x83112204
	pc = 0x83112204; continue 'dispatch;
	// 831121C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 831121C4: 809EDEC8  lwz r4, -0x2138(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 831121C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831121CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831121D0: 4BFFD361  bl 0x8310f530
	ctx.lr = 0x831121D4;
	sub_8310F530(ctx, base);
	// 831121D4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831121D8: 40800024  bge 0x831121fc
	if !ctx.cr[0].lt {
	pc = 0x831121FC; continue 'dispatch;
	}
	// 831121DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831121E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831121E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831121E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831121EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831121F0: 4E800421  bctrl
	ctx.lr = 0x831121F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831121F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831121F8: 4800000C  b 0x83112204
	pc = 0x83112204; continue 'dispatch;
	// 831121FC: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83112200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83112204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83112208: 48095FAC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112210 size=180
    let mut pc: u32 = 0x83112210;
    'dispatch: loop {
        match pc {
            0x83112210 => {
    //   block [0x83112210..0x831122C4)
	// 83112210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112214: 48095F51  bl 0x831a8164
	ctx.lr = 0x83112218;
	sub_831A8130(ctx, base);
	// 83112218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311221C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83112220: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83112224: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83112228: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8311222C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83112230: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83112234: 817EDEC8  lwz r11, -0x2138(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83112238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8311223C: 409A0010  bne cr6, 0x8311224c
	if !ctx.cr[6].eq {
	pc = 0x8311224C; continue 'dispatch;
	}
	// 83112240: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 83112244: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 83112248: 48000074  b 0x831122bc
	pc = 0x831122BC; continue 'dispatch;
	// 8311224C: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 83112250: 4BFCB261  bl 0x830dd4b0
	ctx.lr = 0x83112254;
	sub_830DD4B0(ctx, base);
	// 83112254: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83112258: 4182000C  beq 0x83112264
	if ctx.cr[0].eq {
	pc = 0x83112264; continue 'dispatch;
	}
	// 8311225C: 4BFFEC05  bl 0x83110e60
	ctx.lr = 0x83112260;
	sub_83110E60(ctx, base);
	// 83112260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112264: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83112268: 409A0010  bne cr6, 0x83112278
	if !ctx.cr[6].eq {
	pc = 0x83112278; continue 'dispatch;
	}
	// 8311226C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83112270: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83112274: 48000048  b 0x831122bc
	pc = 0x831122BC; continue 'dispatch;
	// 83112278: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8311227C: 809EDEC8  lwz r4, -0x2138(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 83112280: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83112284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112288: 4BFFD609  bl 0x8310f890
	ctx.lr = 0x8311228C;
	sub_8310F890(ctx, base);
	// 8311228C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83112290: 40800024  bge 0x831122b4
	if !ctx.cr[0].lt {
	pc = 0x831122B4; continue 'dispatch;
	}
	// 83112294: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112298: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8311229C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831122A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831122A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831122A8: 4E800421  bctrl
	ctx.lr = 0x831122AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831122AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831122B0: 4800000C  b 0x831122bc
	pc = 0x831122BC; continue 'dispatch;
	// 831122B4: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831122B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831122BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831122C0: 48095EF4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831122C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831122C8 size=80
    let mut pc: u32 = 0x831122C8;
    'dispatch: loop {
        match pc {
            0x831122C8 => {
    //   block [0x831122C8..0x83112318)
	// 831122C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831122CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831122D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831122D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831122D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831122DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831122E0: 409A0010  bne cr6, 0x831122f0
	if !ctx.cr[6].eq {
	pc = 0x831122F0; continue 'dispatch;
	}
	// 831122E4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831122E8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 831122EC: 48000018  b 0x83112304
	pc = 0x83112304; continue 'dispatch;
	// 831122F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831122F4: 4BFFA6BD  bl 0x8310c9b0
	ctx.lr = 0x831122F8;
	sub_8310C9B0(ctx, base);
	// 831122F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831122FC: 4BFCB1DD  bl 0x830dd4d8
	ctx.lr = 0x83112300;
	sub_830DD4D8(ctx, base);
	// 83112300: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83112304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83112308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8311230C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83112310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83112314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112318 size=260
    let mut pc: u32 = 0x83112318;
    'dispatch: loop {
        match pc {
            0x83112318 => {
    //   block [0x83112318..0x8311241C)
	// 83112318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311231C: 48095E45  bl 0x831a8160
	ctx.lr = 0x83112320;
	sub_831A8130(ctx, base);
	// 83112320: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112324: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83112328: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8311232C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83112330: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83112334: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83112338: 817EDEC8  lwz r11, -0x2138(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 8311233C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112340: 409A0010  bne cr6, 0x83112350
	if !ctx.cr[6].eq {
	pc = 0x83112350; continue 'dispatch;
	}
	// 83112344: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 83112348: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8311234C: 480000C8  b 0x83112414
	pc = 0x83112414; continue 'dispatch;
	// 83112350: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83112354: 816BDEC4  lwz r11, -0x213c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8508 as u32) ) } as u64;
	// 83112358: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8311235C: 409A0010  bne cr6, 0x8311236c
	if !ctx.cr[6].eq {
	pc = 0x8311236C; continue 'dispatch;
	}
	// 83112360: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 83112364: 60630030  ori r3, r3, 0x30
	ctx.r[3].u64 = ctx.r[3].u64 | 48;
	// 83112368: 480000AC  b 0x83112414
	pc = 0x83112414; continue 'dispatch;
	// 8311236C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83112370: 4BFCB141  bl 0x830dd4b0
	ctx.lr = 0x83112374;
	sub_830DD4B0(ctx, base);
	// 83112374: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83112378: 41820028  beq 0x831123a0
	if ctx.cr[0].eq {
	pc = 0x831123A0; continue 'dispatch;
	}
	// 8311237C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83112380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83112384: 396BB120  addi r11, r11, -0x4ee0
	ctx.r[11].s64 = ctx.r[11].s64 + -20192;
	// 83112388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8311238C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83112390: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83112394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112398: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8311239C: 48000008  b 0x831123a4
	pc = 0x831123A4; continue 'dispatch;
	// 831123A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831123A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831123A8: 409A0010  bne cr6, 0x831123b8
	if !ctx.cr[6].eq {
	pc = 0x831123B8; continue 'dispatch;
	}
	// 831123AC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 831123B0: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 831123B4: 48000060  b 0x83112414
	pc = 0x83112414; continue 'dispatch;
	// 831123B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831123BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831123C0: 4BFF9DE1  bl 0x8310c1a0
	ctx.lr = 0x831123C4;
	sub_8310C1A0(ctx, base);
	// 831123C4: 807EDEC8  lwz r3, -0x2138(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 831123C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831123CC: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 831123D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831123D4: 4E800421  bctrl
	ctx.lr = 0x831123D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831123D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831123DC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 831123E0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 831123E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831123E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831123EC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831123F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831123F4: 4E800421  bctrl
	ctx.lr = 0x831123F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831123F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831123FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83112400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112404: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83112408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311240C: 4E800421  bctrl
	ctx.lr = 0x83112410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83112410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112414: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83112418: 48095D98  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112420 size=188
    let mut pc: u32 = 0x83112420;
    'dispatch: loop {
        match pc {
            0x83112420 => {
    //   block [0x83112420..0x831124DC)
	// 83112420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83112428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8311242C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83112430: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112438: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8311243C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83112440: 396BB1A8  addi r11, r11, -0x4e58
	ctx.r[11].s64 = ctx.r[11].s64 + -20056;
	// 83112444: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 83112448: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8311244C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83112450: 48095D91  bl 0x831a81e0
	ctx.lr = 0x83112454;
	sub_831A81E0(ctx, base);
	// 83112454: 3CE01828  lis r7, 0x1828
	ctx.r[7].s64 = 405274624;
	// 83112458: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8311245C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 83112460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83112464: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 83112468: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8311246C: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 83112470: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83112474: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83112478: 60E70086  ori r7, r7, 0x86
	ctx.r[7].u64 = ctx.r[7].u64 | 134;
	// 8311247C: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83112480: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83112484: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 83112488: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8311248C: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 83112490: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83112494: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 83112498: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8311249C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831124A0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 831124A4: 4BBBD74D  bl 0x82ccfbf0
	ctx.lr = 0x831124A8;
	sub_82CCFBF0(ctx, base);
	// 831124A8: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 831124AC: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 831124B0: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 831124B4: 48130529  bl 0x832429dc
	ctx.lr = 0x831124B8;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 831124B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831124BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831124C0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 831124C4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 831124C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831124CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831124D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831124D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831124D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831124E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831124E0 size=16
    let mut pc: u32 = 0x831124E0;
    'dispatch: loop {
        match pc {
            0x831124E0 => {
    //   block [0x831124E0..0x831124F0)
	// 831124E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831124E4: 806B0050  lwz r3, 0x50(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 831124E8: 908B0050  stw r4, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 831124EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831124F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831124F0 size=40
    let mut pc: u32 = 0x831124F0;
    'dispatch: loop {
        match pc {
            0x831124F0 => {
    //   block [0x831124F0..0x83112518)
	// 831124F0: 39630080  addi r11, r3, 0x80
	ctx.r[11].s64 = ctx.r[3].s64 + 128;
	// 831124F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 831124F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 831124FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83112500: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83112504: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83112508: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8311250C: 4082FFE8  bne 0x831124f4
	if !ctx.cr[0].eq {
	pc = 0x831124F4; continue 'dispatch;
	}
	// 83112510: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83112514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112518 size=128
    let mut pc: u32 = 0x83112518;
    'dispatch: loop {
        match pc {
            0x83112518 => {
    //   block [0x83112518..0x83112598)
	// 83112518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311251C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83112520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83112524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83112528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311252C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83112530: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83112534: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83112538: 409A0010  bne cr6, 0x83112548
	if !ctx.cr[6].eq {
	pc = 0x83112548; continue 'dispatch;
	}
	// 8311253C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83112540: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 83112544: 4800003C  b 0x83112580
	pc = 0x83112580; continue 'dispatch;
	// 83112548: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8311254C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83112550: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83112554: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112558: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311255C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83112560: 4E800421  bctrl
	ctx.lr = 0x83112564;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83112564: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83112568: 419A0014  beq cr6, 0x8311257c
	if ctx.cr[6].eq {
	pc = 0x8311257C; continue 'dispatch;
	}
	// 8311256C: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 83112570: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 83112574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112578: 48095F99  bl 0x831a8510
	ctx.lr = 0x8311257C;
	sub_831A8510(ctx, base);
	// 8311257C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83112580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83112584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83112588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8311258C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83112590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83112594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112598 size=84
    let mut pc: u32 = 0x83112598;
    'dispatch: loop {
        match pc {
            0x83112598 => {
    //   block [0x83112598..0x831125EC)
	// 83112598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831125A0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831125A4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 831125A8: 3CE01828  lis r7, 0x1828
	ctx.r[7].s64 = 405274624;
	// 831125AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831125B0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 831125B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831125B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831125BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 831125C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831125C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 831125C8: 60E70086  ori r7, r7, 0x86
	ctx.r[7].u64 = ctx.r[7].u64 | 134;
	// 831125CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831125D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831125D4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831125D8: 4BBBD619  bl 0x82ccfbf0
	ctx.lr = 0x831125DC;
	sub_82CCFBF0(ctx, base);
	// 831125DC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831125E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831125E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831125E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831125F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831125F0 size=96
    let mut pc: u32 = 0x831125F0;
    'dispatch: loop {
        match pc {
            0x831125F0 => {
    //   block [0x831125F0..0x83112650)
	// 831125F0: 81640048  lwz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 831125F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831125F8: 419A000C  beq cr6, 0x83112604
	if ctx.cr[6].eq {
	pc = 0x83112604; continue 'dispatch;
	}
	// 831125FC: 81440044  lwz r10, 0x44(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 83112600: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 83112604: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 83112608: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8311260C: 419A000C  beq cr6, 0x83112618
	if ctx.cr[6].eq {
	pc = 0x83112618; continue 'dispatch;
	}
	// 83112610: 81440048  lwz r10, 0x48(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 83112614: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 83112618: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8311261C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83112620: 409A000C  bne cr6, 0x8311262c
	if !ctx.cr[6].eq {
	pc = 0x8311262C; continue 'dispatch;
	}
	// 83112624: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 83112628: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8311262C: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 83112630: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83112634: 409A000C  bne cr6, 0x83112640
	if !ctx.cr[6].eq {
	pc = 0x83112640; continue 'dispatch;
	}
	// 83112638: 81640048  lwz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 8311263C: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83112640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83112644: 91640044  stw r11, 0x44(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83112648: 91640048  stw r11, 0x48(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8311264C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112650 size=92
    let mut pc: u32 = 0x83112650;
    'dispatch: loop {
        match pc {
            0x83112650 => {
    //   block [0x83112650..0x831126AC)
	// 83112650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83112658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8311265C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83112660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112664: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83112668: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8311266C: 48000018  b 0x83112684
	pc = 0x83112684; continue 'dispatch;
	// 83112670: 480A0361  bl 0x831b29d0
	ctx.lr = 0x83112674;
	sub_831B29D0(ctx, base);
	// 83112674: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83112678: 1D5E0021  mulli r10, r30, 0x21
	ctx.r[10].s64 = ctx.r[30].s64 * 33;
	// 8311267C: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 83112680: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83112684: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112688: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8311268C: 4082FFE4  bne 0x83112670
	if !ctx.cr[0].eq {
	pc = 0x83112670; continue 'dispatch;
	}
	// 83112690: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112694: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83112698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8311269C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831126A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831126A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831126A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831126B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831126B0 size=120
    let mut pc: u32 = 0x831126B0;
    'dispatch: loop {
        match pc {
            0x831126B0 => {
    //   block [0x831126B0..0x83112728)
	// 831126B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831126B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831126B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831126BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831126C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831126C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831126C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831126CC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 831126D0: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 831126D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831126D8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 831126DC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 831126E0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 831126E4: 48095AFD  bl 0x831a81e0
	ctx.lr = 0x831126E8;
	sub_831A81E0(ctx, base);
	// 831126E8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 831126EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831126F0: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 831126F4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 831126F8: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 831126FC: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 83112700: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 83112704: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 83112708: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 8311270C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 83112710: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83112714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83112718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8311271C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83112720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83112724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112728 size=88
    let mut pc: u32 = 0x83112728;
    'dispatch: loop {
        match pc {
            0x83112728 => {
    //   block [0x83112728..0x83112780)
	// 83112728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311272C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83112730: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83112734: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8311273C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112740: 4BFCAD99  bl 0x830dd4d8
	ctx.lr = 0x83112744;
	sub_830DD4D8(ctx, base);
	// 83112744: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83112748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8311274C: 419A0018  beq cr6, 0x83112764
	if ctx.cr[6].eq {
	pc = 0x83112764; continue 'dispatch;
	}
	// 83112750: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83112754: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112758: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8311275C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83112760: 4E800421  bctrl
	ctx.lr = 0x83112764;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83112764: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83112768: 4BFCAD71  bl 0x830dd4d8
	ctx.lr = 0x8311276C;
	sub_830DD4D8(ctx, base);
	// 8311276C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83112770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83112774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83112778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8311277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112780 size=176
    let mut pc: u32 = 0x83112780;
    'dispatch: loop {
        match pc {
            0x83112780 => {
    //   block [0x83112780..0x83112830)
	// 83112780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112784: 480959D9  bl 0x831a815c
	ctx.lr = 0x83112788;
	sub_831A8130(ctx, base);
	// 83112788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311278C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83112790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112794: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83112798: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8311279C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 831127A0: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 831127A4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831127A8: 41980078  blt cr6, 0x83112820
	if ctx.cr[6].lt {
	pc = 0x83112820; continue 'dispatch;
	}
	// 831127AC: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 831127B0: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 831127B4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831127B8: 40980068  bge cr6, 0x83112820
	if !ctx.cr[6].lt {
	pc = 0x83112820; continue 'dispatch;
	}
	// 831127BC: 57BC1838  slwi r28, r29, 3
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 831127C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831127C4: 4BFCACED  bl 0x830dd4b0
	ctx.lr = 0x831127C8;
	sub_830DD4B0(ctx, base);
	// 831127C8: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 831127CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831127D0: 41820050  beq 0x83112820
	if ctx.cr[0].eq {
	pc = 0x83112820; continue 'dispatch;
	}
	// 831127D4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831127D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831127DC: 48095D35  bl 0x831a8510
	ctx.lr = 0x831127E0;
	sub_831A8510(ctx, base);
	// 831127E0: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 831127E4: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 831127E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831127EC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 831127F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831127F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831127F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831127FC: 4E800421  bctrl
	ctx.lr = 0x83112800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83112800: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83112804: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 83112808: 933F0038  stw r25, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[25].u32 ) };
	// 8311280C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83112810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83112814: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83112818: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8311281C: 4800000C  b 0x83112828
	pc = 0x83112828; continue 'dispatch;
	// 83112820: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83112824: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83112828: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8311282C: 48095980  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112830 size=112
    let mut pc: u32 = 0x83112830;
    'dispatch: loop {
        match pc {
            0x83112830 => {
    //   block [0x83112830..0x831128A0)
	// 83112830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112834: 48095931  bl 0x831a8164
	ctx.lr = 0x83112838;
	sub_831A8130(ctx, base);
	// 83112838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311283C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112840: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83112844: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83112848: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8311284C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83112850: 4BFBC439  bl 0x830cec88
	ctx.lr = 0x83112854;
	sub_830CEC88(ctx, base);
	// 83112854: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83112858: 41800040  blt 0x83112898
	if ctx.cr[0].lt {
	pc = 0x83112898; continue 'dispatch;
	}
	// 8311285C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83112860: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112864: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 83112868: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311286C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83112874: 4E800421  bctrl
	ctx.lr = 0x83112878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83112878: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8311287C: 937F0038  stw r27, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[27].u32 ) };
	// 83112880: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83112884: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83112888: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8311288C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83112890: 48095C81  bl 0x831a8510
	ctx.lr = 0x83112894;
	sub_831A8510(ctx, base);
	// 83112894: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83112898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8311289C: 48095918  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831128A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831128A0 size=236
    let mut pc: u32 = 0x831128A0;
    'dispatch: loop {
        match pc {
            0x831128A0 => {
    //   block [0x831128A0..0x8311298C)
	// 831128A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831128A4: 480958BD  bl 0x831a8160
	ctx.lr = 0x831128A8;
	sub_831A8130(ctx, base);
	// 831128A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831128AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831128B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831128B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831128B8: 419A00CC  beq cr6, 0x83112984
	if ctx.cr[6].eq {
	pc = 0x83112984; continue 'dispatch;
	}
	// 831128BC: 3B7D0064  addi r27, r29, 0x64
	ctx.r[27].s64 = ctx.r[29].s64 + 100;
	// 831128C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831128C4: 481300A9  bl 0x8324296c
	ctx.lr = 0x831128C8;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 831128C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831128CC: 3B9D0004  addi r28, r29, 4
	ctx.r[28].s64 = ctx.r[29].s64 + 4;
	// 831128D0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831128D4: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 831128D8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831128DC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831128E0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831128E4: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831128E8: 4800006C  b 0x83112954
	pc = 0x83112954; continue 'dispatch;
	// 831128EC: 807D0060  lwz r3, 0x60(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 831128F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831128F4: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 831128F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831128FC: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 83112900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83112904: 4E800421  bctrl
	ctx.lr = 0x83112908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83112908: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8311290C: 41800064  blt 0x83112970
	if ctx.cr[0].lt {
	pc = 0x83112970; continue 'dispatch;
	}
	// 83112910: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83112914: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83112918: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8311291C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83112920: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83112924: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83112928: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8311292C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83112930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112934: 409A001C  bne cr6, 0x83112950
	if !ctx.cr[6].eq {
	pc = 0x83112950; continue 'dispatch;
	}
	// 83112938: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8311293C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83112940: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83112944: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 83112948: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8311294C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83112950: 83DE003C  lwz r30, 0x3c(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83112954: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83112958: 409AFF94  bne cr6, 0x831128ec
	if !ctx.cr[6].eq {
	pc = 0x831128EC; continue 'dispatch;
	}
	// 8311295C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 83112960: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83112964: 2F1A0011  cmpwi cr6, r26, 0x11
	ctx.cr[6].compare_i32(ctx.r[26].s32, 17, &mut ctx.xer);
	// 83112968: 4198FF7C  blt cr6, 0x831128e4
	if ctx.cr[6].lt {
	pc = 0x831128E4; continue 'dispatch;
	}
	// 8311296C: 48000010  b 0x8311297c
	pc = 0x8311297C; continue 'dispatch;
	// 83112970: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83112974: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83112978: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8311297C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83112980: 4812FFDD  bl 0x8324295c
	ctx.lr = 0x83112984;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83112984: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83112988: 48095828  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112990 size=116
    let mut pc: u32 = 0x83112990;
    'dispatch: loop {
        match pc {
            0x83112990 => {
    //   block [0x83112990..0x83112A04)
	// 83112990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112994: 480957D9  bl 0x831a816c
	ctx.lr = 0x83112998;
	sub_831A8130(ctx, base);
	// 83112998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311299C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831129A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831129A4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831129A8: 419A0054  beq cr6, 0x831129fc
	if ctx.cr[6].eq {
	pc = 0x831129FC; continue 'dispatch;
	}
	// 831129AC: 3BBF0064  addi r29, r31, 0x64
	ctx.r[29].s64 = ctx.r[31].s64 + 100;
	// 831129B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831129B4: 4812FFB9  bl 0x8324296c
	ctx.lr = 0x831129B8;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 831129B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831129BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831129C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831129C4: 816BDEBC  lwz r11, -0x2144(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8516 as u32) ) } as u64;
	// 831129C8: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 831129CC: 4BFFFC25  bl 0x831125f0
	ctx.lr = 0x831129D0;
	sub_831125F0(ctx, base);
	// 831129D0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831129D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831129D8: 409A000C  bne cr6, 0x831129e4
	if !ctx.cr[6].eq {
	pc = 0x831129E4; continue 'dispatch;
	}
	// 831129DC: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 831129E0: 48000010  b 0x831129f0
	pc = 0x831129F0; continue 'dispatch;
	// 831129E4: 93CB0044  stw r30, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 831129E8: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831129EC: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 831129F0: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 831129F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831129F8: 4812FF65  bl 0x8324295c
	ctx.lr = 0x831129FC;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831129FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83112A00: 480957BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112A08 size=140
    let mut pc: u32 = 0x83112A08;
    'dispatch: loop {
        match pc {
            0x83112A08 => {
    //   block [0x83112A08..0x83112A94)
	// 83112A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112A0C: 4809575D  bl 0x831a8168
	ctx.lr = 0x83112A10;
	sub_831A8130(ctx, base);
	// 83112A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112A18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83112A1C: 3BBF0064  addi r29, r31, 0x64
	ctx.r[29].s64 = ctx.r[31].s64 + 100;
	// 83112A20: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83112A24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83112A28: 4812FF45  bl 0x8324296c
	ctx.lr = 0x83112A2C;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83112A2C: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 83112A30: 7D7E5B96  divwu r11, r30, r11
	ctx.r[11].u32 = ctx.r[30].u32 / ctx.r[11].u32;
	// 83112A34: 1D6B0011  mulli r11, r11, 0x11
	ctx.r[11].s64 = ctx.r[11].s64 * 17;
	// 83112A38: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 83112A3C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83112A40: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83112A44: 7FEBF82E  lwzx r31, r11, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83112A48: 48000030  b 0x83112a78
	pc = 0x83112A78; continue 'dispatch;
	// 83112A4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83112A50: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83112A54: 409A0020  bne cr6, 0x83112a74
	if !ctx.cr[6].eq {
	pc = 0x83112A74; continue 'dispatch;
	}
	// 83112A58: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112A5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83112A60: 419A0014  beq cr6, 0x83112a74
	if ctx.cr[6].eq {
	pc = 0x83112A74; continue 'dispatch;
	}
	// 83112A64: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83112A68: 4809F051  bl 0x831b1ab8
	ctx.lr = 0x83112A6C;
	sub_831B1AB8(ctx, base);
	// 83112A6C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83112A70: 41820010  beq 0x83112a80
	if ctx.cr[0].eq {
	pc = 0x83112A80; continue 'dispatch;
	}
	// 83112A74: 83FF003C  lwz r31, 0x3c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83112A78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83112A7C: 409AFFD0  bne cr6, 0x83112a4c
	if !ctx.cr[6].eq {
	pc = 0x83112A4C; continue 'dispatch;
	}
	// 83112A80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83112A84: 4812FED9  bl 0x8324295c
	ctx.lr = 0x83112A88;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83112A88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83112A90: 48095728  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112A98 size=156
    let mut pc: u32 = 0x83112A98;
    'dispatch: loop {
        match pc {
            0x83112A98 => {
    //   block [0x83112A98..0x83112B34)
	// 83112A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112A9C: 480956C9  bl 0x831a8164
	ctx.lr = 0x83112AA0;
	sub_831A8130(ctx, base);
	// 83112AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112AA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83112AAC: 3B7F0064  addi r27, r31, 0x64
	ctx.r[27].s64 = ctx.r[31].s64 + 100;
	// 83112AB0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83112AB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83112AB8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83112ABC: 4812FEB1  bl 0x8324296c
	ctx.lr = 0x83112AC0;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83112AC0: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 83112AC4: 7D7E5B96  divwu r11, r30, r11
	ctx.r[11].u32 = ctx.r[30].u32 / ctx.r[11].u32;
	// 83112AC8: 1D6B0011  mulli r11, r11, 0x11
	ctx.r[11].s64 = ctx.r[11].s64 * 17;
	// 83112ACC: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 83112AD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83112AD4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83112AD8: 7FEBF82E  lwzx r31, r11, r31
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83112ADC: 4800003C  b 0x83112b18
	pc = 0x83112B18; continue 'dispatch;
	// 83112AE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83112AE4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83112AE8: 409A002C  bne cr6, 0x83112b14
	if !ctx.cr[6].eq {
	pc = 0x83112B14; continue 'dispatch;
	}
	// 83112AEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112AF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112AF4: 409A0020  bne cr6, 0x83112b14
	if !ctx.cr[6].eq {
	pc = 0x83112B14; continue 'dispatch;
	}
	// 83112AF8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83112AFC: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83112B00: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83112B04: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83112B08: 48007991  bl 0x8311a498
	ctx.lr = 0x83112B0C;
	sub_8311A498(ctx, base);
	// 83112B0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83112B10: 40820010  bne 0x83112b20
	if !ctx.cr[0].eq {
	pc = 0x83112B20; continue 'dispatch;
	}
	// 83112B14: 83FF003C  lwz r31, 0x3c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83112B18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83112B1C: 409AFFC4  bne cr6, 0x83112ae0
	if !ctx.cr[6].eq {
	pc = 0x83112AE0; continue 'dispatch;
	}
	// 83112B20: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83112B24: 4812FE39  bl 0x8324295c
	ctx.lr = 0x83112B28;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83112B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112B2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83112B30: 48095684  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112B38 size=148
    let mut pc: u32 = 0x83112B38;
    'dispatch: loop {
        match pc {
            0x83112B38 => {
    //   block [0x83112B38..0x83112BCC)
	// 83112B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112B3C: 48095621  bl 0x831a815c
	ctx.lr = 0x83112B40;
	sub_831A8130(ctx, base);
	// 83112B40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112B48: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83112B4C: 3B3F0064  addi r25, r31, 0x64
	ctx.r[25].s64 = ctx.r[31].s64 + 100;
	// 83112B50: 396BB1A8  addi r11, r11, -0x4e58
	ctx.r[11].s64 = ctx.r[11].s64 + -20056;
	// 83112B54: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83112B58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83112B5C: 4812FE11  bl 0x8324296c
	ctx.lr = 0x83112B60;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83112B60: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 83112B64: 3B600011  li r27, 0x11
	ctx.r[27].s64 = 17;
	// 83112B68: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83112B6C: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112B70: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83112B74: 419A0024  beq cr6, 0x83112b98
	if ctx.cr[6].eq {
	pc = 0x83112B98; continue 'dispatch;
	}
	// 83112B78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83112B7C: 835C003C  lwz r26, 0x3c(r28)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 83112B80: 4BFFFBA9  bl 0x83112728
	ctx.lr = 0x83112B84;
	sub_83112728(ctx, base);
	// 83112B84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83112B88: 4BFCA951  bl 0x830dd4d8
	ctx.lr = 0x83112B8C;
	sub_830DD4D8(ctx, base);
	// 83112B8C: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 83112B90: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 83112B94: 409AFFE4  bne cr6, 0x83112b78
	if !ctx.cr[6].eq {
	pc = 0x83112B78; continue 'dispatch;
	}
	// 83112B98: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83112B9C: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83112BA0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83112BA4: 4082FFC8  bne 0x83112b6c
	if !ctx.cr[0].eq {
	pc = 0x83112B6C; continue 'dispatch;
	}
	// 83112BA8: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 83112BAC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83112BB0: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 83112BB4: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83112BB8: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 83112BBC: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83112BC0: 4812FD9D  bl 0x8324295c
	ctx.lr = 0x83112BC4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83112BC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83112BC8: 480955E4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83112BD0 size=52
    let mut pc: u32 = 0x83112BD0;
    'dispatch: loop {
        match pc {
            0x83112BD0 => {
    //   block [0x83112BD0..0x83112C04)
	// 83112BD0: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 83112BD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112BD8: 409A002C  bne cr6, 0x83112c04
	if !ctx.cr[6].eq {
		sub_83112C04(ctx, base);
		return;
	}
	// 83112BDC: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83112BE0: 39400011  li r10, 0x11
	ctx.r[10].s64 = 17;
	// 83112BE4: 8124003C  lwz r9, 0x3c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 83112BE8: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 83112BEC: 1D4A0011  mulli r10, r10, 0x11
	ctx.r[10].s64 = ctx.r[10].s64 * 17;
	// 83112BF0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83112BF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83112BF8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83112BFC: 7D2B192E  stwx r9, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 83112C00: 4800000C  b 0x83112c0c
	sub_83112C04(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112C04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83112C04 size=32
    let mut pc: u32 = 0x83112C04;
    'dispatch: loop {
        match pc {
            0x83112C04 => {
    //   block [0x83112C04..0x83112C24)
	// 83112C04: 8144003C  lwz r10, 0x3c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 83112C08: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 83112C0C: 8164003C  lwz r11, 0x3c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 83112C10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112C14: 419A000C  beq cr6, 0x83112c20
	if ctx.cr[6].eq {
	pc = 0x83112C20; continue 'dispatch;
	}
	// 83112C18: 81440040  lwz r10, 0x40(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 83112C1C: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83112C20: 4BFFF9D0  b 0x831125f0
	sub_831125F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112C28 size=128
    let mut pc: u32 = 0x83112C28;
    'dispatch: loop {
        match pc {
            0x83112C28 => {
    //   block [0x83112C28..0x83112CA8)
	// 83112C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112C2C: 48095541  bl 0x831a816c
	ctx.lr = 0x83112C30;
	sub_831A8130(ctx, base);
	// 83112C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112C38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83112C3C: 3BBF0064  addi r29, r31, 0x64
	ctx.r[29].s64 = ctx.r[31].s64 + 100;
	// 83112C40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83112C44: 4812FD29  bl 0x8324296c
	ctx.lr = 0x83112C48;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83112C48: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83112C4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112C50: 409A0048  bne cr6, 0x83112c98
	if !ctx.cr[6].eq {
	pc = 0x83112C98; continue 'dispatch;
	}
	// 83112C54: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83112C58: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83112C5C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83112C60: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83112C64: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83112C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83112C6C: 419A0010  beq cr6, 0x83112c7c
	if ctx.cr[6].eq {
	pc = 0x83112C7C; continue 'dispatch;
	}
	// 83112C70: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83112C74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83112C78: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83112C7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83112C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112C84: 4BFFFF4D  bl 0x83112bd0
	ctx.lr = 0x83112C88;
	sub_83112BD0(ctx, base);
	// 83112C88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112C8C: 4BFFFA9D  bl 0x83112728
	ctx.lr = 0x83112C90;
	sub_83112728(ctx, base);
	// 83112C90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83112C94: 4BFCA845  bl 0x830dd4d8
	ctx.lr = 0x83112C98;
	sub_830DD4D8(ctx, base);
	// 83112C98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83112C9C: 4812FCC1  bl 0x8324295c
	ctx.lr = 0x83112CA0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83112CA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83112CA4: 48095518  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83112CA8 size=68
    let mut pc: u32 = 0x83112CA8;
    'dispatch: loop {
        match pc {
            0x83112CA8 => {
    //   block [0x83112CA8..0x83112CEC)
	// 83112CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83112CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83112CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83112CBC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83112CC0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83112CC4: 396BAE90  addi r11, r11, -0x5170
	ctx.r[11].s64 = ctx.r[11].s64 + -20848;
	// 83112CC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83112CCC: 41820008  beq 0x83112cd4
	if ctx.cr[0].eq {
	pc = 0x83112CD4; continue 'dispatch;
	}
	// 83112CD0: 4BFCA809  bl 0x830dd4d8
	ctx.lr = 0x83112CD4;
	sub_830DD4D8(ctx, base);
	// 83112CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83112CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83112CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83112CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83112CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83112CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83112CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83112CF0 size=952
    let mut pc: u32 = 0x83112CF0;
    'dispatch: loop {
        match pc {
            0x83112CF0 => {
    //   block [0x83112CF0..0x831130A8)
	// 83112CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83112CF4: 4809545D  bl 0x831a8150
	ctx.lr = 0x83112CF8;
	sub_831A8130(ctx, base);
	// 83112CF8: DBC1FF98  stfd f30, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 83112CFC: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 83112D00: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83112D04: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83112D08: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83112D0C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83112D10: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83112D14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83112D18: 3B0BAEC4  addi r24, r11, -0x513c
	ctx.r[24].s64 = ctx.r[11].s64 + -20796;
	// 83112D1C: C3E99F60  lfs f31, -0x60a0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-24736 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83112D20: C3C81FBC  lfs f30, 0x1fbc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8124 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 83112D24: 93A10084  stw r29, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 83112D28: D3E10094  stfs f31, 0x94(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 83112D2C: 93010080  stw r24, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 83112D30: D3E10098  stfs f31, 0x98(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 83112D34: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 83112D38: D3C1009C  stfs f30, 0x9c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 83112D3C: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 83112D40: D3C100A0  stfs f30, 0xa0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 83112D44: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 83112D48: 93A100A4  stw r29, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 83112D4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83112D50: 914100A8  stw r10, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 83112D54: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 83112D58: 93A100B0  stw r29, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[29].u32 ) };
	// 83112D5C: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 83112D60: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83112D64: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83112D68: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 83112D6C: 4BFF9CB5  bl 0x8310ca20
	ctx.lr = 0x83112D70;
	sub_8310CA20(ctx, base);
	// 83112D70: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83112D74: 418002F8  blt 0x8311306c
	if ctx.cr[0].lt {
	pc = 0x8311306C; continue 'dispatch;
	}
	// 83112D78: C0190000  lfs f0, 0(r25)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83112D7C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 83112D80: C1BB0000  lfs f13, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83112D84: 3BF90004  addi r31, r25, 4
	ctx.r[31].s64 = ctx.r[25].s64 + 4;
	// 83112D88: EC200372  fmuls f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 83112D8C: C1990004  lfs f12, 4(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83112D90: C01B0004  lfs f0, 4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83112D94: EC400332  fmuls f2, f0, f12
	ctx.f[2].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 83112D98: 4BFF9DA9  bl 0x8310cb40
	ctx.lr = 0x83112D9C;
	sub_8310CB40(ctx, base);
	// 83112D9C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83112DA0: 40800034  bge 0x83112dd4
	if !ctx.cr[0].lt {
	pc = 0x83112DD4; continue 'dispatch;
	}
	// 83112DA4: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83112DA8: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83112DAC: 93010080  stw r24, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 83112DB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83112DB4: 419A0014  beq cr6, 0x83112dc8
	if ctx.cr[6].eq {
	pc = 0x83112DC8; continue 'dispatch;
	}
	// 83112DB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83112DBC: 419A000C  beq cr6, 0x83112dc8
	if ctx.cr[6].eq {
	pc = 0x83112DC8; continue 'dispatch;
	}
	// 83112DC0: 4BFBB519  bl 0x830ce2d8
	ctx.lr = 0x83112DC4;
	sub_830CE2D8(ctx, base);
	// 83112DC4: 48000008  b 0x83112dcc
	pc = 0x83112DCC; continue 'dispatch;
	// 83112DC8: 4BFCA711  bl 0x830dd4d8
	ctx.lr = 0x83112DCC;
	sub_830DD4D8(ctx, base);
	// 83112DCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83112DD0: 480002C8  b 0x83113098
	pc = 0x83113098; continue 'dispatch;
	// 83112DD4: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 83112DD8: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83112DDC: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83112DE0: 409900BC  ble cr6, 0x83112e9c
	if !ctx.cr[6].gt {
	pc = 0x83112E9C; continue 'dispatch;
	}
	// 83112DE4: 3AFAFFFF  addi r23, r26, -1
	ctx.r[23].s64 = ctx.r[26].s64 + -1;
	// 83112DE8: 7F1CB800  cmpw cr6, r28, r23
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[23].s32, &mut ctx.xer);
	// 83112DEC: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 83112DF0: 41980008  blt cr6, 0x83112df8
	if ctx.cr[6].lt {
	pc = 0x83112DF8; continue 'dispatch;
	}
	// 83112DF4: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 83112DF8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112DFC: C01B0000  lfs f0, 0(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83112E00: 8121008C  lwz r9, 0x8c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83112E04: C1BB0004  lfs f13, 4(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83112E08: C19FFFFC  lfs f12, -4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83112E0C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83112E10: C17F0000  lfs f11, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83112E14: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 83112E18: C15F0004  lfs f10, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83112E1C: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 83112E20: C13F0008  lfs f9, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 83112E24: ED6D02F2  fmuls f11, f13, f11
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 83112E28: C11F000C  lfs f8, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83112E2C: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 83112E30: C0FF0010  lfs f7, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83112E34: ED290372  fmuls f9, f9, f13
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 83112E38: C0CB0000  lfs f6, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 83112E3C: ED080032  fmuls f8, f8, f0
	ctx.f[8].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 83112E40: C0AB0004  lfs f5, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 83112E44: ECE70372  fmuls f7, f7, f13
	ctx.f[7].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 83112E48: EC060032  fmuls f0, f6, f0
	ctx.f[0].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 83112E4C: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 83112E50: EDA50372  fmuls f13, f5, f13
	ctx.f[13].f64 = (((ctx.f[5].f64 * ctx.f[13].f64) as f32) as f64);
	// 83112E54: D1610064  stfs f11, 0x64(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 83112E58: D1410068  stfs f10, 0x68(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 83112E5C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 83112E60: D121006C  stfs f9, 0x6c(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 83112E64: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83112E68: D1010070  stfs f8, 0x70(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 83112E6C: 913E0038  stw r9, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 83112E70: D0E10074  stfs f7, 0x74(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 83112E74: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83112E78: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 83112E7C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83112E80: D1A1007C  stfs f13, 0x7c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 83112E84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83112E88: 4BFD5A59  bl 0x830e88e0
	ctx.lr = 0x83112E8C;
	sub_830E88E0(ctx, base);
	// 83112E8C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83112E90: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 83112E94: 7F1CD000  cmpw cr6, r28, r26
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[26].s32, &mut ctx.xer);
	// 83112E98: 4198FF50  blt cr6, 0x83112de8
	if ctx.cr[6].lt {
	pc = 0x83112DE8; continue 'dispatch;
	}
	// 83112E9C: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83112EA0: 3B8BFFFD  addi r28, r11, -3
	ctx.r[28].s64 = ctx.r[11].s64 + -3;
	// 83112EA4: 939E0024  stw r28, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 83112EA8: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 83112EAC: 40980034  bge cr6, 0x83112ee0
	if !ctx.cr[6].lt {
	pc = 0x83112EE0; continue 'dispatch;
	}
	// 83112EB0: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83112EB4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83112EB8: 93010080  stw r24, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 83112EBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83112EC0: 419A0014  beq cr6, 0x83112ed4
	if ctx.cr[6].eq {
	pc = 0x83112ED4; continue 'dispatch;
	}
	// 83112EC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83112EC8: 419A000C  beq cr6, 0x83112ed4
	if ctx.cr[6].eq {
	pc = 0x83112ED4; continue 'dispatch;
	}
	// 83112ECC: 4BFBB40D  bl 0x830ce2d8
	ctx.lr = 0x83112ED0;
	sub_830CE2D8(ctx, base);
	// 83112ED0: 48000008  b 0x83112ed8
	pc = 0x83112ED8; continue 'dispatch;
	// 83112ED4: 4BFCA605  bl 0x830dd4d8
	ctx.lr = 0x83112ED8;
	sub_830DD4D8(ctx, base);
	// 83112ED8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83112EDC: 480001BC  b 0x83113098
	pc = 0x83113098; continue 'dispatch;
	// 83112EE0: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83112EE4: 4BFCA5CD  bl 0x830dd4b0
	ctx.lr = 0x83112EE8;
	sub_830DD4B0(ctx, base);
	// 83112EE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83112EEC: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83112EF0: 40820038  bne 0x83112f28
	if !ctx.cr[0].eq {
	pc = 0x83112F28; continue 'dispatch;
	}
	// 83112EF4: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83112EF8: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83112EFC: 93010080  stw r24, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 83112F00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83112F04: 419A0014  beq cr6, 0x83112f18
	if ctx.cr[6].eq {
	pc = 0x83112F18; continue 'dispatch;
	}
	// 83112F08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83112F0C: 419A000C  beq cr6, 0x83112f18
	if ctx.cr[6].eq {
	pc = 0x83112F18; continue 'dispatch;
	}
	// 83112F10: 4BFBB3C9  bl 0x830ce2d8
	ctx.lr = 0x83112F14;
	sub_830CE2D8(ctx, base);
	// 83112F14: 48000008  b 0x83112f1c
	pc = 0x83112F1C; continue 'dispatch;
	// 83112F18: 4BFCA5C1  bl 0x830dd4d8
	ctx.lr = 0x83112F1C;
	sub_830DD4D8(ctx, base);
	// 83112F1C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83112F20: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83112F24: 48000174  b 0x83113098
	pc = 0x83113098; continue 'dispatch;
	// 83112F28: 8161008C  lwz r11, 0x8c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83112F2C: C0010094  lfs f0, 0x94(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83112F30: C1A10098  lfs f13, 0x98(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83112F34: 80810084  lwz r4, 0x84(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83112F38: C181009C  lfs f12, 0x9c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83112F3C: C16100A0  lfs f11, 0xa0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83112F40: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83112F44: D01E0028  stfs f0, 0x28(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 83112F48: D1BE002C  stfs f13, 0x2c(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 83112F4C: D19E0030  stfs f12, 0x30(r30)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 83112F50: D17E0034  stfs f11, 0x34(r30)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 83112F54: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83112F58: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83112F5C: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83112F60: 480955B1  bl 0x831a8510
	ctx.lr = 0x83112F64;
	sub_831A8510(ctx, base);
	// 83112F64: 816100A8  lwz r11, 0xa8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 83112F68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83112F6C: 419A0020  beq cr6, 0x83112f8c
	if ctx.cr[6].eq {
	pc = 0x83112F8C; continue 'dispatch;
	}
	// 83112F70: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83112F74: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83112F78: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 83112F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83112F80: 4E800421  bctrl
	ctx.lr = 0x83112F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83112F84: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83112F88: 4082FF28  bne 0x83112eb0
	if !ctx.cr[0].eq {
	pc = 0x83112EB0; continue 'dispatch;
	}
	// 83112F8C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83112F90: D3E10060  stfs f31, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 83112F94: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 83112F98: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 83112F9C: D3C10068  stfs f30, 0x68(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 83112FA0: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 83112FA4: D3C1006C  stfs f30, 0x6c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 83112FA8: 93A10078  stw r29, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 83112FAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83112FB0: 40990034  ble cr6, 0x83112fe4
	if !ctx.cr[6].gt {
	pc = 0x83112FE4; continue 'dispatch;
	}
	// 83112FB4: 83E10084  lwz r31, 0x84(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83112FB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83112FBC: C05F0004  lfs f2, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83112FC0: C03F0000  lfs f1, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83112FC4: 4BFF92A5  bl 0x8310c268
	ctx.lr = 0x83112FC8;
	sub_8310C268(ctx, base);
	// 83112FC8: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83112FCC: 4180003C  blt 0x83113008
	if ctx.cr[0].lt {
	pc = 0x83113008; continue 'dispatch;
	}
	// 83112FD0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83112FD4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83112FD8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 83112FDC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83112FE0: 4198FFD8  blt cr6, 0x83112fb8
	if ctx.cr[6].lt {
	pc = 0x83112FB8; continue 'dispatch;
	}
	// 83112FE4: 1FBC0006  mulli r29, r28, 6
	ctx.r[29].s64 = ctx.r[28].s64 * 6;
	// 83112FE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83112FEC: 4BFCA4C5  bl 0x830dd4b0
	ctx.lr = 0x83112FF0;
	sub_830DD4B0(ctx, base);
	// 83112FF0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83112FF4: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83112FF8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83112FFC: 40820044  bne 0x83113040
	if !ctx.cr[0].eq {
	pc = 0x83113040; continue 'dispatch;
	}
	// 83113000: 4BFF9209  bl 0x8310c208
	ctx.lr = 0x83113004;
	sub_8310C208(ctx, base);
	// 83113004: 4BFFFEF0  b 0x83112ef4
	pc = 0x83112EF4; continue 'dispatch;
	// 83113008: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8311300C: 4BFF91FD  bl 0x8310c208
	ctx.lr = 0x83113010;
	sub_8310C208(ctx, base);
	// 83113010: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83113014: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83113018: 93010080  stw r24, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 8311301C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83113020: 419A0014  beq cr6, 0x83113034
	if ctx.cr[6].eq {
	pc = 0x83113034; continue 'dispatch;
	}
	// 83113024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83113028: 419A000C  beq cr6, 0x83113034
	if ctx.cr[6].eq {
	pc = 0x83113034; continue 'dispatch;
	}
	// 8311302C: 4BFBB2AD  bl 0x830ce2d8
	ctx.lr = 0x83113030;
	sub_830CE2D8(ctx, base);
	// 83113030: 48000008  b 0x83113038
	pc = 0x83113038; continue 'dispatch;
	// 83113034: 4BFCA4A5  bl 0x830dd4d8
	ctx.lr = 0x83113038;
	sub_830DD4D8(ctx, base);
	// 83113038: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8311303C: 4800005C  b 0x83113098
	pc = 0x83113098; continue 'dispatch;
	// 83113040: 1D7C0003  mulli r11, r28, 3
	ctx.r[11].s64 = ctx.r[28].s64 * 3;
	// 83113044: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83113048: 4BFFDFA9  bl 0x83110ff0
	ctx.lr = 0x8311304C;
	sub_83110FF0(ctx, base);
	// 8311304C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83113050: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83113054: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83113058: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8311305C: 4BFFE04D  bl 0x831110a8
	ctx.lr = 0x83113060;
	sub_831110A8(ctx, base);
	// 83113060: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83113064: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83113068: 4BFF91A1  bl 0x8310c208
	ctx.lr = 0x8311306C;
	sub_8310C208(ctx, base);
	// 8311306C: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83113070: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 83113074: 93010080  stw r24, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 83113078: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8311307C: 419A0014  beq cr6, 0x83113090
	if ctx.cr[6].eq {
	pc = 0x83113090; continue 'dispatch;
	}
	// 83113080: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83113084: 419A000C  beq cr6, 0x83113090
	if ctx.cr[6].eq {
	pc = 0x83113090; continue 'dispatch;
	}
	// 83113088: 4BFBB251  bl 0x830ce2d8
	ctx.lr = 0x8311308C;
	sub_830CE2D8(ctx, base);
	// 8311308C: 48000008  b 0x83113094
	pc = 0x83113094; continue 'dispatch;
	// 83113090: 4BFCA449  bl 0x830dd4d8
	ctx.lr = 0x83113094;
	sub_830DD4D8(ctx, base);
	// 83113094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113098: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8311309C: CBC1FF98  lfd f30, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 831130A0: CBE1FFA0  lfd f31, -0x60(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 831130A4: 480950FC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831130A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831130A8 size=228
    let mut pc: u32 = 0x831130A8;
    'dispatch: loop {
        match pc {
            0x831130A8 => {
    //   block [0x831130A8..0x8311318C)
	// 831130A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831130AC: 480950B5  bl 0x831a8160
	ctx.lr = 0x831130B0;
	sub_831A8130(ctx, base);
	// 831130B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831130B4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 831130B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831130BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831130C0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831130C4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 831130C8: 817EDEC8  lwz r11, -0x2138(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 831130CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831130D0: 409A0010  bne cr6, 0x831130e0
	if !ctx.cr[6].eq {
	pc = 0x831130E0; continue 'dispatch;
	}
	// 831130D4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 831130D8: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 831130DC: 480000A8  b 0x83113184
	pc = 0x83113184; continue 'dispatch;
	// 831130E0: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 831130E4: 4BFCA3CD  bl 0x830dd4b0
	ctx.lr = 0x831130E8;
	sub_830DD4B0(ctx, base);
	// 831130E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831130EC: 41820040  beq 0x8311312c
	if ctx.cr[0].eq {
	pc = 0x8311312C; continue 'dispatch;
	}
	// 831130F0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 831130F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831130F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831130FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83113100: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83113104: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83113108: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8311310C: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83113110: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83113114: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83113118: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8311311C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83113120: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83113124: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83113128: 48000008  b 0x83113130
	pc = 0x83113130; continue 'dispatch;
	// 8311312C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83113130: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83113134: 409A0010  bne cr6, 0x83113144
	if !ctx.cr[6].eq {
	pc = 0x83113144; continue 'dispatch;
	}
	// 83113138: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 8311313C: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 83113140: 48000044  b 0x83113184
	pc = 0x83113184; continue 'dispatch;
	// 83113144: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83113148: 809EDEC8  lwz r4, -0x2138(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8504 as u32) ) } as u64;
	// 8311314C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83113150: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83113154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113158: 4BFFFB99  bl 0x83112cf0
	ctx.lr = 0x8311315C;
	sub_83112CF0(ctx, base);
	// 8311315C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83113160: 4080001C  bge 0x8311317c
	if !ctx.cr[0].lt {
	pc = 0x8311317C; continue 'dispatch;
	}
	// 83113164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113168: 4BFF9849  bl 0x8310c9b0
	ctx.lr = 0x8311316C;
	sub_8310C9B0(ctx, base);
	// 8311316C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113170: 4BFCA369  bl 0x830dd4d8
	ctx.lr = 0x83113174;
	sub_830DD4D8(ctx, base);
	// 83113174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113178: 4800000C  b 0x83113184
	pc = 0x83113184; continue 'dispatch;
	// 8311317C: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83113180: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83113184: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83113188: 48095028  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83113190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83113190 size=216
    let mut pc: u32 = 0x83113190;
    'dispatch: loop {
        match pc {
            0x83113190 => {
    //   block [0x83113190..0x83113268)
	// 83113190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83113194: 48094FCD  bl 0x831a8160
	ctx.lr = 0x83113198;
	sub_831A8130(ctx, base);
	// 83113198: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311319C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831131A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831131A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831131A8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831131AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831131B0: 419A0048  beq cr6, 0x831131f8
	if ctx.cr[6].eq {
	pc = 0x831131F8; continue 'dispatch;
	}
	// 831131B4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831131B8: 419A0040  beq cr6, 0x831131f8
	if ctx.cr[6].eq {
	pc = 0x831131F8; continue 'dispatch;
	}
	// 831131BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831131C0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831131C4: 4BFFF48D  bl 0x83112650
	ctx.lr = 0x831131C8;
	sub_83112650(ctx, base);
	// 831131C8: 3B9E0064  addi r28, r30, 0x64
	ctx.r[28].s64 = ctx.r[30].s64 + 100;
	// 831131CC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 831131D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831131D4: 4812F799  bl 0x8324296c
	ctx.lr = 0x831131D8;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 831131D8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 831131DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831131E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831131E4: 4BFFF825  bl 0x83112a08
	ctx.lr = 0x831131E8;
	sub_83112A08(ctx, base);
	// 831131E8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831131EC: 40820018  bne 0x83113204
	if !ctx.cr[0].eq {
	pc = 0x83113204; continue 'dispatch;
	}
	// 831131F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831131F4: 4812F769  bl 0x8324295c
	ctx.lr = 0x831131F8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831131F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831131FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83113200: 48094FB0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83113204: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83113208: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8311320C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83113210: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83113214: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83113218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311321C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83113220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83113224: 4E800421  bctrl
	ctx.lr = 0x83113228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83113228: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311322C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83113230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113234: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83113238: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8311323C: 4E800421  bctrl
	ctx.lr = 0x83113240;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83113240: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83113244: 419A0014  beq cr6, 0x83113258
	if ctx.cr[6].eq {
	pc = 0x83113258; continue 'dispatch;
	}
	// 83113248: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 8311324C: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 83113250: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83113254: 480952BD  bl 0x831a8510
	ctx.lr = 0x83113258;
	sub_831A8510(ctx, base);
	// 83113258: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8311325C: 4812F701  bl 0x8324295c
	ctx.lr = 0x83113260;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83113260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113264: 4BFFFF98  b 0x831131fc
	pc = 0x831131FC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83113268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83113268 size=244
    let mut pc: u32 = 0x83113268;
    'dispatch: loop {
        match pc {
            0x83113268 => {
    //   block [0x83113268..0x8311335C)
	// 83113268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311326C: 48094EF5  bl 0x831a8160
	ctx.lr = 0x83113270;
	sub_831A8130(ctx, base);
	// 83113270: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83113274: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83113278: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8311327C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83113280: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83113284: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83113288: 419A007C  beq cr6, 0x83113304
	if ctx.cr[6].eq {
	pc = 0x83113304; continue 'dispatch;
	}
	// 8311328C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 83113290: 419A0074  beq cr6, 0x83113304
	if ctx.cr[6].eq {
	pc = 0x83113304; continue 'dispatch;
	}
	// 83113294: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83113298: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8311329C: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831132A0: 40990034  ble cr6, 0x831132d4
	if !ctx.cr[6].gt {
	pc = 0x831132D4; continue 'dispatch;
	}
	// 831132A4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 831132A8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 831132AC: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831132B0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831132B4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 831132B8: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 831132BC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831132C0: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 831132C4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831132C8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831132CC: 7FE9FA14  add r31, r9, r31
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 831132D0: 4082FFDC  bne 0x831132ac
	if !ctx.cr[0].eq {
	pc = 0x831132AC; continue 'dispatch;
	}
	// 831132D4: 3BDD0064  addi r30, r29, 0x64
	ctx.r[30].s64 = ctx.r[29].s64 + 100;
	// 831132D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831132DC: 4812F691  bl 0x8324296c
	ctx.lr = 0x831132E0;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 831132E0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 831132E4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 831132E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831132EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831132F0: 4BFFF7A9  bl 0x83112a98
	ctx.lr = 0x831132F4;
	sub_83112A98(ctx, base);
	// 831132F4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831132F8: 40820018  bne 0x83113310
	if !ctx.cr[0].eq {
	pc = 0x83113310; continue 'dispatch;
	}
	// 831132FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113300: 4812F65D  bl 0x8324295c
	ctx.lr = 0x83113304;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83113304: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83113308: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8311330C: 48094EA4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83113310: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83113314: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83113318: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8311331C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83113320: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83113324: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83113328: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311332C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83113330: 4E800421  bctrl
	ctx.lr = 0x83113334;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83113334: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83113338: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8311333C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83113340: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83113344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83113348: 4E800421  bctrl
	ctx.lr = 0x8311334C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8311334C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113350: 4812F60D  bl 0x8324295c
	ctx.lr = 0x83113354;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83113354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113358: 4BFFFFB0  b 0x83113308
	pc = 0x83113308; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83113360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83113360 size=144
    let mut pc: u32 = 0x83113360;
    'dispatch: loop {
        match pc {
            0x83113360 => {
    //   block [0x83113360..0x831133F0)
	// 83113360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83113364: 48094E05  bl 0x831a8168
	ctx.lr = 0x83113368;
	sub_831A8130(ctx, base);
	// 83113368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8311336C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83113370: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83113374: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83113378: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8311337C: 419A006C  beq cr6, 0x831133e8
	if ctx.cr[6].eq {
	pc = 0x831133E8; continue 'dispatch;
	}
	// 83113380: 3BBE0064  addi r29, r30, 0x64
	ctx.r[29].s64 = ctx.r[30].s64 + 100;
	// 83113384: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83113388: 4812F5E5  bl 0x8324296c
	ctx.lr = 0x8311338C;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8311338C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83113390: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83113394: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83113398: 40820048  bne 0x831133e0
	if !ctx.cr[0].eq {
	pc = 0x831133E0; continue 'dispatch;
	}
	// 8311339C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831133A0: 419A0040  beq cr6, 0x831133e0
	if ctx.cr[6].eq {
	pc = 0x831133E0; continue 'dispatch;
	}
	// 831133A4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831133A8: 816BDEB8  lwz r11, -0x2148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8520 as u32) ) } as u64;
	// 831133AC: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831133B0: 41820024  beq 0x831133d4
	if ctx.cr[0].eq {
	pc = 0x831133D4; continue 'dispatch;
	}
	// 831133B4: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831133B8: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 831133BC: 814ADEBC  lwz r10, -0x2144(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8516 as u32) ) } as u64;
	// 831133C0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831133C4: 419A001C  beq cr6, 0x831133e0
	if ctx.cr[6].eq {
	pc = 0x831133E0; continue 'dispatch;
	}
	// 831133C8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831133CC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831133D0: 419A0010  beq cr6, 0x831133e0
	if ctx.cr[6].eq {
	pc = 0x831133E0; continue 'dispatch;
	}
	// 831133D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831133D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831133DC: 4BFFF84D  bl 0x83112c28
	ctx.lr = 0x831133E0;
	sub_83112C28(ctx, base);
	// 831133E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831133E4: 4812F579  bl 0x8324295c
	ctx.lr = 0x831133E8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 831133E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831133EC: 48094DCC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831133F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831133F0 size=436
    let mut pc: u32 = 0x831133F0;
    'dispatch: loop {
        match pc {
            0x831133F0 => {
    //   block [0x831133F0..0x831135A4)
	// 831133F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831133F4: 48094D5D  bl 0x831a8150
	ctx.lr = 0x831133F8;
	sub_831A8130(ctx, base);
	// 831133F8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831133FC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83113400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83113404: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 83113408: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8311340C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 83113410: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83113414: 419A0180  beq cr6, 0x83113594
	if ctx.cr[6].eq {
	pc = 0x83113594; continue 'dispatch;
	}
	// 83113418: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8311341C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83113420: 41820174  beq 0x83113594
	if ctx.cr[0].eq {
	pc = 0x83113594; continue 'dispatch;
	}
	// 83113424: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83113428: 419A016C  beq cr6, 0x83113594
	if ctx.cr[6].eq {
	pc = 0x83113594; continue 'dispatch;
	}
	// 8311342C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83113430: 419A0164  beq cr6, 0x83113594
	if ctx.cr[6].eq {
	pc = 0x83113594; continue 'dispatch;
	}
	// 83113434: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83113438: 93D80000  stw r30, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8311343C: 4BFFF215  bl 0x83112650
	ctx.lr = 0x83113440;
	sub_83112650(ctx, base);
	// 83113440: 3ADF0064  addi r22, r31, 0x64
	ctx.r[22].s64 = ctx.r[31].s64 + 100;
	// 83113444: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83113448: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8311344C: 4812F521  bl 0x8324296c
	ctx.lr = 0x83113450;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 83113450: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83113454: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83113458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311345C: 4BFFF5AD  bl 0x83112a08
	ctx.lr = 0x83113460;
	sub_83112A08(ctx, base);
	// 83113460: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83113464: 4182001C  beq 0x83113480
	if ctx.cr[0].eq {
	pc = 0x83113480; continue 'dispatch;
	}
	// 83113468: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8311346C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83113470: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83113474: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83113478: 90780000  stw r3, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8311347C: 48000108  b 0x83113584
	pc = 0x83113584; continue 'dispatch;
	// 83113480: 80990004  lwz r4, 4(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 83113484: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83113488: 4BFFF111  bl 0x83112598
	ctx.lr = 0x8311348C;
	sub_83112598(ctx, base);
	// 8311348C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83113490: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83113494: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83113498: 419A002C  beq cr6, 0x831134c4
	if ctx.cr[6].eq {
	pc = 0x831134C4; continue 'dispatch;
	}
	// 8311349C: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 831134A0: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 831134A4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831134A8: 4099001C  ble cr6, 0x831134c4
	if !ctx.cr[6].gt {
	pc = 0x831134C4; continue 'dispatch;
	}
	// 831134AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831134B0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831134B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831134B8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 831134BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831134C0: 4E800421  bctrl
	ctx.lr = 0x831134C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831134C4: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 831134C8: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 831134CC: 7D7D5B96  divwu r11, r29, r11
	ctx.r[11].u32 = ctx.r[29].u32 / ctx.r[11].u32;
	// 831134D0: 1D6B0011  mulli r11, r11, 0x11
	ctx.r[11].s64 = ctx.r[11].s64 * 17;
	// 831134D4: 7F4BE850  subf r26, r11, r29
	ctx.r[26].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 831134D8: 4BFC9FD9  bl 0x830dd4b0
	ctx.lr = 0x831134DC;
	sub_830DD4B0(ctx, base);
	// 831134DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831134E0: 4182000C  beq 0x831134ec
	if ctx.cr[0].eq {
	pc = 0x831134EC; continue 'dispatch;
	}
	// 831134E4: 4BFFF1CD  bl 0x831126b0
	ctx.lr = 0x831134E8;
	sub_831126B0(ctx, base);
	// 831134E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831134EC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831134F0: 409A0010  bne cr6, 0x83113500
	if !ctx.cr[6].eq {
	pc = 0x83113500; continue 'dispatch;
	}
	// 831134F4: 3FA08007  lis r29, -0x7ff9
	ctx.r[29].s64 = -2147024896;
	// 831134F8: 63BD000E  ori r29, r29, 0xe
	ctx.r[29].u64 = ctx.r[29].u64 | 14;
	// 831134FC: 48000088  b 0x83113584
	pc = 0x83113584; continue 'dispatch;
	// 83113500: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 83113504: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83113508: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 8311350C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83113510: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83113514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113518: 4BFFF319  bl 0x83112830
	ctx.lr = 0x8311351C;
	sub_83112830(ctx, base);
	// 8311351C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83113520: 40800018  bge 0x83113538
	if !ctx.cr[0].lt {
	pc = 0x83113538; continue 'dispatch;
	}
	// 83113524: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113528: 4BFFF201  bl 0x83112728
	ctx.lr = 0x8311352C;
	sub_83112728(ctx, base);
	// 8311352C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113530: 4BFC9FA9  bl 0x830dd4d8
	ctx.lr = 0x83113534;
	sub_830DD4D8(ctx, base);
	// 83113534: 48000050  b 0x83113584
	pc = 0x83113584; continue 'dispatch;
	// 83113538: 397A0001  addi r11, r26, 1
	ctx.r[11].s64 = ctx.r[26].s64 + 1;
	// 8311353C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83113540: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83113544: 915E003C  stw r10, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 83113548: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8311354C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83113550: 419A0008  beq cr6, 0x83113558
	if ctx.cr[6].eq {
	pc = 0x83113558; continue 'dispatch;
	}
	// 83113554: 93CA0040  stw r30, 0x40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 83113558: 7FCBF92E  stwx r30, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 8311355C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83113560: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83113564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113568: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8311356C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83113570: 4E800421  bctrl
	ctx.lr = 0x83113574;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83113574: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83113578: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8311357C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83113580: 93D80000  stw r30, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83113584: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83113588: 4812F3D5  bl 0x8324295c
	ctx.lr = 0x8311358C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8311358C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83113590: 4800000C  b 0x8311359c
	pc = 0x8311359C; continue 'dispatch;
	// 83113594: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 83113598: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 8311359C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831135A0: 48094C00  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831135A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831135A8 size=224
    let mut pc: u32 = 0x831135A8;
    'dispatch: loop {
        match pc {
            0x831135A8 => {
    //   block [0x831135A8..0x83113688)
	// 831135A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831135AC: 48094BAD  bl 0x831a8158
	ctx.lr = 0x831135B0;
	sub_831A8130(ctx, base);
	// 831135B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831135B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831135B8: 3B3E0064  addi r25, r30, 0x64
	ctx.r[25].s64 = ctx.r[30].s64 + 100;
	// 831135BC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 831135C0: 4812F3AD  bl 0x8324296c
	ctx.lr = 0x831135C4;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 831135C4: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 831135C8: 3B7E0004  addi r27, r30, 4
	ctx.r[27].s64 = ctx.r[30].s64 + 4;
	// 831135CC: 3B400011  li r26, 0x11
	ctx.r[26].s64 = 17;
	// 831135D0: 3F008339  lis r24, -0x7cc7
	ctx.r[24].s64 = -2093416448;
	// 831135D4: 815CDEBC  lwz r10, -0x2144(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8516 as u32) ) } as u64;
	// 831135D8: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831135DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831135E0: 419A008C  beq cr6, 0x8311366c
	if ctx.cr[6].eq {
	pc = 0x8311366C; continue 'dispatch;
	}
	// 831135E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831135E8: 83BF003C  lwz r29, 0x3c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831135EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831135F0: 409A0070  bne cr6, 0x83113660
	if !ctx.cr[6].eq {
	pc = 0x83113660; continue 'dispatch;
	}
	// 831135F4: 8178DEB8  lwz r11, -0x2148(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-8520 as u32) ) } as u64;
	// 831135F8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831135FC: 4182001C  beq 0x83113618
	if ctx.cr[0].eq {
	pc = 0x83113618; continue 'dispatch;
	}
	// 83113600: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83113604: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83113608: 419A0058  beq cr6, 0x83113660
	if ctx.cr[6].eq {
	pc = 0x83113660; continue 'dispatch;
	}
	// 8311360C: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 83113610: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83113614: 419A004C  beq cr6, 0x83113660
	if ctx.cr[6].eq {
	pc = 0x83113660; continue 'dispatch;
	}
	// 83113618: 817E0054  lwz r11, 0x54(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 8311361C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83113620: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83113624: 917E0054  stw r11, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83113628: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8311362C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83113630: 419A0010  beq cr6, 0x83113640
	if ctx.cr[6].eq {
	pc = 0x83113640; continue 'dispatch;
	}
	// 83113634: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83113638: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8311363C: 917E005C  stw r11, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83113640: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83113644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83113648: 4BFFF589  bl 0x83112bd0
	ctx.lr = 0x8311364C;
	sub_83112BD0(ctx, base);
	// 8311364C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113650: 4BFFF0D9  bl 0x83112728
	ctx.lr = 0x83113654;
	sub_83112728(ctx, base);
	// 83113654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113658: 4BFC9E81  bl 0x830dd4d8
	ctx.lr = 0x8311365C;
	sub_830DD4D8(ctx, base);
	// 8311365C: 815CDEBC  lwz r10, -0x2144(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8516 as u32) ) } as u64;
	// 83113660: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83113664: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83113668: 409AFF7C  bne cr6, 0x831135e4
	if !ctx.cr[6].eq {
	pc = 0x831135E4; continue 'dispatch;
	}
	// 8311366C: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83113670: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 83113674: 4082FF64  bne 0x831135d8
	if !ctx.cr[0].eq {
	pc = 0x831135D8; continue 'dispatch;
	}
	// 83113678: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8311367C: 4812F2E1  bl 0x8324295c
	ctx.lr = 0x83113680;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83113680: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83113684: 48094B24  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83113688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83113688 size=200
    let mut pc: u32 = 0x83113688;
    'dispatch: loop {
        match pc {
            0x83113688 => {
    //   block [0x83113688..0x83113750)
	// 83113688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8311368C: 48094AD9  bl 0x831a8164
	ctx.lr = 0x83113690;
	sub_831A8130(ctx, base);
	// 83113690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83113694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83113698: 817E0050  lwz r11, 0x50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 8311369C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831136A0: 409A00A8  bne cr6, 0x83113748
	if !ctx.cr[6].eq {
	pc = 0x83113748; continue 'dispatch;
	}
	// 831136A4: 3B7E0064  addi r27, r30, 0x64
	ctx.r[27].s64 = ctx.r[30].s64 + 100;
	// 831136A8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831136AC: 4812F2C1  bl 0x8324296c
	ctx.lr = 0x831136B0;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 831136B0: 83FE0048  lwz r31, 0x48(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 831136B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831136B8: 419A0088  beq cr6, 0x83113740
	if ctx.cr[6].eq {
	pc = 0x83113740; continue 'dispatch;
	}
	// 831136BC: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 831136C0: 815CDEBC  lwz r10, -0x2144(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8516 as u32) ) } as u64;
	// 831136C4: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 831136C8: 83BF0044  lwz r29, 0x44(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 831136CC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 831136D0: 419A0070  beq cr6, 0x83113740
	if ctx.cr[6].eq {
	pc = 0x83113740; continue 'dispatch;
	}
	// 831136D4: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 831136D8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 831136DC: 419A0064  beq cr6, 0x83113740
	if ctx.cr[6].eq {
	pc = 0x83113740; continue 'dispatch;
	}
	// 831136E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831136E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831136E8: 409A004C  bne cr6, 0x83113734
	if !ctx.cr[6].eq {
	pc = 0x83113734; continue 'dispatch;
	}
	// 831136EC: 817E0054  lwz r11, 0x54(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 831136F0: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 831136F4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831136F8: 917E0054  stw r11, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831136FC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83113700: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83113704: 419A0010  beq cr6, 0x83113714
	if ctx.cr[6].eq {
	pc = 0x83113714; continue 'dispatch;
	}
	// 83113708: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8311370C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83113710: 917E005C  stw r11, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83113714: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83113718: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8311371C: 4BFFF4B5  bl 0x83112bd0
	ctx.lr = 0x83113720;
	sub_83112BD0(ctx, base);
	// 83113720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83113724: 4BFFF005  bl 0x83112728
	ctx.lr = 0x83113728;
	sub_83112728(ctx, base);
	// 83113728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8311372C: 4BFC9DAD  bl 0x830dd4d8
	ctx.lr = 0x83113730;
	sub_830DD4D8(ctx, base);
	// 83113730: 815CDEBC  lwz r10, -0x2144(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8516 as u32) ) } as u64;
	// 83113734: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83113738: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8311373C: 409AFF88  bne cr6, 0x831136c4
	if !ctx.cr[6].eq {
	pc = 0x831136C4; continue 'dispatch;
	}
	// 83113740: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83113744: 4812F219  bl 0x8324295c
	ctx.lr = 0x83113748;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 83113748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8311374C: 48094A68  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


