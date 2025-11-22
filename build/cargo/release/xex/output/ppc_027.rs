pub fn sub_823B4E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B4E10 size=12
    let mut pc: u32 = 0x823B4E10;
    'dispatch: loop {
        match pc {
            0x823B4E10 => {
    //   block [0x823B4E10..0x823B4E1C)
	// 823B4E10: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B4E14: 986B94F2  stb r3, -0x6b0e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27406 as u32), ctx.r[3].u8 ) };
	// 823B4E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B4E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B4E20 size=144
    let mut pc: u32 = 0x823B4E20;
    'dispatch: loop {
        match pc {
            0x823B4E20 => {
    //   block [0x823B4E20..0x823B4EB0)
	// 823B4E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B4E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B4E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B4E2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B4E30: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 823B4E34: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B4E38: 3CE08349  lis r7, -0x7cb7
	ctx.r[7].s64 = -2092367872;
	// 823B4E3C: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 823B4E40: C02B9490  lfs f1, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823B4E44: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 823B4E48: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823B4E4C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B4E50: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 823B4E54: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823B4E58: 81676AB8  lwz r11, 0x6ab8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(27320 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B4EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B4EB0 size=188
    let mut pc: u32 = 0x823B4EB0;
    'dispatch: loop {
        match pc {
            0x823B4EB0 => {
    //   block [0x823B4EB0..0x823B4F6C)
	// 823B4EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B4EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B4EB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B4EBC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823B4EC0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823B4EC4: 419A0018  beq cr6, 0x823b4edc
	if ctx.cr[6].eq {
	pc = 0x823B4EDC; continue 'dispatch;
	}
	// 823B4EC8: 89650090  lbz r11, 0x90(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B4ECC: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B4ED0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B4ED4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B4ED8: 409A0008  bne cr6, 0x823b4ee0
	if !ctx.cr[6].eq {
	pc = 0x823B4EE0; continue 'dispatch;
	}
	// 823B4EDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B4EE0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B4EE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B4EE8: 419A0074  beq cr6, 0x823b4f5c
	if ctx.cr[6].eq {
	pc = 0x823B4F5C; continue 'dispatch;
	}
	// 823B4EEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B4EF0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 823B4EF4: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B4EF8: 3CE08349  lis r7, -0x7cb7
	ctx.r[7].s64 = -2092367872;
	// 823B4EFC: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 823B4F00: C02B9490  lfs f1, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823B4F04: 38C10064  addi r6, r1, 0x64
	ctx.r[6].s64 = ctx.r[1].s64 + 100;
	// 823B4F08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B4F0C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B4F10: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 823B4F14: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 823B4F18: 81676AB8  lwz r11, 0x6ab8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(27320 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B4F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B4F70 size=348
    let mut pc: u32 = 0x823B4F70;
    'dispatch: loop {
        match pc {
            0x823B4F70 => {
    //   block [0x823B4F70..0x823B50CC)
	// 823B4F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B4F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B4F78: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B50D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B50D0 size=468
    let mut pc: u32 = 0x823B50D0;
    'dispatch: loop {
        match pc {
            0x823B50D0 => {
    //   block [0x823B50D0..0x823B52A4)
	// 823B50D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B50D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B50D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B50DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B50E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B50E4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B50E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B50EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B50F0: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B50F4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B50F8: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B50FC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5100: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5104: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B5108: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B510C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5110: 419A0080  beq cr6, 0x823b5190
	if ctx.cr[6].eq {
	pc = 0x823B5190; continue 'dispatch;
	}
	// 823B5114: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5118: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B511C: 419A0070  beq cr6, 0x823b518c
	if ctx.cr[6].eq {
	pc = 0x823B518C; continue 'dispatch;
	}
	// 823B5120: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B5124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5128: 419A0018  beq cr6, 0x823b5140
	if ctx.cr[6].eq {
	pc = 0x823B5140; continue 'dispatch;
	}
	// 823B512C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B5130: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B5134: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B5138: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B513C: 409A0008  bne cr6, 0x823b5144
	if !ctx.cr[6].eq {
	pc = 0x823B5144; continue 'dispatch;
	}
	// 823B5140: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 823B5144: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B5148: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B514C: 419A0140  beq cr6, 0x823b528c
	if ctx.cr[6].eq {
	pc = 0x823B528C; continue 'dispatch;
	}
	// 823B5150: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 823B5154: 554937FE  rlwinm r9, r10, 6, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 823B5158: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B515C: 419A0100  beq cr6, 0x823b525c
	if ctx.cr[6].eq {
	pc = 0x823B525C; continue 'dispatch;
	}
	// 823B5160: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B5164: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5168: 419A0030  beq cr6, 0x823b5198
	if ctx.cr[6].eq {
	pc = 0x823B5198; continue 'dispatch;
	}
	// 823B516C: 894A009A  lbz r10, 0x9a(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(154 as u32) ) } as u64;
	// 823B5170: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5174: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B5178: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B517C: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5180: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B5184: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5188: 480000D8  b 0x823b5260
	pc = 0x823B5260; continue 'dispatch;
	// 823B518C: 4BDDECAD  bl 0x82193e38
	ctx.lr = 0x823B5190;
	sub_82193E38(ctx, base);
	// 823B5190: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B5194: 4BFFFFAC  b 0x823b5140
	pc = 0x823B5140; continue 'dispatch;
	// 823B5198: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B519C: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B51A0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B51A4: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B51A8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B51AC: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B51B0: 40810054  ble 0x823b5204
	if !ctx.cr[0].gt {
	pc = 0x823B5204; continue 'dispatch;
	}
	// 823B51B4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B51B8: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B51BC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B51C0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B51C4: 2F07009A  cmpwi cr6, r7, 0x9a
	ctx.cr[6].compare_i32(ctx.r[7].s32, 154, &mut ctx.xer);
	// 823B51C8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B51CC: 41980008  blt cr6, 0x823b51d4
	if ctx.cr[6].lt {
	pc = 0x823B51D4; continue 'dispatch;
	}
	// 823B51D0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B51D4: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B51D8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B51DC: 419A0014  beq cr6, 0x823b51f0
	if ctx.cr[6].eq {
	pc = 0x823B51F0; continue 'dispatch;
	}
	// 823B51E0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B51E4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B51E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B51EC: 4800000C  b 0x823b51f8
	pc = 0x823B51F8; continue 'dispatch;
	// 823B51F0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B51F4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B51F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B51FC: 4199FFB8  bgt cr6, 0x823b51b4
	if ctx.cr[6].gt {
	pc = 0x823B51B4; continue 'dispatch;
	}
	// 823B5200: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B5204: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B5208: 419A0040  beq cr6, 0x823b5248
	if ctx.cr[6].eq {
	pc = 0x823B5248; continue 'dispatch;
	}
	// 823B520C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5210: 2F0B009A  cmpwi cr6, r11, 0x9a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 154, &mut ctx.xer);
	// 823B5214: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5218: 41990008  bgt cr6, 0x823b5220
	if ctx.cr[6].gt {
	pc = 0x823B5220; continue 'dispatch;
	}
	// 823B521C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B5220: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B5224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5228: 409A0020  bne cr6, 0x823b5248
	if !ctx.cr[6].eq {
	pc = 0x823B5248; continue 'dispatch;
	}
	// 823B522C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B5230: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B5234: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B5238: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B523C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B5240: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5244: 4800001C  b 0x823b5260
	pc = 0x823B5260; continue 'dispatch;
	// 823B5248: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B524C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5250: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B5254: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5258: 48000008  b 0x823b5260
	pc = 0x823B5260; continue 'dispatch;
	// 823B525C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B5260: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B5264: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5268: 419A0024  beq cr6, 0x823b528c
	if ctx.cr[6].eq {
	pc = 0x823B528C; continue 'dispatch;
	}
	// 823B526C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823B5270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5274: 4BDE7375  bl 0x8219c5e8
	ctx.lr = 0x823B5278;
	sub_8219C5E8(ctx, base);
	// 823B5278: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B527C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5280: 419A000C  beq cr6, 0x823b528c
	if ctx.cr[6].eq {
	pc = 0x823B528C; continue 'dispatch;
	}
	// 823B5284: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B5288: F97F0078  std r11, 0x78(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 823B528C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B5290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B529C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B52A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B52A8 size=12
    let mut pc: u32 = 0x823B52A8;
    'dispatch: loop {
        match pc {
            0x823B52A8 => {
    //   block [0x823B52A8..0x823B52B4)
	// 823B52A8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B52AC: 986B9362  stb r3, -0x6c9e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27806 as u32), ctx.r[3].u8 ) };
	// 823B52B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B52B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B52B8 size=12
    let mut pc: u32 = 0x823B52B8;
    'dispatch: loop {
        match pc {
            0x823B52B8 => {
    //   block [0x823B52B8..0x823B52C4)
	// 823B52B8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B52BC: 986B9363  stb r3, -0x6c9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27805 as u32), ctx.r[3].u8 ) };
	// 823B52C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B52C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B52C8 size=12
    let mut pc: u32 = 0x823B52C8;
    'dispatch: loop {
        match pc {
            0x823B52C8 => {
    //   block [0x823B52C8..0x823B52D4)
	// 823B52C8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B52CC: 986B9408  stb r3, -0x6bf8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27640 as u32), ctx.r[3].u8 ) };
	// 823B52D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B52D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B52D8 size=12
    let mut pc: u32 = 0x823B52D8;
    'dispatch: loop {
        match pc {
            0x823B52D8 => {
    //   block [0x823B52D8..0x823B52E4)
	// 823B52D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B52DC: 986B6B50  stb r3, 0x6b50(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27472 as u32), ctx.r[3].u8 ) };
	// 823B52E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B52E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B52E8 size=12
    let mut pc: u32 = 0x823B52E8;
    'dispatch: loop {
        match pc {
            0x823B52E8 => {
    //   block [0x823B52E8..0x823B52F4)
	// 823B52E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B52EC: 986B6B51  stb r3, 0x6b51(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27473 as u32), ctx.r[3].u8 ) };
	// 823B52F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B52F8 size=12
    let mut pc: u32 = 0x823B52F8;
    'dispatch: loop {
        match pc {
            0x823B52F8 => {
    //   block [0x823B52F8..0x823B5304)
	// 823B52F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B52FC: 986B6B52  stb r3, 0x6b52(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27474 as u32), ctx.r[3].u8 ) };
	// 823B5300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5308 size=12
    let mut pc: u32 = 0x823B5308;
    'dispatch: loop {
        match pc {
            0x823B5308 => {
    //   block [0x823B5308..0x823B5314)
	// 823B5308: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B530C: 986B6B53  stb r3, 0x6b53(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27475 as u32), ctx.r[3].u8 ) };
	// 823B5310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5318 size=12
    let mut pc: u32 = 0x823B5318;
    'dispatch: loop {
        match pc {
            0x823B5318 => {
    //   block [0x823B5318..0x823B5324)
	// 823B5318: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B531C: 986B940C  stb r3, -0x6bf4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27636 as u32), ctx.r[3].u8 ) };
	// 823B5320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5328 size=12
    let mut pc: u32 = 0x823B5328;
    'dispatch: loop {
        match pc {
            0x823B5328 => {
    //   block [0x823B5328..0x823B5334)
	// 823B5328: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B532C: 986B6B54  stb r3, 0x6b54(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27476 as u32), ctx.r[3].u8 ) };
	// 823B5330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5338 size=12
    let mut pc: u32 = 0x823B5338;
    'dispatch: loop {
        match pc {
            0x823B5338 => {
    //   block [0x823B5338..0x823B5344)
	// 823B5338: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B533C: 986B6B55  stb r3, 0x6b55(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27477 as u32), ctx.r[3].u8 ) };
	// 823B5340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5348 size=12
    let mut pc: u32 = 0x823B5348;
    'dispatch: loop {
        match pc {
            0x823B5348 => {
    //   block [0x823B5348..0x823B5354)
	// 823B5348: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B534C: 986B6B56  stb r3, 0x6b56(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27478 as u32), ctx.r[3].u8 ) };
	// 823B5350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5358 size=12
    let mut pc: u32 = 0x823B5358;
    'dispatch: loop {
        match pc {
            0x823B5358 => {
    //   block [0x823B5358..0x823B5364)
	// 823B5358: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B535C: 986B6B57  stb r3, 0x6b57(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27479 as u32), ctx.r[3].u8 ) };
	// 823B5360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5368 size=12
    let mut pc: u32 = 0x823B5368;
    'dispatch: loop {
        match pc {
            0x823B5368 => {
    //   block [0x823B5368..0x823B5374)
	// 823B5368: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B536C: 986B940D  stb r3, -0x6bf3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27635 as u32), ctx.r[3].u8 ) };
	// 823B5370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5378 size=12
    let mut pc: u32 = 0x823B5378;
    'dispatch: loop {
        match pc {
            0x823B5378 => {
    //   block [0x823B5378..0x823B5384)
	// 823B5378: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B537C: 986B940E  stb r3, -0x6bf2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27634 as u32), ctx.r[3].u8 ) };
	// 823B5380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5388 size=32
    let mut pc: u32 = 0x823B5388;
    'dispatch: loop {
        match pc {
            0x823B5388 => {
    //   block [0x823B5388..0x823B53A8)
	// 823B5388: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B538C: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 823B5390: 816B6B08  lwz r11, 0x6b08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27400 as u32) ) } as u64;
	// 823B5394: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823B5398: 81690044  lwz r11, 0x44(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(68 as u32) ) } as u64;
	// 823B539C: 890B0004  lbz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B53A0: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823B53A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B53A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B53A8 size=12
    let mut pc: u32 = 0x823B53A8;
    'dispatch: loop {
        match pc {
            0x823B53A8 => {
    //   block [0x823B53A8..0x823B53B4)
	// 823B53A8: 986B0004  stb r3, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u8 ) };
	// 823B53AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B53B0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B53B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B53B4 size=12
    let mut pc: u32 = 0x823B53B4;
    'dispatch: loop {
        match pc {
            0x823B53B4 => {
    //   block [0x823B53B4..0x823B53C0)
	// 823B53B4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823B53B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B53BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B53C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B53C0 size=12
    let mut pc: u32 = 0x823B53C0;
    'dispatch: loop {
        match pc {
            0x823B53C0 => {
    //   block [0x823B53C0..0x823B53CC)
	// 823B53C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B53C4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B53C8: 48432FC8  b 0x827e8390
	sub_827E8390(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B53CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B53CC size=4
    let mut pc: u32 = 0x823B53CC;
    'dispatch: loop {
        match pc {
            0x823B53CC => {
    //   block [0x823B53CC..0x823B53D0)
	// 823B53CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B53D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B53D0 size=12
    let mut pc: u32 = 0x823B53D0;
    'dispatch: loop {
        match pc {
            0x823B53D0 => {
    //   block [0x823B53D0..0x823B53DC)
	// 823B53D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B53D4: D02B9410  stfs f1, -0x6bf0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27632 as u32), tmp.u32 ) };
	// 823B53D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B53E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B53E0 size=12
    let mut pc: u32 = 0x823B53E0;
    'dispatch: loop {
        match pc {
            0x823B53E0 => {
    //   block [0x823B53E0..0x823B53EC)
	// 823B53E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B53E4: D02B6B9C  stfs f1, 0x6b9c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(27548 as u32), tmp.u32 ) };
	// 823B53E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B53F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B53F0 size=12
    let mut pc: u32 = 0x823B53F0;
    'dispatch: loop {
        match pc {
            0x823B53F0 => {
    //   block [0x823B53F0..0x823B53FC)
	// 823B53F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B53F4: D02B959C  stfs f1, -0x6a64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27236 as u32), tmp.u32 ) };
	// 823B53F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5400 size=12
    let mut pc: u32 = 0x823B5400;
    'dispatch: loop {
        match pc {
            0x823B5400 => {
    //   block [0x823B5400..0x823B540C)
	// 823B5400: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5404: D02B95A0  stfs f1, -0x6a60(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27232 as u32), tmp.u32 ) };
	// 823B5408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5410 size=12
    let mut pc: u32 = 0x823B5410;
    'dispatch: loop {
        match pc {
            0x823B5410 => {
    //   block [0x823B5410..0x823B541C)
	// 823B5410: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5414: D02B95A4  stfs f1, -0x6a5c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27228 as u32), tmp.u32 ) };
	// 823B5418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5420 size=12
    let mut pc: u32 = 0x823B5420;
    'dispatch: loop {
        match pc {
            0x823B5420 => {
    //   block [0x823B5420..0x823B542C)
	// 823B5420: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5424: 986B6B58  stb r3, 0x6b58(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27480 as u32), ctx.r[3].u8 ) };
	// 823B5428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5430 size=12
    let mut pc: u32 = 0x823B5430;
    'dispatch: loop {
        match pc {
            0x823B5430 => {
    //   block [0x823B5430..0x823B543C)
	// 823B5430: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5434: 986B6B59  stb r3, 0x6b59(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27481 as u32), ctx.r[3].u8 ) };
	// 823B5438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5440 size=12
    let mut pc: u32 = 0x823B5440;
    'dispatch: loop {
        match pc {
            0x823B5440 => {
    //   block [0x823B5440..0x823B544C)
	// 823B5440: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5444: 986B6B5B  stb r3, 0x6b5b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27483 as u32), ctx.r[3].u8 ) };
	// 823B5448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5450 size=12
    let mut pc: u32 = 0x823B5450;
    'dispatch: loop {
        match pc {
            0x823B5450 => {
    //   block [0x823B5450..0x823B545C)
	// 823B5450: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5454: 986B940F  stb r3, -0x6bf1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27633 as u32), ctx.r[3].u8 ) };
	// 823B5458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5460 size=12
    let mut pc: u32 = 0x823B5460;
    'dispatch: loop {
        match pc {
            0x823B5460 => {
    //   block [0x823B5460..0x823B546C)
	// 823B5460: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5464: 986B6B5C  stb r3, 0x6b5c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27484 as u32), ctx.r[3].u8 ) };
	// 823B5468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5470 size=12
    let mut pc: u32 = 0x823B5470;
    'dispatch: loop {
        match pc {
            0x823B5470 => {
    //   block [0x823B5470..0x823B547C)
	// 823B5470: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5474: 986B9414  stb r3, -0x6bec(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27628 as u32), ctx.r[3].u8 ) };
	// 823B5478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5480 size=12
    let mut pc: u32 = 0x823B5480;
    'dispatch: loop {
        match pc {
            0x823B5480 => {
    //   block [0x823B5480..0x823B548C)
	// 823B5480: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5484: 986B9415  stb r3, -0x6beb(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27627 as u32), ctx.r[3].u8 ) };
	// 823B5488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5490 size=12
    let mut pc: u32 = 0x823B5490;
    'dispatch: loop {
        match pc {
            0x823B5490 => {
    //   block [0x823B5490..0x823B549C)
	// 823B5490: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5494: 986B6B5D  stb r3, 0x6b5d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27485 as u32), ctx.r[3].u8 ) };
	// 823B5498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B54A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B54A0 size=12
    let mut pc: u32 = 0x823B54A0;
    'dispatch: loop {
        match pc {
            0x823B54A0 => {
    //   block [0x823B54A0..0x823B54AC)
	// 823B54A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B54A4: 986B6B5E  stb r3, 0x6b5e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27486 as u32), ctx.r[3].u8 ) };
	// 823B54A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B54B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B54B0 size=12
    let mut pc: u32 = 0x823B54B0;
    'dispatch: loop {
        match pc {
            0x823B54B0 => {
    //   block [0x823B54B0..0x823B54BC)
	// 823B54B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B54B4: 986B6B5F  stb r3, 0x6b5f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27487 as u32), ctx.r[3].u8 ) };
	// 823B54B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B54C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B54C0 size=12
    let mut pc: u32 = 0x823B54C0;
    'dispatch: loop {
        match pc {
            0x823B54C0 => {
    //   block [0x823B54C0..0x823B54CC)
	// 823B54C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B54C4: 986B6B60  stb r3, 0x6b60(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27488 as u32), ctx.r[3].u8 ) };
	// 823B54C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B54D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B54D0 size=12
    let mut pc: u32 = 0x823B54D0;
    'dispatch: loop {
        match pc {
            0x823B54D0 => {
    //   block [0x823B54D0..0x823B54DC)
	// 823B54D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B54D4: 986B6B5A  stb r3, 0x6b5a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27482 as u32), ctx.r[3].u8 ) };
	// 823B54D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B54E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B54E0 size=12
    let mut pc: u32 = 0x823B54E0;
    'dispatch: loop {
        match pc {
            0x823B54E0 => {
    //   block [0x823B54E0..0x823B54EC)
	// 823B54E0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B54E4: D02B9418  stfs f1, -0x6be8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27624 as u32), tmp.u32 ) };
	// 823B54E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B54F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B54F0 size=12
    let mut pc: u32 = 0x823B54F0;
    'dispatch: loop {
        match pc {
            0x823B54F0 => {
    //   block [0x823B54F0..0x823B54FC)
	// 823B54F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B54F4: 986B6B61  stb r3, 0x6b61(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27489 as u32), ctx.r[3].u8 ) };
	// 823B54F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5500 size=12
    let mut pc: u32 = 0x823B5500;
    'dispatch: loop {
        match pc {
            0x823B5500 => {
    //   block [0x823B5500..0x823B550C)
	// 823B5500: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5504: D02B941C  stfs f1, -0x6be4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27620 as u32), tmp.u32 ) };
	// 823B5508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5510 size=12
    let mut pc: u32 = 0x823B5510;
    'dispatch: loop {
        match pc {
            0x823B5510 => {
    //   block [0x823B5510..0x823B551C)
	// 823B5510: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5514: 986B6B62  stb r3, 0x6b62(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27490 as u32), ctx.r[3].u8 ) };
	// 823B5518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5520 size=12
    let mut pc: u32 = 0x823B5520;
    'dispatch: loop {
        match pc {
            0x823B5520 => {
    //   block [0x823B5520..0x823B552C)
	// 823B5520: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5524: 986B9416  stb r3, -0x6bea(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27626 as u32), ctx.r[3].u8 ) };
	// 823B5528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5530 size=12
    let mut pc: u32 = 0x823B5530;
    'dispatch: loop {
        match pc {
            0x823B5530 => {
    //   block [0x823B5530..0x823B553C)
	// 823B5530: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5534: 986B6B63  stb r3, 0x6b63(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27491 as u32), ctx.r[3].u8 ) };
	// 823B5538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5540 size=12
    let mut pc: u32 = 0x823B5540;
    'dispatch: loop {
        match pc {
            0x823B5540 => {
    //   block [0x823B5540..0x823B554C)
	// 823B5540: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5544: D02B9420  stfs f1, -0x6be0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27616 as u32), tmp.u32 ) };
	// 823B5548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5550 size=12
    let mut pc: u32 = 0x823B5550;
    'dispatch: loop {
        match pc {
            0x823B5550 => {
    //   block [0x823B5550..0x823B555C)
	// 823B5550: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5554: D02B9424  stfs f1, -0x6bdc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27612 as u32), tmp.u32 ) };
	// 823B5558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5560 size=12
    let mut pc: u32 = 0x823B5560;
    'dispatch: loop {
        match pc {
            0x823B5560 => {
    //   block [0x823B5560..0x823B556C)
	// 823B5560: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5564: D02B9428  stfs f1, -0x6bd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27608 as u32), tmp.u32 ) };
	// 823B5568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5570 size=12
    let mut pc: u32 = 0x823B5570;
    'dispatch: loop {
        match pc {
            0x823B5570 => {
    //   block [0x823B5570..0x823B557C)
	// 823B5570: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5574: D02B942C  stfs f1, -0x6bd4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27604 as u32), tmp.u32 ) };
	// 823B5578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5580 size=12
    let mut pc: u32 = 0x823B5580;
    'dispatch: loop {
        match pc {
            0x823B5580 => {
    //   block [0x823B5580..0x823B558C)
	// 823B5580: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5584: D02B9430  stfs f1, -0x6bd0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27600 as u32), tmp.u32 ) };
	// 823B5588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5590 size=12
    let mut pc: u32 = 0x823B5590;
    'dispatch: loop {
        match pc {
            0x823B5590 => {
    //   block [0x823B5590..0x823B559C)
	// 823B5590: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5594: D02B9434  stfs f1, -0x6bcc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27596 as u32), tmp.u32 ) };
	// 823B5598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B55A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B55A0 size=12
    let mut pc: u32 = 0x823B55A0;
    'dispatch: loop {
        match pc {
            0x823B55A0 => {
    //   block [0x823B55A0..0x823B55AC)
	// 823B55A0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B55A4: D02B9438  stfs f1, -0x6bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27592 as u32), tmp.u32 ) };
	// 823B55A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B55B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B55B0 size=12
    let mut pc: u32 = 0x823B55B0;
    'dispatch: loop {
        match pc {
            0x823B55B0 => {
    //   block [0x823B55B0..0x823B55BC)
	// 823B55B0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B55B4: D02B943C  stfs f1, -0x6bc4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27588 as u32), tmp.u32 ) };
	// 823B55B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B55C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B55C0 size=16
    let mut pc: u32 = 0x823B55C0;
    'dispatch: loop {
        match pc {
            0x823B55C0 => {
    //   block [0x823B55C0..0x823B55D0)
	// 823B55C0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B55C4: EC010072  fmuls f0, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 823B55C8: D00B99D8  stfs f0, -0x6628(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26152 as u32), tmp.u32 ) };
	// 823B55CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B55D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B55D0 size=16
    let mut pc: u32 = 0x823B55D0;
    'dispatch: loop {
        match pc {
            0x823B55D0 => {
    //   block [0x823B55D0..0x823B55E0)
	// 823B55D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B55D4: EC010072  fmuls f0, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 823B55D8: D00B99DC  stfs f0, -0x6624(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26148 as u32), tmp.u32 ) };
	// 823B55DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B55E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B55E0 size=12
    let mut pc: u32 = 0x823B55E0;
    'dispatch: loop {
        match pc {
            0x823B55E0 => {
    //   block [0x823B55E0..0x823B55EC)
	// 823B55E0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B55E4: D02B9440  stfs f1, -0x6bc0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27584 as u32), tmp.u32 ) };
	// 823B55E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B55F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B55F0 size=12
    let mut pc: u32 = 0x823B55F0;
    'dispatch: loop {
        match pc {
            0x823B55F0 => {
    //   block [0x823B55F0..0x823B55FC)
	// 823B55F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B55F4: 906B9444  stw r3, -0x6bbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27580 as u32), ctx.r[3].u32 ) };
	// 823B55F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5600 size=12
    let mut pc: u32 = 0x823B5600;
    'dispatch: loop {
        match pc {
            0x823B5600 => {
    //   block [0x823B5600..0x823B560C)
	// 823B5600: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5604: D02B9448  stfs f1, -0x6bb8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27576 as u32), tmp.u32 ) };
	// 823B5608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5610 size=12
    let mut pc: u32 = 0x823B5610;
    'dispatch: loop {
        match pc {
            0x823B5610 => {
    //   block [0x823B5610..0x823B561C)
	// 823B5610: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5614: D02B944C  stfs f1, -0x6bb4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27572 as u32), tmp.u32 ) };
	// 823B5618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5620 size=12
    let mut pc: u32 = 0x823B5620;
    'dispatch: loop {
        match pc {
            0x823B5620 => {
    //   block [0x823B5620..0x823B562C)
	// 823B5620: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5624: D02B9450  stfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27568 as u32), tmp.u32 ) };
	// 823B5628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5630 size=12
    let mut pc: u32 = 0x823B5630;
    'dispatch: loop {
        match pc {
            0x823B5630 => {
    //   block [0x823B5630..0x823B563C)
	// 823B5630: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5634: D02B9454  stfs f1, -0x6bac(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27564 as u32), tmp.u32 ) };
	// 823B5638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5640 size=12
    let mut pc: u32 = 0x823B5640;
    'dispatch: loop {
        match pc {
            0x823B5640 => {
    //   block [0x823B5640..0x823B564C)
	// 823B5640: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5644: D02B9458  stfs f1, -0x6ba8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27560 as u32), tmp.u32 ) };
	// 823B5648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5650 size=16
    let mut pc: u32 = 0x823B5650;
    'dispatch: loop {
        match pc {
            0x823B5650 => {
    //   block [0x823B5650..0x823B5660)
	// 823B5650: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5654: EC010072  fmuls f0, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 823B5658: D00B99E0  stfs f0, -0x6620(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-26144 as u32), tmp.u32 ) };
	// 823B565C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5660 size=12
    let mut pc: u32 = 0x823B5660;
    'dispatch: loop {
        match pc {
            0x823B5660 => {
    //   block [0x823B5660..0x823B566C)
	// 823B5660: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5664: D02B945C  stfs f1, -0x6ba4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27556 as u32), tmp.u32 ) };
	// 823B5668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5670 size=12
    let mut pc: u32 = 0x823B5670;
    'dispatch: loop {
        match pc {
            0x823B5670 => {
    //   block [0x823B5670..0x823B567C)
	// 823B5670: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5674: 906B9460  stw r3, -0x6ba0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27552 as u32), ctx.r[3].u32 ) };
	// 823B5678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5680 size=480
    let mut pc: u32 = 0x823B5680;
    'dispatch: loop {
        match pc {
            0x823B5680 => {
    //   block [0x823B5680..0x823B5860)
	// 823B5680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B568C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5694: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5698: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823B569C: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B56A0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B56A4: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B56A8: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B56AC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B56B0: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B56B4: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B56B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B56BC: 419A0080  beq cr6, 0x823b573c
	if ctx.cr[6].eq {
	pc = 0x823B573C; continue 'dispatch;
	}
	// 823B56C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B56C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B56C8: 419A0070  beq cr6, 0x823b5738
	if ctx.cr[6].eq {
	pc = 0x823B5738; continue 'dispatch;
	}
	// 823B56CC: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B56D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B56D4: 419A0018  beq cr6, 0x823b56ec
	if ctx.cr[6].eq {
	pc = 0x823B56EC; continue 'dispatch;
	}
	// 823B56D8: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B56DC: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B56E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B56E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B56E8: 409A0008  bne cr6, 0x823b56f0
	if !ctx.cr[6].eq {
	pc = 0x823B56F0; continue 'dispatch;
	}
	// 823B56EC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 823B56F0: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B56F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B56F8: 419A0150  beq cr6, 0x823b5848
	if ctx.cr[6].eq {
	pc = 0x823B5848; continue 'dispatch;
	}
	// 823B56FC: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B5700: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B5704: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B5708: 419A0100  beq cr6, 0x823b5808
	if ctx.cr[6].eq {
	pc = 0x823B5808; continue 'dispatch;
	}
	// 823B570C: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B5710: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5714: 419A0030  beq cr6, 0x823b5744
	if ctx.cr[6].eq {
	pc = 0x823B5744; continue 'dispatch;
	}
	// 823B5718: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B571C: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5720: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B5724: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B5728: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B572C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B5730: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5734: 480000D8  b 0x823b580c
	pc = 0x823B580C; continue 'dispatch;
	// 823B5738: 4BDDE701  bl 0x82193e38
	ctx.lr = 0x823B573C;
	sub_82193E38(ctx, base);
	// 823B573C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B5740: 4BFFFFAC  b 0x823b56ec
	pc = 0x823B56EC; continue 'dispatch;
	// 823B5744: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5748: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B574C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823B5750: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B5754: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B5758: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B575C: 40810054  ble 0x823b57b0
	if !ctx.cr[0].gt {
	pc = 0x823B57B0; continue 'dispatch;
	}
	// 823B5760: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B5764: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B5768: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B576C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5770: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B5774: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B5778: 41980008  blt cr6, 0x823b5780
	if ctx.cr[6].lt {
	pc = 0x823B5780; continue 'dispatch;
	}
	// 823B577C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823B5780: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B5784: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B5788: 419A0014  beq cr6, 0x823b579c
	if ctx.cr[6].eq {
	pc = 0x823B579C; continue 'dispatch;
	}
	// 823B578C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B5790: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B5794: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B5798: 4800000C  b 0x823b57a4
	pc = 0x823B57A4; continue 'dispatch;
	// 823B579C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B57A0: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B57A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B57A8: 4199FFB8  bgt cr6, 0x823b5760
	if ctx.cr[6].gt {
	pc = 0x823B5760; continue 'dispatch;
	}
	// 823B57AC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B57B0: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B57B4: 419A0040  beq cr6, 0x823b57f4
	if ctx.cr[6].eq {
	pc = 0x823B57F4; continue 'dispatch;
	}
	// 823B57B8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B57BC: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B57C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B57C4: 41990008  bgt cr6, 0x823b57cc
	if ctx.cr[6].gt {
	pc = 0x823B57CC; continue 'dispatch;
	}
	// 823B57C8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B57CC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B57D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B57D4: 409A0020  bne cr6, 0x823b57f4
	if !ctx.cr[6].eq {
	pc = 0x823B57F4; continue 'dispatch;
	}
	// 823B57D8: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B57DC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B57E0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B57E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B57E8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B57EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B57F0: 4800001C  b 0x823b580c
	pc = 0x823B580C; continue 'dispatch;
	// 823B57F4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B57F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B57FC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B5800: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5804: 48000008  b 0x823b580c
	pc = 0x823B580C; continue 'dispatch;
	// 823B5808: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B580C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B5810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5814: 419A0034  beq cr6, 0x823b5848
	if ctx.cr[6].eq {
	pc = 0x823B5848; continue 'dispatch;
	}
	// 823B5818: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 823B581C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5820: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B5824: 4812D51D  bl 0x824e2d40
	ctx.lr = 0x823B5828;
	sub_824E2D40(ctx, base);
	// 823B5828: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823B582C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823B5830: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B5838: 4812D6F1  bl 0x824e2f28
	ctx.lr = 0x823B583C;
	sub_824E2F28(ctx, base);
	// 823B583C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 823B5840: 2F1F000F  cmpwi cr6, r31, 0xf
	ctx.cr[6].compare_i32(ctx.r[31].s32, 15, &mut ctx.xer);
	// 823B5844: 4198FFD8  blt cr6, 0x823b581c
	if ctx.cr[6].lt {
	pc = 0x823B581C; continue 'dispatch;
	}
	// 823B5848: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B584C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5854: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B5858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B585C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5860 size=448
    let mut pc: u32 = 0x823B5860;
    'dispatch: loop {
        match pc {
            0x823B5860 => {
    //   block [0x823B5860..0x823B5A20)
	// 823B5860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5864: 488F3BA9  bl 0x82ca940c
	ctx.lr = 0x823B5868;
	sub_82CA93D0(ctx, base);
	// 823B5868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B586C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5870: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823B5874: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823B5878: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B587C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B5880: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B5884: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5888: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B588C: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B5890: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B5894: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5898: 419A0080  beq cr6, 0x823b5918
	if ctx.cr[6].eq {
	pc = 0x823B5918; continue 'dispatch;
	}
	// 823B589C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B58A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B58A4: 419A0070  beq cr6, 0x823b5914
	if ctx.cr[6].eq {
	pc = 0x823B5914; continue 'dispatch;
	}
	// 823B58A8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B58AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B58B0: 419A0018  beq cr6, 0x823b58c8
	if ctx.cr[6].eq {
	pc = 0x823B58C8; continue 'dispatch;
	}
	// 823B58B4: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B58B8: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B58BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B58C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B58C4: 409A0008  bne cr6, 0x823b58cc
	if !ctx.cr[6].eq {
	pc = 0x823B58CC; continue 'dispatch;
	}
	// 823B58C8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 823B58CC: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B58D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B58D4: 419A0144  beq cr6, 0x823b5a18
	if ctx.cr[6].eq {
	pc = 0x823B5A18; continue 'dispatch;
	}
	// 823B58D8: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B58DC: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B58E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B58E4: 419A0100  beq cr6, 0x823b59e4
	if ctx.cr[6].eq {
	pc = 0x823B59E4; continue 'dispatch;
	}
	// 823B58E8: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B58EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B58F0: 419A0030  beq cr6, 0x823b5920
	if ctx.cr[6].eq {
	pc = 0x823B5920; continue 'dispatch;
	}
	// 823B58F4: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B58F8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B58FC: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B5900: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B5904: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5908: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B590C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5910: 480000D8  b 0x823b59e8
	pc = 0x823B59E8; continue 'dispatch;
	// 823B5914: 4BDDE525  bl 0x82193e38
	ctx.lr = 0x823B5918;
	sub_82193E38(ctx, base);
	// 823B5918: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B591C: 4BFFFFAC  b 0x823b58c8
	pc = 0x823B58C8; continue 'dispatch;
	// 823B5920: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5924: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B5928: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823B592C: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B5930: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B5934: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5938: 40810054  ble 0x823b598c
	if !ctx.cr[0].gt {
	pc = 0x823B598C; continue 'dispatch;
	}
	// 823B593C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B5940: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B5944: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B5948: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B594C: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B5950: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B5954: 41980008  blt cr6, 0x823b595c
	if ctx.cr[6].lt {
	pc = 0x823B595C; continue 'dispatch;
	}
	// 823B5958: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823B595C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B5960: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B5964: 419A0014  beq cr6, 0x823b5978
	if ctx.cr[6].eq {
	pc = 0x823B5978; continue 'dispatch;
	}
	// 823B5968: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B596C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B5970: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B5974: 4800000C  b 0x823b5980
	pc = 0x823B5980; continue 'dispatch;
	// 823B5978: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B597C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B5980: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5984: 4199FFB8  bgt cr6, 0x823b593c
	if ctx.cr[6].gt {
	pc = 0x823B593C; continue 'dispatch;
	}
	// 823B5988: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B598C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B5990: 419A0040  beq cr6, 0x823b59d0
	if ctx.cr[6].eq {
	pc = 0x823B59D0; continue 'dispatch;
	}
	// 823B5994: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5998: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B599C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B59A0: 41990008  bgt cr6, 0x823b59a8
	if ctx.cr[6].gt {
	pc = 0x823B59A8; continue 'dispatch;
	}
	// 823B59A4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B59A8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B59AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B59B0: 409A0020  bne cr6, 0x823b59d0
	if !ctx.cr[6].eq {
	pc = 0x823B59D0; continue 'dispatch;
	}
	// 823B59B4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B59B8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B59BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B59C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B59C4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B59C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B59CC: 4800001C  b 0x823b59e8
	pc = 0x823B59E8; continue 'dispatch;
	// 823B59D0: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B59D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B59D8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B59DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B59E0: 48000008  b 0x823b59e8
	pc = 0x823B59E8; continue 'dispatch;
	// 823B59E4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B59E8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B59EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B59F0: 419A0028  beq cr6, 0x823b5a18
	if ctx.cr[6].eq {
	pc = 0x823B5A18; continue 'dispatch;
	}
	// 823B59F4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 823B59F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823B59FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823B5A00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5A04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B5A08: 4812D521  bl 0x824e2f28
	ctx.lr = 0x823B5A0C;
	sub_824E2F28(ctx, base);
	// 823B5A0C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 823B5A10: 2F1F000F  cmpwi cr6, r31, 0xf
	ctx.cr[6].compare_i32(ctx.r[31].s32, 15, &mut ctx.xer);
	// 823B5A14: 4198FFE4  blt cr6, 0x823b59f8
	if ctx.cr[6].lt {
	pc = 0x823B59F8; continue 'dispatch;
	}
	// 823B5A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B5A1C: 488F3A40  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5A20 size=484
    let mut pc: u32 = 0x823B5A20;
    'dispatch: loop {
        match pc {
            0x823B5A20 => {
    //   block [0x823B5A20..0x823B5C04)
	// 823B5A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5A28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5A2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5A30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5A34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B5A38: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B5A3C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B5A40: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B5A44: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5A48: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5A4C: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B5A50: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B5A54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5A58: 419A0084  beq cr6, 0x823b5adc
	if ctx.cr[6].eq {
	pc = 0x823B5ADC; continue 'dispatch;
	}
	// 823B5A5C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5A60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5A64: 419A0074  beq cr6, 0x823b5ad8
	if ctx.cr[6].eq {
	pc = 0x823B5AD8; continue 'dispatch;
	}
	// 823B5A68: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B5A6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5A70: 419A0018  beq cr6, 0x823b5a88
	if ctx.cr[6].eq {
	pc = 0x823B5A88; continue 'dispatch;
	}
	// 823B5A74: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B5A78: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B5A7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B5A80: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B5A84: 409A0008  bne cr6, 0x823b5a8c
	if !ctx.cr[6].eq {
	pc = 0x823B5A8C; continue 'dispatch;
	}
	// 823B5A88: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 823B5A8C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B5A90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5A94: 419A015C  beq cr6, 0x823b5bf0
	if ctx.cr[6].eq {
	pc = 0x823B5BF0; continue 'dispatch;
	}
	// 823B5A98: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B5A9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B5AA0: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B5AA4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B5AA8: 419A0100  beq cr6, 0x823b5ba8
	if ctx.cr[6].eq {
	pc = 0x823B5BA8; continue 'dispatch;
	}
	// 823B5AAC: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B5AB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5AB4: 419A0030  beq cr6, 0x823b5ae4
	if ctx.cr[6].eq {
	pc = 0x823B5AE4; continue 'dispatch;
	}
	// 823B5AB8: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B5ABC: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5AC0: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B5AC4: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B5AC8: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5ACC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B5AD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5AD4: 480000D8  b 0x823b5bac
	pc = 0x823B5BAC; continue 'dispatch;
	// 823B5AD8: 4BDDE361  bl 0x82193e38
	ctx.lr = 0x823B5ADC;
	sub_82193E38(ctx, base);
	// 823B5ADC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B5AE0: 4BFFFFA8  b 0x823b5a88
	pc = 0x823B5A88; continue 'dispatch;
	// 823B5AE4: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5AE8: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B5AEC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B5AF0: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B5AF4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B5AF8: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5AFC: 40810054  ble 0x823b5b50
	if !ctx.cr[0].gt {
	pc = 0x823B5B50; continue 'dispatch;
	}
	// 823B5B00: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B5B04: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B5B08: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B5B0C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5B10: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B5B14: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B5B18: 41980008  blt cr6, 0x823b5b20
	if ctx.cr[6].lt {
	pc = 0x823B5B20; continue 'dispatch;
	}
	// 823B5B1C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B5B20: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B5B24: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B5B28: 419A0014  beq cr6, 0x823b5b3c
	if ctx.cr[6].eq {
	pc = 0x823B5B3C; continue 'dispatch;
	}
	// 823B5B2C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B5B30: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B5B34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B5B38: 4800000C  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5B3C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B5B40: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B5B44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5B48: 4199FFB8  bgt cr6, 0x823b5b00
	if ctx.cr[6].gt {
	pc = 0x823B5B00; continue 'dispatch;
	}
	// 823B5B4C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B5B50: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B5B54: 419A0040  beq cr6, 0x823b5b94
	if ctx.cr[6].eq {
	pc = 0x823B5B94; continue 'dispatch;
	}
	// 823B5B58: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5B5C: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B5B60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5B64: 41990008  bgt cr6, 0x823b5b6c
	if ctx.cr[6].gt {
	pc = 0x823B5B6C; continue 'dispatch;
	}
	// 823B5B68: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B5B6C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B5B70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5B74: 409A0020  bne cr6, 0x823b5b94
	if !ctx.cr[6].eq {
	pc = 0x823B5B94; continue 'dispatch;
	}
	// 823B5B78: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B5B7C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B5B80: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B5B84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5B88: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B5B8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5B90: 4800001C  b 0x823b5bac
	pc = 0x823B5BAC; continue 'dispatch;
	// 823B5B94: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B5B98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5B9C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B5BA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5BA4: 48000008  b 0x823b5bac
	pc = 0x823B5BAC; continue 'dispatch;
	// 823B5BA8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B5BAC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B5BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5BB4: 419A003C  beq cr6, 0x823b5bf0
	if ctx.cr[6].eq {
	pc = 0x823B5BF0; continue 'dispatch;
	}
	// 823B5BB8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 823B5BBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823B5BC0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 823B5BC4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 823B5BC8: 4812EFA9  bl 0x824e4b70
	ctx.lr = 0x823B5BCC;
	sub_824E4B70(ctx, base);
	// 823B5BCC: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B5BD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B5BD4: 419A001C  beq cr6, 0x823b5bf0
	if ctx.cr[6].eq {
	pc = 0x823B5BF0; continue 'dispatch;
	}
	// 823B5BD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5BDC: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B5BE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5BE4: 4800969D  bl 0x823bf280
	ctx.lr = 0x823B5BE8;
	sub_823BF280(ctx, base);
	// 823B5BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B5BEC: 4BE6614D  bl 0x8221bd38
	ctx.lr = 0x823B5BF0;
	sub_8221BD38(ctx, base);
	// 823B5BF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B5BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5BFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B5C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5C08 size=124
    let mut pc: u32 = 0x823B5C08;
    'dispatch: loop {
        match pc {
            0x823B5C08 => {
    //   block [0x823B5C08..0x823B5C84)
	// 823B5C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5C14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B5C18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5C1C: 4BE0C1A5  bl 0x821c1dc0
	ctx.lr = 0x823B5C20;
	sub_821C1DC0(ctx, base);
	// 823B5C20: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5C24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823B5C28: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B5C2C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823B5C30: 806A0054  lwz r3, 0x54(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B5C34: 4BFC504D  bl 0x8237ac80
	ctx.lr = 0x823B5C38;
	sub_8237AC80(ctx, base);
	// 823B5C38: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5C3C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 823B5C40: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823B5C44: 40990008  ble cr6, 0x823b5c4c
	if !ctx.cr[6].gt {
	pc = 0x823B5C4C; continue 'dispatch;
	}
	// 823B5C48: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823B5C4C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 823B5C50: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823B5C54: 40990008  ble cr6, 0x823b5c5c
	if !ctx.cr[6].gt {
	pc = 0x823B5C5C; continue 'dispatch;
	}
	// 823B5C58: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823B5C5C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823B5C60: 419A0014  beq cr6, 0x823b5c74
	if ctx.cr[6].eq {
	pc = 0x823B5C74; continue 'dispatch;
	}
	// 823B5C64: 41980008  blt cr6, 0x823b5c6c
	if ctx.cr[6].lt {
	pc = 0x823B5C6C; continue 'dispatch;
	}
	// 823B5C68: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823B5C6C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 823B5C70: 4BFFFFE0  b 0x823b5c50
	pc = 0x823B5C50; continue 'dispatch;
	// 823B5C74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823B5C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5C88 size=112
    let mut pc: u32 = 0x823B5C88;
    'dispatch: loop {
        match pc {
            0x823B5C88 => {
    //   block [0x823B5C88..0x823B5CF8)
	// 823B5C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5C90: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 823B5C94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5C98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 823B5C9C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B5CA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B5CA4: 388BE438  addi r4, r11, -0x1bc8
	ctx.r[4].s64 = ctx.r[11].s64 + -7112;
	// 823B5CA8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823B5CAC: 4BE4AAC5  bl 0x82200770
	ctx.lr = 0x823B5CB0;
	sub_82200770(ctx, base);
	// 823B5CB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5CB4: 487E11C5  bl 0x82b96e78
	ctx.lr = 0x823B5CB8;
	sub_82B96E78(ctx, base);
	// 823B5CB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B5CBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5CC0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823B5CC4: 4884CEA5  bl 0x82c02b68
	ctx.lr = 0x823B5CC8;
	sub_82C02B68(ctx, base);
	// 823B5CC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5CCC: 487E11AD  bl 0x82b96e78
	ctx.lr = 0x823B5CD0;
	sub_82B96E78(ctx, base);
	// 823B5CD0: 8921005C  lbz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823B5CD4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B5CD8: 419A000C  beq cr6, 0x823b5ce4
	if ctx.cr[6].eq {
	pc = 0x823B5CE4; continue 'dispatch;
	}
	// 823B5CDC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B5CE0: 48F03C75  bl 0x832b9954
	ctx.lr = 0x823B5CE4;
	// extern call 0x832B9954 → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 823B5CE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B5CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5CF0: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B5CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5CF8 size=12
    let mut pc: u32 = 0x823B5CF8;
    'dispatch: loop {
        match pc {
            0x823B5CF8 => {
    //   block [0x823B5CF8..0x823B5D04)
	// 823B5CF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5CFC: 986B6B4A  stb r3, 0x6b4a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27466 as u32), ctx.r[3].u8 ) };
	// 823B5D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D08 size=12
    let mut pc: u32 = 0x823B5D08;
    'dispatch: loop {
        match pc {
            0x823B5D08 => {
    //   block [0x823B5D08..0x823B5D14)
	// 823B5D08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D0C: 986B6B4B  stb r3, 0x6b4b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27467 as u32), ctx.r[3].u8 ) };
	// 823B5D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D18 size=12
    let mut pc: u32 = 0x823B5D18;
    'dispatch: loop {
        match pc {
            0x823B5D18 => {
    //   block [0x823B5D18..0x823B5D24)
	// 823B5D18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D1C: 986B6B4C  stb r3, 0x6b4c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27468 as u32), ctx.r[3].u8 ) };
	// 823B5D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D28 size=16
    let mut pc: u32 = 0x823B5D28;
    'dispatch: loop {
        match pc {
            0x823B5D28 => {
    //   block [0x823B5D28..0x823B5D38)
	// 823B5D28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B5D30: 386B79E4  addi r3, r11, 0x79e4
	ctx.r[3].s64 = ctx.r[11].s64 + 31204;
	// 823B5D34: 4BEAF46C  b 0x822651a0
	sub_822651A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D38 size=12
    let mut pc: u32 = 0x823B5D38;
    'dispatch: loop {
        match pc {
            0x823B5D38 => {
    //   block [0x823B5D38..0x823B5D44)
	// 823B5D38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D3C: 986B6B96  stb r3, 0x6b96(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27542 as u32), ctx.r[3].u8 ) };
	// 823B5D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D48 size=12
    let mut pc: u32 = 0x823B5D48;
    'dispatch: loop {
        match pc {
            0x823B5D48 => {
    //   block [0x823B5D48..0x823B5D54)
	// 823B5D48: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5D4C: 986B9581  stb r3, -0x6a7f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27263 as u32), ctx.r[3].u8 ) };
	// 823B5D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D58 size=12
    let mut pc: u32 = 0x823B5D58;
    'dispatch: loop {
        match pc {
            0x823B5D58 => {
    //   block [0x823B5D58..0x823B5D64)
	// 823B5D58: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B5D5C: 986B9582  stb r3, -0x6a7e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27262 as u32), ctx.r[3].u8 ) };
	// 823B5D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D68 size=12
    let mut pc: u32 = 0x823B5D68;
    'dispatch: loop {
        match pc {
            0x823B5D68 => {
    //   block [0x823B5D68..0x823B5D74)
	// 823B5D68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D6C: 986B6B97  stb r3, 0x6b97(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27543 as u32), ctx.r[3].u8 ) };
	// 823B5D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D78 size=12
    let mut pc: u32 = 0x823B5D78;
    'dispatch: loop {
        match pc {
            0x823B5D78 => {
    //   block [0x823B5D78..0x823B5D84)
	// 823B5D78: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D7C: 986B6B98  stb r3, 0x6b98(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27544 as u32), ctx.r[3].u8 ) };
	// 823B5D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D88 size=12
    let mut pc: u32 = 0x823B5D88;
    'dispatch: loop {
        match pc {
            0x823B5D88 => {
    //   block [0x823B5D88..0x823B5D94)
	// 823B5D88: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D8C: 986B6B99  stb r3, 0x6b99(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27545 as u32), ctx.r[3].u8 ) };
	// 823B5D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5D98 size=12
    let mut pc: u32 = 0x823B5D98;
    'dispatch: loop {
        match pc {
            0x823B5D98 => {
    //   block [0x823B5D98..0x823B5DA4)
	// 823B5D98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5D9C: 986B6B9A  stb r3, 0x6b9a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27546 as u32), ctx.r[3].u8 ) };
	// 823B5DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5DA8 size=252
    let mut pc: u32 = 0x823B5DA8;
    'dispatch: loop {
        match pc {
            0x823B5DA8 => {
    //   block [0x823B5DA8..0x823B5EA4)
	// 823B5DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5DAC: 488F365D  bl 0x82ca9408
	ctx.lr = 0x823B5DB0;
	sub_82CA93D0(ctx, base);
	// 823B5DB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5DB4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 823B5DB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 823B5DBC: 81696B44  lwz r11, 0x6b44(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(27460 as u32) ) } as u64;
	// 823B5DC0: 814A6AB8  lwz r10, 0x6ab8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B5DC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5DC8: 409A001C  bne cr6, 0x823b5de4
	if !ctx.cr[6].eq {
	pc = 0x823B5DE4; continue 'dispatch;
	}
	// 823B5DCC: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B5DD0: 810B0084  lwz r8, 0x84(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 823B5DD4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5DD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823B5DDC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B5DE0: 91696B44  stw r11, 0x6b44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(27460 as u32), ctx.r[11].u32 ) };
	// 823B5DE4: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 823B5DE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823B5DEC: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 823B5DF0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B5DF4: C0099490  lfs f0, -0x6b70(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B5DF8: ED810028  fsubs f12, f1, f0
	ctx.f[12].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 823B5DFC: C1A89A80  lfs f13, -0x6580(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-25984 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B5E00: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 823B5E04: FD606210  fabs f11, f12
	ctx.f[11].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 823B5E08: FF0B6800  fcmpu cr6, f11, f13
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[13].f64);
	// 823B5E0C: 40990028  ble cr6, 0x823b5e34
	if !ctx.cr[6].gt {
	pc = 0x823B5E34; continue 'dispatch;
	}
	// 823B5E10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823B5E14: D0210060  stfs f1, 0x60(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 823B5E18: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 823B5E1C: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 823B5E20: 48336209  bl 0x826ec028
	ctx.lr = 0x823B5E24;
	sub_826EC028(ctx, base);
	// 823B5E24: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 823B5E28: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 823B5E2C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823B5E30: 488F3628  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 823B5E34: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 823B5E38: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823B5E3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B5E40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5E44: 48336585  bl 0x826ec3c8
	ctx.lr = 0x823B5E48;
	sub_826EC3C8(ctx, base);
	// 823B5E48: EBA10060  ld r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 823B5E4C: 8381006C  lwz r28, 0x6c(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B5E50: 83C10068  lwz r30, 0x68(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 823B5E54: FBA10050  std r29, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u64 ) };
	// 823B5E58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B5E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5E60: 419A000C  beq cr6, 0x823b5e6c
	if ctx.cr[6].eq {
	pc = 0x823B5E6C; continue 'dispatch;
	}
	// 823B5E64: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823B5E68: 419A0008  beq cr6, 0x823b5e70
	if ctx.cr[6].eq {
	pc = 0x823B5E70; continue 'dispatch;
	}
	// 823B5E6C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823B5E70: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B5E74: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 823B5E78: 419A0010  beq cr6, 0x823b5e88
	if ctx.cr[6].eq {
	pc = 0x823B5E88; continue 'dispatch;
	}
	// 823B5E7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5E80: 48086F71  bl 0x8243cdf0
	ctx.lr = 0x823B5E84;
	sub_8243CDF0(ctx, base);
	// 823B5E84: 4BFFFFD4  b 0x823b5e58
	pc = 0x823B5E58; continue 'dispatch;
	// 823B5E88: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823B5E8C: E8C10068  ld r6, 0x68(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 823B5E90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5E94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B5E98: 4BF3A971  bl 0x822f0808
	ctx.lr = 0x823B5E9C;
	sub_822F0808(ctx, base);
	// 823B5E9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823B5EA0: 488F35B8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5EA8 size=48
    let mut pc: u32 = 0x823B5EA8;
    'dispatch: loop {
        match pc {
            0x823B5EA8 => {
    //   block [0x823B5EA8..0x823B5ED8)
	// 823B5EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5EB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5EB8: 4890C589  bl 0x82cc2440
	ctx.lr = 0x823B5EBC;
	sub_82CC2440(ctx, base);
	// 823B5EBC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823B5EC0: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B5EC4: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 823B5EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B5ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5ED8 size=448
    let mut pc: u32 = 0x823B5ED8;
    'dispatch: loop {
        match pc {
            0x823B5ED8 => {
    //   block [0x823B5ED8..0x823B6098)
	// 823B5ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5EE0: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 823B5EE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5EE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B5EEC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B5EF0: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B5EF4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B5EF8: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B5EFC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5F00: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5F04: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B5F08: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B5F0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5F10: 419A0084  beq cr6, 0x823b5f94
	if ctx.cr[6].eq {
	pc = 0x823B5F94; continue 'dispatch;
	}
	// 823B5F14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5F18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5F1C: 419A0074  beq cr6, 0x823b5f90
	if ctx.cr[6].eq {
	pc = 0x823B5F90; continue 'dispatch;
	}
	// 823B5F20: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B5F24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5F28: 419A0018  beq cr6, 0x823b5f40
	if ctx.cr[6].eq {
	pc = 0x823B5F40; continue 'dispatch;
	}
	// 823B5F2C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B5F30: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B5F34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B5F38: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B5F3C: 409A0008  bne cr6, 0x823b5f44
	if !ctx.cr[6].eq {
	pc = 0x823B5F44; continue 'dispatch;
	}
	// 823B5F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B5F44: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B5F48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5F4C: 419A0138  beq cr6, 0x823b6084
	if ctx.cr[6].eq {
	pc = 0x823B6084; continue 'dispatch;
	}
	// 823B5F50: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B5F54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B5F58: 5549E7FE  rlwinm r9, r10, 0x1c, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 823B5F5C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B5F60: 419A0104  beq cr6, 0x823b6064
	if ctx.cr[6].eq {
	pc = 0x823B6064; continue 'dispatch;
	}
	// 823B5F64: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B5F68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B5F6C: 419A0030  beq cr6, 0x823b5f9c
	if ctx.cr[6].eq {
	pc = 0x823B5F9C; continue 'dispatch;
	}
	// 823B5F70: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B5F74: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5F78: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B5F7C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B5F80: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B5F84: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B5F88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B5F8C: 480000DC  b 0x823b6068
	pc = 0x823B6068; continue 'dispatch;
	// 823B5F90: 4BDDDEA9  bl 0x82193e38
	ctx.lr = 0x823B5F94;
	sub_82193E38(ctx, base);
	// 823B5F94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B5F98: 4BFFFFA8  b 0x823b5f40
	pc = 0x823B5F40; continue 'dispatch;
	// 823B5F9C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B5FA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B5FA4: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B5FA8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B5FAC: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B5FB0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B5FB4: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5FB8: 40810054  ble 0x823b600c
	if !ctx.cr[0].gt {
	pc = 0x823B600C; continue 'dispatch;
	}
	// 823B5FBC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B5FC0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B5FC4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B5FC8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5FCC: 2F070024  cmpwi cr6, r7, 0x24
	ctx.cr[6].compare_i32(ctx.r[7].s32, 36, &mut ctx.xer);
	// 823B5FD0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B5FD4: 41980008  blt cr6, 0x823b5fdc
	if ctx.cr[6].lt {
	pc = 0x823B5FDC; continue 'dispatch;
	}
	// 823B5FD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B5FDC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B5FE0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B5FE4: 419A0014  beq cr6, 0x823b5ff8
	if ctx.cr[6].eq {
	pc = 0x823B5FF8; continue 'dispatch;
	}
	// 823B5FE8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B5FEC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B5FF0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B5FF4: 4800000C  b 0x823b6000
	pc = 0x823B6000; continue 'dispatch;
	// 823B5FF8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B5FFC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B6000: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6004: 4199FFB8  bgt cr6, 0x823b5fbc
	if ctx.cr[6].gt {
	pc = 0x823B5FBC; continue 'dispatch;
	}
	// 823B6008: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B600C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B6010: 419A0040  beq cr6, 0x823b6050
	if ctx.cr[6].eq {
	pc = 0x823B6050; continue 'dispatch;
	}
	// 823B6014: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6018: 2F0B0024  cmpwi cr6, r11, 0x24
	ctx.cr[6].compare_i32(ctx.r[11].s32, 36, &mut ctx.xer);
	// 823B601C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6020: 41990008  bgt cr6, 0x823b6028
	if ctx.cr[6].gt {
	pc = 0x823B6028; continue 'dispatch;
	}
	// 823B6024: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6028: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B602C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6030: 409A0020  bne cr6, 0x823b6050
	if !ctx.cr[6].eq {
	pc = 0x823B6050; continue 'dispatch;
	}
	// 823B6034: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B6038: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B603C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6040: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6044: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B6048: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B604C: 4800001C  b 0x823b6068
	pc = 0x823B6068; continue 'dispatch;
	// 823B6050: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6054: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6058: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B605C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6060: 48000008  b 0x823b6068
	pc = 0x823B6068; continue 'dispatch;
	// 823B6064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6068: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B606C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6070: 419A0014  beq cr6, 0x823b6084
	if ctx.cr[6].eq {
	pc = 0x823B6084; continue 'dispatch;
	}
	// 823B6074: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B6078: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B607C: EC3F0028  fsubs f1, f31, f0
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 823B6080: 48115BE9  bl 0x824cbc68
	ctx.lr = 0x823B6084;
	sub_824CBC68(ctx, base);
	// 823B6084: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B608C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6090: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6098 size=444
    let mut pc: u32 = 0x823B6098;
    'dispatch: loop {
        match pc {
            0x823B6098 => {
    //   block [0x823B6098..0x823B6254)
	// 823B6098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B609C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B60A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B60A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B60A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B60AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B60B0: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B60B4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B60B8: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B60BC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B60C0: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B60C4: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B60C8: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B60CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B60D0: 419A0084  beq cr6, 0x823b6154
	if ctx.cr[6].eq {
	pc = 0x823B6154; continue 'dispatch;
	}
	// 823B60D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B60D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B60DC: 419A0074  beq cr6, 0x823b6150
	if ctx.cr[6].eq {
	pc = 0x823B6150; continue 'dispatch;
	}
	// 823B60E0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B60E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B60E8: 419A0018  beq cr6, 0x823b6100
	if ctx.cr[6].eq {
	pc = 0x823B6100; continue 'dispatch;
	}
	// 823B60EC: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B60F0: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B60F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B60F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B60FC: 409A0008  bne cr6, 0x823b6104
	if !ctx.cr[6].eq {
	pc = 0x823B6104; continue 'dispatch;
	}
	// 823B6100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B6104: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B6108: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B610C: 419A0134  beq cr6, 0x823b6240
	if ctx.cr[6].eq {
	pc = 0x823B6240; continue 'dispatch;
	}
	// 823B6110: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 823B6114: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B6118: 55491FFE  rlwinm r9, r10, 3, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 823B611C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6120: 419A0104  beq cr6, 0x823b6224
	if ctx.cr[6].eq {
	pc = 0x823B6224; continue 'dispatch;
	}
	// 823B6124: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B6128: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B612C: 419A0030  beq cr6, 0x823b615c
	if ctx.cr[6].eq {
	pc = 0x823B615C; continue 'dispatch;
	}
	// 823B6130: 894A005D  lbz r10, 0x5d(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(93 as u32) ) } as u64;
	// 823B6134: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6138: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B613C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B6140: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6144: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B6148: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B614C: 480000DC  b 0x823b6228
	pc = 0x823B6228; continue 'dispatch;
	// 823B6150: 4BDDDCE9  bl 0x82193e38
	ctx.lr = 0x823B6154;
	sub_82193E38(ctx, base);
	// 823B6154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6158: 4BFFFFA8  b 0x823b6100
	pc = 0x823B6100; continue 'dispatch;
	// 823B615C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6160: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B6164: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B6168: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B616C: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B6170: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6174: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6178: 40810054  ble 0x823b61cc
	if !ctx.cr[0].gt {
	pc = 0x823B61CC; continue 'dispatch;
	}
	// 823B617C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B6180: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B6184: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B6188: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B618C: 2F07005D  cmpwi cr6, r7, 0x5d
	ctx.cr[6].compare_i32(ctx.r[7].s32, 93, &mut ctx.xer);
	// 823B6190: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B6194: 41980008  blt cr6, 0x823b619c
	if ctx.cr[6].lt {
	pc = 0x823B619C; continue 'dispatch;
	}
	// 823B6198: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B619C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B61A0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B61A4: 419A0014  beq cr6, 0x823b61b8
	if ctx.cr[6].eq {
	pc = 0x823B61B8; continue 'dispatch;
	}
	// 823B61A8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B61AC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B61B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B61B4: 4800000C  b 0x823b61c0
	pc = 0x823B61C0; continue 'dispatch;
	// 823B61B8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B61BC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B61C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B61C4: 4199FFB8  bgt cr6, 0x823b617c
	if ctx.cr[6].gt {
	pc = 0x823B617C; continue 'dispatch;
	}
	// 823B61C8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B61CC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B61D0: 419A0040  beq cr6, 0x823b6210
	if ctx.cr[6].eq {
	pc = 0x823B6210; continue 'dispatch;
	}
	// 823B61D4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B61D8: 2F0B005D  cmpwi cr6, r11, 0x5d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 93, &mut ctx.xer);
	// 823B61DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B61E0: 41990008  bgt cr6, 0x823b61e8
	if ctx.cr[6].gt {
	pc = 0x823B61E8; continue 'dispatch;
	}
	// 823B61E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B61E8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B61EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B61F0: 409A0020  bne cr6, 0x823b6210
	if !ctx.cr[6].eq {
	pc = 0x823B6210; continue 'dispatch;
	}
	// 823B61F4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B61F8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B61FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6200: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6204: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B6208: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B620C: 4800001C  b 0x823b6228
	pc = 0x823B6228; continue 'dispatch;
	// 823B6210: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6214: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6218: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B621C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6220: 48000008  b 0x823b6228
	pc = 0x823B6228; continue 'dispatch;
	// 823B6224: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6228: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B622C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6230: 419A0010  beq cr6, 0x823b6240
	if ctx.cr[6].eq {
	pc = 0x823B6240; continue 'dispatch;
	}
	// 823B6234: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B6238: 7C8BF850  subf r4, r11, r31
	ctx.r[4].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 823B623C: 48438035  bl 0x827ee270
	ctx.lr = 0x823B6240;
	sub_827EE270(ctx, base);
	// 823B6240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B624C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B6258 size=880
    let mut pc: u32 = 0x823B6258;
    'dispatch: loop {
        match pc {
            0x823B6258 => {
    //   block [0x823B6258..0x823B65C8)
	// 823B6258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B625C: 488F31B1  bl 0x82ca940c
	ctx.lr = 0x823B6260;
	sub_82CA93D0(ctx, base);
	// 823B6260: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823B6264: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6268: 3FC08349  lis r30, -0x7cb7
	ctx.r[30].s64 = -2092367872;
	// 823B626C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B6270: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823B6274: 817E6AB8  lwz r11, 0x6ab8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B6278: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B627C: 814B0058  lwz r10, 0x58(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B6280: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6284: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6288: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B628C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B6290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6294: 419A0080  beq cr6, 0x823b6314
	if ctx.cr[6].eq {
	pc = 0x823B6314; continue 'dispatch;
	}
	// 823B6298: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B629C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B62A0: 419A0070  beq cr6, 0x823b6310
	if ctx.cr[6].eq {
	pc = 0x823B6310; continue 'dispatch;
	}
	// 823B62A4: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B62A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B62AC: 419A0018  beq cr6, 0x823b62c4
	if ctx.cr[6].eq {
	pc = 0x823B62C4; continue 'dispatch;
	}
	// 823B62B0: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B62B4: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B62B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B62BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B62C0: 409A0008  bne cr6, 0x823b62c8
	if !ctx.cr[6].eq {
	pc = 0x823B62C8; continue 'dispatch;
	}
	// 823B62C4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 823B62C8: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B62CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B62D0: 419A02EC  beq cr6, 0x823b65bc
	if ctx.cr[6].eq {
	pc = 0x823B65BC; continue 'dispatch;
	}
	// 823B62D4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B62D8: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 823B62DC: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B62E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B62E4: 419A00F4  beq cr6, 0x823b63d8
	if ctx.cr[6].eq {
	pc = 0x823B63D8; continue 'dispatch;
	}
	// 823B62E8: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B62EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B62F0: 419A002C  beq cr6, 0x823b631c
	if ctx.cr[6].eq {
	pc = 0x823B631C; continue 'dispatch;
	}
	// 823B62F4: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B62F8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B62FC: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B6300: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B6304: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6308: 83E90004  lwz r31, 4(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B630C: 480000D0  b 0x823b63dc
	pc = 0x823B63DC; continue 'dispatch;
	// 823B6310: 4BDDDB29  bl 0x82193e38
	ctx.lr = 0x823B6314;
	sub_82193E38(ctx, base);
	// 823B6314: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B6318: 4BFFFFAC  b 0x823b62c4
	pc = 0x823B62C4; continue 'dispatch;
	// 823B631C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6320: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B6324: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B6328: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B632C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6330: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6334: 40810054  ble 0x823b6388
	if !ctx.cr[0].gt {
	pc = 0x823B6388; continue 'dispatch;
	}
	// 823B6338: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B633C: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B6340: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B6344: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6348: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B634C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B6350: 41980008  blt cr6, 0x823b6358
	if ctx.cr[6].lt {
	pc = 0x823B6358; continue 'dispatch;
	}
	// 823B6354: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B6358: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B635C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B6360: 419A0014  beq cr6, 0x823b6374
	if ctx.cr[6].eq {
	pc = 0x823B6374; continue 'dispatch;
	}
	// 823B6364: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B6368: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B636C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B6370: 4800000C  b 0x823b637c
	pc = 0x823B637C; continue 'dispatch;
	// 823B6374: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B6378: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B637C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6380: 4199FFB8  bgt cr6, 0x823b6338
	if ctx.cr[6].gt {
	pc = 0x823B6338; continue 'dispatch;
	}
	// 823B6384: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6388: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B638C: 419A003C  beq cr6, 0x823b63c8
	if ctx.cr[6].eq {
	pc = 0x823B63C8; continue 'dispatch;
	}
	// 823B6390: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6394: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B6398: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B639C: 41990008  bgt cr6, 0x823b63a4
	if ctx.cr[6].gt {
	pc = 0x823B63A4; continue 'dispatch;
	}
	// 823B63A0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B63A4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B63A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B63AC: 409A001C  bne cr6, 0x823b63c8
	if !ctx.cr[6].eq {
	pc = 0x823B63C8; continue 'dispatch;
	}
	// 823B63B0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B63B4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B63B8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B63BC: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B63C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B63C4: 48000018  b 0x823b63dc
	pc = 0x823B63DC; continue 'dispatch;
	// 823B63C8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B63CC: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B63D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B63D4: 48000008  b 0x823b63dc
	pc = 0x823B63DC; continue 'dispatch;
	// 823B63D8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B63DC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B63E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B63E4: 419A01D8  beq cr6, 0x823b65bc
	if ctx.cr[6].eq {
	pc = 0x823B65BC; continue 'dispatch;
	}
	// 823B63E8: C1BF0014  lfs f13, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B63EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B63F0: ED9F6828  fsubs f12, f31, f13
	ctx.f[12].f64 = (((ctx.f[31].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B63F4: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B63F8: C17F0018  lfs f11, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B63FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6400: 392A0E68  addi r9, r10, 0xe68
	ctx.r[9].s64 = ctx.r[10].s64 + 3688;
	// 823B6404: C00B9484  lfs f0, -0x6b7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B6408: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 823B640C: 7D000026  mfcr r8
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[8].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6410: 5507DF7A  rlwinm r7, r8, 0x1b, 0x1d, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 823B6414: 5506F77A  rlwinm r6, r8, 0x1e, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 823B6418: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 823B641C: 7D492C2E  lfsx f10, r9, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 823B6420: FD2A6FEE  fsel f9, f10, f31, f13
	ctx.f[9].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[13].f64 };
	// 823B6424: ED095828  fsubs f8, f9, f11
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[11].f64) as f32) as f64);
	// 823B6428: FF080000  fcmpu cr6, f8, f0
	ctx.cr[6].compare_f64(ctx.f[8].f64, ctx.f[0].f64);
	// 823B642C: 7C800026  mfcr r4
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[4].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6430: 548BDF7A  rlwinm r11, r4, 0x1b, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 823B6434: 548AF77A  rlwinm r10, r4, 0x1e, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000003u64;
	// 823B6438: 7D685378  or r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823B643C: 7CE9442E  lfsx f7, r9, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 823B6440: FFE74AEE  fsel f31, f7, f11, f9
	ctx.f[31].f64 = if ctx.f[7].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[9].f64 };
	// 823B6444: 4BE86EDD  bl 0x8223d320
	ctx.lr = 0x823B6448;
	sub_8223D320(ctx, base);
	// 823B6448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B644C: EC3F0828  fsubs f1, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 823B6450: 4811F771  bl 0x824d5bc0
	ctx.lr = 0x823B6454;
	sub_824D5BC0(ctx, base);
	// 823B6454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6458: 481256C9  bl 0x824dbb20
	ctx.lr = 0x823B645C;
	sub_824DBB20(ctx, base);
	// 823B645C: 817E6AB8  lwz r11, 0x6ab8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B6460: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B6464: 80C70058  lwz r6, 0x58(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B6468: 80A60004  lwz r5, 4(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B646C: 80650000  lwz r3, 0(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6470: 4BFAD8C9  bl 0x82363d38
	ctx.lr = 0x823B6474;
	sub_82363D38(ctx, base);
	// 823B6474: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6478: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B647C: 419A0018  beq cr6, 0x823b6494
	if ctx.cr[6].eq {
	pc = 0x823B6494; continue 'dispatch;
	}
	// 823B6480: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B6484: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B6488: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B648C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6490: 409A0008  bne cr6, 0x823b6498
	if !ctx.cr[6].eq {
	pc = 0x823B6498; continue 'dispatch;
	}
	// 823B6494: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 823B6498: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B649C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B64A0: 419A011C  beq cr6, 0x823b65bc
	if ctx.cr[6].eq {
	pc = 0x823B65BC; continue 'dispatch;
	}
	// 823B64A4: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 823B64A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823B64AC: 55494FFE  rlwinm r9, r10, 9, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x007FFFFFu64;
	// 823B64B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B64B4: 419A00F4  beq cr6, 0x823b65a8
	if ctx.cr[6].eq {
	pc = 0x823B65A8; continue 'dispatch;
	}
	// 823B64B8: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B64BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B64C0: 419A0024  beq cr6, 0x823b64e4
	if ctx.cr[6].eq {
	pc = 0x823B64E4; continue 'dispatch;
	}
	// 823B64C4: 894A00D7  lbz r10, 0xd7(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(215 as u32) ) } as u64;
	// 823B64C8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B64CC: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B64D0: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B64D4: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B64D8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B64DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B64E0: 480000CC  b 0x823b65ac
	pc = 0x823B65AC; continue 'dispatch;
	// 823B64E4: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B64E8: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B64EC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B64F0: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B64F4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B64F8: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B64FC: 40810054  ble 0x823b6550
	if !ctx.cr[0].gt {
	pc = 0x823B6550; continue 'dispatch;
	}
	// 823B6500: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B6504: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B6508: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B650C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6510: 2F0700D7  cmpwi cr6, r7, 0xd7
	ctx.cr[6].compare_i32(ctx.r[7].s32, 215, &mut ctx.xer);
	// 823B6514: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B6518: 41980008  blt cr6, 0x823b6520
	if ctx.cr[6].lt {
	pc = 0x823B6520; continue 'dispatch;
	}
	// 823B651C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B6520: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B6524: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B6528: 419A0014  beq cr6, 0x823b653c
	if ctx.cr[6].eq {
	pc = 0x823B653C; continue 'dispatch;
	}
	// 823B652C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B6530: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B6534: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B6538: 4800000C  b 0x823b6544
	pc = 0x823B6544; continue 'dispatch;
	// 823B653C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B6540: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B6544: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6548: 4199FFB8  bgt cr6, 0x823b6500
	if ctx.cr[6].gt {
	pc = 0x823B6500; continue 'dispatch;
	}
	// 823B654C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6550: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B6554: 419A0040  beq cr6, 0x823b6594
	if ctx.cr[6].eq {
	pc = 0x823B6594; continue 'dispatch;
	}
	// 823B6558: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B655C: 2F0B00D7  cmpwi cr6, r11, 0xd7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 215, &mut ctx.xer);
	// 823B6560: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6564: 41990008  bgt cr6, 0x823b656c
	if ctx.cr[6].gt {
	pc = 0x823B656C; continue 'dispatch;
	}
	// 823B6568: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B656C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B6570: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6574: 409A0020  bne cr6, 0x823b6594
	if !ctx.cr[6].eq {
	pc = 0x823B6594; continue 'dispatch;
	}
	// 823B6578: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B657C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B6580: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6584: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6588: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B658C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6590: 4800001C  b 0x823b65ac
	pc = 0x823B65AC; continue 'dispatch;
	// 823B6594: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6598: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B659C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B65A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B65A4: 48000008  b 0x823b65ac
	pc = 0x823B65AC; continue 'dispatch;
	// 823B65A8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B65AC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B65B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B65B4: 419A0008  beq cr6, 0x823b65bc
	if ctx.cr[6].eq {
	pc = 0x823B65BC; continue 'dispatch;
	}
	// 823B65B8: 484392D9  bl 0x827ef890
	ctx.lr = 0x823B65BC;
	sub_827EF890(ctx, base);
	// 823B65BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B65C0: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823B65C4: 488F2E98  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B65C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B65C8 size=460
    let mut pc: u32 = 0x823B65C8;
    'dispatch: loop {
        match pc {
            0x823B65C8 => {
    //   block [0x823B65C8..0x823B6794)
	// 823B65C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B65CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B65D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B65D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B65D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B65DC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B65E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B65E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B65E8: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B65EC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B65F0: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B65F4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B65F8: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B65FC: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B6600: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B6604: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6608: 419A0080  beq cr6, 0x823b6688
	if ctx.cr[6].eq {
	pc = 0x823B6688; continue 'dispatch;
	}
	// 823B660C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6610: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6614: 419A0070  beq cr6, 0x823b6684
	if ctx.cr[6].eq {
	pc = 0x823B6684; continue 'dispatch;
	}
	// 823B6618: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B661C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6620: 419A0018  beq cr6, 0x823b6638
	if ctx.cr[6].eq {
	pc = 0x823B6638; continue 'dispatch;
	}
	// 823B6624: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B6628: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B662C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6630: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6634: 409A0008  bne cr6, 0x823b663c
	if !ctx.cr[6].eq {
	pc = 0x823B663C; continue 'dispatch;
	}
	// 823B6638: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 823B663C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B6640: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6644: 419A0138  beq cr6, 0x823b677c
	if ctx.cr[6].eq {
	pc = 0x823B677C; continue 'dispatch;
	}
	// 823B6648: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B664C: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B6650: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6654: 419A0100  beq cr6, 0x823b6754
	if ctx.cr[6].eq {
	pc = 0x823B6754; continue 'dispatch;
	}
	// 823B6658: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B665C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6660: 419A0030  beq cr6, 0x823b6690
	if ctx.cr[6].eq {
	pc = 0x823B6690; continue 'dispatch;
	}
	// 823B6664: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B6668: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B666C: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B6670: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B6674: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6678: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B667C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6680: 480000D8  b 0x823b6758
	pc = 0x823B6758; continue 'dispatch;
	// 823B6684: 4BDDD7B5  bl 0x82193e38
	ctx.lr = 0x823B6688;
	sub_82193E38(ctx, base);
	// 823B6688: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B668C: 4BFFFFAC  b 0x823b6638
	pc = 0x823B6638; continue 'dispatch;
	// 823B6690: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6694: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B6698: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B669C: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B66A0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B66A4: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B66A8: 40810054  ble 0x823b66fc
	if !ctx.cr[0].gt {
	pc = 0x823B66FC; continue 'dispatch;
	}
	// 823B66AC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B66B0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B66B4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B66B8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B66BC: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B66C0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B66C4: 41980008  blt cr6, 0x823b66cc
	if ctx.cr[6].lt {
	pc = 0x823B66CC; continue 'dispatch;
	}
	// 823B66C8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B66CC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B66D0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B66D4: 419A0014  beq cr6, 0x823b66e8
	if ctx.cr[6].eq {
	pc = 0x823B66E8; continue 'dispatch;
	}
	// 823B66D8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B66DC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B66E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B66E4: 4800000C  b 0x823b66f0
	pc = 0x823B66F0; continue 'dispatch;
	// 823B66E8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B66EC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B66F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B66F4: 4199FFB8  bgt cr6, 0x823b66ac
	if ctx.cr[6].gt {
	pc = 0x823B66AC; continue 'dispatch;
	}
	// 823B66F8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B66FC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B6700: 419A0040  beq cr6, 0x823b6740
	if ctx.cr[6].eq {
	pc = 0x823B6740; continue 'dispatch;
	}
	// 823B6704: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6708: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B670C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6710: 41990008  bgt cr6, 0x823b6718
	if ctx.cr[6].gt {
	pc = 0x823B6718; continue 'dispatch;
	}
	// 823B6714: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B6718: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B671C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6720: 409A0020  bne cr6, 0x823b6740
	if !ctx.cr[6].eq {
	pc = 0x823B6740; continue 'dispatch;
	}
	// 823B6724: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B6728: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B672C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6730: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6734: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B6738: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B673C: 4800001C  b 0x823b6758
	pc = 0x823B6758; continue 'dispatch;
	// 823B6740: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6744: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6748: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B674C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6750: 48000008  b 0x823b6758
	pc = 0x823B6758; continue 'dispatch;
	// 823B6754: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B6758: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B675C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6760: 419A001C  beq cr6, 0x823b677c
	if ctx.cr[6].eq {
	pc = 0x823B677C; continue 'dispatch;
	}
	// 823B6764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6768: 4BE115C9  bl 0x821c7d30
	ctx.lr = 0x823B676C;
	sub_821C7D30(ctx, base);
	// 823B676C: 7C83F050  subf r4, r3, r30
	ctx.r[4].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 823B6770: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B6774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6778: 4811FB29  bl 0x824d62a0
	ctx.lr = 0x823B677C;
	sub_824D62A0(ctx, base);
	// 823B677C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6788: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B678C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B6798 size=844
    let mut pc: u32 = 0x823B6798;
    'dispatch: loop {
        match pc {
            0x823B6798 => {
    //   block [0x823B6798..0x823B6AE4)
	// 823B6798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B679C: 488F2C71  bl 0x82ca940c
	ctx.lr = 0x823B67A0;
	sub_82CA93D0(ctx, base);
	// 823B67A0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823B67A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B67A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B67AC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B67B0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823B67B4: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B67B8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B67BC: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B67C0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B67C4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B67C8: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B67CC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B67D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B67D4: 419A0084  beq cr6, 0x823b6858
	if ctx.cr[6].eq {
	pc = 0x823B6858; continue 'dispatch;
	}
	// 823B67D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B67DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B67E0: 419A0074  beq cr6, 0x823b6854
	if ctx.cr[6].eq {
	pc = 0x823B6854; continue 'dispatch;
	}
	// 823B67E4: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 823B67E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B67EC: 419A0018  beq cr6, 0x823b6804
	if ctx.cr[6].eq {
	pc = 0x823B6804; continue 'dispatch;
	}
	// 823B67F0: 897F0090  lbz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B67F4: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B67F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B67FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6800: 409A0008  bne cr6, 0x823b6808
	if !ctx.cr[6].eq {
	pc = 0x823B6808; continue 'dispatch;
	}
	// 823B6804: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B6808: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B680C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6810: 419A02C8  beq cr6, 0x823b6ad8
	if ctx.cr[6].eq {
	pc = 0x823B6AD8; continue 'dispatch;
	}
	// 823B6814: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B6818: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 823B681C: 556A3FFE  rlwinm r10, r11, 7, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 823B6820: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6824: 419A0100  beq cr6, 0x823b6924
	if ctx.cr[6].eq {
	pc = 0x823B6924; continue 'dispatch;
	}
	// 823B6828: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B682C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6830: 419A0030  beq cr6, 0x823b6860
	if ctx.cr[6].eq {
	pc = 0x823B6860; continue 'dispatch;
	}
	// 823B6834: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B6838: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B683C: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B6840: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B6844: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6848: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B684C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6850: 480000D8  b 0x823b6928
	pc = 0x823B6928; continue 'dispatch;
	// 823B6854: 4BDDD5E5  bl 0x82193e38
	ctx.lr = 0x823B6858;
	sub_82193E38(ctx, base);
	// 823B6858: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 823B685C: 4BFFFFA8  b 0x823b6804
	pc = 0x823B6804; continue 'dispatch;
	// 823B6860: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6864: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B6868: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B686C: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B6870: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6874: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6878: 40810054  ble 0x823b68cc
	if !ctx.cr[0].gt {
	pc = 0x823B68CC; continue 'dispatch;
	}
	// 823B687C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B6880: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B6884: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B6888: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B688C: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B6890: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B6894: 41980008  blt cr6, 0x823b689c
	if ctx.cr[6].lt {
	pc = 0x823B689C; continue 'dispatch;
	}
	// 823B6898: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B689C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B68A0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B68A4: 419A0014  beq cr6, 0x823b68b8
	if ctx.cr[6].eq {
	pc = 0x823B68B8; continue 'dispatch;
	}
	// 823B68A8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B68AC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B68B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B68B4: 4800000C  b 0x823b68c0
	pc = 0x823B68C0; continue 'dispatch;
	// 823B68B8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B68BC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B68C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B68C4: 4199FFB8  bgt cr6, 0x823b687c
	if ctx.cr[6].gt {
	pc = 0x823B687C; continue 'dispatch;
	}
	// 823B68C8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B68CC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B68D0: 419A0040  beq cr6, 0x823b6910
	if ctx.cr[6].eq {
	pc = 0x823B6910; continue 'dispatch;
	}
	// 823B68D4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B68D8: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B68DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B68E0: 41990008  bgt cr6, 0x823b68e8
	if ctx.cr[6].gt {
	pc = 0x823B68E8; continue 'dispatch;
	}
	// 823B68E4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B68E8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B68EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B68F0: 409A0020  bne cr6, 0x823b6910
	if !ctx.cr[6].eq {
	pc = 0x823B6910; continue 'dispatch;
	}
	// 823B68F4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B68F8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B68FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6900: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6904: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B6908: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B690C: 4800001C  b 0x823b6928
	pc = 0x823B6928; continue 'dispatch;
	// 823B6910: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6914: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6918: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B691C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6920: 48000008  b 0x823b6928
	pc = 0x823B6928; continue 'dispatch;
	// 823B6924: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B6928: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B692C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6930: 419A01A8  beq cr6, 0x823b6ad8
	if ctx.cr[6].eq {
	pc = 0x823B6AD8; continue 'dispatch;
	}
	// 823B6934: C1BE0028  lfs f13, 0x28(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B6938: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B693C: ED9F6828  fsubs f12, f31, f13
	ctx.f[12].f64 = (((ctx.f[31].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B6940: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B6944: C17E002C  lfs f11, 0x2c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B6948: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B694C: 392A0E68  addi r9, r10, 0xe68
	ctx.r[9].s64 = ctx.r[10].s64 + 3688;
	// 823B6950: C00B9484  lfs f0, -0x6b7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B6954: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 823B6958: 7D000026  mfcr r8
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[8].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[8].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B695C: 5507DF7A  rlwinm r7, r8, 0x1b, 0x1d, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 823B6960: 5506F77A  rlwinm r6, r8, 0x1e, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 823B6964: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 823B6968: 7D492C2E  lfsx f10, r9, r5
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32)) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 823B696C: FD2A6FEE  fsel f9, f10, f31, f13
	ctx.f[9].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[13].f64 };
	// 823B6970: ED095828  fsubs f8, f9, f11
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[11].f64) as f32) as f64);
	// 823B6974: FF080000  fcmpu cr6, f8, f0
	ctx.cr[6].compare_f64(ctx.f[8].f64, ctx.f[0].f64);
	// 823B6978: 7C800026  mfcr r4
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[4].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[4].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B697C: 548BDF7A  rlwinm r11, r4, 0x1b, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 823B6980: 548AF77A  rlwinm r10, r4, 0x1e, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000003u64;
	// 823B6984: 7D685378  or r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 823B6988: 7CE9442E  lfsx f7, r9, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 823B698C: FFE74AEE  fsel f31, f7, f11, f9
	ctx.f[31].f64 = if ctx.f[7].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[9].f64 };
	// 823B6990: 4BE86F81  bl 0x8223d910
	ctx.lr = 0x823B6994;
	sub_8223D910(ctx, base);
	// 823B6994: ECDF0828  fsubs f6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[6].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 823B6998: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B699C: FCA0301E  fctiwz f5, f6
	ctx.f[5].s64 = if ctx.f[6].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[6].f64.trunc() as i32 as i64 };
	// 823B69A0: D8A10050  stfd f5, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[5].u64 ) };
	// 823B69A4: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B69A8: 7CE607B4  extsw r6, r7
	ctx.r[6].s64 = ctx.r[7].s32 as i64;
	// 823B69AC: F8C10050  std r6, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u64 ) };
	// 823B69B0: C8810050  lfd f4, 0x50(r1)
	ctx.f[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B69B4: FC60269C  fcfid f3, f4
	ctx.f[3].f64 = (ctx.f[4].s64 as f64);
	// 823B69B8: FC201818  frsp f1, f3
	ctx.f[1].f64 = (ctx.f[3].f64 as f32) as f64;
	// 823B69BC: 48120225  bl 0x824d6be0
	ctx.lr = 0x823B69C0;
	sub_824D6BE0(ctx, base);
	// 823B69C0: 80BF0024  lwz r5, 0x24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B69C4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B69C8: 54A45FFE  rlwinm r4, r5, 0xb, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x001FFFFFu64;
	// 823B69CC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 823B69D0: 419A00E8  beq cr6, 0x823b6ab8
	if ctx.cr[6].eq {
	pc = 0x823B6AB8; continue 'dispatch;
	}
	// 823B69D4: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B69D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B69DC: 419A0020  beq cr6, 0x823b69fc
	if ctx.cr[6].eq {
	pc = 0x823B69FC; continue 'dispatch;
	}
	// 823B69E0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 823B69E4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B69E8: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B69EC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B69F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B69F4: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B69F8: 480000C4  b 0x823b6abc
	pc = 0x823B6ABC; continue 'dispatch;
	// 823B69FC: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6A00: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B6A04: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B6A08: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B6A0C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6A10: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6A14: 40810054  ble 0x823b6a68
	if !ctx.cr[0].gt {
	pc = 0x823B6A68; continue 'dispatch;
	}
	// 823B6A18: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B6A1C: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B6A20: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B6A24: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6A28: 2F070015  cmpwi cr6, r7, 0x15
	ctx.cr[6].compare_i32(ctx.r[7].s32, 21, &mut ctx.xer);
	// 823B6A2C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B6A30: 41980008  blt cr6, 0x823b6a38
	if ctx.cr[6].lt {
	pc = 0x823B6A38; continue 'dispatch;
	}
	// 823B6A34: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B6A38: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B6A3C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B6A40: 419A0014  beq cr6, 0x823b6a54
	if ctx.cr[6].eq {
	pc = 0x823B6A54; continue 'dispatch;
	}
	// 823B6A44: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B6A48: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B6A4C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B6A50: 4800000C  b 0x823b6a5c
	pc = 0x823B6A5C; continue 'dispatch;
	// 823B6A54: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B6A58: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B6A5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6A60: 4199FFB8  bgt cr6, 0x823b6a18
	if ctx.cr[6].gt {
	pc = 0x823B6A18; continue 'dispatch;
	}
	// 823B6A64: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6A68: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B6A6C: 419A003C  beq cr6, 0x823b6aa8
	if ctx.cr[6].eq {
	pc = 0x823B6AA8; continue 'dispatch;
	}
	// 823B6A70: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6A74: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 823B6A78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6A7C: 41990008  bgt cr6, 0x823b6a84
	if ctx.cr[6].gt {
	pc = 0x823B6A84; continue 'dispatch;
	}
	// 823B6A80: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B6A84: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B6A88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6A8C: 409A001C  bne cr6, 0x823b6aa8
	if !ctx.cr[6].eq {
	pc = 0x823B6AA8; continue 'dispatch;
	}
	// 823B6A90: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B6A94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6A98: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B6A9C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6AA0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6AA4: 48000018  b 0x823b6abc
	pc = 0x823B6ABC; continue 'dispatch;
	// 823B6AA8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6AAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6AB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6AB4: 48000008  b 0x823b6abc
	pc = 0x823B6ABC; continue 'dispatch;
	// 823B6AB8: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 823B6ABC: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B6AC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6AC4: 419A000C  beq cr6, 0x823b6ad0
	if ctx.cr[6].eq {
	pc = 0x823B6AD0; continue 'dispatch;
	}
	// 823B6AC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B6ACC: 4806A2C5  bl 0x82420d90
	ctx.lr = 0x823B6AD0;
	sub_82420D90(ctx, base);
	// 823B6AD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6AD4: 4812504D  bl 0x824dbb20
	ctx.lr = 0x823B6AD8;
	sub_824DBB20(ctx, base);
	// 823B6AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B6ADC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823B6AE0: 488F297C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B6AE8 size=596
    let mut pc: u32 = 0x823B6AE8;
    'dispatch: loop {
        match pc {
            0x823B6AE8 => {
    //   block [0x823B6AE8..0x823B6D3C)
	// 823B6AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6AF0: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 823B6AF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6AF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B6AFC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B6B00: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B6B04: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B6B08: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B6B0C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6B10: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6B14: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B6B18: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B6B1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6B20: 419A0080  beq cr6, 0x823b6ba0
	if ctx.cr[6].eq {
	pc = 0x823B6BA0; continue 'dispatch;
	}
	// 823B6B24: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6B28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6B2C: 419A0070  beq cr6, 0x823b6b9c
	if ctx.cr[6].eq {
	pc = 0x823B6B9C; continue 'dispatch;
	}
	// 823B6B30: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B6B34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6B38: 419A0018  beq cr6, 0x823b6b50
	if ctx.cr[6].eq {
	pc = 0x823B6B50; continue 'dispatch;
	}
	// 823B6B3C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B6B40: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B6B44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6B48: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6B4C: 409A0008  bne cr6, 0x823b6b54
	if !ctx.cr[6].eq {
	pc = 0x823B6B54; continue 'dispatch;
	}
	// 823B6B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B6B54: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B6B58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6B5C: 419A01CC  beq cr6, 0x823b6d28
	if ctx.cr[6].eq {
	pc = 0x823B6D28; continue 'dispatch;
	}
	// 823B6B60: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B6B64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B6B68: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B6B6C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6B70: 419A00F8  beq cr6, 0x823b6c68
	if ctx.cr[6].eq {
	pc = 0x823B6C68; continue 'dispatch;
	}
	// 823B6B74: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B6B78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6B7C: 419A002C  beq cr6, 0x823b6ba8
	if ctx.cr[6].eq {
	pc = 0x823B6BA8; continue 'dispatch;
	}
	// 823B6B80: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B6B84: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6B88: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B6B8C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B6B90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6B94: 80690004  lwz r3, 4(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6B98: 480000D4  b 0x823b6c6c
	pc = 0x823B6C6C; continue 'dispatch;
	// 823B6B9C: 4BDDD29D  bl 0x82193e38
	ctx.lr = 0x823B6BA0;
	sub_82193E38(ctx, base);
	// 823B6BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6BA4: 4BFFFFAC  b 0x823b6b50
	pc = 0x823B6B50; continue 'dispatch;
	// 823B6BA8: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B6BB0: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B6BB4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B6BB8: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B6BBC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6BC0: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6BC4: 40810054  ble 0x823b6c18
	if !ctx.cr[0].gt {
	pc = 0x823B6C18; continue 'dispatch;
	}
	// 823B6BC8: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B6BCC: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B6BD0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B6BD4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6BD8: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B6BDC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B6BE0: 41980008  blt cr6, 0x823b6be8
	if ctx.cr[6].lt {
	pc = 0x823B6BE8; continue 'dispatch;
	}
	// 823B6BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B6BE8: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B6BEC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B6BF0: 419A0014  beq cr6, 0x823b6c04
	if ctx.cr[6].eq {
	pc = 0x823B6C04; continue 'dispatch;
	}
	// 823B6BF4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B6BF8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B6BFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B6C00: 4800000C  b 0x823b6c0c
	pc = 0x823B6C0C; continue 'dispatch;
	// 823B6C04: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B6C08: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B6C0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6C10: 4199FFB8  bgt cr6, 0x823b6bc8
	if ctx.cr[6].gt {
	pc = 0x823B6BC8; continue 'dispatch;
	}
	// 823B6C14: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6C18: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B6C1C: 419A003C  beq cr6, 0x823b6c58
	if ctx.cr[6].eq {
	pc = 0x823B6C58; continue 'dispatch;
	}
	// 823B6C20: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6C24: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B6C28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6C2C: 41990008  bgt cr6, 0x823b6c34
	if ctx.cr[6].gt {
	pc = 0x823B6C34; continue 'dispatch;
	}
	// 823B6C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6C34: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B6C38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6C3C: 409A001C  bne cr6, 0x823b6c58
	if !ctx.cr[6].eq {
	pc = 0x823B6C58; continue 'dispatch;
	}
	// 823B6C40: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B6C44: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B6C48: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6C4C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6C50: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6C54: 48000018  b 0x823b6c6c
	pc = 0x823B6C6C; continue 'dispatch;
	// 823B6C58: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6C5C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6C60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6C64: 48000008  b 0x823b6c6c
	pc = 0x823B6C6C; continue 'dispatch;
	// 823B6C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6C6C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B6C70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6C74: 419A00B4  beq cr6, 0x823b6d28
	if ctx.cr[6].eq {
	pc = 0x823B6D28; continue 'dispatch;
	}
	// 823B6C78: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B6C7C: C1830034  lfs f12, 0x34(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823B6C80: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B6C84: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B6C88: 390A0E68  addi r8, r10, 0xe68
	ctx.r[8].s64 = ctx.r[10].s64 + 3688;
	// 823B6C8C: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B6C90: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 823B6C94: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B6C98: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 823B6C9C: 7CE00026  mfcr r7
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6CA0: 54E6DF7A  rlwinm r6, r7, 0x1b, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 823B6CA4: 54E5F77A  rlwinm r5, r7, 0x1e, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 823B6CA8: 7CC42B78  or r4, r6, r5
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 823B6CAC: 7D68242E  lfsx f11, r8, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B6CB0: FD4B07EE  fsel f10, f11, f31, f0
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[0].f64 };
	// 823B6CB4: ED2A6828  fsubs f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B6CB8: FF090000  fcmpu cr6, f9, f0
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[0].f64);
	// 823B6CBC: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6CC0: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823B6CC4: 5569F77A  rlwinm r9, r11, 0x1e, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823B6CC8: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823B6CCC: 7D083C2E  lfsx f8, r8, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 823B6CD0: FCE8536E  fsel f7, f8, f13, f10
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[10].f64 };
	// 823B6CD4: ECC76028  fsubs f6, f7, f12
	ctx.f[6].f64 = (((ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 823B6CD8: ECA6602A  fadds f5, f6, f12
	ctx.f[5].f64 = ((ctx.f[6].f64 + ctx.f[12].f64) as f32) as f64;
	// 823B6CDC: D0A30034  stfs f5, 0x34(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 823B6CE0: FC802890  fmr f4, f5
	ctx.f[4].f64 = ctx.f[5].f64;
	// 823B6CE4: FF040000  fcmpu cr6, f4, f0
	ctx.cr[6].compare_f64(ctx.f[4].f64, ctx.f[0].f64);
	// 823B6CE8: 7CC00026  mfcr r6
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[6].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[6].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6CEC: 54C5DF7A  rlwinm r5, r6, 0x1b, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 823B6CF0: 54C4F77A  rlwinm r4, r6, 0x1e, 0x1d, 0x1d
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 823B6CF4: 7CAB2378  or r11, r5, r4
	ctx.r[11].u64 = ctx.r[5].u64 | ctx.r[4].u64;
	// 823B6CF8: 7C685C2E  lfsx f3, r8, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 823B6CFC: FC43012E  fsel f2, f3, f4, f0
	ctx.f[2].f64 = if ctx.f[3].f64 >= 0.0 { ctx.f[4].f64 } else { ctx.f[0].f64 };
	// 823B6D00: EC226828  fsubs f1, f2, f13
	ctx.f[1].f64 = (((ctx.f[2].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B6D04: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B6D08: 7D400026  mfcr r10
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[10].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[10].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6D0C: 5549DF7A  rlwinm r9, r10, 0x1b, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 823B6D10: 5547F77A  rlwinm r7, r10, 0x1e, 0x1d, 0x1d
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 823B6D14: 7D263B78  or r6, r9, r7
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 823B6D18: 7C08342E  lfsx f0, r8, r6
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B6D1C: FDA0136E  fsel f13, f0, f13, f2
	ctx.f[13].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[2].f64 };
	// 823B6D20: D1A30034  stfs f13, 0x34(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 823B6D24: 48124DFD  bl 0x824dbb20
	ctx.lr = 0x823B6D28;
	sub_824DBB20(ctx, base);
	// 823B6D28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6D34: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B6D40 size=544
    let mut pc: u32 = 0x823B6D40;
    'dispatch: loop {
        match pc {
            0x823B6D40 => {
    //   block [0x823B6D40..0x823B6F60)
	// 823B6D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6D48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6D4C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 823B6D50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6D54: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B6D58: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B6D5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B6D60: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B6D64: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B6D68: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B6D6C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6D70: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6D74: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B6D78: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B6D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6D80: 419A0080  beq cr6, 0x823b6e00
	if ctx.cr[6].eq {
	pc = 0x823B6E00; continue 'dispatch;
	}
	// 823B6D84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6D88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6D8C: 419A0070  beq cr6, 0x823b6dfc
	if ctx.cr[6].eq {
	pc = 0x823B6DFC; continue 'dispatch;
	}
	// 823B6D90: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B6D94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6D98: 419A0018  beq cr6, 0x823b6db0
	if ctx.cr[6].eq {
	pc = 0x823B6DB0; continue 'dispatch;
	}
	// 823B6D9C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B6DA0: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B6DA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6DA8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6DAC: 409A0008  bne cr6, 0x823b6db4
	if !ctx.cr[6].eq {
	pc = 0x823B6DB4; continue 'dispatch;
	}
	// 823B6DB0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 823B6DB4: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B6DB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6DBC: 419A018C  beq cr6, 0x823b6f48
	if ctx.cr[6].eq {
	pc = 0x823B6F48; continue 'dispatch;
	}
	// 823B6DC0: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B6DC4: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B6DC8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6DCC: 419A0100  beq cr6, 0x823b6ecc
	if ctx.cr[6].eq {
	pc = 0x823B6ECC; continue 'dispatch;
	}
	// 823B6DD0: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B6DD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6DD8: 419A0030  beq cr6, 0x823b6e08
	if ctx.cr[6].eq {
	pc = 0x823B6E08; continue 'dispatch;
	}
	// 823B6DDC: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B6DE0: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6DE4: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B6DE8: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B6DEC: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6DF0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B6DF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6DF8: 480000D8  b 0x823b6ed0
	pc = 0x823B6ED0; continue 'dispatch;
	// 823B6DFC: 4BDDD03D  bl 0x82193e38
	ctx.lr = 0x823B6E00;
	sub_82193E38(ctx, base);
	// 823B6E00: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B6E04: 4BFFFFAC  b 0x823b6db0
	pc = 0x823B6DB0; continue 'dispatch;
	// 823B6E08: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B6E0C: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B6E10: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B6E14: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B6E18: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6E1C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6E20: 40810054  ble 0x823b6e74
	if !ctx.cr[0].gt {
	pc = 0x823B6E74; continue 'dispatch;
	}
	// 823B6E24: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B6E28: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B6E2C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B6E30: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6E34: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B6E38: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B6E3C: 41980008  blt cr6, 0x823b6e44
	if ctx.cr[6].lt {
	pc = 0x823B6E44; continue 'dispatch;
	}
	// 823B6E40: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B6E44: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B6E48: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B6E4C: 419A0014  beq cr6, 0x823b6e60
	if ctx.cr[6].eq {
	pc = 0x823B6E60; continue 'dispatch;
	}
	// 823B6E50: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B6E54: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B6E58: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B6E5C: 4800000C  b 0x823b6e68
	pc = 0x823B6E68; continue 'dispatch;
	// 823B6E60: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B6E64: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B6E68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B6E6C: 4199FFB8  bgt cr6, 0x823b6e24
	if ctx.cr[6].gt {
	pc = 0x823B6E24; continue 'dispatch;
	}
	// 823B6E70: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B6E74: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B6E78: 419A0040  beq cr6, 0x823b6eb8
	if ctx.cr[6].eq {
	pc = 0x823B6EB8; continue 'dispatch;
	}
	// 823B6E7C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6E80: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B6E84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6E88: 41990008  bgt cr6, 0x823b6e90
	if ctx.cr[6].gt {
	pc = 0x823B6E90; continue 'dispatch;
	}
	// 823B6E8C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B6E90: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B6E94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6E98: 409A0020  bne cr6, 0x823b6eb8
	if !ctx.cr[6].eq {
	pc = 0x823B6EB8; continue 'dispatch;
	}
	// 823B6E9C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B6EA0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B6EA4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B6EA8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6EAC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B6EB0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6EB4: 4800001C  b 0x823b6ed0
	pc = 0x823B6ED0; continue 'dispatch;
	// 823B6EB8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B6EBC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6EC0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B6EC4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B6EC8: 48000008  b 0x823b6ed0
	pc = 0x823B6ED0; continue 'dispatch;
	// 823B6ECC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B6ED0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B6ED4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6ED8: 419A0070  beq cr6, 0x823b6f48
	if ctx.cr[6].eq {
	pc = 0x823B6F48; continue 'dispatch;
	}
	// 823B6EDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B6EE0: C19F0038  lfs f12, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823B6EE4: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B6EE8: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B6EEC: 390A0E68  addi r8, r10, 0xe68
	ctx.r[8].s64 = ctx.r[10].s64 + 3688;
	// 823B6EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6EF4: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B6EF8: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B6EFC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 823B6F00: 7CE00026  mfcr r7
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6F04: 54E6DF7A  rlwinm r6, r7, 0x1b, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 823B6F08: 54E5F77A  rlwinm r5, r7, 0x1e, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 823B6F0C: 7CC42B78  or r4, r6, r5
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 823B6F10: 7D68242E  lfsx f11, r8, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B6F14: FD4B07EE  fsel f10, f11, f31, f0
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[0].f64 };
	// 823B6F18: ED2A6828  fsubs f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B6F1C: FF090000  fcmpu cr6, f9, f0
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[0].f64);
	// 823B6F20: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B6F24: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823B6F28: 5569F77A  rlwinm r9, r11, 0x1e, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823B6F2C: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823B6F30: 7D083C2E  lfsx f8, r8, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 823B6F34: FCE8536E  fsel f7, f8, f13, f10
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[10].f64 };
	// 823B6F38: EC276028  fsubs f1, f7, f12
	ctx.f[1].f64 = (((ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 823B6F3C: 4812092D  bl 0x824d7868
	ctx.lr = 0x823B6F40;
	sub_824D7868(ctx, base);
	// 823B6F40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6F44: 48124BDD  bl 0x824dbb20
	ctx.lr = 0x823B6F48;
	sub_824DBB20(ctx, base);
	// 823B6F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6F54: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B6F60 size=528
    let mut pc: u32 = 0x823B6F60;
    'dispatch: loop {
        match pc {
            0x823B6F60 => {
    //   block [0x823B6F60..0x823B7170)
	// 823B6F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6F68: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 823B6F6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6F70: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B6F74: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B6F78: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B6F7C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B6F80: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B6F84: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6F88: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6F8C: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B6F90: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B6F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6F98: 419A0084  beq cr6, 0x823b701c
	if ctx.cr[6].eq {
	pc = 0x823B701C; continue 'dispatch;
	}
	// 823B6F9C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6FA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6FA4: 419A0074  beq cr6, 0x823b7018
	if ctx.cr[6].eq {
	pc = 0x823B7018; continue 'dispatch;
	}
	// 823B6FA8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B6FAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6FB0: 419A0018  beq cr6, 0x823b6fc8
	if ctx.cr[6].eq {
	pc = 0x823B6FC8; continue 'dispatch;
	}
	// 823B6FB4: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B6FB8: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B6FBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6FC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6FC4: 409A0008  bne cr6, 0x823b6fcc
	if !ctx.cr[6].eq {
	pc = 0x823B6FCC; continue 'dispatch;
	}
	// 823B6FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B6FCC: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B6FD0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6FD4: 419A0188  beq cr6, 0x823b715c
	if ctx.cr[6].eq {
	pc = 0x823B715C; continue 'dispatch;
	}
	// 823B6FD8: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B6FDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B6FE0: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B6FE4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B6FE8: 419A0104  beq cr6, 0x823b70ec
	if ctx.cr[6].eq {
	pc = 0x823B70EC; continue 'dispatch;
	}
	// 823B6FEC: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B6FF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B6FF4: 419A0030  beq cr6, 0x823b7024
	if ctx.cr[6].eq {
	pc = 0x823B7024; continue 'dispatch;
	}
	// 823B6FF8: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B6FFC: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7000: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7004: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7008: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B700C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B7010: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7014: 480000DC  b 0x823b70f0
	pc = 0x823B70F0; continue 'dispatch;
	// 823B7018: 4BDDCE21  bl 0x82193e38
	ctx.lr = 0x823B701C;
	sub_82193E38(ctx, base);
	// 823B701C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7020: 4BFFFFA8  b 0x823b6fc8
	pc = 0x823B6FC8; continue 'dispatch;
	// 823B7024: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7028: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B702C: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7030: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B7034: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B7038: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B703C: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7040: 40810054  ble 0x823b7094
	if !ctx.cr[0].gt {
	pc = 0x823B7094; continue 'dispatch;
	}
	// 823B7044: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B7048: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B704C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7050: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7054: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B7058: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B705C: 41980008  blt cr6, 0x823b7064
	if ctx.cr[6].lt {
	pc = 0x823B7064; continue 'dispatch;
	}
	// 823B7060: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B7064: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B7068: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B706C: 419A0014  beq cr6, 0x823b7080
	if ctx.cr[6].eq {
	pc = 0x823B7080; continue 'dispatch;
	}
	// 823B7070: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B7074: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B7078: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B707C: 4800000C  b 0x823b7088
	pc = 0x823B7088; continue 'dispatch;
	// 823B7080: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B7084: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B7088: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B708C: 4199FFB8  bgt cr6, 0x823b7044
	if ctx.cr[6].gt {
	pc = 0x823B7044; continue 'dispatch;
	}
	// 823B7090: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7094: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B7098: 419A0040  beq cr6, 0x823b70d8
	if ctx.cr[6].eq {
	pc = 0x823B70D8; continue 'dispatch;
	}
	// 823B709C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B70A0: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B70A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B70A8: 41990008  bgt cr6, 0x823b70b0
	if ctx.cr[6].gt {
	pc = 0x823B70B0; continue 'dispatch;
	}
	// 823B70AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B70B0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B70B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B70B8: 409A0020  bne cr6, 0x823b70d8
	if !ctx.cr[6].eq {
	pc = 0x823B70D8; continue 'dispatch;
	}
	// 823B70BC: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B70C0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B70C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B70C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B70CC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B70D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B70D4: 4800001C  b 0x823b70f0
	pc = 0x823B70F0; continue 'dispatch;
	// 823B70D8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B70DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B70E0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B70E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B70E8: 48000008  b 0x823b70f0
	pc = 0x823B70F0; continue 'dispatch;
	// 823B70EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B70F0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B70F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B70F8: 419A0064  beq cr6, 0x823b715c
	if ctx.cr[6].eq {
	pc = 0x823B715C; continue 'dispatch;
	}
	// 823B70FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B7100: C183003C  lfs f12, 0x3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823B7104: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B7108: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B710C: 390A0E68  addi r8, r10, 0xe68
	ctx.r[8].s64 = ctx.r[10].s64 + 3688;
	// 823B7110: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B7114: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B7118: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 823B711C: 7CE00026  mfcr r7
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B7120: 54E6DF7A  rlwinm r6, r7, 0x1b, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 823B7124: 54E5F77A  rlwinm r5, r7, 0x1e, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 823B7128: 7CC42B78  or r4, r6, r5
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 823B712C: 7D68242E  lfsx f11, r8, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B7130: FD4B07EE  fsel f10, f11, f31, f0
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[0].f64 };
	// 823B7134: ED2A6828  fsubs f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B7138: FF090000  fcmpu cr6, f9, f0
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[0].f64);
	// 823B713C: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B7140: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823B7144: 5569F77A  rlwinm r9, r11, 0x1e, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823B7148: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823B714C: 7D083C2E  lfsx f8, r8, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 823B7150: FCE8536E  fsel f7, f8, f13, f10
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[10].f64 };
	// 823B7154: EC276028  fsubs f1, f7, f12
	ctx.f[1].f64 = (((ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 823B7158: 48120911  bl 0x824d7a68
	ctx.lr = 0x823B715C;
	sub_824D7A68(ctx, base);
	// 823B715C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7168: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B716C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7170 size=716
    let mut pc: u32 = 0x823B7170;
    'dispatch: loop {
        match pc {
            0x823B7170 => {
    //   block [0x823B7170..0x823B743C)
	// 823B7170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B717C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7180: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823B7184: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7188: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B718C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B7190: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823B7194: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B7198: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B719C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B71A0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B71A4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B71A8: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B71AC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B71B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B71B4: 419A0080  beq cr6, 0x823b7234
	if ctx.cr[6].eq {
	pc = 0x823B7234; continue 'dispatch;
	}
	// 823B71B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B71BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B71C0: 419A0070  beq cr6, 0x823b7230
	if ctx.cr[6].eq {
	pc = 0x823B7230; continue 'dispatch;
	}
	// 823B71C4: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 823B71C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B71CC: 419A0018  beq cr6, 0x823b71e4
	if ctx.cr[6].eq {
	pc = 0x823B71E4; continue 'dispatch;
	}
	// 823B71D0: 897F0090  lbz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B71D4: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B71D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B71DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B71E0: 409A0008  bne cr6, 0x823b71e8
	if !ctx.cr[6].eq {
	pc = 0x823B71E8; continue 'dispatch;
	}
	// 823B71E4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B71E8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B71EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B71F0: 419A0230  beq cr6, 0x823b7420
	if ctx.cr[6].eq {
	pc = 0x823B7420; continue 'dispatch;
	}
	// 823B71F4: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 823B71F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B71FC: 554937FE  rlwinm r9, r10, 6, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 823B7200: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B7204: 419A00F4  beq cr6, 0x823b72f8
	if ctx.cr[6].eq {
	pc = 0x823B72F8; continue 'dispatch;
	}
	// 823B7208: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B720C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7210: 419A002C  beq cr6, 0x823b723c
	if ctx.cr[6].eq {
	pc = 0x823B723C; continue 'dispatch;
	}
	// 823B7214: 894B007A  lbz r10, 0x7a(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(122 as u32) ) } as u64;
	// 823B7218: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B721C: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7220: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7224: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7228: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B722C: 480000D0  b 0x823b72fc
	pc = 0x823B72FC; continue 'dispatch;
	// 823B7230: 4BDDCC09  bl 0x82193e38
	ctx.lr = 0x823B7234;
	sub_82193E38(ctx, base);
	// 823B7234: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 823B7238: 4BFFFFAC  b 0x823b71e4
	pc = 0x823B71E4; continue 'dispatch;
	// 823B723C: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7240: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7244: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823B7248: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B724C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7250: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7254: 40810054  ble 0x823b72a8
	if !ctx.cr[0].gt {
	pc = 0x823B72A8; continue 'dispatch;
	}
	// 823B7258: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B725C: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7260: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7264: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7268: 2F07007A  cmpwi cr6, r7, 0x7a
	ctx.cr[6].compare_i32(ctx.r[7].s32, 122, &mut ctx.xer);
	// 823B726C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B7270: 41980008  blt cr6, 0x823b7278
	if ctx.cr[6].lt {
	pc = 0x823B7278; continue 'dispatch;
	}
	// 823B7274: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823B7278: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B727C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B7280: 419A0014  beq cr6, 0x823b7294
	if ctx.cr[6].eq {
	pc = 0x823B7294; continue 'dispatch;
	}
	// 823B7284: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B7288: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B728C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B7290: 4800000C  b 0x823b729c
	pc = 0x823B729C; continue 'dispatch;
	// 823B7294: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B7298: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B729C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B72A0: 4199FFB8  bgt cr6, 0x823b7258
	if ctx.cr[6].gt {
	pc = 0x823B7258; continue 'dispatch;
	}
	// 823B72A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B72A8: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B72AC: 419A003C  beq cr6, 0x823b72e8
	if ctx.cr[6].eq {
	pc = 0x823B72E8; continue 'dispatch;
	}
	// 823B72B0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B72B4: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 823B72B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B72BC: 41990008  bgt cr6, 0x823b72c4
	if ctx.cr[6].gt {
	pc = 0x823B72C4; continue 'dispatch;
	}
	// 823B72C0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B72C4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B72C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B72CC: 409A001C  bne cr6, 0x823b72e8
	if !ctx.cr[6].eq {
	pc = 0x823B72E8; continue 'dispatch;
	}
	// 823B72D0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B72D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B72D8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B72DC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B72E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B72E4: 48000018  b 0x823b72fc
	pc = 0x823B72FC; continue 'dispatch;
	// 823B72E8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B72EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B72F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B72F4: 48000008  b 0x823b72fc
	pc = 0x823B72FC; continue 'dispatch;
	// 823B72F8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 823B72FC: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B7300: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7304: 419A011C  beq cr6, 0x823b7420
	if ctx.cr[6].eq {
	pc = 0x823B7420; continue 'dispatch;
	}
	// 823B7308: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B730C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823B7310: 4817E219  bl 0x82535528
	ctx.lr = 0x823B7314;
	sub_82535528(ctx, base);
	// 823B7314: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B7318: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B731C: 55493FFE  rlwinm r9, r10, 7, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x01FFFFFFu64;
	// 823B7320: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B7324: 419A00E4  beq cr6, 0x823b7408
	if ctx.cr[6].eq {
	pc = 0x823B7408; continue 'dispatch;
	}
	// 823B7328: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B732C: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7330: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7334: 419A001C  beq cr6, 0x823b7350
	if ctx.cr[6].eq {
	pc = 0x823B7350; continue 'dispatch;
	}
	// 823B7338: 896B0039  lbz r11, 0x39(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B733C: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 823B7340: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B7344: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7348: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B734C: 480000C0  b 0x823b740c
	pc = 0x823B740C; continue 'dispatch;
	// 823B7350: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7354: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823B7358: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B735C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7360: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7364: 40810054  ble 0x823b73b8
	if !ctx.cr[0].gt {
	pc = 0x823B73B8; continue 'dispatch;
	}
	// 823B7368: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B736C: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7370: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7374: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7378: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B737C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B7380: 41980008  blt cr6, 0x823b7388
	if ctx.cr[6].lt {
	pc = 0x823B7388; continue 'dispatch;
	}
	// 823B7384: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823B7388: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B738C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B7390: 419A0014  beq cr6, 0x823b73a4
	if ctx.cr[6].eq {
	pc = 0x823B73A4; continue 'dispatch;
	}
	// 823B7394: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B7398: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B739C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B73A0: 4800000C  b 0x823b73ac
	pc = 0x823B73AC; continue 'dispatch;
	// 823B73A4: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B73A8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B73AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B73B0: 4199FFB8  bgt cr6, 0x823b7368
	if ctx.cr[6].gt {
	pc = 0x823B7368; continue 'dispatch;
	}
	// 823B73B4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B73B8: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B73BC: 419A003C  beq cr6, 0x823b73f8
	if ctx.cr[6].eq {
	pc = 0x823B73F8; continue 'dispatch;
	}
	// 823B73C0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B73C4: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B73C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B73CC: 41990008  bgt cr6, 0x823b73d4
	if ctx.cr[6].gt {
	pc = 0x823B73D4; continue 'dispatch;
	}
	// 823B73D0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B73D4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B73D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B73DC: 409A001C  bne cr6, 0x823b73f8
	if !ctx.cr[6].eq {
	pc = 0x823B73F8; continue 'dispatch;
	}
	// 823B73E0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B73E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B73E8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B73EC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B73F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B73F4: 48000018  b 0x823b740c
	pc = 0x823B740C; continue 'dispatch;
	// 823B73F8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B73FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7400: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7404: 48000008  b 0x823b740c
	pc = 0x823B740C; continue 'dispatch;
	// 823B7408: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 823B740C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B7410: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7414: 419A000C  beq cr6, 0x823b7420
	if ctx.cr[6].eq {
	pc = 0x823B7420; continue 'dispatch;
	}
	// 823B7418: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B741C: 48124705  bl 0x824dbb20
	ctx.lr = 0x823B7420;
	sub_824DBB20(ctx, base);
	// 823B7420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B7424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B742C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 823B7430: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B7440 size=804
    let mut pc: u32 = 0x823B7440;
    'dispatch: loop {
        match pc {
            0x823B7440 => {
    //   block [0x823B7440..0x823B7764)
	// 823B7440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B744C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7450: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823B7454: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7458: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B745C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B7460: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823B7464: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B7468: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B746C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B7470: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7474: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7478: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B747C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B7480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7484: 419A0080  beq cr6, 0x823b7504
	if ctx.cr[6].eq {
	pc = 0x823B7504; continue 'dispatch;
	}
	// 823B7488: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B748C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7490: 419A0070  beq cr6, 0x823b7500
	if ctx.cr[6].eq {
	pc = 0x823B7500; continue 'dispatch;
	}
	// 823B7494: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 823B7498: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B749C: 419A0018  beq cr6, 0x823b74b4
	if ctx.cr[6].eq {
	pc = 0x823B74B4; continue 'dispatch;
	}
	// 823B74A0: 897F0090  lbz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B74A4: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B74A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B74AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B74B0: 409A0008  bne cr6, 0x823b74b8
	if !ctx.cr[6].eq {
	pc = 0x823B74B8; continue 'dispatch;
	}
	// 823B74B4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B74B8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B74BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B74C0: 419A0288  beq cr6, 0x823b7748
	if ctx.cr[6].eq {
	pc = 0x823B7748; continue 'dispatch;
	}
	// 823B74C4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B74C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B74CC: 556A3FFE  rlwinm r10, r11, 7, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 823B74D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B74D4: 419A00F4  beq cr6, 0x823b75c8
	if ctx.cr[6].eq {
	pc = 0x823B75C8; continue 'dispatch;
	}
	// 823B74D8: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B74DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B74E0: 419A002C  beq cr6, 0x823b750c
	if ctx.cr[6].eq {
	pc = 0x823B750C; continue 'dispatch;
	}
	// 823B74E4: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B74E8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B74EC: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B74F0: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B74F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B74F8: 80690004  lwz r3, 4(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B74FC: 480000D0  b 0x823b75cc
	pc = 0x823B75CC; continue 'dispatch;
	// 823B7500: 4BDDC939  bl 0x82193e38
	ctx.lr = 0x823B7504;
	sub_82193E38(ctx, base);
	// 823B7504: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 823B7508: 4BFFFFAC  b 0x823b74b4
	pc = 0x823B74B4; continue 'dispatch;
	// 823B750C: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7510: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7514: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823B7518: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B751C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7520: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7524: 40810054  ble 0x823b7578
	if !ctx.cr[0].gt {
	pc = 0x823B7578; continue 'dispatch;
	}
	// 823B7528: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B752C: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7530: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7534: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7538: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B753C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B7540: 41980008  blt cr6, 0x823b7548
	if ctx.cr[6].lt {
	pc = 0x823B7548; continue 'dispatch;
	}
	// 823B7544: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823B7548: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B754C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B7550: 419A0014  beq cr6, 0x823b7564
	if ctx.cr[6].eq {
	pc = 0x823B7564; continue 'dispatch;
	}
	// 823B7554: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B7558: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B755C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B7560: 4800000C  b 0x823b756c
	pc = 0x823B756C; continue 'dispatch;
	// 823B7564: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B7568: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B756C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7570: 4199FFB8  bgt cr6, 0x823b7528
	if ctx.cr[6].gt {
	pc = 0x823B7528; continue 'dispatch;
	}
	// 823B7574: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7578: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B757C: 419A003C  beq cr6, 0x823b75b8
	if ctx.cr[6].eq {
	pc = 0x823B75B8; continue 'dispatch;
	}
	// 823B7580: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7584: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B7588: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B758C: 41990008  bgt cr6, 0x823b7594
	if ctx.cr[6].gt {
	pc = 0x823B7594; continue 'dispatch;
	}
	// 823B7590: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B7594: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B759C: 409A001C  bne cr6, 0x823b75b8
	if !ctx.cr[6].eq {
	pc = 0x823B75B8; continue 'dispatch;
	}
	// 823B75A0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B75A4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B75A8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B75AC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B75B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B75B4: 48000018  b 0x823b75cc
	pc = 0x823B75CC; continue 'dispatch;
	// 823B75B8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B75BC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B75C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B75C4: 48000008  b 0x823b75cc
	pc = 0x823B75CC; continue 'dispatch;
	// 823B75C8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B75CC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B75D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B75D4: 419A0174  beq cr6, 0x823b7748
	if ctx.cr[6].eq {
	pc = 0x823B7748; continue 'dispatch;
	}
	// 823B75D8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B75DC: C1830040  lfs f12, 0x40(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823B75E0: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B75E4: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B75E8: 390A0E68  addi r8, r10, 0xe68
	ctx.r[8].s64 = ctx.r[10].s64 + 3688;
	// 823B75EC: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B75F0: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B75F4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 823B75F8: 7CE00026  mfcr r7
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B75FC: 54E6DF7A  rlwinm r6, r7, 0x1b, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 823B7600: 54E5F77A  rlwinm r5, r7, 0x1e, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 823B7604: 7CC42B78  or r4, r6, r5
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 823B7608: 7D68242E  lfsx f11, r8, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B760C: FD4B07EE  fsel f10, f11, f31, f0
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[0].f64 };
	// 823B7610: ED2A6828  fsubs f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B7614: FF090000  fcmpu cr6, f9, f0
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[0].f64);
	// 823B7618: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B761C: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823B7620: 5569F77A  rlwinm r9, r11, 0x1e, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823B7624: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823B7628: 7D083C2E  lfsx f8, r8, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 823B762C: FCE8536E  fsel f7, f8, f13, f10
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[10].f64 };
	// 823B7630: EC276028  fsubs f1, f7, f12
	ctx.f[1].f64 = (((ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 823B7634: 481205FD  bl 0x824d7c30
	ctx.lr = 0x823B7638;
	sub_824D7C30(ctx, base);
	// 823B7638: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B763C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B7640: 54C55FFE  rlwinm r5, r6, 0xb, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x001FFFFFu64;
	// 823B7644: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823B7648: 419A00E8  beq cr6, 0x823b7730
	if ctx.cr[6].eq {
	pc = 0x823B7730; continue 'dispatch;
	}
	// 823B764C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B7650: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7654: 419A0020  beq cr6, 0x823b7674
	if ctx.cr[6].eq {
	pc = 0x823B7674; continue 'dispatch;
	}
	// 823B7658: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 823B765C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7660: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7664: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7668: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B766C: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7670: 480000C4  b 0x823b7734
	pc = 0x823B7734; continue 'dispatch;
	// 823B7674: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7678: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B767C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823B7680: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B7684: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7688: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B768C: 40810054  ble 0x823b76e0
	if !ctx.cr[0].gt {
	pc = 0x823B76E0; continue 'dispatch;
	}
	// 823B7690: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B7694: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7698: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B769C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B76A0: 2F070015  cmpwi cr6, r7, 0x15
	ctx.cr[6].compare_i32(ctx.r[7].s32, 21, &mut ctx.xer);
	// 823B76A4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B76A8: 41980008  blt cr6, 0x823b76b0
	if ctx.cr[6].lt {
	pc = 0x823B76B0; continue 'dispatch;
	}
	// 823B76AC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823B76B0: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B76B4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B76B8: 419A0014  beq cr6, 0x823b76cc
	if ctx.cr[6].eq {
	pc = 0x823B76CC; continue 'dispatch;
	}
	// 823B76BC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B76C0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B76C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B76C8: 4800000C  b 0x823b76d4
	pc = 0x823B76D4; continue 'dispatch;
	// 823B76CC: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B76D0: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B76D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B76D8: 4199FFB8  bgt cr6, 0x823b7690
	if ctx.cr[6].gt {
	pc = 0x823B7690; continue 'dispatch;
	}
	// 823B76DC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B76E0: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B76E4: 419A003C  beq cr6, 0x823b7720
	if ctx.cr[6].eq {
	pc = 0x823B7720; continue 'dispatch;
	}
	// 823B76E8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B76EC: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 823B76F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B76F4: 41990008  bgt cr6, 0x823b76fc
	if ctx.cr[6].gt {
	pc = 0x823B76FC; continue 'dispatch;
	}
	// 823B76F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 823B76FC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7700: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7704: 409A001C  bne cr6, 0x823b7720
	if !ctx.cr[6].eq {
	pc = 0x823B7720; continue 'dispatch;
	}
	// 823B7708: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B770C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7710: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B7714: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B7718: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B771C: 48000018  b 0x823b7734
	pc = 0x823B7734; continue 'dispatch;
	// 823B7720: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B7724: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7728: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B772C: 48000008  b 0x823b7734
	pc = 0x823B7734; continue 'dispatch;
	// 823B7730: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 823B7734: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B7738: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B773C: 419A000C  beq cr6, 0x823b7748
	if ctx.cr[6].eq {
	pc = 0x823B7748; continue 'dispatch;
	}
	// 823B7740: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B7744: 4806964D  bl 0x82420d90
	ctx.lr = 0x823B7748;
	sub_82420D90(ctx, base);
	// 823B7748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B774C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7754: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 823B7758: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B775C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B7768 size=804
    let mut pc: u32 = 0x823B7768;
    'dispatch: loop {
        match pc {
            0x823B7768 => {
    //   block [0x823B7768..0x823B7A8C)
	// 823B7768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B776C: 488F1CA1  bl 0x82ca940c
	ctx.lr = 0x823B7770;
	sub_82CA93D0(ctx, base);
	// 823B7770: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823B7774: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7778: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B777C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B7780: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823B7784: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B7788: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B778C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B7790: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7794: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7798: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B779C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B77A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B77A4: 419A0084  beq cr6, 0x823b7828
	if ctx.cr[6].eq {
	pc = 0x823B7828; continue 'dispatch;
	}
	// 823B77A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B77AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B77B0: 419A0074  beq cr6, 0x823b7824
	if ctx.cr[6].eq {
	pc = 0x823B7824; continue 'dispatch;
	}
	// 823B77B4: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 823B77B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B77BC: 419A0018  beq cr6, 0x823b77d4
	if ctx.cr[6].eq {
	pc = 0x823B77D4; continue 'dispatch;
	}
	// 823B77C0: 897F0090  lbz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B77C4: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B77C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B77CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B77D0: 409A0008  bne cr6, 0x823b77d8
	if !ctx.cr[6].eq {
	pc = 0x823B77D8; continue 'dispatch;
	}
	// 823B77D4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B77D8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B77DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B77E0: 419A02A0  beq cr6, 0x823b7a80
	if ctx.cr[6].eq {
	pc = 0x823B7A80; continue 'dispatch;
	}
	// 823B77E4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B77E8: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 823B77EC: 556A3FFE  rlwinm r10, r11, 7, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 823B77F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B77F4: 419A0100  beq cr6, 0x823b78f4
	if ctx.cr[6].eq {
	pc = 0x823B78F4; continue 'dispatch;
	}
	// 823B77F8: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B77FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7800: 419A0030  beq cr6, 0x823b7830
	if ctx.cr[6].eq {
	pc = 0x823B7830; continue 'dispatch;
	}
	// 823B7804: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B7808: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B780C: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7810: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7814: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7818: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B781C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7820: 480000D8  b 0x823b78f8
	pc = 0x823B78F8; continue 'dispatch;
	// 823B7824: 4BDDC615  bl 0x82193e38
	ctx.lr = 0x823B7828;
	sub_82193E38(ctx, base);
	// 823B7828: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 823B782C: 4BFFFFA8  b 0x823b77d4
	pc = 0x823B77D4; continue 'dispatch;
	// 823B7830: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7834: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7838: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B783C: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B7840: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7844: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7848: 40810054  ble 0x823b789c
	if !ctx.cr[0].gt {
	pc = 0x823B789C; continue 'dispatch;
	}
	// 823B784C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B7850: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7854: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7858: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B785C: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B7860: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B7864: 41980008  blt cr6, 0x823b786c
	if ctx.cr[6].lt {
	pc = 0x823B786C; continue 'dispatch;
	}
	// 823B7868: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B786C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B7870: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B7874: 419A0014  beq cr6, 0x823b7888
	if ctx.cr[6].eq {
	pc = 0x823B7888; continue 'dispatch;
	}
	// 823B7878: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B787C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B7880: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B7884: 4800000C  b 0x823b7890
	pc = 0x823B7890; continue 'dispatch;
	// 823B7888: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B788C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B7890: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7894: 4199FFB8  bgt cr6, 0x823b784c
	if ctx.cr[6].gt {
	pc = 0x823B784C; continue 'dispatch;
	}
	// 823B7898: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B789C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B78A0: 419A0040  beq cr6, 0x823b78e0
	if ctx.cr[6].eq {
	pc = 0x823B78E0; continue 'dispatch;
	}
	// 823B78A4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B78A8: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B78AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B78B0: 41990008  bgt cr6, 0x823b78b8
	if ctx.cr[6].gt {
	pc = 0x823B78B8; continue 'dispatch;
	}
	// 823B78B4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B78B8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B78BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B78C0: 409A0020  bne cr6, 0x823b78e0
	if !ctx.cr[6].eq {
	pc = 0x823B78E0; continue 'dispatch;
	}
	// 823B78C4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B78C8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B78CC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B78D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B78D4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B78D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B78DC: 4800001C  b 0x823b78f8
	pc = 0x823B78F8; continue 'dispatch;
	// 823B78E0: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B78E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B78E8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B78EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B78F0: 48000008  b 0x823b78f8
	pc = 0x823B78F8; continue 'dispatch;
	// 823B78F4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B78F8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B78FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7900: 419A0180  beq cr6, 0x823b7a80
	if ctx.cr[6].eq {
	pc = 0x823B7A80; continue 'dispatch;
	}
	// 823B7904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B7908: C19E0044  lfs f12, 0x44(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823B790C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B7910: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B7914: 390A0E68  addi r8, r10, 0xe68
	ctx.r[8].s64 = ctx.r[10].s64 + 3688;
	// 823B7918: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B791C: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B7920: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B7924: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 823B7928: 7CE00026  mfcr r7
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B792C: 54E6DF7A  rlwinm r6, r7, 0x1b, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 823B7930: 54E5F77A  rlwinm r5, r7, 0x1e, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 823B7934: 7CC42B78  or r4, r6, r5
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 823B7938: 7D68242E  lfsx f11, r8, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B793C: FD4B07EE  fsel f10, f11, f31, f0
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[0].f64 };
	// 823B7940: ED2A6828  fsubs f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B7944: FF090000  fcmpu cr6, f9, f0
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[0].f64);
	// 823B7948: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B794C: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823B7950: 5569F77A  rlwinm r9, r11, 0x1e, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823B7954: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823B7958: 7D083C2E  lfsx f8, r8, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 823B795C: FCE8536E  fsel f7, f8, f13, f10
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[10].f64 };
	// 823B7960: EC276028  fsubs f1, f7, f12
	ctx.f[1].f64 = (((ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 823B7964: 48120415  bl 0x824d7d78
	ctx.lr = 0x823B7968;
	sub_824D7D78(ctx, base);
	// 823B7968: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B796C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B7970: 54C55FFE  rlwinm r5, r6, 0xb, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x001FFFFFu64;
	// 823B7974: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823B7978: 419A00E8  beq cr6, 0x823b7a60
	if ctx.cr[6].eq {
	pc = 0x823B7A60; continue 'dispatch;
	}
	// 823B797C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B7980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7984: 419A0020  beq cr6, 0x823b79a4
	if ctx.cr[6].eq {
	pc = 0x823B79A4; continue 'dispatch;
	}
	// 823B7988: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 823B798C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7990: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7994: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7998: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B799C: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B79A0: 480000C4  b 0x823b7a64
	pc = 0x823B7A64; continue 'dispatch;
	// 823B79A4: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B79A8: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B79AC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B79B0: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B79B4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B79B8: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B79BC: 40810054  ble 0x823b7a10
	if !ctx.cr[0].gt {
	pc = 0x823B7A10; continue 'dispatch;
	}
	// 823B79C0: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B79C4: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B79C8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B79CC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B79D0: 2F070015  cmpwi cr6, r7, 0x15
	ctx.cr[6].compare_i32(ctx.r[7].s32, 21, &mut ctx.xer);
	// 823B79D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B79D8: 41980008  blt cr6, 0x823b79e0
	if ctx.cr[6].lt {
	pc = 0x823B79E0; continue 'dispatch;
	}
	// 823B79DC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B79E0: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B79E4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B79E8: 419A0014  beq cr6, 0x823b79fc
	if ctx.cr[6].eq {
	pc = 0x823B79FC; continue 'dispatch;
	}
	// 823B79EC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B79F0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B79F4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B79F8: 4800000C  b 0x823b7a04
	pc = 0x823B7A04; continue 'dispatch;
	// 823B79FC: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B7A00: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B7A04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7A08: 4199FFB8  bgt cr6, 0x823b79c0
	if ctx.cr[6].gt {
	pc = 0x823B79C0; continue 'dispatch;
	}
	// 823B7A0C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7A10: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B7A14: 419A003C  beq cr6, 0x823b7a50
	if ctx.cr[6].eq {
	pc = 0x823B7A50; continue 'dispatch;
	}
	// 823B7A18: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7A1C: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 823B7A20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7A24: 41990008  bgt cr6, 0x823b7a2c
	if ctx.cr[6].gt {
	pc = 0x823B7A2C; continue 'dispatch;
	}
	// 823B7A28: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B7A2C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7A30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7A34: 409A001C  bne cr6, 0x823b7a50
	if !ctx.cr[6].eq {
	pc = 0x823B7A50; continue 'dispatch;
	}
	// 823B7A38: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B7A3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7A40: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B7A44: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B7A48: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7A4C: 48000018  b 0x823b7a64
	pc = 0x823B7A64; continue 'dispatch;
	// 823B7A50: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B7A54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7A58: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7A5C: 48000008  b 0x823b7a64
	pc = 0x823B7A64; continue 'dispatch;
	// 823B7A60: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 823B7A64: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B7A68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7A6C: 419A000C  beq cr6, 0x823b7a78
	if ctx.cr[6].eq {
	pc = 0x823B7A78; continue 'dispatch;
	}
	// 823B7A70: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B7A74: 4806931D  bl 0x82420d90
	ctx.lr = 0x823B7A78;
	sub_82420D90(ctx, base);
	// 823B7A78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7A7C: 481240A5  bl 0x824dbb20
	ctx.lr = 0x823B7A80;
	sub_824DBB20(ctx, base);
	// 823B7A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B7A84: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823B7A88: 488F19D4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B7A90 size=804
    let mut pc: u32 = 0x823B7A90;
    'dispatch: loop {
        match pc {
            0x823B7A90 => {
    //   block [0x823B7A90..0x823B7DB4)
	// 823B7A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7A94: 488F1979  bl 0x82ca940c
	ctx.lr = 0x823B7A98;
	sub_82CA93D0(ctx, base);
	// 823B7A98: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823B7A9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7AA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B7AA4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B7AA8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823B7AAC: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B7AB0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B7AB4: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B7AB8: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7ABC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7AC0: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B7AC4: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B7AC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7ACC: 419A0084  beq cr6, 0x823b7b50
	if ctx.cr[6].eq {
	pc = 0x823B7B50; continue 'dispatch;
	}
	// 823B7AD0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7AD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7AD8: 419A0074  beq cr6, 0x823b7b4c
	if ctx.cr[6].eq {
	pc = 0x823B7B4C; continue 'dispatch;
	}
	// 823B7ADC: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 823B7AE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7AE4: 419A0018  beq cr6, 0x823b7afc
	if ctx.cr[6].eq {
	pc = 0x823B7AFC; continue 'dispatch;
	}
	// 823B7AE8: 897F0090  lbz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B7AEC: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B7AF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7AF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7AF8: 409A0008  bne cr6, 0x823b7b00
	if !ctx.cr[6].eq {
	pc = 0x823B7B00; continue 'dispatch;
	}
	// 823B7AFC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B7B00: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7B04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7B08: 419A02A0  beq cr6, 0x823b7da8
	if ctx.cr[6].eq {
	pc = 0x823B7DA8; continue 'dispatch;
	}
	// 823B7B0C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B7B10: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 823B7B14: 556A3FFE  rlwinm r10, r11, 7, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 823B7B18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7B1C: 419A0100  beq cr6, 0x823b7c1c
	if ctx.cr[6].eq {
	pc = 0x823B7C1C; continue 'dispatch;
	}
	// 823B7B20: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B7B24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7B28: 419A0030  beq cr6, 0x823b7b58
	if ctx.cr[6].eq {
	pc = 0x823B7B58; continue 'dispatch;
	}
	// 823B7B2C: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B7B30: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7B34: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7B38: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7B3C: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7B40: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B7B44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7B48: 480000D8  b 0x823b7c20
	pc = 0x823B7C20; continue 'dispatch;
	// 823B7B4C: 4BDDC2ED  bl 0x82193e38
	ctx.lr = 0x823B7B50;
	sub_82193E38(ctx, base);
	// 823B7B50: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 823B7B54: 4BFFFFA8  b 0x823b7afc
	pc = 0x823B7AFC; continue 'dispatch;
	// 823B7B58: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7B5C: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7B60: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B7B64: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B7B68: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7B6C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7B70: 40810054  ble 0x823b7bc4
	if !ctx.cr[0].gt {
	pc = 0x823B7BC4; continue 'dispatch;
	}
	// 823B7B74: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B7B78: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7B7C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7B80: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7B84: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B7B88: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B7B8C: 41980008  blt cr6, 0x823b7b94
	if ctx.cr[6].lt {
	pc = 0x823B7B94; continue 'dispatch;
	}
	// 823B7B90: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B7B94: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B7B98: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B7B9C: 419A0014  beq cr6, 0x823b7bb0
	if ctx.cr[6].eq {
	pc = 0x823B7BB0; continue 'dispatch;
	}
	// 823B7BA0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B7BA4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B7BA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B7BAC: 4800000C  b 0x823b7bb8
	pc = 0x823B7BB8; continue 'dispatch;
	// 823B7BB0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B7BB4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B7BB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7BBC: 4199FFB8  bgt cr6, 0x823b7b74
	if ctx.cr[6].gt {
	pc = 0x823B7B74; continue 'dispatch;
	}
	// 823B7BC0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7BC4: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B7BC8: 419A0040  beq cr6, 0x823b7c08
	if ctx.cr[6].eq {
	pc = 0x823B7C08; continue 'dispatch;
	}
	// 823B7BCC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7BD0: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B7BD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7BD8: 41990008  bgt cr6, 0x823b7be0
	if ctx.cr[6].gt {
	pc = 0x823B7BE0; continue 'dispatch;
	}
	// 823B7BDC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B7BE0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7BE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7BE8: 409A0020  bne cr6, 0x823b7c08
	if !ctx.cr[6].eq {
	pc = 0x823B7C08; continue 'dispatch;
	}
	// 823B7BEC: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B7BF0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B7BF4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B7BF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7BFC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B7C00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7C04: 4800001C  b 0x823b7c20
	pc = 0x823B7C20; continue 'dispatch;
	// 823B7C08: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B7C0C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7C10: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B7C14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7C18: 48000008  b 0x823b7c20
	pc = 0x823B7C20; continue 'dispatch;
	// 823B7C1C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B7C20: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7C28: 419A0180  beq cr6, 0x823b7da8
	if ctx.cr[6].eq {
	pc = 0x823B7DA8; continue 'dispatch;
	}
	// 823B7C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B7C30: C19E0048  lfs f12, 0x48(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823B7C34: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 823B7C38: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823B7C3C: 390A0E68  addi r8, r10, 0xe68
	ctx.r[8].s64 = ctx.r[10].s64 + 3688;
	// 823B7C40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7C44: C1AB9490  lfs f13, -0x6b70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B7C48: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B7C4C: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 823B7C50: 7CE00026  mfcr r7
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[7].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[7].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B7C54: 54E6DF7A  rlwinm r6, r7, 0x1b, 0x1d, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 823B7C58: 54E5F77A  rlwinm r5, r7, 0x1e, 0x1d, 0x1d
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 823B7C5C: 7CC42B78  or r4, r6, r5
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 823B7C60: 7D68242E  lfsx f11, r8, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 823B7C64: FD4B07EE  fsel f10, f11, f31, f0
	ctx.f[10].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[0].f64 };
	// 823B7C68: ED2A6828  fsubs f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 823B7C6C: FF090000  fcmpu cr6, f9, f0
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[0].f64);
	// 823B7C70: 7D600026  mfcr r11
	// MFCR packs CR[0..7] (lt,gt,eq,so per field) into GPR
	ctx.r[11].u64 = if ctx.cr[0].lt { 0x80000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].gt { 0x40000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].eq { 0x20000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[0].so { 0x10000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].lt { 0x8000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].gt { 0x4000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].eq { 0x2000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[1].so { 0x1000000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].lt { 0x800000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].gt { 0x400000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].eq { 0x200000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[2].so { 0x100000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].lt { 0x80000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].gt { 0x40000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].eq { 0x20000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[3].so { 0x10000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].lt { 0x8000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].gt { 0x4000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].eq { 0x2000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[4].so { 0x1000 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].lt { 0x800 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].gt { 0x400 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].eq { 0x200 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[5].so { 0x100 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].lt { 0x80 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].gt { 0x40 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].eq { 0x20 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[6].so { 0x10 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].lt { 0x8 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].gt { 0x4 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].eq { 0x2 } else { 0 };
	ctx.r[11].u64 |= if ctx.cr[7].so { 0x1 } else { 0 };
	// 823B7C74: 556ADF7A  rlwinm r10, r11, 0x1b, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823B7C78: 5569F77A  rlwinm r9, r11, 0x1e, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 823B7C7C: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 823B7C80: 7D083C2E  lfsx f8, r8, r7
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 823B7C84: FCE8536E  fsel f7, f8, f13, f10
	ctx.f[7].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[10].f64 };
	// 823B7C88: EC276028  fsubs f1, f7, f12
	ctx.f[1].f64 = (((ctx.f[7].f64 - ctx.f[12].f64) as f32) as f64);
	// 823B7C8C: 481202F5  bl 0x824d7f80
	ctx.lr = 0x823B7C90;
	sub_824D7F80(ctx, base);
	// 823B7C90: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B7C94: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B7C98: 54C55FFE  rlwinm r5, r6, 0xb, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0x001FFFFFu64;
	// 823B7C9C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823B7CA0: 419A00E8  beq cr6, 0x823b7d88
	if ctx.cr[6].eq {
	pc = 0x823B7D88; continue 'dispatch;
	}
	// 823B7CA4: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B7CA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7CAC: 419A0020  beq cr6, 0x823b7ccc
	if ctx.cr[6].eq {
	pc = 0x823B7CCC; continue 'dispatch;
	}
	// 823B7CB0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 823B7CB4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7CB8: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7CBC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7CC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7CC4: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7CC8: 480000C4  b 0x823b7d8c
	pc = 0x823B7D8C; continue 'dispatch;
	// 823B7CCC: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7CD0: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7CD4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B7CD8: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B7CDC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7CE0: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7CE4: 40810054  ble 0x823b7d38
	if !ctx.cr[0].gt {
	pc = 0x823B7D38; continue 'dispatch;
	}
	// 823B7CE8: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B7CEC: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7CF0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7CF4: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7CF8: 2F070015  cmpwi cr6, r7, 0x15
	ctx.cr[6].compare_i32(ctx.r[7].s32, 21, &mut ctx.xer);
	// 823B7CFC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B7D00: 41980008  blt cr6, 0x823b7d08
	if ctx.cr[6].lt {
	pc = 0x823B7D08; continue 'dispatch;
	}
	// 823B7D04: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B7D08: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B7D0C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B7D10: 419A0014  beq cr6, 0x823b7d24
	if ctx.cr[6].eq {
	pc = 0x823B7D24; continue 'dispatch;
	}
	// 823B7D14: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B7D18: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B7D1C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B7D20: 4800000C  b 0x823b7d2c
	pc = 0x823B7D2C; continue 'dispatch;
	// 823B7D24: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B7D28: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B7D2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7D30: 4199FFB8  bgt cr6, 0x823b7ce8
	if ctx.cr[6].gt {
	pc = 0x823B7CE8; continue 'dispatch;
	}
	// 823B7D34: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7D38: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B7D3C: 419A003C  beq cr6, 0x823b7d78
	if ctx.cr[6].eq {
	pc = 0x823B7D78; continue 'dispatch;
	}
	// 823B7D40: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7D44: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 823B7D48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7D4C: 41990008  bgt cr6, 0x823b7d54
	if ctx.cr[6].gt {
	pc = 0x823B7D54; continue 'dispatch;
	}
	// 823B7D50: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B7D54: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7D58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7D5C: 409A001C  bne cr6, 0x823b7d78
	if !ctx.cr[6].eq {
	pc = 0x823B7D78; continue 'dispatch;
	}
	// 823B7D60: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B7D64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7D68: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B7D6C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B7D70: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7D74: 48000018  b 0x823b7d8c
	pc = 0x823B7D8C; continue 'dispatch;
	// 823B7D78: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B7D7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7D80: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7D84: 48000008  b 0x823b7d8c
	pc = 0x823B7D8C; continue 'dispatch;
	// 823B7D88: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 823B7D8C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B7D90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7D94: 419A000C  beq cr6, 0x823b7da0
	if ctx.cr[6].eq {
	pc = 0x823B7DA0; continue 'dispatch;
	}
	// 823B7D98: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B7D9C: 48068FF5  bl 0x82420d90
	ctx.lr = 0x823B7DA0;
	sub_82420D90(ctx, base);
	// 823B7DA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7DA4: 48123D7D  bl 0x824dbb20
	ctx.lr = 0x823B7DA8;
	sub_824DBB20(ctx, base);
	// 823B7DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B7DAC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823B7DB0: 488F16AC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7DB8 size=428
    let mut pc: u32 = 0x823B7DB8;
    'dispatch: loop {
        match pc {
            0x823B7DB8 => {
    //   block [0x823B7DB8..0x823B7F64)
	// 823B7DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7DC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7DC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7DC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B7DCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7DD0: 4BDE4819  bl 0x8219c5e8
	ctx.lr = 0x823B7DD4;
	sub_8219C5E8(ctx, base);
	// 823B7DD4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B7DD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7DDC: 419A0174  beq cr6, 0x823b7f50
	if ctx.cr[6].eq {
	pc = 0x823B7F50; continue 'dispatch;
	}
	// 823B7DE0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B7DE4: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B7DE8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B7DEC: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B7DF0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7DF4: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7DF8: 4BE743C1  bl 0x8222c1b8
	ctx.lr = 0x823B7DFC;
	sub_8222C1B8(ctx, base);
	// 823B7DFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7E00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7E04: 419A0018  beq cr6, 0x823b7e1c
	if ctx.cr[6].eq {
	pc = 0x823B7E1C; continue 'dispatch;
	}
	// 823B7E08: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B7E0C: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B7E10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7E14: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B7E18: 409A0008  bne cr6, 0x823b7e20
	if !ctx.cr[6].eq {
	pc = 0x823B7E20; continue 'dispatch;
	}
	// 823B7E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B7E20: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B7E24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7E28: 419A0128  beq cr6, 0x823b7f50
	if ctx.cr[6].eq {
	pc = 0x823B7F50; continue 'dispatch;
	}
	// 823B7E2C: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B7E30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B7E34: 554927FE  rlwinm r9, r10, 4, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0FFFFFFFu64;
	// 823B7E38: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B7E3C: 419A00F8  beq cr6, 0x823b7f34
	if ctx.cr[6].eq {
	pc = 0x823B7F34; continue 'dispatch;
	}
	// 823B7E40: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B7E44: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7E48: 419A0024  beq cr6, 0x823b7e6c
	if ctx.cr[6].eq {
	pc = 0x823B7E6C; continue 'dispatch;
	}
	// 823B7E4C: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 823B7E50: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7E54: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B7E58: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B7E5C: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7E60: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B7E64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7E68: 480000D0  b 0x823b7f38
	pc = 0x823B7F38; continue 'dispatch;
	// 823B7E6C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B7E70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B7E74: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B7E78: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B7E7C: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B7E80: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7E84: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7E88: 40810054  ble 0x823b7edc
	if !ctx.cr[0].gt {
	pc = 0x823B7EDC; continue 'dispatch;
	}
	// 823B7E8C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B7E90: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B7E94: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B7E98: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7E9C: 2F07001C  cmpwi cr6, r7, 0x1c
	ctx.cr[6].compare_i32(ctx.r[7].s32, 28, &mut ctx.xer);
	// 823B7EA0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B7EA4: 41980008  blt cr6, 0x823b7eac
	if ctx.cr[6].lt {
	pc = 0x823B7EAC; continue 'dispatch;
	}
	// 823B7EA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B7EAC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B7EB0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B7EB4: 419A0014  beq cr6, 0x823b7ec8
	if ctx.cr[6].eq {
	pc = 0x823B7EC8; continue 'dispatch;
	}
	// 823B7EB8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B7EBC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B7EC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B7EC4: 4800000C  b 0x823b7ed0
	pc = 0x823B7ED0; continue 'dispatch;
	// 823B7EC8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B7ECC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B7ED0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B7ED4: 4199FFB8  bgt cr6, 0x823b7e8c
	if ctx.cr[6].gt {
	pc = 0x823B7E8C; continue 'dispatch;
	}
	// 823B7ED8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B7EDC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B7EE0: 419A0040  beq cr6, 0x823b7f20
	if ctx.cr[6].eq {
	pc = 0x823B7F20; continue 'dispatch;
	}
	// 823B7EE4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7EE8: 2F0B001C  cmpwi cr6, r11, 0x1c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 28, &mut ctx.xer);
	// 823B7EEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7EF0: 41990008  bgt cr6, 0x823b7ef8
	if ctx.cr[6].gt {
	pc = 0x823B7EF8; continue 'dispatch;
	}
	// 823B7EF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7EF8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7F00: 409A0020  bne cr6, 0x823b7f20
	if !ctx.cr[6].eq {
	pc = 0x823B7F20; continue 'dispatch;
	}
	// 823B7F04: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B7F08: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B7F0C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B7F10: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7F14: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B7F18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7F1C: 4800001C  b 0x823b7f38
	pc = 0x823B7F38; continue 'dispatch;
	// 823B7F20: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B7F24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7F28: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B7F2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7F30: 48000008  b 0x823b7f38
	pc = 0x823B7F38; continue 'dispatch;
	// 823B7F34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7F38: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7F3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7F40: 419A0010  beq cr6, 0x823b7f50
	if ctx.cr[6].eq {
	pc = 0x823B7F50; continue 'dispatch;
	}
	// 823B7F44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823B7F48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B7F4C: 480F37AD  bl 0x824ab6f8
	ctx.lr = 0x823B7F50;
	sub_824AB6F8(ctx, base);
	// 823B7F50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7F5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7F68 size=1120
    let mut pc: u32 = 0x823B7F68;
    'dispatch: loop {
        match pc {
            0x823B7F68 => {
    //   block [0x823B7F68..0x823B83C8)
	// 823B7F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7F6C: 488F1499  bl 0x82ca9404
	ctx.lr = 0x823B7F70;
	sub_82CA93D0(ctx, base);
	// 823B7F70: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7F74: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B7F78: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823B7F7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7F80: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 823B7F84: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B7F88: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B7F8C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B7F90: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7F94: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7F98: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B7F9C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B7FA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7FA4: 419A0090  beq cr6, 0x823b8034
	if ctx.cr[6].eq {
	pc = 0x823B8034; continue 'dispatch;
	}
	// 823B7FA8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7FAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7FB0: 419A0080  beq cr6, 0x823b8030
	if ctx.cr[6].eq {
	pc = 0x823B8030; continue 'dispatch;
	}
	// 823B7FB4: 555E003E  slwi r30, r10, 0
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 823B7FB8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823B7FBC: 419A0018  beq cr6, 0x823b7fd4
	if ctx.cr[6].eq {
	pc = 0x823B7FD4; continue 'dispatch;
	}
	// 823B7FC0: 897E0090  lbz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B7FC4: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B7FC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B7FCC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B7FD0: 409A0008  bne cr6, 0x823b7fd8
	if !ctx.cr[6].eq {
	pc = 0x823B7FD8; continue 'dispatch;
	}
	// 823B7FD4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 823B7FD8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B7FDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7FE0: 419A03E0  beq cr6, 0x823b83c0
	if ctx.cr[6].eq {
	pc = 0x823B83C0; continue 'dispatch;
	}
	// 823B7FE4: 57FD063E  clrlwi r29, r31, 0x18
	ctx.r[29].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 823B7FE8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823B7FEC: 419A017C  beq cr6, 0x823b8168
	if ctx.cr[6].eq {
	pc = 0x823B8168; continue 'dispatch;
	}
	// 823B7FF0: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B7FF4: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 823B7FF8: 556A9FFE  rlwinm r10, r11, 0x13, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00001FFFu64;
	// 823B7FFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8000: 419A0100  beq cr6, 0x823b8100
	if ctx.cr[6].eq {
	pc = 0x823B8100; continue 'dispatch;
	}
	// 823B8004: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B800C: 419A0030  beq cr6, 0x823b803c
	if ctx.cr[6].eq {
	pc = 0x823B803C; continue 'dispatch;
	}
	// 823B8010: 894B000D  lbz r10, 0xd(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 823B8014: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8018: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B801C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B8020: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8024: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B8028: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B802C: 480000D8  b 0x823b8104
	pc = 0x823B8104; continue 'dispatch;
	// 823B8030: 4BDDBE09  bl 0x82193e38
	ctx.lr = 0x823B8034;
	sub_82193E38(ctx, base);
	// 823B8034: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 823B8038: 4BFFFF9C  b 0x823b7fd4
	pc = 0x823B7FD4; continue 'dispatch;
	// 823B803C: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8040: 80DE004C  lwz r6, 0x4c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B8044: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 823B8048: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B804C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8050: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8054: 40810054  ble 0x823b80a8
	if !ctx.cr[0].gt {
	pc = 0x823B80A8; continue 'dispatch;
	}
	// 823B8058: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B805C: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B8060: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B8064: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8068: 2F07000D  cmpwi cr6, r7, 0xd
	ctx.cr[6].compare_i32(ctx.r[7].s32, 13, &mut ctx.xer);
	// 823B806C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B8070: 41980008  blt cr6, 0x823b8078
	if ctx.cr[6].lt {
	pc = 0x823B8078; continue 'dispatch;
	}
	// 823B8074: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 823B8078: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B807C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B8080: 419A0014  beq cr6, 0x823b8094
	if ctx.cr[6].eq {
	pc = 0x823B8094; continue 'dispatch;
	}
	// 823B8084: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8088: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B808C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B8090: 4800000C  b 0x823b809c
	pc = 0x823B809C; continue 'dispatch;
	// 823B8094: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8098: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B809C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B80A0: 4199FFB8  bgt cr6, 0x823b8058
	if ctx.cr[6].gt {
	pc = 0x823B8058; continue 'dispatch;
	}
	// 823B80A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B80A8: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B80AC: 419A0040  beq cr6, 0x823b80ec
	if ctx.cr[6].eq {
	pc = 0x823B80EC; continue 'dispatch;
	}
	// 823B80B0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B80B4: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 823B80B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B80BC: 41990008  bgt cr6, 0x823b80c4
	if ctx.cr[6].gt {
	pc = 0x823B80C4; continue 'dispatch;
	}
	// 823B80C0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 823B80C4: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B80C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B80CC: 409A0020  bne cr6, 0x823b80ec
	if !ctx.cr[6].eq {
	pc = 0x823B80EC; continue 'dispatch;
	}
	// 823B80D0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B80D4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B80D8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B80DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B80E0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B80E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B80E8: 4800001C  b 0x823b8104
	pc = 0x823B8104; continue 'dispatch;
	// 823B80EC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B80F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B80F4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B80F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B80FC: 48000008  b 0x823b8104
	pc = 0x823B8104; continue 'dispatch;
	// 823B8100: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 823B8104: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B810C: 419A005C  beq cr6, 0x823b8168
	if ctx.cr[6].eq {
	pc = 0x823B8168; continue 'dispatch;
	}
	// 823B8110: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823B8114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8118: 4813A181  bl 0x824f2298
	ctx.lr = 0x823B811C;
	sub_824F2298(ctx, base);
	// 823B811C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 823B8120: 409A0048  bne cr6, 0x823b8168
	if !ctx.cr[6].eq {
	pc = 0x823B8168; continue 'dispatch;
	}
	// 823B8124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B8128: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823B812C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8130: 48046789  bl 0x823fe8b8
	ctx.lr = 0x823B8134;
	sub_823FE8B8(ctx, base);
	// 823B8134: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B8138: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823B813C: 419A006C  beq cr6, 0x823b81a8
	if ctx.cr[6].eq {
	pc = 0x823B81A8; continue 'dispatch;
	}
	// 823B8140: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 823B8144: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B8148: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B814C: 481386D5  bl 0x824f0820
	ctx.lr = 0x823B8150;
	sub_824F0820(ctx, base);
	// 823B8150: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 823B8154: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B8158: 419A000C  beq cr6, 0x823b8164
	if ctx.cr[6].eq {
	pc = 0x823B8164; continue 'dispatch;
	}
	// 823B815C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 823B8160: 483ACC61  bl 0x82764dc0
	ctx.lr = 0x823B8164;
	sub_82764DC0(ctx, base);
	// 823B8164: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 823B8168: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B816C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 823B8170: 556A8FFE  rlwinm r10, r11, 0x11, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 823B8174: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8178: 419A0100  beq cr6, 0x823b8278
	if ctx.cr[6].eq {
	pc = 0x823B8278; continue 'dispatch;
	}
	// 823B817C: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8180: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8184: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8188: 419A0030  beq cr6, 0x823b81b8
	if ctx.cr[6].eq {
	pc = 0x823B81B8; continue 'dispatch;
	}
	// 823B818C: 896B000F  lbz r11, 0xf(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(15 as u32) ) } as u64;
	// 823B8190: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 823B8194: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B8198: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B819C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B81A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B81A4: 480000D8  b 0x823b827c
	pc = 0x823B827C; continue 'dispatch;
	// 823B81A8: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 823B81AC: 93610068  stw r27, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 823B81B0: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 823B81B4: 4BFFFFB0  b 0x823b8164
	pc = 0x823B8164; continue 'dispatch;
	// 823B81B8: 80DE004C  lwz r6, 0x4c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B81BC: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 823B81C0: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B81C4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B81C8: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B81CC: 40810054  ble 0x823b8220
	if !ctx.cr[0].gt {
	pc = 0x823B8220; continue 'dispatch;
	}
	// 823B81D0: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B81D4: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B81D8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B81DC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B81E0: 2F07000F  cmpwi cr6, r7, 0xf
	ctx.cr[6].compare_i32(ctx.r[7].s32, 15, &mut ctx.xer);
	// 823B81E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B81E8: 41980008  blt cr6, 0x823b81f0
	if ctx.cr[6].lt {
	pc = 0x823B81F0; continue 'dispatch;
	}
	// 823B81EC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 823B81F0: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B81F4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B81F8: 419A0014  beq cr6, 0x823b820c
	if ctx.cr[6].eq {
	pc = 0x823B820C; continue 'dispatch;
	}
	// 823B81FC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8200: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B8204: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B8208: 4800000C  b 0x823b8214
	pc = 0x823B8214; continue 'dispatch;
	// 823B820C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8210: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B8214: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8218: 4199FFB8  bgt cr6, 0x823b81d0
	if ctx.cr[6].gt {
	pc = 0x823B81D0; continue 'dispatch;
	}
	// 823B821C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8220: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B8224: 419A0040  beq cr6, 0x823b8264
	if ctx.cr[6].eq {
	pc = 0x823B8264; continue 'dispatch;
	}
	// 823B8228: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B822C: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 823B8230: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8234: 41990008  bgt cr6, 0x823b823c
	if ctx.cr[6].gt {
	pc = 0x823B823C; continue 'dispatch;
	}
	// 823B8238: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 823B823C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8240: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8244: 409A0020  bne cr6, 0x823b8264
	if !ctx.cr[6].eq {
	pc = 0x823B8264; continue 'dispatch;
	}
	// 823B8248: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B824C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B8250: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B8254: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8258: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B825C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8260: 4800001C  b 0x823b827c
	pc = 0x823B827C; continue 'dispatch;
	// 823B8264: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B8268: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B826C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B8270: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8274: 48000008  b 0x823b827c
	pc = 0x823B827C; continue 'dispatch;
	// 823B8278: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 823B827C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8284: 419A0030  beq cr6, 0x823b82b4
	if ctx.cr[6].eq {
	pc = 0x823B82B4; continue 'dispatch;
	}
	// 823B8288: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823B828C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823B8290: 419A0010  beq cr6, 0x823b82a0
	if ctx.cr[6].eq {
	pc = 0x823B82A0; continue 'dispatch;
	}
	// 823B8294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8298: 48106D61  bl 0x824beff8
	ctx.lr = 0x823B829C;
	sub_824BEFF8(ctx, base);
	// 823B829C: 48000018  b 0x823b82b4
	pc = 0x823B82B4; continue 'dispatch;
	// 823B82A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B82A4: 4BDE4345  bl 0x8219c5e8
	ctx.lr = 0x823B82A8;
	sub_8219C5E8(ctx, base);
	// 823B82A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B82AC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B82B0: 48107139  bl 0x824bf3e8
	ctx.lr = 0x823B82B4;
	sub_824BF3E8(ctx, base);
	// 823B82B4: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B82B8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 823B82BC: 55497FFE  rlwinm r9, r10, 0xf, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0001FFFFu64;
	// 823B82C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B82C4: 419A00E4  beq cr6, 0x823b83a8
	if ctx.cr[6].eq {
	pc = 0x823B83A8; continue 'dispatch;
	}
	// 823B82C8: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B82CC: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B82D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B82D4: 419A001C  beq cr6, 0x823b82f0
	if ctx.cr[6].eq {
	pc = 0x823B82F0; continue 'dispatch;
	}
	// 823B82D8: 896B0011  lbz r11, 0x11(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 823B82DC: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 823B82E0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B82E4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B82E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B82EC: 480000C0  b 0x823b83ac
	pc = 0x823B83AC; continue 'dispatch;
	// 823B82F0: 80DE004C  lwz r6, 0x4c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B82F4: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 823B82F8: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B82FC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8300: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8304: 40810054  ble 0x823b8358
	if !ctx.cr[0].gt {
	pc = 0x823B8358; continue 'dispatch;
	}
	// 823B8308: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B830C: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B8310: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B8314: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8318: 2F070011  cmpwi cr6, r7, 0x11
	ctx.cr[6].compare_i32(ctx.r[7].s32, 17, &mut ctx.xer);
	// 823B831C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B8320: 41980008  blt cr6, 0x823b8328
	if ctx.cr[6].lt {
	pc = 0x823B8328; continue 'dispatch;
	}
	// 823B8324: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 823B8328: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B832C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B8330: 419A0014  beq cr6, 0x823b8344
	if ctx.cr[6].eq {
	pc = 0x823B8344; continue 'dispatch;
	}
	// 823B8334: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8338: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B833C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B8340: 4800000C  b 0x823b834c
	pc = 0x823B834C; continue 'dispatch;
	// 823B8344: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8348: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B834C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8350: 4199FFB8  bgt cr6, 0x823b8308
	if ctx.cr[6].gt {
	pc = 0x823B8308; continue 'dispatch;
	}
	// 823B8354: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8358: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B835C: 419A003C  beq cr6, 0x823b8398
	if ctx.cr[6].eq {
	pc = 0x823B8398; continue 'dispatch;
	}
	// 823B8360: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8364: 2F0B0011  cmpwi cr6, r11, 0x11
	ctx.cr[6].compare_i32(ctx.r[11].s32, 17, &mut ctx.xer);
	// 823B8368: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B836C: 41990008  bgt cr6, 0x823b8374
	if ctx.cr[6].gt {
	pc = 0x823B8374; continue 'dispatch;
	}
	// 823B8370: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 823B8374: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8378: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B837C: 409A001C  bne cr6, 0x823b8398
	if !ctx.cr[6].eq {
	pc = 0x823B8398; continue 'dispatch;
	}
	// 823B8380: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B8384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8388: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B838C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B8390: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8394: 48000018  b 0x823b83ac
	pc = 0x823B83AC; continue 'dispatch;
	// 823B8398: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B839C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B83A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B83A4: 48000008  b 0x823b83ac
	pc = 0x823B83AC; continue 'dispatch;
	// 823B83A8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 823B83AC: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B83B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B83B4: 419A000C  beq cr6, 0x823b83c0
	if ctx.cr[6].eq {
	pc = 0x823B83C0; continue 'dispatch;
	}
	// 823B83B8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B83BC: 481FDD3D  bl 0x825b60f8
	ctx.lr = 0x823B83C0;
	sub_825B60F8(ctx, base);
	// 823B83C0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823B83C4: 488F1090  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B83C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B83C8 size=456
    let mut pc: u32 = 0x823B83C8;
    'dispatch: loop {
        match pc {
            0x823B83C8 => {
    //   block [0x823B83C8..0x823B8590)
	// 823B83C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B83CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B83D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B83D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B83D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B83DC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B83E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B83E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823B83E8: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B83EC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B83F0: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B83F4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B83F8: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B83FC: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B8400: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B8404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8408: 419A0084  beq cr6, 0x823b848c
	if ctx.cr[6].eq {
	pc = 0x823B848C; continue 'dispatch;
	}
	// 823B840C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8410: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8414: 419A0074  beq cr6, 0x823b8488
	if ctx.cr[6].eq {
	pc = 0x823B8488; continue 'dispatch;
	}
	// 823B8418: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B841C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8420: 419A0018  beq cr6, 0x823b8438
	if ctx.cr[6].eq {
	pc = 0x823B8438; continue 'dispatch;
	}
	// 823B8424: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B8428: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B842C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8430: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8434: 409A0008  bne cr6, 0x823b843c
	if !ctx.cr[6].eq {
	pc = 0x823B843C; continue 'dispatch;
	}
	// 823B8438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B843C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B8440: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8444: 419A0134  beq cr6, 0x823b8578
	if ctx.cr[6].eq {
	pc = 0x823B8578; continue 'dispatch;
	}
	// 823B8448: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B844C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B8450: 55498FFE  rlwinm r9, r10, 0x11, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00007FFFu64;
	// 823B8454: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8458: 419A0104  beq cr6, 0x823b855c
	if ctx.cr[6].eq {
	pc = 0x823B855C; continue 'dispatch;
	}
	// 823B845C: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8460: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8464: 419A0030  beq cr6, 0x823b8494
	if ctx.cr[6].eq {
	pc = 0x823B8494; continue 'dispatch;
	}
	// 823B8468: 894A000F  lbz r10, 0xf(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(15 as u32) ) } as u64;
	// 823B846C: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8470: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B8474: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B8478: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B847C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8480: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8484: 480000DC  b 0x823b8560
	pc = 0x823B8560; continue 'dispatch;
	// 823B8488: 4BDDB9B1  bl 0x82193e38
	ctx.lr = 0x823B848C;
	sub_82193E38(ctx, base);
	// 823B848C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8490: 4BFFFFA8  b 0x823b8438
	pc = 0x823B8438; continue 'dispatch;
	// 823B8494: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8498: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B849C: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B84A0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B84A4: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B84A8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B84AC: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B84B0: 40810054  ble 0x823b8504
	if !ctx.cr[0].gt {
	pc = 0x823B8504; continue 'dispatch;
	}
	// 823B84B4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B84B8: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B84BC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B84C0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B84C4: 2F07000F  cmpwi cr6, r7, 0xf
	ctx.cr[6].compare_i32(ctx.r[7].s32, 15, &mut ctx.xer);
	// 823B84C8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B84CC: 41980008  blt cr6, 0x823b84d4
	if ctx.cr[6].lt {
	pc = 0x823B84D4; continue 'dispatch;
	}
	// 823B84D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B84D4: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B84D8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B84DC: 419A0014  beq cr6, 0x823b84f0
	if ctx.cr[6].eq {
	pc = 0x823B84F0; continue 'dispatch;
	}
	// 823B84E0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B84E4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B84E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B84EC: 4800000C  b 0x823b84f8
	pc = 0x823B84F8; continue 'dispatch;
	// 823B84F0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B84F4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B84F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B84FC: 4199FFB8  bgt cr6, 0x823b84b4
	if ctx.cr[6].gt {
	pc = 0x823B84B4; continue 'dispatch;
	}
	// 823B8500: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8504: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B8508: 419A0040  beq cr6, 0x823b8548
	if ctx.cr[6].eq {
	pc = 0x823B8548; continue 'dispatch;
	}
	// 823B850C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8510: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 823B8514: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8518: 41990008  bgt cr6, 0x823b8520
	if ctx.cr[6].gt {
	pc = 0x823B8520; continue 'dispatch;
	}
	// 823B851C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8520: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8528: 409A0020  bne cr6, 0x823b8548
	if !ctx.cr[6].eq {
	pc = 0x823B8548; continue 'dispatch;
	}
	// 823B852C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B8530: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B8534: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B8538: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B853C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8540: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8544: 4800001C  b 0x823b8560
	pc = 0x823B8560; continue 'dispatch;
	// 823B8548: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B854C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8550: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8554: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8558: 48000008  b 0x823b8560
	pc = 0x823B8560; continue 'dispatch;
	// 823B855C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8560: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8564: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8568: 419A0010  beq cr6, 0x823b8578
	if ctx.cr[6].eq {
	pc = 0x823B8578; continue 'dispatch;
	}
	// 823B856C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823B8570: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B8574: 4810720D  bl 0x824bf780
	ctx.lr = 0x823B8578;
	sub_824BF780(ctx, base);
	// 823B8578: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B857C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8584: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B858C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8590 size=764
    let mut pc: u32 = 0x823B8590;
    'dispatch: loop {
        match pc {
            0x823B8590 => {
    //   block [0x823B8590..0x823B888C)
	// 823B8590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8594: 488F0E69  bl 0x82ca93fc
	ctx.lr = 0x823B8598;
	sub_82CA93D0(ctx, base);
	// 823B8598: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B859C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B85A0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 823B85A4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 823B85A8: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B85AC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B85B0: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B85B4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B85B8: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B85BC: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B85C0: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B85C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B85C8: 419A001C  beq cr6, 0x823b85e4
	if ctx.cr[6].eq {
	pc = 0x823B85E4; continue 'dispatch;
	}
	// 823B85CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B85D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B85D4: 419A000C  beq cr6, 0x823b85e0
	if ctx.cr[6].eq {
	pc = 0x823B85E0; continue 'dispatch;
	}
	// 823B85D8: 5546003E  slwi r6, r10, 0
	ctx.r[6].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 823B85DC: 4800000C  b 0x823b85e8
	pc = 0x823B85E8; continue 'dispatch;
	// 823B85E0: 4BDDB859  bl 0x82193e38
	ctx.lr = 0x823B85E4;
	sub_82193E38(ctx, base);
	// 823B85E4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 823B85E8: 80860024  lwz r4, 0x24(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B85EC: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 823B85F0: 548B8FFE  rlwinm r11, r4, 0x11, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00007FFFu64;
	// 823B85F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B85F8: 419A00F4  beq cr6, 0x823b86ec
	if ctx.cr[6].eq {
	pc = 0x823B86EC; continue 'dispatch;
	}
	// 823B85FC: 8166008C  lwz r11, 0x8c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8600: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8604: 419A0024  beq cr6, 0x823b8628
	if ctx.cr[6].eq {
	pc = 0x823B8628; continue 'dispatch;
	}
	// 823B8608: 894B000F  lbz r10, 0xf(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(15 as u32) ) } as u64;
	// 823B860C: 81660048  lwz r11, 0x48(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8610: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B8614: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B8618: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B861C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 823B8620: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8624: 480000CC  b 0x823b86f0
	pc = 0x823B86F0; continue 'dispatch;
	// 823B8628: 81460048  lwz r10, 0x48(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B862C: 80A6004C  lwz r5, 0x4c(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B8630: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 823B8634: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 823B8638: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B863C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8640: 40810054  ble 0x823b8694
	if !ctx.cr[0].gt {
	pc = 0x823B8694; continue 'dispatch;
	}
	// 823B8644: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B8648: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B864C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B8650: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8654: 2F07000F  cmpwi cr6, r7, 0xf
	ctx.cr[6].compare_i32(ctx.r[7].s32, 15, &mut ctx.xer);
	// 823B8658: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B865C: 41980008  blt cr6, 0x823b8664
	if ctx.cr[6].lt {
	pc = 0x823B8664; continue 'dispatch;
	}
	// 823B8660: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 823B8664: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B8668: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B866C: 419A0014  beq cr6, 0x823b8680
	if ctx.cr[6].eq {
	pc = 0x823B8680; continue 'dispatch;
	}
	// 823B8670: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8674: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B8678: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B867C: 4800000C  b 0x823b8688
	pc = 0x823B8688; continue 'dispatch;
	// 823B8680: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8684: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B8688: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B868C: 4199FFB8  bgt cr6, 0x823b8644
	if ctx.cr[6].gt {
	pc = 0x823B8644; continue 'dispatch;
	}
	// 823B8690: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8694: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 823B8698: 419A0040  beq cr6, 0x823b86d8
	if ctx.cr[6].eq {
	pc = 0x823B86D8; continue 'dispatch;
	}
	// 823B869C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B86A0: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 823B86A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B86A8: 41990008  bgt cr6, 0x823b86b0
	if ctx.cr[6].gt {
	pc = 0x823B86B0; continue 'dispatch;
	}
	// 823B86AC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 823B86B0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B86B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B86B8: 409A0020  bne cr6, 0x823b86d8
	if !ctx.cr[6].eq {
	pc = 0x823B86D8; continue 'dispatch;
	}
	// 823B86BC: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B86C0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B86C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B86C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B86CC: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 823B86D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B86D4: 4800001C  b 0x823b86f0
	pc = 0x823B86F0; continue 'dispatch;
	// 823B86D8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 823B86DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B86E0: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 823B86E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B86E8: 48000008  b 0x823b86f0
	pc = 0x823B86F0; continue 'dispatch;
	// 823B86EC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 823B86F0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B86F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B86F8: 419A018C  beq cr6, 0x823b8884
	if ctx.cr[6].eq {
	pc = 0x823B8884; continue 'dispatch;
	}
	// 823B86FC: 548B9FFE  rlwinm r11, r4, 0x13, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00001FFFu64;
	// 823B8700: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 823B8704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8708: 419A00F0  beq cr6, 0x823b87f8
	if ctx.cr[6].eq {
	pc = 0x823B87F8; continue 'dispatch;
	}
	// 823B870C: 8166008C  lwz r11, 0x8c(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8710: 81460048  lwz r10, 0x48(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8718: 419A0020  beq cr6, 0x823b8738
	if ctx.cr[6].eq {
	pc = 0x823B8738; continue 'dispatch;
	}
	// 823B871C: 896B000D  lbz r11, 0xd(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 823B8720: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 823B8724: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B8728: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B872C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B8730: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8734: 480000C8  b 0x823b87fc
	pc = 0x823B87FC; continue 'dispatch;
	// 823B8738: 80C6004C  lwz r6, 0x4c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B873C: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 823B8740: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B8744: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8748: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B874C: 40810054  ble 0x823b87a0
	if !ctx.cr[0].gt {
	pc = 0x823B87A0; continue 'dispatch;
	}
	// 823B8750: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B8754: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B8758: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B875C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8760: 2F07000D  cmpwi cr6, r7, 0xd
	ctx.cr[6].compare_i32(ctx.r[7].s32, 13, &mut ctx.xer);
	// 823B8764: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B8768: 41980008  blt cr6, 0x823b8770
	if ctx.cr[6].lt {
	pc = 0x823B8770; continue 'dispatch;
	}
	// 823B876C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 823B8770: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B8774: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B8778: 419A0014  beq cr6, 0x823b878c
	if ctx.cr[6].eq {
	pc = 0x823B878C; continue 'dispatch;
	}
	// 823B877C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8780: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B8784: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B8788: 4800000C  b 0x823b8794
	pc = 0x823B8794; continue 'dispatch;
	// 823B878C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8790: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B8794: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8798: 4199FFB8  bgt cr6, 0x823b8750
	if ctx.cr[6].gt {
	pc = 0x823B8750; continue 'dispatch;
	}
	// 823B879C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B87A0: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B87A4: 419A0040  beq cr6, 0x823b87e4
	if ctx.cr[6].eq {
	pc = 0x823B87E4; continue 'dispatch;
	}
	// 823B87A8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B87AC: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 823B87B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B87B4: 41990008  bgt cr6, 0x823b87bc
	if ctx.cr[6].gt {
	pc = 0x823B87BC; continue 'dispatch;
	}
	// 823B87B8: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 823B87BC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B87C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B87C4: 409A0020  bne cr6, 0x823b87e4
	if !ctx.cr[6].eq {
	pc = 0x823B87E4; continue 'dispatch;
	}
	// 823B87C8: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B87CC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B87D0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B87D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B87D8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B87DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B87E0: 4800001C  b 0x823b87fc
	pc = 0x823B87FC; continue 'dispatch;
	// 823B87E4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B87E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B87EC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B87F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B87F4: 48000008  b 0x823b87fc
	pc = 0x823B87FC; continue 'dispatch;
	// 823B87F8: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 823B87FC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8800: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8804: 419A0080  beq cr6, 0x823b8884
	if ctx.cr[6].eq {
	pc = 0x823B8884; continue 'dispatch;
	}
	// 823B8808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B880C: 48139BC5  bl 0x824f23d0
	ctx.lr = 0x823B8810;
	sub_824F23D0(ctx, base);
	// 823B8810: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823B8814: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 823B8818: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823B881C: 40990068  ble cr6, 0x823b8884
	if !ctx.cr[6].gt {
	pc = 0x823B8884; continue 'dispatch;
	}
	// 823B8820: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 823B8824: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 823B8828: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823B882C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823B8830: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B8834: 4813951D  bl 0x824f1d50
	ctx.lr = 0x823B8838;
	sub_824F1D50(ctx, base);
	// 823B8838: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 823B883C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B8840: 419A0034  beq cr6, 0x823b8874
	if ctx.cr[6].eq {
	pc = 0x823B8874; continue 'dispatch;
	}
	// 823B8844: 897B003D  lbz r11, 0x3d(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(61 as u32) ) } as u64;
	// 823B8848: 572A063E  clrlwi r10, r25, 0x18
	ctx.r[10].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 823B884C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823B8850: 419A0014  beq cr6, 0x823b8864
	if ctx.cr[6].eq {
	pc = 0x823B8864; continue 'dispatch;
	}
	// 823B8854: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 823B8858: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 823B885C: 48106E65  bl 0x824bf6c0
	ctx.lr = 0x823B8860;
	sub_824BF6C0(ctx, base);
	// 823B8860: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 823B8864: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B8868: 419A000C  beq cr6, 0x823b8874
	if ctx.cr[6].eq {
	pc = 0x823B8874; continue 'dispatch;
	}
	// 823B886C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 823B8870: 483AC551  bl 0x82764dc0
	ctx.lr = 0x823B8874;
	sub_82764DC0(ctx, base);
	// 823B8874: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 823B8878: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 823B887C: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 823B8880: 4198FFA4  blt cr6, 0x823b8824
	if ctx.cr[6].lt {
	pc = 0x823B8824; continue 'dispatch;
	}
	// 823B8884: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 823B8888: 488F0BC4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8890 size=772
    let mut pc: u32 = 0x823B8890;
    'dispatch: loop {
        match pc {
            0x823B8890 => {
    //   block [0x823B8890..0x823B8B94)
	// 823B8890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8894: 488F0B69  bl 0x82ca93fc
	ctx.lr = 0x823B8898;
	sub_82CA93D0(ctx, base);
	// 823B8898: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 823B889C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B88A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B88A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B88A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B88AC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823B88B0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 823B88B4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 823B88B8: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B88BC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 823B88C0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823B88C4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B88C8: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B88CC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B88D0: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B88D4: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B88D8: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B88DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B88E0: 419A0080  beq cr6, 0x823b8960
	if ctx.cr[6].eq {
	pc = 0x823B8960; continue 'dispatch;
	}
	// 823B88E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B88E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B88EC: 419A0070  beq cr6, 0x823b895c
	if ctx.cr[6].eq {
	pc = 0x823B895C; continue 'dispatch;
	}
	// 823B88F0: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 823B88F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B88F8: 419A0018  beq cr6, 0x823b8910
	if ctx.cr[6].eq {
	pc = 0x823B8910; continue 'dispatch;
	}
	// 823B88FC: 897F0090  lbz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B8900: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B8904: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8908: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B890C: 409A0008  bne cr6, 0x823b8914
	if !ctx.cr[6].eq {
	pc = 0x823B8914; continue 'dispatch;
	}
	// 823B8910: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B8914: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B891C: 419A026C  beq cr6, 0x823b8b88
	if ctx.cr[6].eq {
	pc = 0x823B8B88; continue 'dispatch;
	}
	// 823B8920: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B8924: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B8928: 55499FFE  rlwinm r9, r10, 0x13, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00001FFFu64;
	// 823B892C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8930: 419A00F4  beq cr6, 0x823b8a24
	if ctx.cr[6].eq {
	pc = 0x823B8A24; continue 'dispatch;
	}
	// 823B8934: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B893C: 419A002C  beq cr6, 0x823b8968
	if ctx.cr[6].eq {
	pc = 0x823B8968; continue 'dispatch;
	}
	// 823B8940: 894B000D  lbz r10, 0xd(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 823B8944: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8948: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B894C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B8950: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8954: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8958: 480000D0  b 0x823b8a28
	pc = 0x823B8A28; continue 'dispatch;
	// 823B895C: 4BDDB4DD  bl 0x82193e38
	ctx.lr = 0x823B8960;
	sub_82193E38(ctx, base);
	// 823B8960: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 823B8964: 4BFFFFAC  b 0x823b8910
	pc = 0x823B8910; continue 'dispatch;
	// 823B8968: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B896C: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B8970: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B8974: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B8978: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B897C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8980: 40810054  ble 0x823b89d4
	if !ctx.cr[0].gt {
	pc = 0x823B89D4; continue 'dispatch;
	}
	// 823B8984: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B8988: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B898C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B8990: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8994: 2F07000D  cmpwi cr6, r7, 0xd
	ctx.cr[6].compare_i32(ctx.r[7].s32, 13, &mut ctx.xer);
	// 823B8998: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B899C: 41980008  blt cr6, 0x823b89a4
	if ctx.cr[6].lt {
	pc = 0x823B89A4; continue 'dispatch;
	}
	// 823B89A0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B89A4: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B89A8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B89AC: 419A0014  beq cr6, 0x823b89c0
	if ctx.cr[6].eq {
	pc = 0x823B89C0; continue 'dispatch;
	}
	// 823B89B0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B89B4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B89B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B89BC: 4800000C  b 0x823b89c8
	pc = 0x823B89C8; continue 'dispatch;
	// 823B89C0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B89C4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B89C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B89CC: 4199FFB8  bgt cr6, 0x823b8984
	if ctx.cr[6].gt {
	pc = 0x823B8984; continue 'dispatch;
	}
	// 823B89D0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B89D4: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B89D8: 419A003C  beq cr6, 0x823b8a14
	if ctx.cr[6].eq {
	pc = 0x823B8A14; continue 'dispatch;
	}
	// 823B89DC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B89E0: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 823B89E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B89E8: 41990008  bgt cr6, 0x823b89f0
	if ctx.cr[6].gt {
	pc = 0x823B89F0; continue 'dispatch;
	}
	// 823B89EC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B89F0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B89F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B89F8: 409A001C  bne cr6, 0x823b8a14
	if !ctx.cr[6].eq {
	pc = 0x823B8A14; continue 'dispatch;
	}
	// 823B89FC: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B8A00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8A04: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B8A08: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B8A0C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8A10: 48000018  b 0x823b8a28
	pc = 0x823B8A28; continue 'dispatch;
	// 823B8A14: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B8A18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8A1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8A20: 48000008  b 0x823b8a28
	pc = 0x823B8A28; continue 'dispatch;
	// 823B8A24: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 823B8A28: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B8A2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8A30: 419A0158  beq cr6, 0x823b8b88
	if ctx.cr[6].eq {
	pc = 0x823B8B88; continue 'dispatch;
	}
	// 823B8A34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823B8A38: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 823B8A3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B8A40: 481391D9  bl 0x824f1c18
	ctx.lr = 0x823B8A44;
	sub_824F1C18(ctx, base);
	// 823B8A44: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 823B8A48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B8A4C: 419A013C  beq cr6, 0x823b8b88
	if ctx.cr[6].eq {
	pc = 0x823B8B88; continue 'dispatch;
	}
	// 823B8A50: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B8A54: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B8A58: 55498FFE  rlwinm r9, r10, 0x11, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00007FFFu64;
	// 823B8A5C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8A60: 419A00E4  beq cr6, 0x823b8b44
	if ctx.cr[6].eq {
	pc = 0x823B8B44; continue 'dispatch;
	}
	// 823B8A64: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8A68: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8A6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8A70: 419A001C  beq cr6, 0x823b8a8c
	if ctx.cr[6].eq {
	pc = 0x823B8A8C; continue 'dispatch;
	}
	// 823B8A74: 896B000F  lbz r11, 0xf(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(15 as u32) ) } as u64;
	// 823B8A78: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 823B8A7C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B8A80: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8A84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8A88: 480000C0  b 0x823b8b48
	pc = 0x823B8B48; continue 'dispatch;
	// 823B8A8C: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B8A90: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823B8A94: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B8A98: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8A9C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8AA0: 40810054  ble 0x823b8af4
	if !ctx.cr[0].gt {
	pc = 0x823B8AF4; continue 'dispatch;
	}
	// 823B8AA4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B8AA8: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B8AAC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B8AB0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8AB4: 2F07000F  cmpwi cr6, r7, 0xf
	ctx.cr[6].compare_i32(ctx.r[7].s32, 15, &mut ctx.xer);
	// 823B8AB8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B8ABC: 41980008  blt cr6, 0x823b8ac4
	if ctx.cr[6].lt {
	pc = 0x823B8AC4; continue 'dispatch;
	}
	// 823B8AC0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 823B8AC4: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B8AC8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B8ACC: 419A0014  beq cr6, 0x823b8ae0
	if ctx.cr[6].eq {
	pc = 0x823B8AE0; continue 'dispatch;
	}
	// 823B8AD0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8AD4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B8AD8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B8ADC: 4800000C  b 0x823b8ae8
	pc = 0x823B8AE8; continue 'dispatch;
	// 823B8AE0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8AE4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B8AE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8AEC: 4199FFB8  bgt cr6, 0x823b8aa4
	if ctx.cr[6].gt {
	pc = 0x823B8AA4; continue 'dispatch;
	}
	// 823B8AF0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8AF4: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B8AF8: 419A003C  beq cr6, 0x823b8b34
	if ctx.cr[6].eq {
	pc = 0x823B8B34; continue 'dispatch;
	}
	// 823B8AFC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8B00: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 823B8B04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8B08: 41990008  bgt cr6, 0x823b8b10
	if ctx.cr[6].gt {
	pc = 0x823B8B10; continue 'dispatch;
	}
	// 823B8B0C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 823B8B10: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8B14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8B18: 409A001C  bne cr6, 0x823b8b34
	if !ctx.cr[6].eq {
	pc = 0x823B8B34; continue 'dispatch;
	}
	// 823B8B1C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B8B20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8B24: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B8B28: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B8B2C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8B30: 48000018  b 0x823b8b48
	pc = 0x823B8B48; continue 'dispatch;
	// 823B8B34: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B8B38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8B3C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8B40: 48000008  b 0x823b8b48
	pc = 0x823B8B48; continue 'dispatch;
	// 823B8B44: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 823B8B48: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B8B4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8B50: 419A0028  beq cr6, 0x823b8b78
	if ctx.cr[6].eq {
	pc = 0x823B8B78; continue 'dispatch;
	}
	// 823B8B54: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 823B8B58: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823B8B5C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 823B8B60: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 823B8B64: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823B8B68: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 823B8B6C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8B70: 48107829  bl 0x824c0398
	ctx.lr = 0x823B8B74;
	sub_824C0398(ctx, base);
	// 823B8B74: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 823B8B78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B8B7C: 419A000C  beq cr6, 0x823b8b88
	if ctx.cr[6].eq {
	pc = 0x823B8B88; continue 'dispatch;
	}
	// 823B8B80: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 823B8B84: 483AC23D  bl 0x82764dc0
	ctx.lr = 0x823B8B88;
	sub_82764DC0(ctx, base);
	// 823B8B88: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823B8B8C: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 823B8B90: 488F08BC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8B98 size=224
    let mut pc: u32 = 0x823B8B98;
    'dispatch: loop {
        match pc {
            0x823B8B98 => {
    //   block [0x823B8B98..0x823B8C78)
	// 823B8B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8B9C: 488F0869  bl 0x82ca9404
	ctx.lr = 0x823B8BA0;
	sub_82CA93D0(ctx, base);
	// 823B8BA0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 823B8BA4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8BA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823B8BAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823B8BB0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 823B8BB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8BB8: 4BDE3A31  bl 0x8219c5e8
	ctx.lr = 0x823B8BBC;
	sub_8219C5E8(ctx, base);
	// 823B8BBC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B8BC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8BC4: 419A00A8  beq cr6, 0x823b8c6c
	if ctx.cr[6].eq {
	pc = 0x823B8C6C; continue 'dispatch;
	}
	// 823B8BC8: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 823B8BCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8BD0: 38EBABB4  addi r7, r11, -0x544c
	ctx.r[7].s64 = ctx.r[11].s64 + -21580;
	// 823B8BD4: 4BDF9925  bl 0x821b24f8
	ctx.lr = 0x823B8BD8;
	sub_821B24F8(ctx, base);
	// 823B8BD8: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 823B8BDC: 4BEF0E0D  bl 0x822a99e8
	ctx.lr = 0x823B8BE0;
	sub_822A99E8(ctx, base);
	// 823B8BE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8BE4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823B8BE8: 419A0084  beq cr6, 0x823b8c6c
	if ctx.cr[6].eq {
	pc = 0x823B8C6C; continue 'dispatch;
	}
	// 823B8BEC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 823B8BF0: 388BABC4  addi r4, r11, -0x543c
	ctx.r[4].s64 = ctx.r[11].s64 + -21564;
	// 823B8BF4: 4BEF0DF5  bl 0x822a99e8
	ctx.lr = 0x823B8BF8;
	sub_822A99E8(ctx, base);
	// 823B8BF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B8BFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8C00: 419A006C  beq cr6, 0x823b8c6c
	if ctx.cr[6].eq {
	pc = 0x823B8C6C; continue 'dispatch;
	}
	// 823B8C04: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 823B8C08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8C0C: 388BABE0  addi r4, r11, -0x5420
	ctx.r[4].s64 = ctx.r[11].s64 + -21536;
	// 823B8C10: 4BDDDF71  bl 0x82196b80
	ctx.lr = 0x823B8C14;
	sub_82196B80(ctx, base);
	// 823B8C14: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 823B8C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8C1C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B8C20: 388AABD8  addi r4, r10, -0x5428
	ctx.r[4].s64 = ctx.r[10].s64 + -21544;
	// 823B8C24: 48006BFD  bl 0x823bf820
	ctx.lr = 0x823B8C28;
	sub_823BF820(ctx, base);
	// 823B8C28: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 823B8C2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8C30: 3889ABD0  addi r4, r9, -0x5430
	ctx.r[4].s64 = ctx.r[9].s64 + -21552;
	// 823B8C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8C38: 48006BE9  bl 0x823bf820
	ctx.lr = 0x823B8C3C;
	sub_823BF820(ctx, base);
	// 823B8C3C: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 823B8C40: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 823B8C44: 3888ABCC  addi r4, r8, -0x5434
	ctx.r[4].s64 = ctx.r[8].s64 + -21556;
	// 823B8C48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8C4C: 48006BD5  bl 0x823bf820
	ctx.lr = 0x823B8C50;
	sub_823BF820(ctx, base);
	// 823B8C50: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823B8C54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823B8C58: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823B8C5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823B8C60: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 823B8C64: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823B8C68: 4BFFFC29  bl 0x823b8890
	ctx.lr = 0x823B8C6C;
	sub_823B8890(ctx, base);
	// 823B8C6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823B8C70: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 823B8C74: 488F07E0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8C78 size=472
    let mut pc: u32 = 0x823B8C78;
    'dispatch: loop {
        match pc {
            0x823B8C78 => {
    //   block [0x823B8C78..0x823B8E50)
	// 823B8C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8C84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8C88: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B8C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B8C90: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B8C94: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B8C98: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B8C9C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8CA0: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8CA4: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B8CA8: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B8CAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8CB0: 419A0084  beq cr6, 0x823b8d34
	if ctx.cr[6].eq {
	pc = 0x823B8D34; continue 'dispatch;
	}
	// 823B8CB4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8CB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8CBC: 419A0074  beq cr6, 0x823b8d30
	if ctx.cr[6].eq {
	pc = 0x823B8D30; continue 'dispatch;
	}
	// 823B8CC0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B8CC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8CC8: 419A0018  beq cr6, 0x823b8ce0
	if ctx.cr[6].eq {
	pc = 0x823B8CE0; continue 'dispatch;
	}
	// 823B8CCC: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B8CD0: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B8CD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8CD8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8CDC: 409A0008  bne cr6, 0x823b8ce4
	if !ctx.cr[6].eq {
	pc = 0x823B8CE4; continue 'dispatch;
	}
	// 823B8CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B8CE4: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B8CE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8CEC: 419A0150  beq cr6, 0x823b8e3c
	if ctx.cr[6].eq {
	pc = 0x823B8E3C; continue 'dispatch;
	}
	// 823B8CF0: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B8CF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B8CF8: 55498FFE  rlwinm r9, r10, 0x11, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00007FFFu64;
	// 823B8CFC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8D00: 419A0104  beq cr6, 0x823b8e04
	if ctx.cr[6].eq {
	pc = 0x823B8E04; continue 'dispatch;
	}
	// 823B8D04: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8D08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8D0C: 419A0030  beq cr6, 0x823b8d3c
	if ctx.cr[6].eq {
	pc = 0x823B8D3C; continue 'dispatch;
	}
	// 823B8D10: 894A000F  lbz r10, 0xf(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(15 as u32) ) } as u64;
	// 823B8D14: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8D18: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B8D1C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B8D20: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8D24: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8D28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8D2C: 480000DC  b 0x823b8e08
	pc = 0x823B8E08; continue 'dispatch;
	// 823B8D30: 4BDDB109  bl 0x82193e38
	ctx.lr = 0x823B8D34;
	sub_82193E38(ctx, base);
	// 823B8D34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8D38: 4BFFFFA8  b 0x823b8ce0
	pc = 0x823B8CE0; continue 'dispatch;
	// 823B8D3C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8D40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B8D44: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B8D48: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B8D4C: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B8D50: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8D54: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8D58: 40810054  ble 0x823b8dac
	if !ctx.cr[0].gt {
	pc = 0x823B8DAC; continue 'dispatch;
	}
	// 823B8D5C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B8D60: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B8D64: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B8D68: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8D6C: 2F07000F  cmpwi cr6, r7, 0xf
	ctx.cr[6].compare_i32(ctx.r[7].s32, 15, &mut ctx.xer);
	// 823B8D70: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B8D74: 41980008  blt cr6, 0x823b8d7c
	if ctx.cr[6].lt {
	pc = 0x823B8D7C; continue 'dispatch;
	}
	// 823B8D78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B8D7C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B8D80: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B8D84: 419A0014  beq cr6, 0x823b8d98
	if ctx.cr[6].eq {
	pc = 0x823B8D98; continue 'dispatch;
	}
	// 823B8D88: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8D8C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B8D90: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B8D94: 4800000C  b 0x823b8da0
	pc = 0x823B8DA0; continue 'dispatch;
	// 823B8D98: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8D9C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B8DA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8DA4: 4199FFB8  bgt cr6, 0x823b8d5c
	if ctx.cr[6].gt {
	pc = 0x823B8D5C; continue 'dispatch;
	}
	// 823B8DA8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8DAC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B8DB0: 419A0040  beq cr6, 0x823b8df0
	if ctx.cr[6].eq {
	pc = 0x823B8DF0; continue 'dispatch;
	}
	// 823B8DB4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8DB8: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 823B8DBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8DC0: 41990008  bgt cr6, 0x823b8dc8
	if ctx.cr[6].gt {
	pc = 0x823B8DC8; continue 'dispatch;
	}
	// 823B8DC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8DC8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8DCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8DD0: 409A0020  bne cr6, 0x823b8df0
	if !ctx.cr[6].eq {
	pc = 0x823B8DF0; continue 'dispatch;
	}
	// 823B8DD4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B8DD8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B8DDC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B8DE0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8DE4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8DE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8DEC: 4800001C  b 0x823b8e08
	pc = 0x823B8E08; continue 'dispatch;
	// 823B8DF0: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B8DF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8DF8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8DFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8E00: 48000008  b 0x823b8e08
	pc = 0x823B8E08; continue 'dispatch;
	// 823B8E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8E08: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8E0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8E10: 419A002C  beq cr6, 0x823b8e3c
	if ctx.cr[6].eq {
	pc = 0x823B8E3C; continue 'dispatch;
	}
	// 823B8E14: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 823B8E18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8E1C: 419A001C  beq cr6, 0x823b8e38
	if ctx.cr[6].eq {
	pc = 0x823B8E38; continue 'dispatch;
	}
	// 823B8E20: 48109F69  bl 0x824c2d88
	ctx.lr = 0x823B8E24;
	sub_824C2D88(ctx, base);
	// 823B8E24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8E34: 4E800020  blr
	return;
	// 823B8E38: 48109B09  bl 0x824c2940
	ctx.lr = 0x823B8E3C;
	sub_824C2940(ctx, base);
	// 823B8E3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8E50 size=428
    let mut pc: u32 = 0x823B8E50;
    'dispatch: loop {
        match pc {
            0x823B8E50 => {
    //   block [0x823B8E50..0x823B8FFC)
	// 823B8E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8E5C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B8E60: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B8E64: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B8E68: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B8E6C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8E70: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8E74: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B8E78: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B8E7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8E80: 419A0084  beq cr6, 0x823b8f04
	if ctx.cr[6].eq {
	pc = 0x823B8F04; continue 'dispatch;
	}
	// 823B8E84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8E88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8E8C: 419A0074  beq cr6, 0x823b8f00
	if ctx.cr[6].eq {
	pc = 0x823B8F00; continue 'dispatch;
	}
	// 823B8E90: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B8E94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8E98: 419A0018  beq cr6, 0x823b8eb0
	if ctx.cr[6].eq {
	pc = 0x823B8EB0; continue 'dispatch;
	}
	// 823B8E9C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B8EA0: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B8EA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8EA8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8EAC: 409A0008  bne cr6, 0x823b8eb4
	if !ctx.cr[6].eq {
	pc = 0x823B8EB4; continue 'dispatch;
	}
	// 823B8EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B8EB4: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B8EB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8EBC: 419A0130  beq cr6, 0x823b8fec
	if ctx.cr[6].eq {
	pc = 0x823B8FEC; continue 'dispatch;
	}
	// 823B8EC0: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 823B8EC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B8EC8: 554967FE  rlwinm r9, r10, 0xc, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 823B8ECC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B8ED0: 419A0104  beq cr6, 0x823b8fd4
	if ctx.cr[6].eq {
	pc = 0x823B8FD4; continue 'dispatch;
	}
	// 823B8ED4: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B8ED8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B8EDC: 419A0030  beq cr6, 0x823b8f0c
	if ctx.cr[6].eq {
	pc = 0x823B8F0C; continue 'dispatch;
	}
	// 823B8EE0: 894A0074  lbz r10, 0x74(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(116 as u32) ) } as u64;
	// 823B8EE4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8EE8: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B8EEC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B8EF0: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8EF4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8EF8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8EFC: 480000DC  b 0x823b8fd8
	pc = 0x823B8FD8; continue 'dispatch;
	// 823B8F00: 4BDDAF39  bl 0x82193e38
	ctx.lr = 0x823B8F04;
	sub_82193E38(ctx, base);
	// 823B8F04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8F08: 4BFFFFA8  b 0x823b8eb0
	pc = 0x823B8EB0; continue 'dispatch;
	// 823B8F0C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B8F10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B8F14: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B8F18: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B8F1C: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B8F20: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8F24: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8F28: 40810054  ble 0x823b8f7c
	if !ctx.cr[0].gt {
	pc = 0x823B8F7C; continue 'dispatch;
	}
	// 823B8F2C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B8F30: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B8F34: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B8F38: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8F3C: 2F070074  cmpwi cr6, r7, 0x74
	ctx.cr[6].compare_i32(ctx.r[7].s32, 116, &mut ctx.xer);
	// 823B8F40: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B8F44: 41980008  blt cr6, 0x823b8f4c
	if ctx.cr[6].lt {
	pc = 0x823B8F4C; continue 'dispatch;
	}
	// 823B8F48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B8F4C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B8F50: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B8F54: 419A0014  beq cr6, 0x823b8f68
	if ctx.cr[6].eq {
	pc = 0x823B8F68; continue 'dispatch;
	}
	// 823B8F58: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B8F5C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B8F60: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B8F64: 4800000C  b 0x823b8f70
	pc = 0x823B8F70; continue 'dispatch;
	// 823B8F68: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B8F6C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B8F70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B8F74: 4199FFB8  bgt cr6, 0x823b8f2c
	if ctx.cr[6].gt {
	pc = 0x823B8F2C; continue 'dispatch;
	}
	// 823B8F78: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B8F7C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B8F80: 419A0040  beq cr6, 0x823b8fc0
	if ctx.cr[6].eq {
	pc = 0x823B8FC0; continue 'dispatch;
	}
	// 823B8F84: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8F88: 2F0B0074  cmpwi cr6, r11, 0x74
	ctx.cr[6].compare_i32(ctx.r[11].s32, 116, &mut ctx.xer);
	// 823B8F8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8F90: 41990008  bgt cr6, 0x823b8f98
	if ctx.cr[6].gt {
	pc = 0x823B8F98; continue 'dispatch;
	}
	// 823B8F94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8F98: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8F9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8FA0: 409A0020  bne cr6, 0x823b8fc0
	if !ctx.cr[6].eq {
	pc = 0x823B8FC0; continue 'dispatch;
	}
	// 823B8FA4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B8FA8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B8FAC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B8FB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8FB4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8FB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8FBC: 4800001C  b 0x823b8fd8
	pc = 0x823B8FD8; continue 'dispatch;
	// 823B8FC0: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B8FC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8FC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B8FCC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B8FD0: 48000008  b 0x823b8fd8
	pc = 0x823B8FD8; continue 'dispatch;
	// 823B8FD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8FD8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B8FDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8FE0: 419A000C  beq cr6, 0x823b8fec
	if ctx.cr[6].eq {
	pc = 0x823B8FEC; continue 'dispatch;
	}
	// 823B8FE4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 823B8FE8: 4823A059  bl 0x825f3040
	ctx.lr = 0x823B8FEC;
	sub_825F3040(ctx, base);
	// 823B8FEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823B8FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9000 size=516
    let mut pc: u32 = 0x823B9000;
    'dispatch: loop {
        match pc {
            0x823B9000 => {
    //   block [0x823B9000..0x823B9204)
	// 823B9000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B900C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9010: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B9014: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B9018: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B901C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9020: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9024: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9028: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B902C: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B9030: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B9034: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9038: 419A0084  beq cr6, 0x823b90bc
	if ctx.cr[6].eq {
	pc = 0x823B90BC; continue 'dispatch;
	}
	// 823B903C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9040: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9044: 419A0074  beq cr6, 0x823b90b8
	if ctx.cr[6].eq {
	pc = 0x823B90B8; continue 'dispatch;
	}
	// 823B9048: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B904C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9050: 419A0018  beq cr6, 0x823b9068
	if ctx.cr[6].eq {
	pc = 0x823B9068; continue 'dispatch;
	}
	// 823B9054: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B9058: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B905C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9060: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B9064: 409A0008  bne cr6, 0x823b906c
	if !ctx.cr[6].eq {
	pc = 0x823B906C; continue 'dispatch;
	}
	// 823B9068: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 823B906C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B9070: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9074: 419A017C  beq cr6, 0x823b91f0
	if ctx.cr[6].eq {
	pc = 0x823B91F0; continue 'dispatch;
	}
	// 823B9078: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B907C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9080: 55499FFE  rlwinm r9, r10, 0x13, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00001FFFu64;
	// 823B9084: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B9088: 419A0100  beq cr6, 0x823b9188
	if ctx.cr[6].eq {
	pc = 0x823B9188; continue 'dispatch;
	}
	// 823B908C: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B9090: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9094: 419A0030  beq cr6, 0x823b90c4
	if ctx.cr[6].eq {
	pc = 0x823B90C4; continue 'dispatch;
	}
	// 823B9098: 894A000D  lbz r10, 0xd(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(13 as u32) ) } as u64;
	// 823B909C: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B90A0: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B90A4: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B90A8: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B90AC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B90B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B90B4: 480000D8  b 0x823b918c
	pc = 0x823B918C; continue 'dispatch;
	// 823B90B8: 4BDDAD81  bl 0x82193e38
	ctx.lr = 0x823B90BC;
	sub_82193E38(ctx, base);
	// 823B90BC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B90C0: 4BFFFFA8  b 0x823b9068
	pc = 0x823B9068; continue 'dispatch;
	// 823B90C4: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B90C8: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B90CC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B90D0: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B90D4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B90D8: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B90DC: 40810054  ble 0x823b9130
	if !ctx.cr[0].gt {
	pc = 0x823B9130; continue 'dispatch;
	}
	// 823B90E0: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B90E4: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B90E8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B90EC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B90F0: 2F07000D  cmpwi cr6, r7, 0xd
	ctx.cr[6].compare_i32(ctx.r[7].s32, 13, &mut ctx.xer);
	// 823B90F4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B90F8: 41980008  blt cr6, 0x823b9100
	if ctx.cr[6].lt {
	pc = 0x823B9100; continue 'dispatch;
	}
	// 823B90FC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B9100: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B9104: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B9108: 419A0014  beq cr6, 0x823b911c
	if ctx.cr[6].eq {
	pc = 0x823B911C; continue 'dispatch;
	}
	// 823B910C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B9110: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B9114: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B9118: 4800000C  b 0x823b9124
	pc = 0x823B9124; continue 'dispatch;
	// 823B911C: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B9120: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B9124: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B9128: 4199FFB8  bgt cr6, 0x823b90e0
	if ctx.cr[6].gt {
	pc = 0x823B90E0; continue 'dispatch;
	}
	// 823B912C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B9130: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B9134: 419A0040  beq cr6, 0x823b9174
	if ctx.cr[6].eq {
	pc = 0x823B9174; continue 'dispatch;
	}
	// 823B9138: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B913C: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 823B9140: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9144: 41990008  bgt cr6, 0x823b914c
	if ctx.cr[6].gt {
	pc = 0x823B914C; continue 'dispatch;
	}
	// 823B9148: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B914C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9150: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9154: 409A0020  bne cr6, 0x823b9174
	if !ctx.cr[6].eq {
	pc = 0x823B9174; continue 'dispatch;
	}
	// 823B9158: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B915C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B9160: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B9164: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9168: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B916C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9170: 4800001C  b 0x823b918c
	pc = 0x823B918C; continue 'dispatch;
	// 823B9174: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B9178: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B917C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B9180: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9184: 48000008  b 0x823b918c
	pc = 0x823B918C; continue 'dispatch;
	// 823B9188: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B918C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9190: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9194: 419A005C  beq cr6, 0x823b91f0
	if ctx.cr[6].eq {
	pc = 0x823B91F0; continue 'dispatch;
	}
	// 823B9198: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 823B919C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 823B91A0: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 823B91A4: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 823B91A8: 48138CB9  bl 0x824f1e60
	ctx.lr = 0x823B91AC;
	sub_824F1E60(ctx, base);
	// 823B91AC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B91B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 823B91B4: 40990034  ble cr6, 0x823b91e8
	if !ctx.cr[6].gt {
	pc = 0x823B91E8; continue 'dispatch;
	}
	// 823B91B8: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 823B91BC: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823B91C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B91C4: 419A0014  beq cr6, 0x823b91d8
	if ctx.cr[6].eq {
	pc = 0x823B91D8; continue 'dispatch;
	}
	// 823B91C8: 7D494050  subf r10, r9, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 823B91CC: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 823B91D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823B91D4: 41980008  blt cr6, 0x823b91dc
	if ctx.cr[6].lt {
	pc = 0x823B91DC; continue 'dispatch;
	}
	// 823B91D8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823B91DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823B91E0: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 823B91E4: 4198FFDC  blt cr6, 0x823b91c0
	if ctx.cr[6].lt {
	pc = 0x823B91C0; continue 'dispatch;
	}
	// 823B91E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B91EC: 4BF5A2D5  bl 0x823134c0
	ctx.lr = 0x823B91F0;
	sub_823134C0(ctx, base);
	// 823B91F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B91F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B91F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B91FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9208 size=264
    let mut pc: u32 = 0x823B9208;
    'dispatch: loop {
        match pc {
            0x823B9208 => {
    //   block [0x823B9208..0x823B9310)
	// 823B9208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B920C: 488F01F5  bl 0x82ca9400
	ctx.lr = 0x823B9210;
	sub_82CA93D0(ctx, base);
	// 823B9210: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9214: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B9218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B921C: 4BDE33CD  bl 0x8219c5e8
	ctx.lr = 0x823B9220;
	sub_8219C5E8(ctx, base);
	// 823B9220: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B9224: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9228: 419A00E0  beq cr6, 0x823b9308
	if ctx.cr[6].eq {
	pc = 0x823B9308; continue 'dispatch;
	}
	// 823B922C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 823B9230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9234: 38EBABEC  addi r7, r11, -0x5414
	ctx.r[7].s64 = ctx.r[11].s64 + -21524;
	// 823B9238: 4BDF92C1  bl 0x821b24f8
	ctx.lr = 0x823B923C;
	sub_821B24F8(ctx, base);
	// 823B923C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 823B9240: 4BEF07A9  bl 0x822a99e8
	ctx.lr = 0x823B9244;
	sub_822A99E8(ctx, base);
	// 823B9244: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B9248: 419A00C0  beq cr6, 0x823b9308
	if ctx.cr[6].eq {
	pc = 0x823B9308; continue 'dispatch;
	}
	// 823B924C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 823B9250: 388BAC08  addi r4, r11, -0x53f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21496;
	// 823B9254: 4BEF0795  bl 0x822a99e8
	ctx.lr = 0x823B9258;
	sub_822A99E8(ctx, base);
	// 823B9258: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B925C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 823B9260: 419A00A8  beq cr6, 0x823b9308
	if ctx.cr[6].eq {
	pc = 0x823B9308; continue 'dispatch;
	}
	// 823B9264: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9268: 48647729  bl 0x82a00990
	ctx.lr = 0x823B926C;
	sub_82A00990(ctx, base);
	// 823B926C: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9270: 83810054  lwz r28, 0x54(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B9274: 83410050  lwz r26, 0x50(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823B9278: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 823B927C: 419A0084  beq cr6, 0x823b9300
	if ctx.cr[6].eq {
	pc = 0x823B9300; continue 'dispatch;
	}
	// 823B9280: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B9284: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 823B9288: 3FA08349  lis r29, -0x7cb7
	ctx.r[29].s64 = -2092367872;
	// 823B928C: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	// 823B9290: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823B9294: 817D6DA0  lwz r11, 0x6da0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28064 as u32) ) } as u64;
	// 823B9298: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 823B929C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823B92A0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B92A4: 7D6AD214  add r11, r10, r26
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 823B92A8: 8069000C  lwz r3, 0xc(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B92AC: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823B92B0: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B92B4: 48647E5D  bl 0x82a01110
	ctx.lr = 0x823B92B8;
	sub_82A01110(ctx, base);
	// 823B92B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B92BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B92C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 823B92C4: 4BE73C0D  bl 0x8222ced0
	ctx.lr = 0x823B92C8;
	sub_8222CED0(ctx, base);
	// 823B92C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B92CC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 823B92D0: 4BE0D499  bl 0x821c6768
	ctx.lr = 0x823B92D4;
	sub_821C6768(ctx, base);
	// 823B92D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823B92D8: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 823B92DC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823B92E0: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 823B92E4: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 823B92E8: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823B92EC: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823B92F0: 4082FFE8  bne 0x823b92d8
	if !ctx.cr[0].eq {
	pc = 0x823B92D8; continue 'dispatch;
	}
	// 823B92F4: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 823B92F8: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 823B92FC: 409AFF94  bne cr6, 0x823b9290
	if !ctx.cr[6].eq {
	pc = 0x823B9290; continue 'dispatch;
	}
	// 823B9300: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 823B9304: 4BE62A35  bl 0x8221bd38
	ctx.lr = 0x823B9308;
	sub_8221BD38(ctx, base);
	// 823B9308: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823B930C: 488F0144  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9310 size=120
    let mut pc: u32 = 0x823B9310;
    'dispatch: loop {
        match pc {
            0x823B9310 => {
    //   block [0x823B9310..0x823B9388)
	// 823B9310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9318: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B931C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9320: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 823B9324: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 823B9328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 823B932C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 823B9330: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 823B9334: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 823B9338: 3CE08349  lis r7, -0x7cb7
	ctx.r[7].s64 = -2092367872;
	// 823B933C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 823B9340: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B9344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 823B9348: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 823B934C: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 823B9350: 81676AB8  lwz r11, 0x6ab8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9354: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9358: 80660058  lwz r3, 0x58(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B935C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9360: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9364: 48006A7D  bl 0x823bfde0
	ctx.lr = 0x823B9368;
	sub_823BFDE0(ctx, base);
	// 823B9368: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 823B936C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B9370: 419A0008  beq cr6, 0x823b9378
	if ctx.cr[6].eq {
	pc = 0x823B9378; continue 'dispatch;
	}
	// 823B9374: 4BE629C5  bl 0x8221bd38
	ctx.lr = 0x823B9378;
	sub_8221BD38(ctx, base);
	// 823B9378: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823B937C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9388 size=112
    let mut pc: u32 = 0x823B9388;
    'dispatch: loop {
        match pc {
            0x823B9388 => {
    //   block [0x823B9388..0x823B93F8)
	// 823B9388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B938C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9394: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B9398: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B939C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B93A0: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B93A4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B93A8: 80E80008  lwz r7, 8(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 823B93AC: 80C70004  lwz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B93B0: 81660024  lwz r11, 0x24(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(36 as u32) ) } as u64;
	// 823B93B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B93B8: 419A0030  beq cr6, 0x823b93e8
	if ctx.cr[6].eq {
	pc = 0x823B93E8; continue 'dispatch;
	}
	// 823B93BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B93C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B93C4: 814B0098  lwz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 823B93C8: 812B009C  lwz r9, 0x9c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 823B93CC: 810B00A0  lwz r8, 0xa0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 823B93D0: 80EB00A4  lwz r7, 0xa4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 823B93D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B93D8: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 823B93DC: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 823B93E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 823B93E4: 4BFFFF2D  bl 0x823b9310
	ctx.lr = 0x823B93E8;
	sub_823B9310(ctx, base);
	// 823B93E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B93EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B93F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B93F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B93F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B93F8 size=468
    let mut pc: u32 = 0x823B93F8;
    'dispatch: loop {
        match pc {
            0x823B93F8 => {
    //   block [0x823B93F8..0x823B95CC)
	// 823B93F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B93FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9404: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9408: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B940C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B9410: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9414: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9418: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B941C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9420: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9424: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B9428: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B942C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9430: 419A0080  beq cr6, 0x823b94b0
	if ctx.cr[6].eq {
	pc = 0x823B94B0; continue 'dispatch;
	}
	// 823B9434: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9438: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B943C: 419A0070  beq cr6, 0x823b94ac
	if ctx.cr[6].eq {
	pc = 0x823B94AC; continue 'dispatch;
	}
	// 823B9440: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B9444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9448: 419A0018  beq cr6, 0x823b9460
	if ctx.cr[6].eq {
	pc = 0x823B9460; continue 'dispatch;
	}
	// 823B944C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B9450: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B9454: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9458: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B945C: 409A0008  bne cr6, 0x823b9464
	if !ctx.cr[6].eq {
	pc = 0x823B9464; continue 'dispatch;
	}
	// 823B9460: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 823B9464: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B9468: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B946C: 419A014C  beq cr6, 0x823b95b8
	if ctx.cr[6].eq {
	pc = 0x823B95B8; continue 'dispatch;
	}
	// 823B9470: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B9474: 5549FFFE  rlwinm r9, r10, 0x1f, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 823B9478: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B947C: 419A0100  beq cr6, 0x823b957c
	if ctx.cr[6].eq {
	pc = 0x823B957C; continue 'dispatch;
	}
	// 823B9480: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B9484: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9488: 419A0030  beq cr6, 0x823b94b8
	if ctx.cr[6].eq {
	pc = 0x823B94B8; continue 'dispatch;
	}
	// 823B948C: 894A0021  lbz r10, 0x21(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 823B9490: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B9494: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B9498: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B949C: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B94A0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B94A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B94A8: 480000D8  b 0x823b9580
	pc = 0x823B9580; continue 'dispatch;
	// 823B94AC: 4BDDA98D  bl 0x82193e38
	ctx.lr = 0x823B94B0;
	sub_82193E38(ctx, base);
	// 823B94B0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B94B4: 4BFFFFAC  b 0x823b9460
	pc = 0x823B9460; continue 'dispatch;
	// 823B94B8: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B94BC: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B94C0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B94C4: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B94C8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B94CC: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B94D0: 40810054  ble 0x823b9524
	if !ctx.cr[0].gt {
	pc = 0x823B9524; continue 'dispatch;
	}
	// 823B94D4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B94D8: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B94DC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B94E0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B94E4: 2F070021  cmpwi cr6, r7, 0x21
	ctx.cr[6].compare_i32(ctx.r[7].s32, 33, &mut ctx.xer);
	// 823B94E8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B94EC: 41980008  blt cr6, 0x823b94f4
	if ctx.cr[6].lt {
	pc = 0x823B94F4; continue 'dispatch;
	}
	// 823B94F0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B94F4: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B94F8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B94FC: 419A0014  beq cr6, 0x823b9510
	if ctx.cr[6].eq {
	pc = 0x823B9510; continue 'dispatch;
	}
	// 823B9500: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B9504: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B9508: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B950C: 4800000C  b 0x823b9518
	pc = 0x823B9518; continue 'dispatch;
	// 823B9510: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B9514: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B9518: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B951C: 4199FFB8  bgt cr6, 0x823b94d4
	if ctx.cr[6].gt {
	pc = 0x823B94D4; continue 'dispatch;
	}
	// 823B9520: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B9524: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B9528: 419A0040  beq cr6, 0x823b9568
	if ctx.cr[6].eq {
	pc = 0x823B9568; continue 'dispatch;
	}
	// 823B952C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9530: 2F0B0021  cmpwi cr6, r11, 0x21
	ctx.cr[6].compare_i32(ctx.r[11].s32, 33, &mut ctx.xer);
	// 823B9534: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9538: 41990008  bgt cr6, 0x823b9540
	if ctx.cr[6].gt {
	pc = 0x823B9540; continue 'dispatch;
	}
	// 823B953C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9540: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9544: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9548: 409A0020  bne cr6, 0x823b9568
	if !ctx.cr[6].eq {
	pc = 0x823B9568; continue 'dispatch;
	}
	// 823B954C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B9550: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B9554: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B9558: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B955C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B9560: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9564: 4800001C  b 0x823b9580
	pc = 0x823B9580; continue 'dispatch;
	// 823B9568: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B956C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9570: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B9574: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9578: 48000008  b 0x823b9580
	pc = 0x823B9580; continue 'dispatch;
	// 823B957C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9580: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9584: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9588: 419A0030  beq cr6, 0x823b95b8
	if ctx.cr[6].eq {
	pc = 0x823B95B8; continue 'dispatch;
	}
	// 823B958C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9590: 4BDFF309  bl 0x821b8898
	ctx.lr = 0x823B9594;
	sub_821B8898(ctx, base);
	// 823B9594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B9598: 419A0020  beq cr6, 0x823b95b8
	if ctx.cr[6].eq {
	pc = 0x823B95B8; continue 'dispatch;
	}
	// 823B959C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B95A0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823B95A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 823B95A8: 4E800421  bctrl
	ctx.lr = 0x823B95AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B95AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B95B0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 823B95B4: 4BFD071D  bl 0x82389cd0
	ctx.lr = 0x823B95B8;
	sub_82389CD0(ctx, base);
	// 823B95B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B95BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B95C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B95C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B95C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B95D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B95D0 size=72
    let mut pc: u32 = 0x823B95D0;
    'dispatch: loop {
        match pc {
            0x823B95D0 => {
    //   block [0x823B95D0..0x823B9618)
	// 823B95D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B95D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B95D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B95DC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B95E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B95E4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823B95E8: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B95EC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B95F0: 806A009C  lwz r3, 0x9c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(156 as u32) ) } as u64;
	// 823B95F4: 4BEAAC3D  bl 0x82264230
	ctx.lr = 0x823B95F8;
	sub_82264230(ctx, base);
	// 823B95F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B95FC: 419A000C  beq cr6, 0x823b9608
	if ctx.cr[6].eq {
	pc = 0x823B9608; continue 'dispatch;
	}
	// 823B9600: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 823B9604: 4BFD09B5  bl 0x82389fb8
	ctx.lr = 0x823B9608;
	sub_82389FB8(ctx, base);
	// 823B9608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823B960C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9618 size=184
    let mut pc: u32 = 0x823B9618;
    'dispatch: loop {
        match pc {
            0x823B9618 => {
    //   block [0x823B9618..0x823B96D0)
	// 823B9618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9620: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B96D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B96D0 size=12
    let mut pc: u32 = 0x823B96D0;
    'dispatch: loop {
        match pc {
            0x823B96D0 => {
    //   block [0x823B96D0..0x823B96DC)
	// 823B96D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B96D4: 986B6B6B  stb r3, 0x6b6b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27499 as u32), ctx.r[3].u8 ) };
	// 823B96D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B96E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B96E0 size=464
    let mut pc: u32 = 0x823B96E0;
    'dispatch: loop {
        match pc {
            0x823B96E0 => {
    //   block [0x823B96E0..0x823B98B0)
	// 823B96E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B96E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B96E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B96EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B96F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B96F4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B96F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B96FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823B9700: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9704: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9708: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B970C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9710: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9714: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B9718: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B971C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9720: 419A0084  beq cr6, 0x823b97a4
	if ctx.cr[6].eq {
	pc = 0x823B97A4; continue 'dispatch;
	}
	// 823B9724: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9728: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B972C: 419A0074  beq cr6, 0x823b97a0
	if ctx.cr[6].eq {
	pc = 0x823B97A0; continue 'dispatch;
	}
	// 823B9730: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B9734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9738: 419A0018  beq cr6, 0x823b9750
	if ctx.cr[6].eq {
	pc = 0x823B9750; continue 'dispatch;
	}
	// 823B973C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B9740: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823B9744: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9748: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B974C: 409A0008  bne cr6, 0x823b9754
	if !ctx.cr[6].eq {
	pc = 0x823B9754; continue 'dispatch;
	}
	// 823B9750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B9754: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823B9758: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B975C: 419A013C  beq cr6, 0x823b9898
	if ctx.cr[6].eq {
	pc = 0x823B9898; continue 'dispatch;
	}
	// 823B9760: 894B0028  lbz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B9764: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B9768: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 823B976C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823B9770: 419A0104  beq cr6, 0x823b9874
	if ctx.cr[6].eq {
	pc = 0x823B9874; continue 'dispatch;
	}
	// 823B9774: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B9778: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B977C: 419A0030  beq cr6, 0x823b97ac
	if ctx.cr[6].eq {
	pc = 0x823B97AC; continue 'dispatch;
	}
	// 823B9780: 894A0038  lbz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 823B9784: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B9788: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B978C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B9790: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9794: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B9798: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B979C: 480000DC  b 0x823b9878
	pc = 0x823B9878; continue 'dispatch;
	// 823B97A0: 4BDDA699  bl 0x82193e38
	ctx.lr = 0x823B97A4;
	sub_82193E38(ctx, base);
	// 823B97A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B97A8: 4BFFFFA8  b 0x823b9750
	pc = 0x823B9750; continue 'dispatch;
	// 823B97AC: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B97B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823B97B4: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B97B8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823B97BC: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B97C0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B97C4: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B97C8: 40810054  ble 0x823b981c
	if !ctx.cr[0].gt {
	pc = 0x823B981C; continue 'dispatch;
	}
	// 823B97CC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B97D0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B97D4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B97D8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B97DC: 2F070038  cmpwi cr6, r7, 0x38
	ctx.cr[6].compare_i32(ctx.r[7].s32, 56, &mut ctx.xer);
	// 823B97E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B97E4: 41980008  blt cr6, 0x823b97ec
	if ctx.cr[6].lt {
	pc = 0x823B97EC; continue 'dispatch;
	}
	// 823B97E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823B97EC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B97F0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B97F4: 419A0014  beq cr6, 0x823b9808
	if ctx.cr[6].eq {
	pc = 0x823B9808; continue 'dispatch;
	}
	// 823B97F8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B97FC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B9800: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B9804: 4800000C  b 0x823b9810
	pc = 0x823B9810; continue 'dispatch;
	// 823B9808: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B980C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B9810: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B9814: 4199FFB8  bgt cr6, 0x823b97cc
	if ctx.cr[6].gt {
	pc = 0x823B97CC; continue 'dispatch;
	}
	// 823B9818: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B981C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B9820: 419A0040  beq cr6, 0x823b9860
	if ctx.cr[6].eq {
	pc = 0x823B9860; continue 'dispatch;
	}
	// 823B9824: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9828: 2F0B0038  cmpwi cr6, r11, 0x38
	ctx.cr[6].compare_i32(ctx.r[11].s32, 56, &mut ctx.xer);
	// 823B982C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9830: 41990008  bgt cr6, 0x823b9838
	if ctx.cr[6].gt {
	pc = 0x823B9838; continue 'dispatch;
	}
	// 823B9834: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9838: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B983C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9840: 409A0020  bne cr6, 0x823b9860
	if !ctx.cr[6].eq {
	pc = 0x823B9860; continue 'dispatch;
	}
	// 823B9844: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B9848: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B984C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B9850: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9854: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B9858: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B985C: 4800001C  b 0x823b9878
	pc = 0x823B9878; continue 'dispatch;
	// 823B9860: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B9864: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9868: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823B986C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9870: 48000008  b 0x823b9878
	pc = 0x823B9878; continue 'dispatch;
	// 823B9874: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9878: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B987C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9880: 419A0018  beq cr6, 0x823b9898
	if ctx.cr[6].eq {
	pc = 0x823B9898; continue 'dispatch;
	}
	// 823B9884: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B9888: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 823B988C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823B9890: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823B9894: 4818869D  bl 0x82541f30
	ctx.lr = 0x823B9898;
	sub_82541F30(ctx, base);
	// 823B9898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B989C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B98A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B98A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B98A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B98AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B98B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B98B0 size=12
    let mut pc: u32 = 0x823B98B0;
    'dispatch: loop {
        match pc {
            0x823B98B0 => {
    //   block [0x823B98B0..0x823B98BC)
	// 823B98B0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B98B4: D02B9220  stfs f1, -0x6de0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28128 as u32), tmp.u32 ) };
	// 823B98B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B98C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B98C0 size=12
    let mut pc: u32 = 0x823B98C0;
    'dispatch: loop {
        match pc {
            0x823B98C0 => {
    //   block [0x823B98C0..0x823B98CC)
	// 823B98C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B98C4: 986B6BFE  stb r3, 0x6bfe(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27646 as u32), ctx.r[3].u8 ) };
	// 823B98C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B98D0 size=12
    let mut pc: u32 = 0x823B98D0;
    'dispatch: loop {
        match pc {
            0x823B98D0 => {
    //   block [0x823B98D0..0x823B98DC)
	// 823B98D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B98D4: D02B9224  stfs f1, -0x6ddc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28124 as u32), tmp.u32 ) };
	// 823B98D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B98E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B98E0 size=32
    let mut pc: u32 = 0x823B98E0;
    'dispatch: loop {
        match pc {
            0x823B98E0 => {
    //   block [0x823B98E0..0x823B9900)
	// 823B98E0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B98E4: 396B9484  addi r11, r11, -0x6b7c
	ctx.r[11].s64 = ctx.r[11].s64 + -27516;
	// 823B98E8: C00B000C  lfs f0, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B98EC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B98F0: 40990010  ble cr6, 0x823b9900
	if !ctx.cr[6].gt {
		sub_823B9900(ctx, base);
		return;
	}
	// 823B98F4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B98F8: D00B9230  stfs f0, -0x6dd0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28112 as u32), tmp.u32 ) };
	// 823B98FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9900 size=24
    let mut pc: u32 = 0x823B9900;
    'dispatch: loop {
        match pc {
            0x823B9900 => {
    //   block [0x823B9900..0x823B9918)
	// 823B9900: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B9904: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B9908: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B990C: 4198FFEC  blt cr6, 0x823b98f8
	if ctx.cr[6].lt {
		sub_823B98E0(ctx, base);
		return;
	}
	// 823B9910: D02B9230  stfs f1, -0x6dd0(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28112 as u32), tmp.u32 ) };
	// 823B9914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9918 size=12
    let mut pc: u32 = 0x823B9918;
    'dispatch: loop {
        match pc {
            0x823B9918 => {
    //   block [0x823B9918..0x823B9924)
	// 823B9918: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B991C: 986B924D  stb r3, -0x6db3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-28083 as u32), ctx.r[3].u8 ) };
	// 823B9920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9928 size=24
    let mut pc: u32 = 0x823B9928;
    'dispatch: loop {
        match pc {
            0x823B9928 => {
    //   block [0x823B9928..0x823B9940)
	// 823B9928: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 823B992C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B9930: C00ACB94  lfs f0, -0x346c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13420 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B9934: D02B8FE8  stfs f1, -0x7018(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28696 as u32), tmp.u32 ) };
	// 823B9938: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B993C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9940 size=8
    let mut pc: u32 = 0x823B9940;
    'dispatch: loop {
        match pc {
            0x823B9940 => {
    //   block [0x823B9940..0x823B9948)
	// 823B9940: D00B8FE8  stfs f0, -0x7018(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28696 as u32), tmp.u32 ) };
	// 823B9944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9948 size=12
    let mut pc: u32 = 0x823B9948;
    'dispatch: loop {
        match pc {
            0x823B9948 => {
    //   block [0x823B9948..0x823B9954)
	// 823B9948: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B994C: 986B924E  stb r3, -0x6db2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-28082 as u32), ctx.r[3].u8 ) };
	// 823B9950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9958 size=12
    let mut pc: u32 = 0x823B9958;
    'dispatch: loop {
        match pc {
            0x823B9958 => {
    //   block [0x823B9958..0x823B9964)
	// 823B9958: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B995C: D02B923C  stfs f1, -0x6dc4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28100 as u32), tmp.u32 ) };
	// 823B9960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9968 size=12
    let mut pc: u32 = 0x823B9968;
    'dispatch: loop {
        match pc {
            0x823B9968 => {
    //   block [0x823B9968..0x823B9974)
	// 823B9968: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B996C: D02B9240  stfs f1, -0x6dc0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28096 as u32), tmp.u32 ) };
	// 823B9970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9978 size=24
    let mut pc: u32 = 0x823B9978;
    'dispatch: loop {
        match pc {
            0x823B9978 => {
    //   block [0x823B9978..0x823B9990)
	// 823B9978: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 823B997C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B9980: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B9984: D02B9244  stfs f1, -0x6dbc(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28092 as u32), tmp.u32 ) };
	// 823B9988: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B998C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9990 size=8
    let mut pc: u32 = 0x823B9990;
    'dispatch: loop {
        match pc {
            0x823B9990 => {
    //   block [0x823B9990..0x823B9998)
	// 823B9990: D00B9244  stfs f0, -0x6dbc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28092 as u32), tmp.u32 ) };
	// 823B9994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9998 size=12
    let mut pc: u32 = 0x823B9998;
    'dispatch: loop {
        match pc {
            0x823B9998 => {
    //   block [0x823B9998..0x823B99A4)
	// 823B9998: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B999C: D02B9234  stfs f1, -0x6dcc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28108 as u32), tmp.u32 ) };
	// 823B99A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B99A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B99A8 size=12
    let mut pc: u32 = 0x823B99A8;
    'dispatch: loop {
        match pc {
            0x823B99A8 => {
    //   block [0x823B99A8..0x823B99B4)
	// 823B99A8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B99AC: D02B9238  stfs f1, -0x6dc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28104 as u32), tmp.u32 ) };
	// 823B99B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B99B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B99B8 size=12
    let mut pc: u32 = 0x823B99B8;
    'dispatch: loop {
        match pc {
            0x823B99B8 => {
    //   block [0x823B99B8..0x823B99C4)
	// 823B99B8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B99BC: D02B922C  stfs f1, -0x6dd4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28116 as u32), tmp.u32 ) };
	// 823B99C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B99C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B99C8 size=12
    let mut pc: u32 = 0x823B99C8;
    'dispatch: loop {
        match pc {
            0x823B99C8 => {
    //   block [0x823B99C8..0x823B99D4)
	// 823B99C8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B99CC: 906B9248  stw r3, -0x6db8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-28088 as u32), ctx.r[3].u32 ) };
	// 823B99D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B99D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B99D8 size=12
    let mut pc: u32 = 0x823B99D8;
    'dispatch: loop {
        match pc {
            0x823B99D8 => {
    //   block [0x823B99D8..0x823B99E4)
	// 823B99D8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B99DC: 986B918F  stb r3, -0x6e71(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-28273 as u32), ctx.r[3].u8 ) };
	// 823B99E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B99E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B99E8 size=12
    let mut pc: u32 = 0x823B99E8;
    'dispatch: loop {
        match pc {
            0x823B99E8 => {
    //   block [0x823B99E8..0x823B99F4)
	// 823B99E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B99EC: 986B6B30  stb r3, 0x6b30(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27440 as u32), ctx.r[3].u8 ) };
	// 823B99F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B99F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B99F8 size=12
    let mut pc: u32 = 0x823B99F8;
    'dispatch: loop {
        match pc {
            0x823B99F8 => {
    //   block [0x823B99F8..0x823B9A04)
	// 823B99F8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823B99FC: 986B924C  stb r3, -0x6db4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-28084 as u32), ctx.r[3].u8 ) };
	// 823B9A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B9A08 size=212
    let mut pc: u32 = 0x823B9A08;
    'dispatch: loop {
        match pc {
            0x823B9A08 => {
    //   block [0x823B9A08..0x823B9ADC)
	// 823B9A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9A10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9A14: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 823B9A18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9A1C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B9A20: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823B9A24: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9A28: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9A2C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9A30: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9A34: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9A38: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B9A3C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B9A40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9A44: 419A006C  beq cr6, 0x823b9ab0
	if ctx.cr[6].eq {
	pc = 0x823B9AB0; continue 'dispatch;
	}
	// 823B9A48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9A4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9A50: 419A005C  beq cr6, 0x823b9aac
	if ctx.cr[6].eq {
	pc = 0x823B9AAC; continue 'dispatch;
	}
	// 823B9A54: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 823B9A58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9A5C: 419A0018  beq cr6, 0x823b9a74
	if ctx.cr[6].eq {
	pc = 0x823B9A74; continue 'dispatch;
	}
	// 823B9A60: 897F0090  lbz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B9A64: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B9A68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9A6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9A70: 409A0008  bne cr6, 0x823b9a78
	if !ctx.cr[6].eq {
	pc = 0x823B9A78; continue 'dispatch;
	}
	// 823B9A74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9A78: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9A7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9A80: 419A0044  beq cr6, 0x823b9ac4
	if ctx.cr[6].eq {
	pc = 0x823B9AC4; continue 'dispatch;
	}
	// 823B9A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 823B9A88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9A8C: 396BD5C8  addi r11, r11, -0x2a38
	ctx.r[11].s64 = ctx.r[11].s64 + -10808;
	// 823B9A90: C02BBEC8  lfs f1, -0x4138(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16696 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823B9A94: C04B0000  lfs f2, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 823B9A98: FF1F0800  fcmpu cr6, f31, f1
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 823B9A9C: 419A001C  beq cr6, 0x823b9ab8
	if ctx.cr[6].eq {
	pc = 0x823B9AB8; continue 'dispatch;
	}
	// 823B9AA0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823B9AA4: 48432F95  bl 0x827eca38
	ctx.lr = 0x823B9AA8;
	sub_827ECA38(ctx, base);
	// 823B9AA8: 4800001C  b 0x823b9ac4
	pc = 0x823B9AC4; continue 'dispatch;
	// 823B9AAC: 4BDDA38D  bl 0x82193e38
	ctx.lr = 0x823B9AB0;
	sub_82193E38(ctx, base);
	// 823B9AB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B9AB4: 4BFFFFC0  b 0x823b9a74
	pc = 0x823B9A74; continue 'dispatch;
	// 823B9AB8: 48432F81  bl 0x827eca38
	ctx.lr = 0x823B9ABC;
	sub_827ECA38(ctx, base);
	// 823B9ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9AC0: 48432B91  bl 0x827ec650
	ctx.lr = 0x823B9AC4;
	sub_827EC650(ctx, base);
	// 823B9AC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9AD0: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9AD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B9AE0 size=916
    let mut pc: u32 = 0x823B9AE0;
    'dispatch: loop {
        match pc {
            0x823B9AE0 => {
    //   block [0x823B9AE0..0x823B9E74)
	// 823B9AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9AE4: 488EF929  bl 0x82ca940c
	ctx.lr = 0x823B9AE8;
	sub_82CA93D0(ctx, base);
	// 823B9AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9AEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9AF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823B9AF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B9AF8: 4098000C  bge cr6, 0x823b9b04
	if !ctx.cr[6].lt {
	pc = 0x823B9B04; continue 'dispatch;
	}
	// 823B9AFC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9B00: 48000010  b 0x823b9b10
	pc = 0x823B9B10; continue 'dispatch;
	// 823B9B04: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 823B9B08: 40990008  ble cr6, 0x823b9b10
	if !ctx.cr[6].gt {
	pc = 0x823B9B10; continue 'dispatch;
	}
	// 823B9B0C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 823B9B10: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 823B9B14: C00A9484  lfs f0, -0x6b7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B9B18: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B9B1C: 40980008  bge cr6, 0x823b9b24
	if !ctx.cr[6].lt {
	pc = 0x823B9B24; continue 'dispatch;
	}
	// 823B9B20: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 823B9B24: 3FA08349  lis r29, -0x7cb7
	ctx.r[29].s64 = -2092367872;
	// 823B9B28: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 823B9B2C: 1D2B002C  mulli r9, r11, 0x2c
	ctx.r[9].s64 = ctx.r[11].s64 * 44;
	// 823B9B30: 811D6AB8  lwz r8, 0x6ab8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9B34: 38EA92A0  addi r7, r10, -0x6d60
	ctx.r[7].s64 = ctx.r[10].s64 + -28000;
	// 823B9B38: 7C293D2E  stfsx f1, r9, r7
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823B9B3C: 80C8000C  lwz r6, 0xc(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9B40: 80A60058  lwz r5, 0x58(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9B44: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9B48: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9B4C: 386B0070  addi r3, r11, 0x70
	ctx.r[3].s64 = ctx.r[11].s64 + 112;
	// 823B9B50: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 823B9B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9B58: 419A0030  beq cr6, 0x823b9b88
	if ctx.cr[6].eq {
	pc = 0x823B9B88; continue 'dispatch;
	}
	// 823B9B5C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9B60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9B64: 419A008C  beq cr6, 0x823b9bf0
	if ctx.cr[6].eq {
	pc = 0x823B9BF0; continue 'dispatch;
	}
	// 823B9B68: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B9B6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9B70: 419A0018  beq cr6, 0x823b9b88
	if ctx.cr[6].eq {
	pc = 0x823B9B88; continue 'dispatch;
	}
	// 823B9B74: 896B0090  lbz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B9B78: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B9B7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9B80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9B84: 409A0008  bne cr6, 0x823b9b8c
	if !ctx.cr[6].eq {
	pc = 0x823B9B8C; continue 'dispatch;
	}
	// 823B9B88: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9B8C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9B90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9B94: 419A0140  beq cr6, 0x823b9cd4
	if ctx.cr[6].eq {
	pc = 0x823B9CD4; continue 'dispatch;
	}
	// 823B9B98: 817D6AB8  lwz r11, 0x6ab8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9B9C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 823B9BA0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9BA4: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9BA8: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9BAC: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9BB0: 4852BEC9  bl 0x828e5a78
	ctx.lr = 0x823B9BB4;
	sub_828E5A78(ctx, base);
	// 823B9BB4: 80E30028  lwz r7, 0x28(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B9BB8: 54E63FFE  rlwinm r6, r7, 7, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x01FFFFFFu64;
	// 823B9BBC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 823B9BC0: 419A00FC  beq cr6, 0x823b9cbc
	if ctx.cr[6].eq {
	pc = 0x823B9CBC; continue 'dispatch;
	}
	// 823B9BC4: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B9BC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9BCC: 419A002C  beq cr6, 0x823b9bf8
	if ctx.cr[6].eq {
	pc = 0x823B9BF8; continue 'dispatch;
	}
	// 823B9BD0: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B9BD4: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B9BD8: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B9BDC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B9BE0: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9BE4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B9BE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9BEC: 480000D4  b 0x823b9cc0
	pc = 0x823B9CC0; continue 'dispatch;
	// 823B9BF0: 4BDDA249  bl 0x82193e38
	ctx.lr = 0x823B9BF4;
	sub_82193E38(ctx, base);
	// 823B9BF4: 4BFFFF94  b 0x823b9b88
	pc = 0x823B9B88; continue 'dispatch;
	// 823B9BF8: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B9BFC: 80C3004C  lwz r6, 0x4c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B9C00: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B9C04: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B9C08: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B9C0C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B9C10: 40810054  ble 0x823b9c64
	if !ctx.cr[0].gt {
	pc = 0x823B9C64; continue 'dispatch;
	}
	// 823B9C14: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B9C18: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B9C1C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B9C20: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9C24: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B9C28: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B9C2C: 41980008  blt cr6, 0x823b9c34
	if ctx.cr[6].lt {
	pc = 0x823B9C34; continue 'dispatch;
	}
	// 823B9C30: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B9C34: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B9C38: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B9C3C: 419A0014  beq cr6, 0x823b9c50
	if ctx.cr[6].eq {
	pc = 0x823B9C50; continue 'dispatch;
	}
	// 823B9C40: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B9C44: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B9C48: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B9C4C: 4800000C  b 0x823b9c58
	pc = 0x823B9C58; continue 'dispatch;
	// 823B9C50: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B9C54: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B9C58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B9C5C: 4199FFB8  bgt cr6, 0x823b9c14
	if ctx.cr[6].gt {
	pc = 0x823B9C14; continue 'dispatch;
	}
	// 823B9C60: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B9C64: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B9C68: 419A0040  beq cr6, 0x823b9ca8
	if ctx.cr[6].eq {
	pc = 0x823B9CA8; continue 'dispatch;
	}
	// 823B9C6C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9C70: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B9C74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9C78: 41990008  bgt cr6, 0x823b9c80
	if ctx.cr[6].gt {
	pc = 0x823B9C80; continue 'dispatch;
	}
	// 823B9C7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9C80: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9C88: 409A0020  bne cr6, 0x823b9ca8
	if !ctx.cr[6].eq {
	pc = 0x823B9CA8; continue 'dispatch;
	}
	// 823B9C8C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B9C90: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B9C94: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B9C98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9C9C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B9CA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9CA4: 4800001C  b 0x823b9cc0
	pc = 0x823B9CC0; continue 'dispatch;
	// 823B9CA8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B9CAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9CB0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823B9CB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9CB8: 48000008  b 0x823b9cc0
	pc = 0x823B9CC0; continue 'dispatch;
	// 823B9CBC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9CC0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9CC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9CC8: 419A000C  beq cr6, 0x823b9cd4
	if ctx.cr[6].eq {
	pc = 0x823B9CD4; continue 'dispatch;
	}
	// 823B9CCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9CD0: 48127209  bl 0x824e0ed8
	ctx.lr = 0x823B9CD4;
	sub_824E0ED8(ctx, base);
	// 823B9CD4: 817D6AB8  lwz r11, 0x6ab8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9CD8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9CDC: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9CE0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9CE4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9CE8: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B9CEC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B9CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9CF4: 419A0030  beq cr6, 0x823b9d24
	if ctx.cr[6].eq {
	pc = 0x823B9D24; continue 'dispatch;
	}
	// 823B9CF8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9CFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9D00: 419A0088  beq cr6, 0x823b9d88
	if ctx.cr[6].eq {
	pc = 0x823B9D88; continue 'dispatch;
	}
	// 823B9D04: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823B9D08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9D0C: 419A0018  beq cr6, 0x823b9d24
	if ctx.cr[6].eq {
	pc = 0x823B9D24; continue 'dispatch;
	}
	// 823B9D10: 896B0090  lbz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823B9D14: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823B9D18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9D1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9D20: 409A0008  bne cr6, 0x823b9d28
	if !ctx.cr[6].eq {
	pc = 0x823B9D28; continue 'dispatch;
	}
	// 823B9D24: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9D28: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9D2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9D30: 419A013C  beq cr6, 0x823b9e6c
	if ctx.cr[6].eq {
	pc = 0x823B9E6C; continue 'dispatch;
	}
	// 823B9D34: 817D6AB8  lwz r11, 0x6ab8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9D38: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9D3C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9D40: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9D44: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9D48: 4BE72471  bl 0x8222c1b8
	ctx.lr = 0x823B9D4C;
	sub_8222C1B8(ctx, base);
	// 823B9D4C: 80E30028  lwz r7, 0x28(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 823B9D50: 54E63FFE  rlwinm r6, r7, 7, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x01FFFFFFu64;
	// 823B9D54: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 823B9D58: 419A00FC  beq cr6, 0x823b9e54
	if ctx.cr[6].eq {
	pc = 0x823B9E54; continue 'dispatch;
	}
	// 823B9D5C: 8163008C  lwz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 823B9D60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9D64: 419A002C  beq cr6, 0x823b9d90
	if ctx.cr[6].eq {
	pc = 0x823B9D90; continue 'dispatch;
	}
	// 823B9D68: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823B9D6C: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B9D70: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823B9D74: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823B9D78: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9D7C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B9D80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9D84: 480000D4  b 0x823b9e58
	pc = 0x823B9E58; continue 'dispatch;
	// 823B9D88: 4BDDA0B1  bl 0x82193e38
	ctx.lr = 0x823B9D8C;
	sub_82193E38(ctx, base);
	// 823B9D8C: 4BFFFF98  b 0x823b9d24
	pc = 0x823B9D24; continue 'dispatch;
	// 823B9D90: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 823B9D94: 80C3004C  lwz r6, 0x4c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 823B9D98: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823B9D9C: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823B9DA0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B9DA4: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B9DA8: 40810054  ble 0x823b9dfc
	if !ctx.cr[0].gt {
	pc = 0x823B9DFC; continue 'dispatch;
	}
	// 823B9DAC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823B9DB0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823B9DB4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823B9DB8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9DBC: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823B9DC0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823B9DC4: 41980008  blt cr6, 0x823b9dcc
	if ctx.cr[6].lt {
	pc = 0x823B9DCC; continue 'dispatch;
	}
	// 823B9DC8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823B9DCC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823B9DD0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823B9DD4: 419A0014  beq cr6, 0x823b9de8
	if ctx.cr[6].eq {
	pc = 0x823B9DE8; continue 'dispatch;
	}
	// 823B9DD8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823B9DDC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823B9DE0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823B9DE4: 4800000C  b 0x823b9df0
	pc = 0x823B9DF0; continue 'dispatch;
	// 823B9DE8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823B9DEC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823B9DF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B9DF4: 4199FFB8  bgt cr6, 0x823b9dac
	if ctx.cr[6].gt {
	pc = 0x823B9DAC; continue 'dispatch;
	}
	// 823B9DF8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823B9DFC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823B9E00: 419A0040  beq cr6, 0x823b9e40
	if ctx.cr[6].eq {
	pc = 0x823B9E40; continue 'dispatch;
	}
	// 823B9E04: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9E08: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823B9E0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9E10: 41990008  bgt cr6, 0x823b9e18
	if ctx.cr[6].gt {
	pc = 0x823B9E18; continue 'dispatch;
	}
	// 823B9E14: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9E18: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9E1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9E20: 409A0020  bne cr6, 0x823b9e40
	if !ctx.cr[6].eq {
	pc = 0x823B9E40; continue 'dispatch;
	}
	// 823B9E24: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823B9E28: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823B9E2C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B9E30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9E34: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B9E38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9E3C: 4800001C  b 0x823b9e58
	pc = 0x823B9E58; continue 'dispatch;
	// 823B9E40: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823B9E44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9E48: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823B9E4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823B9E50: 48000008  b 0x823b9e58
	pc = 0x823B9E58; continue 'dispatch;
	// 823B9E54: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823B9E58: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823B9E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9E60: 419A000C  beq cr6, 0x823b9e6c
	if ctx.cr[6].eq {
	pc = 0x823B9E6C; continue 'dispatch;
	}
	// 823B9E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9E68: 48127071  bl 0x824e0ed8
	ctx.lr = 0x823B9E6C;
	sub_824E0ED8(ctx, base);
	// 823B9E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B9E70: 488EF5EC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9E78 size=24
    let mut pc: u32 = 0x823B9E78;
    'dispatch: loop {
        match pc {
            0x823B9E78 => {
    //   block [0x823B9E78..0x823B9E90)
	// 823B9E78: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 823B9E7C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9E80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 823B9E84: 4098000C  bge cr6, 0x823b9e90
	if !ctx.cr[6].lt {
		sub_823B9E90(ctx, base);
		return;
	}
	// 823B9E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9E8C: 48000010  b 0x823b9e9c
	sub_823B9E90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9E90 size=28
    let mut pc: u32 = 0x823B9E90;
    'dispatch: loop {
        match pc {
            0x823B9E90 => {
    //   block [0x823B9E90..0x823B9EAC)
	// 823B9E90: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 823B9E94: 40990008  ble cr6, 0x823b9e9c
	if !ctx.cr[6].gt {
	pc = 0x823B9E9C; continue 'dispatch;
	}
	// 823B9E98: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 823B9E9C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 823B9EA0: 4098000C  bge cr6, 0x823b9eac
	if !ctx.cr[6].lt {
		sub_823B9EAC(ctx, base);
		return;
	}
	// 823B9EA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B9EA8: 48000010  b 0x823b9eb8
	sub_823B9EAC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9EAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9EAC size=64
    let mut pc: u32 = 0x823B9EAC;
    'dispatch: loop {
        match pc {
            0x823B9EAC => {
    //   block [0x823B9EAC..0x823B9EEC)
	// 823B9EAC: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 823B9EB0: 40990008  ble cr6, 0x823b9eb8
	if !ctx.cr[6].gt {
	pc = 0x823B9EB8; continue 'dispatch;
	}
	// 823B9EB4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 823B9EB8: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 823B9EBC: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B9EC0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B9EC4: 40980008  bge cr6, 0x823b9ecc
	if !ctx.cr[6].lt {
	pc = 0x823B9ECC; continue 'dispatch;
	}
	// 823B9EC8: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 823B9ECC: 1D6B000B  mulli r11, r11, 0xb
	ctx.r[11].s64 = ctx.r[11].s64 * 11;
	// 823B9ED0: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 823B9ED4: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B9ED8: 396992A0  addi r11, r9, -0x6d60
	ctx.r[11].s64 = ctx.r[9].s64 + -28000;
	// 823B9EDC: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 823B9EE0: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 823B9EE4: 7C27352E  stfsx f1, r7, r6
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 823B9EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9EF0 size=24
    let mut pc: u32 = 0x823B9EF0;
    'dispatch: loop {
        match pc {
            0x823B9EF0 => {
    //   block [0x823B9EF0..0x823B9F08)
	// 823B9EF0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 823B9EF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9EF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 823B9EFC: 4098000C  bge cr6, 0x823b9f08
	if !ctx.cr[6].lt {
		sub_823B9F08(ctx, base);
		return;
	}
	// 823B9F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9F04: 48000010  b 0x823b9f14
	sub_823B9F08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9F08 size=28
    let mut pc: u32 = 0x823B9F08;
    'dispatch: loop {
        match pc {
            0x823B9F08 => {
    //   block [0x823B9F08..0x823B9F24)
	// 823B9F08: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 823B9F0C: 40990008  ble cr6, 0x823b9f14
	if !ctx.cr[6].gt {
	pc = 0x823B9F14; continue 'dispatch;
	}
	// 823B9F10: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 823B9F14: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 823B9F18: 4098000C  bge cr6, 0x823b9f24
	if !ctx.cr[6].lt {
		sub_823B9F24(ctx, base);
		return;
	}
	// 823B9F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B9F20: 48000010  b 0x823b9f30
	sub_823B9F24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9F24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B9F24 size=64
    let mut pc: u32 = 0x823B9F24;
    'dispatch: loop {
        match pc {
            0x823B9F24 => {
    //   block [0x823B9F24..0x823B9F64)
	// 823B9F24: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 823B9F28: 40990008  ble cr6, 0x823b9f30
	if !ctx.cr[6].gt {
	pc = 0x823B9F30; continue 'dispatch;
	}
	// 823B9F2C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 823B9F30: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 823B9F34: C0099484  lfs f0, -0x6b7c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B9F38: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823B9F3C: 40980008  bge cr6, 0x823b9f44
	if !ctx.cr[6].lt {
	pc = 0x823B9F44; continue 'dispatch;
	}
	// 823B9F40: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 823B9F44: 1D6B000B  mulli r11, r11, 0xb
	ctx.r[11].s64 = ctx.r[11].s64 * 11;
	// 823B9F48: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 823B9F4C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B9F50: 396992A0  addi r11, r9, -0x6d60
	ctx.r[11].s64 = ctx.r[9].s64 + -28000;
	// 823B9F54: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 823B9F58: 38CB0018  addi r6, r11, 0x18
	ctx.r[6].s64 = ctx.r[11].s64 + 24;
	// 823B9F5C: 7C27352E  stfsx f1, r7, r6
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 823B9F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9F68 size=108
    let mut pc: u32 = 0x823B9F68;
    'dispatch: loop {
        match pc {
            0x823B9F68 => {
    //   block [0x823B9F68..0x823B9FD4)
	// 823B9F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9F74: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823B9F78: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823B9F7C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823B9F80: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823B9F84: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9F88: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9F8C: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823B9F90: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823B9F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9F98: 419A001C  beq cr6, 0x823b9fb4
	if ctx.cr[6].eq {
	pc = 0x823B9FB4; continue 'dispatch;
	}
	// 823B9F9C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9FA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823B9FA4: 419A000C  beq cr6, 0x823b9fb0
	if ctx.cr[6].eq {
	pc = 0x823B9FB0; continue 'dispatch;
	}
	// 823B9FA8: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823B9FAC: 4800000C  b 0x823b9fb8
	pc = 0x823B9FB8; continue 'dispatch;
	// 823B9FB0: 4BDD9E89  bl 0x82193e38
	ctx.lr = 0x823B9FB4;
	sub_82193E38(ctx, base);
	// 823B9FB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B9FB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B9FBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823B9FC0: 4823D359  bl 0x825f7318
	ctx.lr = 0x823B9FC4;
	sub_825F7318(ctx, base);
	// 823B9FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823B9FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B9FD8 size=4
    let mut pc: u32 = 0x823B9FD8;
    'dispatch: loop {
        match pc {
            0x823B9FD8 => {
    //   block [0x823B9FD8..0x823B9FDC)
	// 823B9FD8: 4823DCE8  b 0x825f7cc0
	sub_825F7CC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9FE0 size=652
    let mut pc: u32 = 0x823B9FE0;
    'dispatch: loop {
        match pc {
            0x823B9FE0 => {
    //   block [0x823B9FE0..0x823BA26C)
	// 823B9FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9FE4: 488EF425  bl 0x82ca9408
	ctx.lr = 0x823B9FE8;
	sub_82CA93D0(ctx, base);
	// 823B9FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9FEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823B9FF0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 823B9FF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 823B9FF8: 419A0018  beq cr6, 0x823ba010
	if ctx.cr[6].eq {
	pc = 0x823BA010; continue 'dispatch;
	}
	// 823B9FFC: 897D0090  lbz r11, 0x90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 823BA000: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823BA004: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA008: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA00C: 409A0008  bne cr6, 0x823ba014
	if !ctx.cr[6].eq {
	pc = 0x823BA014; continue 'dispatch;
	}
	// 823BA010: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 823BA014: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BA018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA01C: 419A0248  beq cr6, 0x823ba264
	if ctx.cr[6].eq {
	pc = 0x823BA264; continue 'dispatch;
	}
	// 823BA020: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 823BA024: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 823BA028: 556A3FFE  rlwinm r10, r11, 7, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x01FFFFFFu64;
	// 823BA02C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA030: 419A00F4  beq cr6, 0x823ba124
	if ctx.cr[6].eq {
	pc = 0x823BA124; continue 'dispatch;
	}
	// 823BA034: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 823BA038: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA03C: 419A0024  beq cr6, 0x823ba060
	if ctx.cr[6].eq {
	pc = 0x823BA060; continue 'dispatch;
	}
	// 823BA040: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 823BA044: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BA048: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823BA04C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823BA050: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA054: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823BA058: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA05C: 480000CC  b 0x823ba128
	pc = 0x823BA128; continue 'dispatch;
	// 823BA060: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BA064: 80DD004C  lwz r6, 0x4c(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 823BA068: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 823BA06C: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823BA070: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BA074: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BA078: 40810054  ble 0x823ba0cc
	if !ctx.cr[0].gt {
	pc = 0x823BA0CC; continue 'dispatch;
	}
	// 823BA07C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823BA080: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823BA084: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823BA088: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA08C: 2F070039  cmpwi cr6, r7, 0x39
	ctx.cr[6].compare_i32(ctx.r[7].s32, 57, &mut ctx.xer);
	// 823BA090: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823BA094: 41980008  blt cr6, 0x823ba09c
	if ctx.cr[6].lt {
	pc = 0x823BA09C; continue 'dispatch;
	}
	// 823BA098: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 823BA09C: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823BA0A0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823BA0A4: 419A0014  beq cr6, 0x823ba0b8
	if ctx.cr[6].eq {
	pc = 0x823BA0B8; continue 'dispatch;
	}
	// 823BA0A8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823BA0AC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823BA0B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823BA0B4: 4800000C  b 0x823ba0c0
	pc = 0x823BA0C0; continue 'dispatch;
	// 823BA0B8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823BA0BC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823BA0C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BA0C4: 4199FFB8  bgt cr6, 0x823ba07c
	if ctx.cr[6].gt {
	pc = 0x823BA07C; continue 'dispatch;
	}
	// 823BA0C8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BA0CC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823BA0D0: 419A0040  beq cr6, 0x823ba110
	if ctx.cr[6].eq {
	pc = 0x823BA110; continue 'dispatch;
	}
	// 823BA0D4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA0D8: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 823BA0DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA0E0: 41990008  bgt cr6, 0x823ba0e8
	if ctx.cr[6].gt {
	pc = 0x823BA0E8; continue 'dispatch;
	}
	// 823BA0E4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 823BA0E8: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BA0EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA0F0: 409A0020  bne cr6, 0x823ba110
	if !ctx.cr[6].eq {
	pc = 0x823BA110; continue 'dispatch;
	}
	// 823BA0F4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823BA0F8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823BA0FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BA100: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA104: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823BA108: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA10C: 4800001C  b 0x823ba128
	pc = 0x823BA128; continue 'dispatch;
	// 823BA110: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823BA114: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA118: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 823BA11C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA120: 48000008  b 0x823ba128
	pc = 0x823BA128; continue 'dispatch;
	// 823BA124: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 823BA128: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BA12C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA130: 419A0028  beq cr6, 0x823ba158
	if ctx.cr[6].eq {
	pc = 0x823BA158; continue 'dispatch;
	}
	// 823BA134: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
	// 823BA138: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 823BA13C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 823BA140: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BA144: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA148: 48128DE1  bl 0x824e2f28
	ctx.lr = 0x823BA14C;
	sub_824E2F28(ctx, base);
	// 823BA14C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 823BA150: 2F1F000E  cmpwi cr6, r31, 0xe
	ctx.cr[6].compare_i32(ctx.r[31].s32, 14, &mut ctx.xer);
	// 823BA154: 4099FFE4  ble cr6, 0x823ba138
	if !ctx.cr[6].gt {
	pc = 0x823BA138; continue 'dispatch;
	}
	// 823BA158: 895D0028  lbz r10, 0x28(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 823BA15C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 823BA160: 554907FE  clrlwi r9, r10, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 823BA164: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BA168: 419A00E4  beq cr6, 0x823ba24c
	if ctx.cr[6].eq {
	pc = 0x823BA24C; continue 'dispatch;
	}
	// 823BA16C: 817D008C  lwz r11, 0x8c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(140 as u32) ) } as u64;
	// 823BA170: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BA174: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA178: 419A001C  beq cr6, 0x823ba194
	if ctx.cr[6].eq {
	pc = 0x823BA194; continue 'dispatch;
	}
	// 823BA17C: 896B0038  lbz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 823BA180: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 823BA184: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823BA188: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA18C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BA190: 480000C0  b 0x823ba250
	pc = 0x823BA250; continue 'dispatch;
	// 823BA194: 80DD004C  lwz r6, 0x4c(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 823BA198: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 823BA19C: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823BA1A0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BA1A4: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BA1A8: 40810054  ble 0x823ba1fc
	if !ctx.cr[0].gt {
	pc = 0x823BA1FC; continue 'dispatch;
	}
	// 823BA1AC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823BA1B0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823BA1B4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823BA1B8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA1BC: 2F070038  cmpwi cr6, r7, 0x38
	ctx.cr[6].compare_i32(ctx.r[7].s32, 56, &mut ctx.xer);
	// 823BA1C0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823BA1C4: 41980008  blt cr6, 0x823ba1cc
	if ctx.cr[6].lt {
	pc = 0x823BA1CC; continue 'dispatch;
	}
	// 823BA1C8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 823BA1CC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823BA1D0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823BA1D4: 419A0014  beq cr6, 0x823ba1e8
	if ctx.cr[6].eq {
	pc = 0x823BA1E8; continue 'dispatch;
	}
	// 823BA1D8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823BA1DC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823BA1E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823BA1E4: 4800000C  b 0x823ba1f0
	pc = 0x823BA1F0; continue 'dispatch;
	// 823BA1E8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823BA1EC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823BA1F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BA1F4: 4199FFB8  bgt cr6, 0x823ba1ac
	if ctx.cr[6].gt {
	pc = 0x823BA1AC; continue 'dispatch;
	}
	// 823BA1F8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BA1FC: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823BA200: 419A003C  beq cr6, 0x823ba23c
	if ctx.cr[6].eq {
	pc = 0x823BA23C; continue 'dispatch;
	}
	// 823BA204: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA208: 2F0B0038  cmpwi cr6, r11, 0x38
	ctx.cr[6].compare_i32(ctx.r[11].s32, 56, &mut ctx.xer);
	// 823BA20C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA210: 41990008  bgt cr6, 0x823ba218
	if ctx.cr[6].gt {
	pc = 0x823BA218; continue 'dispatch;
	}
	// 823BA214: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 823BA218: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BA21C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA220: 409A001C  bne cr6, 0x823ba23c
	if !ctx.cr[6].eq {
	pc = 0x823BA23C; continue 'dispatch;
	}
	// 823BA224: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823BA228: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BA22C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823BA230: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BA234: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA238: 48000018  b 0x823ba250
	pc = 0x823BA250; continue 'dispatch;
	// 823BA23C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823BA240: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BA244: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA248: 48000008  b 0x823ba250
	pc = 0x823BA250; continue 'dispatch;
	// 823BA24C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 823BA250: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823BA254: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA258: 419A000C  beq cr6, 0x823ba264
	if ctx.cr[6].eq {
	pc = 0x823BA264; continue 'dispatch;
	}
	// 823BA25C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823BA260: 48187D79  bl 0x82541fd8
	ctx.lr = 0x823BA264;
	sub_82541FD8(ctx, base);
	// 823BA264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BA268: 488EF1F0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA270 size=116
    let mut pc: u32 = 0x823BA270;
    'dispatch: loop {
        match pc {
            0x823BA270 => {
    //   block [0x823BA270..0x823BA2E4)
	// 823BA270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA27C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BA280: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA284: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA288: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA28C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA290: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA294: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823BA298: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823BA29C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA2A0: 419A002C  beq cr6, 0x823ba2cc
	if ctx.cr[6].eq {
	pc = 0x823BA2CC; continue 'dispatch;
	}
	// 823BA2A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA2A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA2AC: 419A001C  beq cr6, 0x823ba2c8
	if ctx.cr[6].eq {
	pc = 0x823BA2C8; continue 'dispatch;
	}
	// 823BA2B0: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823BA2B4: 4BFFFD2D  bl 0x823b9fe0
	ctx.lr = 0x823BA2B8;
	sub_823B9FE0(ctx, base);
	// 823BA2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA2C4: 4E800020  blr
	return;
	// 823BA2C8: 4BDD9B71  bl 0x82193e38
	ctx.lr = 0x823BA2CC;
	sub_82193E38(ctx, base);
	// 823BA2CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823BA2D0: 4BFFFD11  bl 0x823b9fe0
	ctx.lr = 0x823BA2D4;
	sub_823B9FE0(ctx, base);
	// 823BA2D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA2E8 size=116
    let mut pc: u32 = 0x823BA2E8;
    'dispatch: loop {
        match pc {
            0x823BA2E8 => {
    //   block [0x823BA2E8..0x823BA35C)
	// 823BA2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA2F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA2F4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BA2F8: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA2FC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA300: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA304: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA308: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA30C: 386B0070  addi r3, r11, 0x70
	ctx.r[3].s64 = ctx.r[11].s64 + 112;
	// 823BA310: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 823BA314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA318: 419A002C  beq cr6, 0x823ba344
	if ctx.cr[6].eq {
	pc = 0x823BA344; continue 'dispatch;
	}
	// 823BA31C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA320: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA324: 419A001C  beq cr6, 0x823ba340
	if ctx.cr[6].eq {
	pc = 0x823BA340; continue 'dispatch;
	}
	// 823BA328: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823BA32C: 4BFFFCB5  bl 0x823b9fe0
	ctx.lr = 0x823BA330;
	sub_823B9FE0(ctx, base);
	// 823BA330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA33C: 4E800020  blr
	return;
	// 823BA340: 4BDD9AF9  bl 0x82193e38
	ctx.lr = 0x823BA344;
	sub_82193E38(ctx, base);
	// 823BA344: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823BA348: 4BFFFC99  bl 0x823b9fe0
	ctx.lr = 0x823BA34C;
	sub_823B9FE0(ctx, base);
	// 823BA34C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA360 size=36
    let mut pc: u32 = 0x823BA360;
    'dispatch: loop {
        match pc {
            0x823BA360 => {
    //   block [0x823BA360..0x823BA384)
	// 823BA360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA36C: 4BFFFF05  bl 0x823ba270
	ctx.lr = 0x823BA370;
	sub_823BA270(ctx, base);
	// 823BA370: 4BFFFF79  bl 0x823ba2e8
	ctx.lr = 0x823BA374;
	sub_823BA2E8(ctx, base);
	// 823BA374: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA388 size=152
    let mut pc: u32 = 0x823BA388;
    'dispatch: loop {
        match pc {
            0x823BA388 => {
    //   block [0x823BA388..0x823BA420)
	// 823BA388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA390: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA394: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA398: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 823BA39C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 823BA3A0: 388BAC18  addi r4, r11, -0x53e8
	ctx.r[4].s64 = ctx.r[11].s64 + -21480;
	// 823BA3A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA3A8: 4BE72B29  bl 0x8222ced0
	ctx.lr = 0x823BA3AC;
	sub_8222CED0(ctx, base);
	// 823BA3AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BA3B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BA3B4: 48787C8D  bl 0x82b42040
	ctx.lr = 0x823BA3B8;
	sub_82B42040(ctx, base);
	// 823BA3B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA3BC: 4BE5AA1D  bl 0x82214dd8
	ctx.lr = 0x823BA3C0;
	sub_82214DD8(ctx, base);
	// 823BA3C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 823BA3C4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 823BA3C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 823BA3CC: 3889AC4C  addi r4, r9, -0x53b4
	ctx.r[4].s64 = ctx.r[9].s64 + -21428;
	// 823BA3D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA3D4: 816A6AB8  lwz r11, 0x6ab8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA3D8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA3DC: 80E80078  lwz r7, 0x78(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(120 as u32) ) } as u64;
	// 823BA3E0: 80C70004  lwz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA3E4: 83E60000  lwz r31, 0(r6)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA3E8: 4BE72AE9  bl 0x8222ced0
	ctx.lr = 0x823BA3EC;
	sub_8222CED0(ctx, base);
	// 823BA3EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823BA3F0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 823BA3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA3F8: 48097A49  bl 0x82451e40
	ctx.lr = 0x823BA3FC;
	sub_82451E40(ctx, base);
	// 823BA3FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA400: 4BE5A9D9  bl 0x82214dd8
	ctx.lr = 0x823BA404;
	sub_82214DD8(ctx, base);
	// 823BA404: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BA408: 4BE5A9D1  bl 0x82214dd8
	ctx.lr = 0x823BA40C;
	sub_82214DD8(ctx, base);
	// 823BA40C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BA420 size=20
    let mut pc: u32 = 0x823BA420;
    'dispatch: loop {
        match pc {
            0x823BA420 => {
    //   block [0x823BA420..0x823BA434)
	// 823BA420: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 823BA424: 1D4300B0  mulli r10, r3, 0xb0
	ctx.r[10].s64 = ctx.r[3].s64 * 176;
	// 823BA428: 392BA638  addi r9, r11, -0x59c8
	ctx.r[9].s64 = ctx.r[11].s64 + -22984;
	// 823BA42C: 7C8A492E  stwx r4, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[4].u32) };
	// 823BA430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BA438 size=24
    let mut pc: u32 = 0x823BA438;
    'dispatch: loop {
        match pc {
            0x823BA438 => {
    //   block [0x823BA438..0x823BA450)
	// 823BA438: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 823BA43C: 1D4300B0  mulli r10, r3, 0xb0
	ctx.r[10].s64 = ctx.r[3].s64 * 176;
	// 823BA440: 396BA638  addi r11, r11, -0x59c8
	ctx.r[11].s64 = ctx.r[11].s64 + -22984;
	// 823BA444: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 823BA448: 7C8A49AE  stbx r4, r10, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[4].u8) };
	// 823BA44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA450 size=32
    let mut pc: u32 = 0x823BA450;
    'dispatch: loop {
        match pc {
            0x823BA450 => {
    //   block [0x823BA450..0x823BA470)
	// 823BA450: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 823BA454: 1D4300B0  mulli r10, r3, 0xb0
	ctx.r[10].s64 = ctx.r[3].s64 * 176;
	// 823BA458: 396BA638  addi r11, r11, -0x59c8
	ctx.r[11].s64 = ctx.r[11].s64 + -22984;
	// 823BA45C: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 823BA460: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 823BA464: 7C2A4D2E  stfsx f1, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 823BA468: 7C4A452E  stfsx f2, r10, r8
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA470 size=108
    let mut pc: u32 = 0x823BA470;
    'dispatch: loop {
        match pc {
            0x823BA470 => {
    //   block [0x823BA470..0x823BA4DC)
	// 823BA470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA474: 488EEF99  bl 0x82ca940c
	ctx.lr = 0x823BA478;
	sub_82CA93D0(ctx, base);
	// 823BA478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA47C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823BA480: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BA484: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BA488: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BA48C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA490: 4BE35DB1  bl 0x821f0240
	ctx.lr = 0x823BA494;
	sub_821F0240(ctx, base);
	// 823BA494: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA498: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 823BA49C: 7D1F5A14  add r8, r31, r11
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 823BA4A0: 1D5E00B0  mulli r10, r30, 0xb0
	ctx.r[10].s64 = ctx.r[30].s64 * 176;
	// 823BA4A4: 550B2036  slwi r11, r8, 4
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA4A8: 3929A638  addi r9, r9, -0x59c8
	ctx.r[9].s64 = ctx.r[9].s64 + -22984;
	// 823BA4AC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823BA4B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BA4B4: 39490010  addi r10, r9, 0x10
	ctx.r[10].s64 = ctx.r[9].s64 + 16;
	// 823BA4B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BA4BC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823BA4C0: 4BEAACE1  bl 0x822651a0
	ctx.lr = 0x823BA4C4;
	sub_822651A0(ctx, base);
	// 823BA4C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA4C8: 4BE5A911  bl 0x82214dd8
	ctx.lr = 0x823BA4CC;
	sub_82214DD8(ctx, base);
	// 823BA4CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BA4D0: 4BE5A909  bl 0x82214dd8
	ctx.lr = 0x823BA4D4;
	sub_82214DD8(ctx, base);
	// 823BA4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BA4D8: 488EEF84  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA4E0 size=40
    let mut pc: u32 = 0x823BA4E0;
    'dispatch: loop {
        match pc {
            0x823BA4E0 => {
    //   block [0x823BA4E0..0x823BA508)
	// 823BA4E0: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA4E4: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 823BA4E8: 7D445A14  add r10, r4, r11
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA4EC: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA4F0: 3968A638  addi r11, r8, -0x59c8
	ctx.r[11].s64 = ctx.r[8].s64 + -22984;
	// 823BA4F4: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823BA4F8: 38EB0018  addi r7, r11, 0x18
	ctx.r[7].s64 = ctx.r[11].s64 + 24;
	// 823BA4FC: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 823BA500: 7C263D2E  stfsx f1, r6, r7
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BA508 size=40
    let mut pc: u32 = 0x823BA508;
    'dispatch: loop {
        match pc {
            0x823BA508 => {
    //   block [0x823BA508..0x823BA530)
	// 823BA508: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA50C: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 823BA510: 7D445A14  add r10, r4, r11
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA514: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA518: 3968A638  addi r11, r8, -0x59c8
	ctx.r[11].s64 = ctx.r[8].s64 + -22984;
	// 823BA51C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823BA520: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 823BA524: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 823BA528: 7CA639AE  stbx r5, r6, r7
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32), ctx.r[5].u8) };
	// 823BA52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA530 size=48
    let mut pc: u32 = 0x823BA530;
    'dispatch: loop {
        match pc {
            0x823BA530 => {
    //   block [0x823BA530..0x823BA560)
	// 823BA530: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA534: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA538: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA53C: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA540: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA544: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA548: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 823BA54C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA550: 38EA0040  addi r7, r10, 0x40
	ctx.r[7].s64 = ctx.r[10].s64 + 64;
	// 823BA554: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA558: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA560 size=48
    let mut pc: u32 = 0x823BA560;
    'dispatch: loop {
        match pc {
            0x823BA560 => {
    //   block [0x823BA560..0x823BA590)
	// 823BA560: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA564: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA568: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA56C: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA570: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA574: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA578: 390A0024  addi r8, r10, 0x24
	ctx.r[8].s64 = ctx.r[10].s64 + 36;
	// 823BA57C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA580: 38EA0044  addi r7, r10, 0x44
	ctx.r[7].s64 = ctx.r[10].s64 + 68;
	// 823BA584: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA588: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA590 size=48
    let mut pc: u32 = 0x823BA590;
    'dispatch: loop {
        match pc {
            0x823BA590 => {
    //   block [0x823BA590..0x823BA5C0)
	// 823BA590: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA594: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA598: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA59C: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA5A0: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA5A4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA5A8: 390A0028  addi r8, r10, 0x28
	ctx.r[8].s64 = ctx.r[10].s64 + 40;
	// 823BA5AC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA5B0: 38EA0048  addi r7, r10, 0x48
	ctx.r[7].s64 = ctx.r[10].s64 + 72;
	// 823BA5B4: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA5B8: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA5C0 size=48
    let mut pc: u32 = 0x823BA5C0;
    'dispatch: loop {
        match pc {
            0x823BA5C0 => {
    //   block [0x823BA5C0..0x823BA5F0)
	// 823BA5C0: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA5C4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA5C8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA5CC: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA5D0: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA5D4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA5D8: 390A002C  addi r8, r10, 0x2c
	ctx.r[8].s64 = ctx.r[10].s64 + 44;
	// 823BA5DC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA5E0: 38EA004C  addi r7, r10, 0x4c
	ctx.r[7].s64 = ctx.r[10].s64 + 76;
	// 823BA5E4: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA5E8: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA5F0 size=48
    let mut pc: u32 = 0x823BA5F0;
    'dispatch: loop {
        match pc {
            0x823BA5F0 => {
    //   block [0x823BA5F0..0x823BA620)
	// 823BA5F0: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA5F4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA5F8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA5FC: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA600: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA604: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA608: 390A0030  addi r8, r10, 0x30
	ctx.r[8].s64 = ctx.r[10].s64 + 48;
	// 823BA60C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA610: 38EA0050  addi r7, r10, 0x50
	ctx.r[7].s64 = ctx.r[10].s64 + 80;
	// 823BA614: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA618: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA620 size=48
    let mut pc: u32 = 0x823BA620;
    'dispatch: loop {
        match pc {
            0x823BA620 => {
    //   block [0x823BA620..0x823BA650)
	// 823BA620: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA624: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA628: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA62C: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA630: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA634: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA638: 390A0034  addi r8, r10, 0x34
	ctx.r[8].s64 = ctx.r[10].s64 + 52;
	// 823BA63C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA640: 38EA0054  addi r7, r10, 0x54
	ctx.r[7].s64 = ctx.r[10].s64 + 84;
	// 823BA644: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA648: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA650 size=48
    let mut pc: u32 = 0x823BA650;
    'dispatch: loop {
        match pc {
            0x823BA650 => {
    //   block [0x823BA650..0x823BA680)
	// 823BA650: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA654: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA658: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA65C: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA660: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA664: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA668: 390A0038  addi r8, r10, 0x38
	ctx.r[8].s64 = ctx.r[10].s64 + 56;
	// 823BA66C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA670: 38EA0058  addi r7, r10, 0x58
	ctx.r[7].s64 = ctx.r[10].s64 + 88;
	// 823BA674: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA678: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA680 size=48
    let mut pc: u32 = 0x823BA680;
    'dispatch: loop {
        match pc {
            0x823BA680 => {
    //   block [0x823BA680..0x823BA6B0)
	// 823BA680: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA684: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BA688: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 823BA68C: 394AA638  addi r10, r10, -0x59c8
	ctx.r[10].s64 = ctx.r[10].s64 + -22984;
	// 823BA690: 1D2300B0  mulli r9, r3, 0xb0
	ctx.r[9].s64 = ctx.r[3].s64 * 176;
	// 823BA694: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA698: 390A003C  addi r8, r10, 0x3c
	ctx.r[8].s64 = ctx.r[10].s64 + 60;
	// 823BA69C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 823BA6A0: 38EA005C  addi r7, r10, 0x5c
	ctx.r[7].s64 = ctx.r[10].s64 + 92;
	// 823BA6A4: 7C2B452E  stfsx f1, r11, r8
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 823BA6A8: 7C4B3D2E  stfsx f2, r11, r7
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 823BA6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BA6B0 size=32
    let mut pc: u32 = 0x823BA6B0;
    'dispatch: loop {
        match pc {
            0x823BA6B0 => {
    //   block [0x823BA6B0..0x823BA6D0)
	// 823BA6B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BA6B4: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA6B8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA6BC: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA6C0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA6C4: 80E80048  lwz r7, 0x48(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BA6C8: 9867000C  stb r3, 0xc(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[3].u8 ) };
	// 823BA6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA6D0 size=176
    let mut pc: u32 = 0x823BA6D0;
    'dispatch: loop {
        match pc {
            0x823BA6D0 => {
    //   block [0x823BA6D0..0x823BA780)
	// 823BA6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA6D8: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 823BA6DC: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 823BA6E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA6E4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BA6E8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823BA6EC: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 823BA6F0: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA6F4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA6F8: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA6FC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA700: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA704: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823BA708: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823BA70C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA710: 419A0068  beq cr6, 0x823ba778
	if ctx.cr[6].eq {
	pc = 0x823BA778; continue 'dispatch;
	}
	// 823BA714: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA718: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA71C: 419A0058  beq cr6, 0x823ba774
	if ctx.cr[6].eq {
	pc = 0x823BA774; continue 'dispatch;
	}
	// 823BA720: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 823BA724: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA728: 419A0018  beq cr6, 0x823ba740
	if ctx.cr[6].eq {
	pc = 0x823BA740; continue 'dispatch;
	}
	// 823BA72C: 89630090  lbz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 823BA730: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823BA734: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA738: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA73C: 409A0008  bne cr6, 0x823ba744
	if !ctx.cr[6].eq {
	pc = 0x823BA744; continue 'dispatch;
	}
	// 823BA740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA744: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BA748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA74C: 419A0010  beq cr6, 0x823ba75c
	if ctx.cr[6].eq {
	pc = 0x823BA75C; continue 'dispatch;
	}
	// 823BA750: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 823BA754: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823BA758: 48432111  bl 0x827ec868
	ctx.lr = 0x823BA75C;
	sub_827EC868(ctx, base);
	// 823BA75C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA768: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BA76C: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA770: 4E800020  blr
	return;
	// 823BA774: 4BDD96C5  bl 0x82193e38
	ctx.lr = 0x823BA778;
	sub_82193E38(ctx, base);
	// 823BA778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823BA77C: 4BFFFFC4  b 0x823ba740
	pc = 0x823BA740; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BA780 size=20
    let mut pc: u32 = 0x823BA780;
    'dispatch: loop {
        match pc {
            0x823BA780 => {
    //   block [0x823BA780..0x823BA794)
	// 823BA780: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BA784: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823BA788: 392B920C  addi r9, r11, -0x6df4
	ctx.r[9].s64 = ctx.r[11].s64 + -28148;
	// 823BA78C: 7C2A4D2E  stfsx f1, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 823BA790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA798 size=216
    let mut pc: u32 = 0x823BA798;
    'dispatch: loop {
        match pc {
            0x823BA798 => {
    //   block [0x823BA798..0x823BA870)
	// 823BA798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA7A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA7A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA7A8: 3FE08349  lis r31, -0x7cb7
	ctx.r[31].s64 = -2092367872;
	// 823BA7AC: 815F6AB8  lwz r10, 0x6ab8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA7B0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA7B4: 812B0058  lwz r9, 0x58(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA7B8: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA7BC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA7C0: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823BA7C4: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823BA7C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA7CC: 419A009C  beq cr6, 0x823ba868
	if ctx.cr[6].eq {
	pc = 0x823BA868; continue 'dispatch;
	}
	// 823BA7D0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA7D4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BA7D8: 419A0088  beq cr6, 0x823ba860
	if ctx.cr[6].eq {
	pc = 0x823BA860; continue 'dispatch;
	}
	// 823BA7DC: 5526003E  slwi r6, r9, 0
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 823BA7E0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 823BA7E4: 419A0018  beq cr6, 0x823ba7fc
	if ctx.cr[6].eq {
	pc = 0x823BA7FC; continue 'dispatch;
	}
	// 823BA7E8: 89660090  lbz r11, 0x90(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(144 as u32) ) } as u64;
	// 823BA7EC: 55690672  rlwinm r9, r11, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 823BA7F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA7F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BA7F8: 409A0008  bne cr6, 0x823ba800
	if !ctx.cr[6].eq {
	pc = 0x823BA800; continue 'dispatch;
	}
	// 823BA7FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA800: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BA804: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA808: 419A0044  beq cr6, 0x823ba84c
	if ctx.cr[6].eq {
	pc = 0x823BA84C; continue 'dispatch;
	}
	// 823BA80C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA810: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823BA814: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 823BA818: 814B0058  lwz r10, 0x58(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA81C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA820: 80690044  lwz r3, 0x44(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(68 as u32) ) } as u64;
	// 823BA824: 4BDB9B05  bl 0x82174328
	ctx.lr = 0x823BA828;
	sub_82174328(ctx, base);
	// 823BA828: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA82C: 419A0020  beq cr6, 0x823ba84c
	if ctx.cr[6].eq {
	pc = 0x823BA84C; continue 'dispatch;
	}
	// 823BA830: 896300C4  lbz r11, 0xc4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 823BA834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA838: 419A0014  beq cr6, 0x823ba84c
	if ctx.cr[6].eq {
	pc = 0x823BA84C; continue 'dispatch;
	}
	// 823BA83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823BA840: 806300C8  lwz r3, 0xc8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) } as u64;
	// 823BA844: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 823BA848: 481E8951  bl 0x825a3198
	ctx.lr = 0x823BA84C;
	sub_825A3198(ctx, base);
	// 823BA84C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA858: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA85C: 4E800020  blr
	return;
	// 823BA860: 4BDD95D9  bl 0x82193e38
	ctx.lr = 0x823BA864;
	sub_82193E38(ctx, base);
	// 823BA864: 815F6AB8  lwz r10, 0x6ab8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA868: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BA86C: 4BFFFF90  b 0x823ba7fc
	pc = 0x823BA7FC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA870 size=160
    let mut pc: u32 = 0x823BA870;
    'dispatch: loop {
        match pc {
            0x823BA870 => {
    //   block [0x823BA870..0x823BA910)
	// 823BA870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA87C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA880: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BA884: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BA888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA88C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 823BA890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 823BA894: 4BE359AD  bl 0x821f0240
	ctx.lr = 0x823BA898;
	sub_821F0240(ctx, base);
	// 823BA898: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BA89C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 823BA8A0: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 823BA8A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BA8A8: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA8AC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA8B0: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA8B4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA8B8: 8068000C  lwz r3, 0xc(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA8BC: 48004FCD  bl 0x823bf888
	ctx.lr = 0x823BA8C0;
	sub_823BF888(ctx, base);
	// 823BA8C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA8C4: 4BE5A515  bl 0x82214dd8
	ctx.lr = 0x823BA8C8;
	sub_82214DD8(ctx, base);
	// 823BA8C8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BA8CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA8D0: 419A0030  beq cr6, 0x823ba900
	if ctx.cr[6].eq {
	pc = 0x823BA900; continue 'dispatch;
	}
	// 823BA8D4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 823BA8D8: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 823BA8DC: 7D4B1670  srawi r11, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 823BA8E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA8E4: 419A0010  beq cr6, 0x823ba8f4
	if ctx.cr[6].eq {
	pc = 0x823BA8F4; continue 'dispatch;
	}
	// 823BA8E8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA8EC: 48243715  bl 0x825fe000
	ctx.lr = 0x823BA8F0;
	sub_825FE000(ctx, base);
	// 823BA8F0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BA8F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA8F8: 419A0008  beq cr6, 0x823ba900
	if ctx.cr[6].eq {
	pc = 0x823BA900; continue 'dispatch;
	}
	// 823BA8FC: 4BE6143D  bl 0x8221bd38
	ctx.lr = 0x823BA900;
	sub_8221BD38(ctx, base);
	// 823BA900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BA904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA910 size=512
    let mut pc: u32 = 0x823BA910;
    'dispatch: loop {
        match pc {
            0x823BA910 => {
    //   block [0x823BA910..0x823BAB10)
	// 823BA910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BA91C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA920: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA924: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BA928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BA92C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BA930: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BA934: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BA938: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BA93C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA940: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA944: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823BA948: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823BA94C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA950: 419A0080  beq cr6, 0x823ba9d0
	if ctx.cr[6].eq {
	pc = 0x823BA9D0; continue 'dispatch;
	}
	// 823BA954: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA958: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA95C: 419A0070  beq cr6, 0x823ba9cc
	if ctx.cr[6].eq {
	pc = 0x823BA9CC; continue 'dispatch;
	}
	// 823BA960: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BA964: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA968: 419A0018  beq cr6, 0x823ba980
	if ctx.cr[6].eq {
	pc = 0x823BA980; continue 'dispatch;
	}
	// 823BA96C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823BA970: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823BA974: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BA978: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BA97C: 409A0008  bne cr6, 0x823ba984
	if !ctx.cr[6].eq {
	pc = 0x823BA984; continue 'dispatch;
	}
	// 823BA980: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 823BA984: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823BA988: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA98C: 419A016C  beq cr6, 0x823baaf8
	if ctx.cr[6].eq {
	pc = 0x823BAAF8; continue 'dispatch;
	}
	// 823BA990: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 823BA994: 5549CFFE  rlwinm r9, r10, 0x19, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000007Fu64;
	// 823BA998: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BA99C: 419A0100  beq cr6, 0x823baa9c
	if ctx.cr[6].eq {
	pc = 0x823BAA9C; continue 'dispatch;
	}
	// 823BA9A0: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823BA9A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BA9A8: 419A0030  beq cr6, 0x823ba9d8
	if ctx.cr[6].eq {
	pc = 0x823BA9D8; continue 'dispatch;
	}
	// 823BA9AC: 894A0087  lbz r10, 0x87(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(135 as u32) ) } as u64;
	// 823BA9B0: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BA9B4: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823BA9B8: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823BA9BC: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BA9C0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823BA9C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BA9C8: 480000D8  b 0x823baaa0
	pc = 0x823BAAA0; continue 'dispatch;
	// 823BA9CC: 4BDD946D  bl 0x82193e38
	ctx.lr = 0x823BA9D0;
	sub_82193E38(ctx, base);
	// 823BA9D0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823BA9D4: 4BFFFFAC  b 0x823ba980
	pc = 0x823BA980; continue 'dispatch;
	// 823BA9D8: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BA9DC: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823BA9E0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823BA9E4: 7D6A3050  subf r11, r10, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823BA9E8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BA9EC: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BA9F0: 40810054  ble 0x823baa44
	if !ctx.cr[0].gt {
	pc = 0x823BAA44; continue 'dispatch;
	}
	// 823BA9F4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823BA9F8: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823BA9FC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823BAA00: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BAA04: 2F070087  cmpwi cr6, r7, 0x87
	ctx.cr[6].compare_i32(ctx.r[7].s32, 135, &mut ctx.xer);
	// 823BAA08: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823BAA0C: 41980008  blt cr6, 0x823baa14
	if ctx.cr[6].lt {
	pc = 0x823BAA14; continue 'dispatch;
	}
	// 823BAA10: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823BAA14: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823BAA18: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823BAA1C: 419A0014  beq cr6, 0x823baa30
	if ctx.cr[6].eq {
	pc = 0x823BAA30; continue 'dispatch;
	}
	// 823BAA20: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823BAA24: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823BAA28: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823BAA2C: 4800000C  b 0x823baa38
	pc = 0x823BAA38; continue 'dispatch;
	// 823BAA30: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823BAA34: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823BAA38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BAA3C: 4199FFB8  bgt cr6, 0x823ba9f4
	if ctx.cr[6].gt {
	pc = 0x823BA9F4; continue 'dispatch;
	}
	// 823BAA40: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BAA44: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823BAA48: 419A0040  beq cr6, 0x823baa88
	if ctx.cr[6].eq {
	pc = 0x823BAA88; continue 'dispatch;
	}
	// 823BAA4C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BAA50: 2F0B0087  cmpwi cr6, r11, 0x87
	ctx.cr[6].compare_i32(ctx.r[11].s32, 135, &mut ctx.xer);
	// 823BAA54: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BAA58: 41990008  bgt cr6, 0x823baa60
	if ctx.cr[6].gt {
	pc = 0x823BAA60; continue 'dispatch;
	}
	// 823BAA5C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823BAA60: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BAA64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BAA68: 409A0020  bne cr6, 0x823baa88
	if !ctx.cr[6].eq {
	pc = 0x823BAA88; continue 'dispatch;
	}
	// 823BAA6C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823BAA70: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823BAA74: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BAA78: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BAA7C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823BAA80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BAA84: 4800001C  b 0x823baaa0
	pc = 0x823BAAA0; continue 'dispatch;
	// 823BAA88: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823BAA8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BAA90: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 823BAA94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BAA98: 48000008  b 0x823baaa0
	pc = 0x823BAAA0; continue 'dispatch;
	// 823BAA9C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 823BAAA0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BAAA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BAAA8: 419A0050  beq cr6, 0x823baaf8
	if ctx.cr[6].eq {
	pc = 0x823BAAF8; continue 'dispatch;
	}
	// 823BAAAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823BAAB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 823BAAB4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 823BAAB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BAABC: 4BE72415  bl 0x8222ced0
	ctx.lr = 0x823BAAC0;
	sub_8222CED0(ctx, base);
	// 823BAAC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BAAC4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823BAAC8: 4BEAC6E1  bl 0x822671a8
	ctx.lr = 0x823BAACC;
	sub_822671A8(ctx, base);
	// 823BAACC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 823BAAD0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 823BAAD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BAAD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BAADC: 4BE1EB2D  bl 0x821d9608
	ctx.lr = 0x823BAAE0;
	sub_821D9608(ctx, base);
	// 823BAAE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BAAE4: 4884B415  bl 0x82c05ef8
	ctx.lr = 0x823BAAE8;
	sub_82C05EF8(ctx, base);
	// 823BAAE8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823BAAEC: 4BECEF5D  bl 0x82289a48
	ctx.lr = 0x823BAAF0;
	sub_82289A48(ctx, base);
	// 823BAAF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BAAF4: 4BE5A2E5  bl 0x82214dd8
	ctx.lr = 0x823BAAF8;
	sub_82214DD8(ctx, base);
	// 823BAAF8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823BAAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAB04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BAB08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAB10 size=136
    let mut pc: u32 = 0x823BAB10;
    'dispatch: loop {
        match pc {
            0x823BAB10 => {
    //   block [0x823BAB10..0x823BAB98)
	// 823BAB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAB14: 488EE8F9  bl 0x82ca940c
	ctx.lr = 0x823BAB18;
	sub_82CA93D0(ctx, base);
	// 823BAB18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAB1C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BAB20: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 823BAB24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BAB28: 986B9409  stb r3, -0x6bf7(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27639 as u32), ctx.r[3].u8 ) };
	// 823BAB2C: 409A0064  bne cr6, 0x823bab90
	if !ctx.cr[6].eq {
	pc = 0x823BAB90; continue 'dispatch;
	}
	// 823BAB30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BAB34: 83AB6B08  lwz r29, 0x6b08(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27400 as u32) ) } as u64;
	// 823BAB38: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 823BAB3C: 83CA001C  lwz r30, 0x1c(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 823BAB40: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BAB44: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BAB48: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 823BAB4C: 419A0030  beq cr6, 0x823bab7c
	if ctx.cr[6].eq {
	pc = 0x823BAB7C; continue 'dispatch;
	}
	// 823BAB50: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BAB54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BAB58: 419A0014  beq cr6, 0x823bab6c
	if ctx.cr[6].eq {
	pc = 0x823BAB6C; continue 'dispatch;
	}
	// 823BAB5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BAB60: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823BAB64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 823BAB68: 4E800421  bctrl
	ctx.lr = 0x823BAB6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BAB6C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BAB70: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 823BAB74: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823BAB78: 409AFFD8  bne cr6, 0x823bab50
	if !ctx.cr[6].eq {
	pc = 0x823BAB50; continue 'dispatch;
	}
	// 823BAB7C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 823BAB80: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BAB84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BAB88: 419A0008  beq cr6, 0x823bab90
	if ctx.cr[6].eq {
	pc = 0x823BAB90; continue 'dispatch;
	}
	// 823BAB8C: 4831B815  bl 0x826d63a0
	ctx.lr = 0x823BAB90;
	sub_826D63A0(ctx, base);
	// 823BAB90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BAB94: 488EE8C8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BAB98 size=12
    let mut pc: u32 = 0x823BAB98;
    'dispatch: loop {
        match pc {
            0x823BAB98 => {
    //   block [0x823BAB98..0x823BABA4)
	// 823BAB98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BAB9C: 986B6B4D  stb r3, 0x6b4d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27469 as u32), ctx.r[3].u8 ) };
	// 823BABA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BABA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BABA8 size=32
    let mut pc: u32 = 0x823BABA8;
    'dispatch: loop {
        match pc {
            0x823BABA8 => {
    //   block [0x823BABA8..0x823BABC8)
	// 823BABA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BABAC: 816B6B08  lwz r11, 0x6b08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27400 as u32) ) } as u64;
	// 823BABB0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823BABB4: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 823BABB8: 98690044  stb r3, 0x44(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[3].u8 ) };
	// 823BABBC: 810A0040  lwz r8, 0x40(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 823BABC0: 9868002D  stb r3, 0x2d(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(45 as u32), ctx.r[3].u8 ) };
	// 823BABC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BABC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BABC8 size=12
    let mut pc: u32 = 0x823BABC8;
    'dispatch: loop {
        match pc {
            0x823BABC8 => {
    //   block [0x823BABC8..0x823BABD4)
	// 823BABC8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BABCC: 986B9583  stb r3, -0x6a7d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27261 as u32), ctx.r[3].u8 ) };
	// 823BABD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BABD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BABD8 size=12
    let mut pc: u32 = 0x823BABD8;
    'dispatch: loop {
        match pc {
            0x823BABD8 => {
    //   block [0x823BABD8..0x823BABE4)
	// 823BABD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BABDC: 986B6B9B  stb r3, 0x6b9b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27547 as u32), ctx.r[3].u8 ) };
	// 823BABE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BABE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BABE8 size=12
    let mut pc: u32 = 0x823BABE8;
    'dispatch: loop {
        match pc {
            0x823BABE8 => {
    //   block [0x823BABE8..0x823BABF4)
	// 823BABE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BABEC: 986B6B4E  stb r3, 0x6b4e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27470 as u32), ctx.r[3].u8 ) };
	// 823BABF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BABF8 size=12
    let mut pc: u32 = 0x823BABF8;
    'dispatch: loop {
        match pc {
            0x823BABF8 => {
    //   block [0x823BABF8..0x823BAC04)
	// 823BABF8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BABFC: 986B940A  stb r3, -0x6bf6(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27638 as u32), ctx.r[3].u8 ) };
	// 823BAC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BAC08 size=36
    let mut pc: u32 = 0x823BAC08;
    'dispatch: loop {
        match pc {
            0x823BAC08 => {
    //   block [0x823BAC08..0x823BAC2C)
	// 823BAC08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BAC0C: 816B6B08  lwz r11, 0x6b08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27400 as u32) ) } as u64;
	// 823BAC10: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 823BAC14: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 823BAC18: 89090016  lbz r8, 0x16(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(22 as u32) ) } as u64;
	// 823BAC1C: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 823BAC20: 54E6DFFE  rlwinm r6, r7, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 823BAC24: 98C90016  stb r6, 0x16(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(22 as u32), ctx.r[6].u8 ) };
	// 823BAC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BAC30 size=24
    let mut pc: u32 = 0x823BAC30;
    'dispatch: loop {
        match pc {
            0x823BAC30 => {
    //   block [0x823BAC30..0x823BAC48)
	// 823BAC30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BAC34: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BAC38: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BAC3C: 812A0074  lwz r9, 0x74(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(116 as u32) ) } as u64;
	// 823BAC40: D029005C  stfs f1, 0x5c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 823BAC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BAC48 size=24
    let mut pc: u32 = 0x823BAC48;
    'dispatch: loop {
        match pc {
            0x823BAC48 => {
    //   block [0x823BAC48..0x823BAC60)
	// 823BAC48: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BAC4C: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BAC50: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BAC54: 812A0074  lwz r9, 0x74(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(116 as u32) ) } as u64;
	// 823BAC58: D0290060  stfs f1, 0x60(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 823BAC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BAC60 size=24
    let mut pc: u32 = 0x823BAC60;
    'dispatch: loop {
        match pc {
            0x823BAC60 => {
    //   block [0x823BAC60..0x823BAC78)
	// 823BAC60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BAC64: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BAC68: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BAC6C: 812A0074  lwz r9, 0x74(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(116 as u32) ) } as u64;
	// 823BAC70: D0290064  stfs f1, 0x64(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 823BAC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAC78 size=236
    let mut pc: u32 = 0x823BAC78;
    'dispatch: loop {
        match pc {
            0x823BAC78 => {
    //   block [0x823BAC78..0x823BAD64)
	// 823BAC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAC80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BAC84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAC8C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BAC90: 3BEB79D0  addi r31, r11, 0x79d0
	ctx.r[31].s64 = ctx.r[11].s64 + 31184;
	// 823BAC94: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BAC98: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BAC9C: 419A00B0  beq cr6, 0x823bad4c
	if ctx.cr[6].eq {
	pc = 0x823BAD4C; continue 'dispatch;
	}
	// 823BACA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BACA4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823BACA8: 7D6A1670  srawi r10, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 823BACAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BACB0: 419A009C  beq cr6, 0x823bad4c
	if ctx.cr[6].eq {
	pc = 0x823BAD4C; continue 'dispatch;
	}
	// 823BACB4: 3FC08349  lis r30, -0x7cb7
	ctx.r[30].s64 = -2092367872;
	// 823BACB8: 817E6B40  lwz r11, 0x6b40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(27456 as u32) ) } as u64;
	// 823BACBC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823BACC0: 41980008  blt cr6, 0x823bacc8
	if ctx.cr[6].lt {
	pc = 0x823BACC8; continue 'dispatch;
	}
	// 823BACC4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823BACC8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823BACCC: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 823BACD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BACD4: 419A0018  beq cr6, 0x823bacec
	if ctx.cr[6].eq {
	pc = 0x823BACEC; continue 'dispatch;
	}
	// 823BACD8: 88E30090  lbz r7, 0x90(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 823BACDC: 54E60672  rlwinm r6, r7, 0, 0x19, 0x19
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 823BACE0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823BACE4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 823BACE8: 409A0008  bne cr6, 0x823bacf0
	if !ctx.cr[6].eq {
	pc = 0x823BACF0; continue 'dispatch;
	}
	// 823BACEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823BACF0: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823BACF4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823BACF8: 419A0010  beq cr6, 0x823bad08
	if ctx.cr[6].eq {
	pc = 0x823BAD08; continue 'dispatch;
	}
	// 823BACFC: 480C810D  bl 0x82482e08
	ctx.lr = 0x823BAD00;
	sub_82482E08(ctx, base);
	// 823BAD00: 817E6B40  lwz r11, 0x6b40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(27456 as u32) ) } as u64;
	// 823BAD04: 48000018  b 0x823bad1c
	pc = 0x823BAD1C; continue 'dispatch;
	// 823BAD08: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823BAD0C: 41980008  blt cr6, 0x823bad14
	if ctx.cr[6].lt {
	pc = 0x823BAD14; continue 'dispatch;
	}
	// 823BAD10: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823BAD14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823BAD18: 7D48492E  stwx r10, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 823BAD1C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BAD20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 823BAD24: 917E6B40  stw r11, 0x6b40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(27456 as u32), ctx.r[11].u32 ) };
	// 823BAD28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BAD2C: 419A0018  beq cr6, 0x823bad44
	if ctx.cr[6].eq {
	pc = 0x823BAD44; continue 'dispatch;
	}
	// 823BAD30: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BAD34: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 823BAD38: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 823BAD3C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823BAD40: 4198000C  blt cr6, 0x823bad4c
	if ctx.cr[6].lt {
	pc = 0x823BAD4C; continue 'dispatch;
	}
	// 823BAD44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAD48: 917E6B40  stw r11, 0x6b40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(27456 as u32), ctx.r[11].u32 ) };
	// 823BAD4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BAD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAD58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BAD5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BAD68 size=492
    let mut pc: u32 = 0x823BAD68;
    'dispatch: loop {
        match pc {
            0x823BAD68 => {
    //   block [0x823BAD68..0x823BAF54)
	// 823BAD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAD70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAD74: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BAF58 size=492
    let mut pc: u32 = 0x823BAF58;
    'dispatch: loop {
        match pc {
            0x823BAF58 => {
    //   block [0x823BAF58..0x823BB144)
	// 823BAF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAF60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAF64: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB148 size=496
    let mut pc: u32 = 0x823BB148;
    'dispatch: loop {
        match pc {
            0x823BB148 => {
    //   block [0x823BB148..0x823BB338)
	// 823BB148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB154: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB338 size=168
    let mut pc: u32 = 0x823BB338;
    'dispatch: loop {
        match pc {
            0x823BB338 => {
    //   block [0x823BB338..0x823BB3E0)
	// 823BB338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BB344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB34C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 823BB350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BB354: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 823BB358: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BB35C: 3BEA79D0  addi r31, r10, 0x79d0
	ctx.r[31].s64 = ctx.r[10].s64 + 31184;
	// 823BB360: 91696B40  stw r11, 0x6b40(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(27456 as u32), ctx.r[11].u32 ) };
	// 823BB364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB368: 4BDF6841  bl 0x821b1ba8
	ctx.lr = 0x823BB36C;
	sub_821B1BA8(ctx, base);
	// 823BB36C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BB370: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB374: 4BE34ECD  bl 0x821f0240
	ctx.lr = 0x823BB378;
	sub_821F0240(ctx, base);
	// 823BB378: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 823BB37C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 823BB380: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 823BB384: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BB388: 81686AB8  lwz r11, 0x6ab8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BB38C: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB390: 80670058  lwz r3, 0x58(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BB394: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB398: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB39C: 480044ED  bl 0x823bf888
	ctx.lr = 0x823BB3A0;
	sub_823BF888(ctx, base);
	// 823BB3A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB3A4: 4BE59A35  bl 0x82214dd8
	ctx.lr = 0x823BB3A8;
	sub_82214DD8(ctx, base);
	// 823BB3A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB3AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB3B0: 419A0018  beq cr6, 0x823bb3c8
	if ctx.cr[6].eq {
	pc = 0x823BB3C8; continue 'dispatch;
	}
	// 823BB3B4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BB3B8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 823BB3BC: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BB3C0: 41820008  beq 0x823bb3c8
	if ctx.cr[0].eq {
	pc = 0x823BB3C8; continue 'dispatch;
	}
	// 823BB3C4: 4BFFF8B5  bl 0x823bac78
	ctx.lr = 0x823BB3C8;
	sub_823BAC78(ctx, base);
	// 823BB3C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB3D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BB3D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB3E0 size=12
    let mut pc: u32 = 0x823BB3E0;
    'dispatch: loop {
        match pc {
            0x823BB3E0 => {
    //   block [0x823BB3E0..0x823BB3EC)
	// 823BB3E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB3E4: 986B6C7E  stb r3, 0x6c7e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27774 as u32), ctx.r[3].u8 ) };
	// 823BB3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB3F0 size=12
    let mut pc: u32 = 0x823BB3F0;
    'dispatch: loop {
        match pc {
            0x823BB3F0 => {
    //   block [0x823BB3F0..0x823BB3FC)
	// 823BB3F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB3F4: 986B6C7F  stb r3, 0x6c7f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27775 as u32), ctx.r[3].u8 ) };
	// 823BB3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB400 size=12
    let mut pc: u32 = 0x823BB400;
    'dispatch: loop {
        match pc {
            0x823BB400 => {
    //   block [0x823BB400..0x823BB40C)
	// 823BB400: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB404: 986B6C9C  stb r3, 0x6c9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27804 as u32), ctx.r[3].u8 ) };
	// 823BB408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB410 size=92
    let mut pc: u32 = 0x823BB410;
    'dispatch: loop {
        match pc {
            0x823BB410 => {
    //   block [0x823BB410..0x823BB46C)
	// 823BB410: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB414: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BB418: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB41C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BB420: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB424: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB428: 390B0014  addi r8, r11, 0x14
	ctx.r[8].s64 = ctx.r[11].s64 + 20;
	// 823BB42C: 9101FFF0  stw r8, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u32 ) };
	// 823BB430: 80CB0018  lwz r6, 0x18(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 823BB434: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 823BB438: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB43C: 90A1FFF4  stw r5, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[5].u32 ) };
	// 823BB440: E881FFF0  ld r4, -0x10(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB444: F881FFF0  std r4, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[4].u64 ) };
	// 823BB448: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 823BB44C: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 823BB450: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BB454: 419A000C  beq cr6, 0x823bb460
	if ctx.cr[6].eq {
	pc = 0x823BB460; continue 'dispatch;
	}
	// 823BB458: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 823BB45C: 419A0008  beq cr6, 0x823bb464
	if ctx.cr[6].eq {
	pc = 0x823BB464; continue 'dispatch;
	}
	// 823BB460: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823BB464: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 823BB468: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB46C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB46C size=40
    let mut pc: u32 = 0x823BB46C;
    'dispatch: loop {
        match pc {
            0x823BB46C => {
    //   block [0x823BB46C..0x823BB494)
	// 823BB46C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BB470: 409A0008  bne cr6, 0x823bb478
	if !ctx.cr[6].eq {
	pc = 0x823BB478; continue 'dispatch;
	}
	// 823BB474: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823BB478: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB47C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823BB480: 409A000C  bne cr6, 0x823bb48c
	if !ctx.cr[6].eq {
	pc = 0x823BB48C; continue 'dispatch;
	}
	// 823BB484: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823BB488: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823BB48C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB490: 4BFFFFC0  b 0x823bb450
	sub_823BB410(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB494(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB494 size=4
    let mut pc: u32 = 0x823BB494;
    'dispatch: loop {
        match pc {
            0x823BB494 => {
    //   block [0x823BB494..0x823BB498)
	// 823BB494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB498 size=32
    let mut pc: u32 = 0x823BB498;
    'dispatch: loop {
        match pc {
            0x823BB498 => {
    //   block [0x823BB498..0x823BB4B8)
	// 823BB498: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB49C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BB4A0: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BB4A4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB4A8: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BB4AC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB4B0: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB4B4: 4BFAB68C  b 0x82366b40
	sub_82366B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB4B8 size=32
    let mut pc: u32 = 0x823BB4B8;
    'dispatch: loop {
        match pc {
            0x823BB4B8 => {
    //   block [0x823BB4B8..0x823BB4D8)
	// 823BB4B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB4BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BB4C0: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BB4C4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB4C8: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BB4CC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB4D0: 80680000  lwz r3, 0(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB4D4: 4BFAB854  b 0x82366d28
	sub_82366D28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB4D8 size=12
    let mut pc: u32 = 0x823BB4D8;
    'dispatch: loop {
        match pc {
            0x823BB4D8 => {
    //   block [0x823BB4D8..0x823BB4E4)
	// 823BB4D8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB4DC: 986B940B  stb r3, -0x6bf5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27637 as u32), ctx.r[3].u8 ) };
	// 823BB4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB4E8 size=436
    let mut pc: u32 = 0x823BB4E8;
    'dispatch: loop {
        match pc {
            0x823BB4E8 => {
    //   block [0x823BB4E8..0x823BB69C)
	// 823BB4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB4F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB4F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB4F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB500: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BB504: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB508: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BB50C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB510: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB514: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 823BB518: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 823BB51C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB520: 419A0084  beq cr6, 0x823bb5a4
	if ctx.cr[6].eq {
	pc = 0x823BB5A4; continue 'dispatch;
	}
	// 823BB524: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB528: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BB52C: 419A0074  beq cr6, 0x823bb5a0
	if ctx.cr[6].eq {
	pc = 0x823BB5A0; continue 'dispatch;
	}
	// 823BB530: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823BB534: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB538: 419A0018  beq cr6, 0x823bb550
	if ctx.cr[6].eq {
	pc = 0x823BB550; continue 'dispatch;
	}
	// 823BB53C: 894B0090  lbz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 823BB540: 55490672  rlwinm r9, r10, 0, 0x19, 0x19
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 823BB544: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BB548: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 823BB54C: 409A0008  bne cr6, 0x823bb554
	if !ctx.cr[6].eq {
	pc = 0x823BB554; continue 'dispatch;
	}
	// 823BB550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823BB554: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 823BB558: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BB55C: 419A012C  beq cr6, 0x823bb688
	if ctx.cr[6].eq {
	pc = 0x823BB688; continue 'dispatch;
	}
	// 823BB560: 812B0024  lwz r9, 0x24(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 823BB564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823BB568: 552877FE  rlwinm r8, r9, 0xe, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0003FFFFu64;
	// 823BB56C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 823BB570: 419A0104  beq cr6, 0x823bb674
	if ctx.cr[6].eq {
	pc = 0x823BB674; continue 'dispatch;
	}
	// 823BB574: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 823BB578: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 823BB57C: 419A0030  beq cr6, 0x823bb5ac
	if ctx.cr[6].eq {
	pc = 0x823BB5AC; continue 'dispatch;
	}
	// 823BB580: 894A0012  lbz r10, 0x12(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(18 as u32) ) } as u64;
	// 823BB584: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BB588: 554A183E  rotlwi r10, r10, 3
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(3)) as u64;
	// 823BB58C: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823BB590: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB594: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 823BB598: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BB59C: 480000DC  b 0x823bb678
	pc = 0x823BB678; continue 'dispatch;
	// 823BB5A0: 4BDD8899  bl 0x82193e38
	ctx.lr = 0x823BB5A4;
	sub_82193E38(ctx, base);
	// 823BB5A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BB5A8: 4BFFFFA8  b 0x823bb550
	pc = 0x823BB550; continue 'dispatch;
	// 823BB5AC: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 823BB5B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 823BB5B4: 80CB004C  lwz r6, 0x4c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 823BB5B8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 823BB5BC: 7D0A3050  subf r8, r10, r6
	ctx.r[8].s64 = ctx.r[6].s64 - ctx.r[10].s64;
	// 823BB5C0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BB5C4: 7D0B1E71  srawi. r11, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[8].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BB5C8: 40810054  ble 0x823bb61c
	if !ctx.cr[0].gt {
	pc = 0x823BB61C; continue 'dispatch;
	}
	// 823BB5CC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 823BB5D0: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823BB5D4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 823BB5D8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB5DC: 2F070012  cmpwi cr6, r7, 0x12
	ctx.cr[6].compare_i32(ctx.r[7].s32, 18, &mut ctx.xer);
	// 823BB5E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 823BB5E4: 41980008  blt cr6, 0x823bb5ec
	if ctx.cr[6].lt {
	pc = 0x823BB5EC; continue 'dispatch;
	}
	// 823BB5E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 823BB5EC: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 823BB5F0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 823BB5F4: 419A0014  beq cr6, 0x823bb608
	if ctx.cr[6].eq {
	pc = 0x823BB608; continue 'dispatch;
	}
	// 823BB5F8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823BB5FC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 823BB600: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 823BB604: 4800000C  b 0x823bb610
	pc = 0x823BB610; continue 'dispatch;
	// 823BB608: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 823BB60C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 823BB610: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BB614: 4199FFB8  bgt cr6, 0x823bb5cc
	if ctx.cr[6].gt {
	pc = 0x823BB5CC; continue 'dispatch;
	}
	// 823BB618: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 823BB61C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 823BB620: 419A0040  beq cr6, 0x823bb660
	if ctx.cr[6].eq {
	pc = 0x823BB660; continue 'dispatch;
	}
	// 823BB624: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB628: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 823BB62C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BB630: 41990008  bgt cr6, 0x823bb638
	if ctx.cr[6].gt {
	pc = 0x823BB638; continue 'dispatch;
	}
	// 823BB634: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BB638: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BB63C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB640: 409A0020  bne cr6, 0x823bb660
	if !ctx.cr[6].eq {
	pc = 0x823BB660; continue 'dispatch;
	}
	// 823BB644: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823BB648: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823BB64C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BB650: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB654: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 823BB658: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BB65C: 4800001C  b 0x823bb678
	pc = 0x823BB678; continue 'dispatch;
	// 823BB660: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 823BB664: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB668: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 823BB66C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BB670: 48000008  b 0x823bb678
	pc = 0x823BB678; continue 'dispatch;
	// 823BB674: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BB678: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BB67C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB680: 419A0008  beq cr6, 0x823bb688
	if ctx.cr[6].eq {
	pc = 0x823BB688; continue 'dispatch;
	}
	// 823BB684: 9BEA0034  stb r31, 0x34(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[31].u8 ) };
	// 823BB688: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB6A0 size=12
    let mut pc: u32 = 0x823BB6A0;
    'dispatch: loop {
        match pc {
            0x823BB6A0 => {
    //   block [0x823BB6A0..0x823BB6AC)
	// 823BB6A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB6A4: 986B6B4F  stb r3, 0x6b4f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27471 as u32), ctx.r[3].u8 ) };
	// 823BB6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB6B0 size=224
    let mut pc: u32 = 0x823BB6B0;
    'dispatch: loop {
        match pc {
            0x823BB6B0 => {
    //   block [0x823BB6B0..0x823BB790)
	// 823BB6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB6B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB6BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 823BB6C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 823BB6C4: 388BAC88  addi r4, r11, -0x5378
	ctx.r[4].s64 = ctx.r[11].s64 + -21368;
	// 823BB6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB6CC: 4BE71805  bl 0x8222ced0
	ctx.lr = 0x823BB6D0;
	sub_8222CED0(ctx, base);
	// 823BB6D0: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 823BB6D4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 823BB6D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BB6DC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 823BB6E0: 48793E81  bl 0x82b4f560
	ctx.lr = 0x823BB6E4;
	sub_82B4F560(ctx, base);
	// 823BB6E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB6E8: 4BE596F1  bl 0x82214dd8
	ctx.lr = 0x823BB6EC;
	sub_82214DD8(ctx, base);
	// 823BB6EC: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 823BB6F0: 4BF85421  bl 0x82340b10
	ctx.lr = 0x823BB6F4;
	sub_82340B10(ctx, base);
	// 823BB6F4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 823BB6F8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 823BB6FC: 388A5F20  addi r4, r10, 0x5f20
	ctx.r[4].s64 = ctx.r[10].s64 + 24352;
	// 823BB700: 48683791  bl 0x82a3ee90
	ctx.lr = 0x823BB704;
	sub_82A3EE90(ctx, base);
	// 823BB704: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 823BB708: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 823BB70C: 4BFF837D  bl 0x823b3a88
	ctx.lr = 0x823BB710;
	sub_823B3A88(ctx, base);
	// 823BB710: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 823BB714: 4BF853FD  bl 0x82340b10
	ctx.lr = 0x823BB718;
	sub_82340B10(ctx, base);
	// 823BB718: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 823BB71C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 823BB720: 3889EA1C  addi r4, r9, -0x15e4
	ctx.r[4].s64 = ctx.r[9].s64 + -5604;
	// 823BB724: 4868376D  bl 0x82a3ee90
	ctx.lr = 0x823BB728;
	sub_82A3EE90(ctx, base);
	// 823BB728: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 823BB72C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 823BB730: 4BFF8359  bl 0x823b3a88
	ctx.lr = 0x823BB734;
	sub_823B3A88(ctx, base);
	// 823BB734: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BB738: 4BF853D9  bl 0x82340b10
	ctx.lr = 0x823BB73C;
	sub_82340B10(ctx, base);
	// 823BB73C: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 823BB740: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BB744: 3888EA28  addi r4, r8, -0x15d8
	ctx.r[4].s64 = ctx.r[8].s64 + -5592;
	// 823BB748: 48683749  bl 0x82a3ee90
	ctx.lr = 0x823BB74C;
	sub_82A3EE90(ctx, base);
	// 823BB74C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 823BB750: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 823BB754: 4BFF8335  bl 0x823b3a88
	ctx.lr = 0x823BB758;
	sub_823B3A88(ctx, base);
	// 823BB758: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 823BB75C: 48794415  bl 0x82b4fb70
	ctx.lr = 0x823BB760;
	sub_82B4FB70(ctx, base);
	// 823BB760: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BB764: 480038A5  bl 0x823bf008
	ctx.lr = 0x823BB768;
	sub_823BF008(ctx, base);
	// 823BB768: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 823BB76C: 4800389D  bl 0x823bf008
	ctx.lr = 0x823BB770;
	sub_823BF008(ctx, base);
	// 823BB770: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 823BB774: 48003895  bl 0x823bf008
	ctx.lr = 0x823BB778;
	sub_823BF008(ctx, base);
	// 823BB778: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 823BB77C: 4BFF81BD  bl 0x823b3938
	ctx.lr = 0x823BB780;
	sub_823B3938(ctx, base);
	// 823BB780: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823BB784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB790 size=80
    let mut pc: u32 = 0x823BB790;
    'dispatch: loop {
        match pc {
            0x823BB790 => {
    //   block [0x823BB790..0x823BB7E0)
	// 823BB790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB798: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB79C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB7A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB7A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823BB7A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 823BB7AC: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 823BB7B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB7B4: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BB7B8: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB7BC: 4BE71715  bl 0x8222ced0
	ctx.lr = 0x823BB7C0;
	sub_8222CED0(ctx, base);
	// 823BB7C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BB7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB7C8: 4BF56D69  bl 0x82312530
	ctx.lr = 0x823BB7CC;
	sub_82312530(ctx, base);
	// 823BB7CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB7D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB7E0 size=140
    let mut pc: u32 = 0x823BB7E0;
    'dispatch: loop {
        match pc {
            0x823BB7E0 => {
    //   block [0x823BB7E0..0x823BB86C)
	// 823BB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB7E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BB7EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB7F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB7F4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB7F8: 806B6AB8  lwz r3, 0x6ab8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BB7FC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB800: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 823BB804: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 823BB808: 4E800421  bctrl
	ctx.lr = 0x823BB80C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BB80C: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB810: 83E80000  lwz r31, 0(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB814: 486938CD  bl 0x82a4f0e0
	ctx.lr = 0x823BB818;
	sub_82A4F0E0(ctx, base);
	// 823BB818: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 823BB81C: 57E6103A  slwi r6, r31, 2
	ctx.r[6].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 823BB820: 38A7F798  addi r5, r7, -0x868
	ctx.r[5].s64 = ctx.r[7].s64 + -2152;
	// 823BB824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB828: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BB82C: 7C86582E  lwzx r4, r6, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 823BB830: 83E40D50  lwz r31, 0xd50(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(3408 as u32) ) } as u64;
	// 823BB834: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BB838: 8BDF0040  lbz r30, 0x40(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 823BB83C: 4BE34A05  bl 0x821f0240
	ctx.lr = 0x823BB840;
	sub_821F0240(ctx, base);
	// 823BB840: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BB844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB848: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823BB84C: 486998F5  bl 0x82a55140
	ctx.lr = 0x823BB850;
	sub_82A55140(ctx, base);
	// 823BB850: 48693989  bl 0x82a4f1d8
	ctx.lr = 0x823BB854;
	sub_82A4F1D8(ctx, base);
	// 823BB854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB860: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BB864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB870 size=12
    let mut pc: u32 = 0x823BB870;
    'dispatch: loop {
        match pc {
            0x823BB870 => {
    //   block [0x823BB870..0x823BB87C)
	// 823BB870: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB874: 986B9417  stb r3, -0x6be9(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27625 as u32), ctx.r[3].u8 ) };
	// 823BB878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB880 size=12
    let mut pc: u32 = 0x823BB880;
    'dispatch: loop {
        match pc {
            0x823BB880 => {
    //   block [0x823BB880..0x823BB88C)
	// 823BB880: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB884: 986B9474  stb r3, -0x6b8c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27532 as u32), ctx.r[3].u8 ) };
	// 823BB888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB890 size=12
    let mut pc: u32 = 0x823BB890;
    'dispatch: loop {
        match pc {
            0x823BB890 => {
    //   block [0x823BB890..0x823BB89C)
	// 823BB890: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB894: 986B6B64  stb r3, 0x6b64(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27492 as u32), ctx.r[3].u8 ) };
	// 823BB898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB8A0 size=12
    let mut pc: u32 = 0x823BB8A0;
    'dispatch: loop {
        match pc {
            0x823BB8A0 => {
    //   block [0x823BB8A0..0x823BB8AC)
	// 823BB8A0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB8A4: 986B9475  stb r3, -0x6b8b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27531 as u32), ctx.r[3].u8 ) };
	// 823BB8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB8B0 size=12
    let mut pc: u32 = 0x823BB8B0;
    'dispatch: loop {
        match pc {
            0x823BB8B0 => {
    //   block [0x823BB8B0..0x823BB8BC)
	// 823BB8B0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB8B4: 986B9476  stb r3, -0x6b8a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27530 as u32), ctx.r[3].u8 ) };
	// 823BB8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB8C0 size=12
    let mut pc: u32 = 0x823BB8C0;
    'dispatch: loop {
        match pc {
            0x823BB8C0 => {
    //   block [0x823BB8C0..0x823BB8CC)
	// 823BB8C0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB8C4: 986B9477  stb r3, -0x6b89(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27529 as u32), ctx.r[3].u8 ) };
	// 823BB8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB8D0 size=12
    let mut pc: u32 = 0x823BB8D0;
    'dispatch: loop {
        match pc {
            0x823BB8D0 => {
    //   block [0x823BB8D0..0x823BB8DC)
	// 823BB8D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB8D4: 986B6B65  stb r3, 0x6b65(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27493 as u32), ctx.r[3].u8 ) };
	// 823BB8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BB8E0 size=12
    let mut pc: u32 = 0x823BB8E0;
    'dispatch: loop {
        match pc {
            0x823BB8E0 => {
    //   block [0x823BB8E0..0x823BB8EC)
	// 823BB8E0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB8E4: D02B9478  stfs f1, -0x6b88(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27528 as u32), tmp.u32 ) };
	// 823BB8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BB8F0 size=20
    let mut pc: u32 = 0x823BB8F0;
    'dispatch: loop {
        match pc {
            0x823BB8F0 => {
    //   block [0x823BB8F0..0x823BB904)
	// 823BB8F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB8F4: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823BB8F8: 392B947C  addi r9, r11, -0x6b84
	ctx.r[9].s64 = ctx.r[11].s64 + -27524;
	// 823BB8FC: 7C2A4D2E  stfsx f1, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 823BB900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BB908 size=20
    let mut pc: u32 = 0x823BB908;
    'dispatch: loop {
        match pc {
            0x823BB908 => {
    //   block [0x823BB908..0x823BB91C)
	// 823BB908: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB90C: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823BB910: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 823BB914: 7C2A4D2E  stfsx f1, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 823BB918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB920 size=16
    let mut pc: u32 = 0x823BB920;
    'dispatch: loop {
        match pc {
            0x823BB920 => {
    //   block [0x823BB920..0x823BB930)
	// 823BB920: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB924: 394B94A4  addi r10, r11, -0x6b5c
	ctx.r[10].s64 = ctx.r[11].s64 + -27484;
	// 823BB928: 7C8351AE  stbx r4, r3, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[10].u32), ctx.r[4].u8) };
	// 823BB92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB930 size=28
    let mut pc: u32 = 0x823BB930;
    'dispatch: loop {
        match pc {
            0x823BB930 => {
    //   block [0x823BB930..0x823BB94C)
	// 823BB930: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB934: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 823BB938: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 823BB93C: 906B94AC  stw r3, -0x6b54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27476 as u32), ctx.r[3].u32 ) };
	// 823BB940: 908A94B0  stw r4, -0x6b50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27472 as u32), ctx.r[4].u32 ) };
	// 823BB944: 90A994B4  stw r5, -0x6b4c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-27468 as u32), ctx.r[5].u32 ) };
	// 823BB948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB950 size=12
    let mut pc: u32 = 0x823BB950;
    'dispatch: loop {
        match pc {
            0x823BB950 => {
    //   block [0x823BB950..0x823BB95C)
	// 823BB950: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB954: 986B6B67  stb r3, 0x6b67(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27495 as u32), ctx.r[3].u8 ) };
	// 823BB958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB960 size=12
    let mut pc: u32 = 0x823BB960;
    'dispatch: loop {
        match pc {
            0x823BB960 => {
    //   block [0x823BB960..0x823BB96C)
	// 823BB960: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB964: 986B6B66  stb r3, 0x6b66(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27494 as u32), ctx.r[3].u8 ) };
	// 823BB968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB970 size=12
    let mut pc: u32 = 0x823BB970;
    'dispatch: loop {
        match pc {
            0x823BB970 => {
    //   block [0x823BB970..0x823BB97C)
	// 823BB970: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB974: 886B6B66  lbz r3, 0x6b66(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(27494 as u32) ) } as u64;
	// 823BB978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB980 size=12
    let mut pc: u32 = 0x823BB980;
    'dispatch: loop {
        match pc {
            0x823BB980 => {
    //   block [0x823BB980..0x823BB98C)
	// 823BB980: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB984: 986B6B68  stb r3, 0x6b68(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27496 as u32), ctx.r[3].u8 ) };
	// 823BB988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB990 size=12
    let mut pc: u32 = 0x823BB990;
    'dispatch: loop {
        match pc {
            0x823BB990 => {
    //   block [0x823BB990..0x823BB99C)
	// 823BB990: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB994: 986B6B69  stb r3, 0x6b69(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27497 as u32), ctx.r[3].u8 ) };
	// 823BB998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB9A0 size=12
    let mut pc: u32 = 0x823BB9A0;
    'dispatch: loop {
        match pc {
            0x823BB9A0 => {
    //   block [0x823BB9A0..0x823BB9AC)
	// 823BB9A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB9A4: 986B6B6A  stb r3, 0x6b6a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27498 as u32), ctx.r[3].u8 ) };
	// 823BB9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB9B0 size=12
    let mut pc: u32 = 0x823BB9B0;
    'dispatch: loop {
        match pc {
            0x823BB9B0 => {
    //   block [0x823BB9B0..0x823BB9BC)
	// 823BB9B0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB9B4: 906B94B8  stw r3, -0x6b48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27464 as u32), ctx.r[3].u32 ) };
	// 823BB9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB9C0 size=12
    let mut pc: u32 = 0x823BB9C0;
    'dispatch: loop {
        match pc {
            0x823BB9C0 => {
    //   block [0x823BB9C0..0x823BB9CC)
	// 823BB9C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BB9C4: 986B6B6C  stb r3, 0x6b6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27500 as u32), ctx.r[3].u8 ) };
	// 823BB9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB9D0 size=12
    let mut pc: u32 = 0x823BB9D0;
    'dispatch: loop {
        match pc {
            0x823BB9D0 => {
    //   block [0x823BB9D0..0x823BB9DC)
	// 823BB9D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB9D4: 906B94BC  stw r3, -0x6b44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27460 as u32), ctx.r[3].u32 ) };
	// 823BB9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB9E0 size=12
    let mut pc: u32 = 0x823BB9E0;
    'dispatch: loop {
        match pc {
            0x823BB9E0 => {
    //   block [0x823BB9E0..0x823BB9EC)
	// 823BB9E0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BB9E4: 906B94C0  stw r3, -0x6b40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27456 as u32), ctx.r[3].u32 ) };
	// 823BB9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823BB9F0 size=128
    let mut pc: u32 = 0x823BB9F0;
    'dispatch: loop {
        match pc {
            0x823BB9F0 => {
    //   block [0x823BB9F0..0x823BBA70)
	// 823BB9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB9F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB9FC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BBA00: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBA04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BBA08: 806B6AB8  lwz r3, 0x6ab8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BBA0C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BBA10: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 823BBA14: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 823BBA18: 4E800421  bctrl
	ctx.lr = 0x823BBA1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BBA1C: 7FE807B4  extsw r8, r31
	ctx.r[8].s64 = ctx.r[31].s32 as i64;
	// 823BBA20: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBA24: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 823BBA28: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 823BBA2C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823BBA30: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 823BBA34: 38661688  addi r3, r6, 0x1688
	ctx.r[3].s64 = ctx.r[6].s64 + 5768;
	// 823BBA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BBA3C: 80870000  lwz r4, 0(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BBA40: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 823BBA44: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 823BBA48: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 823BBA4C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 823BBA50: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823BBA54: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BBA58: 4BE22891  bl 0x821de2e8
	ctx.lr = 0x823BBA5C;
	sub_821DE2E8(ctx, base);
	// 823BBA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BBA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BBA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BBA68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BBA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BBA70 size=144
    let mut pc: u32 = 0x823BBA70;
    'dispatch: loop {
        match pc {
            0x823BBA70 => {
    //   block [0x823BBA70..0x823BBB00)
	// 823BBA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BBA74: 488ED995  bl 0x82ca9408
	ctx.lr = 0x823BBA78;
	sub_82CA93D0(ctx, base);
	// 823BBA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BBA7C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBA80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823BBA84: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823BBA88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823BBA8C: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BBA90: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BBA94: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BBA98: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBA9C: 83E80008  lwz r31, 8(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BBAA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBAA4: 38830014  addi r4, r3, 0x14
	ctx.r[4].s64 = ctx.r[3].s64 + 20;
	// 823BBAA8: 4BF60F71  bl 0x8231ca18
	ctx.lr = 0x823BBAAC;
	sub_8231CA18(ctx, base);
	// 823BBAAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBAB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BBAB4: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 823BBAB8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823BBABC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823BBAC0: 4BF60F59  bl 0x8231ca18
	ctx.lr = 0x823BBAC4;
	sub_8231CA18(ctx, base);
	// 823BBAC4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823BBAC8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 823BBACC: 419A0028  beq cr6, 0x823bbaf4
	if ctx.cr[6].eq {
	pc = 0x823BBAF4; continue 'dispatch;
	}
	// 823BBAD0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823BBAD4: 419A0020  beq cr6, 0x823bbaf4
	if ctx.cr[6].eq {
	pc = 0x823BBAF4; continue 'dispatch;
	}
	// 823BBAD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBADC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 823BBAE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BBAE4: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BBAE8: 48157FA1  bl 0x82513a88
	ctx.lr = 0x823BBAEC;
	sub_82513A88(ctx, base);
	// 823BBAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BBAF0: 488ED968  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 823BBAF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823BBAF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BBAFC: 488ED95C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BBB00 size=172
    let mut pc: u32 = 0x823BBB00;
    'dispatch: loop {
        match pc {
            0x823BBB00 => {
    //   block [0x823BBB00..0x823BBBAC)
	// 823BBB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BBB04: 488ED905  bl 0x82ca9408
	ctx.lr = 0x823BBB08;
	sub_82CA93D0(ctx, base);
	// 823BBB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BBB0C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBB10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BBB14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BBB18: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 823BBB1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823BBB20: 816B6AB8  lwz r11, 0x6ab8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27320 as u32) ) } as u64;
	// 823BBB24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BBB28: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BBB2C: 812A0058  lwz r9, 0x58(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 823BBB30: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBB34: 83A80008  lwz r29, 8(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BBB38: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBB3C: 8067000C  lwz r3, 0xc(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BBB40: 4815E351  bl 0x82519e90
	ctx.lr = 0x823BBB44;
	sub_82519E90(ctx, base);
	// 823BBB44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BBB48: 419A000C  beq cr6, 0x823bbb54
	if ctx.cr[6].eq {
	pc = 0x823BBB54; continue 'dispatch;
	}
	// 823BBB4C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BBB50: 48000008  b 0x823bbb58
	pc = 0x823BBB58; continue 'dispatch;
	// 823BBB54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BBB58: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823BBB5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BBB60: 419A0044  beq cr6, 0x823bbba4
	if ctx.cr[6].eq {
	pc = 0x823BBBA4; continue 'dispatch;
	}
	// 823BBB64: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBB68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823BBB6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BBB70: 83CB000C  lwz r30, 0xc(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BBB74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BBB78: 4815E319  bl 0x82519e90
	ctx.lr = 0x823BBB7C;
	sub_82519E90(ctx, base);
	// 823BBB7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823BBB80: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 823BBB84: 419A0020  beq cr6, 0x823bbba4
	if ctx.cr[6].eq {
	pc = 0x823BBBA4; continue 'dispatch;
	}
	// 823BBB88: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BBB8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BBB90: 419A0014  beq cr6, 0x823bbba4
	if ctx.cr[6].eq {
	pc = 0x823BBBA4; continue 'dispatch;
	}
	// 823BBB94: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 823BBB98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BBB9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BBBA0: 4815DF69  bl 0x82519b08
	ctx.lr = 0x823BBBA4;
	sub_82519B08(ctx, base);
	// 823BBBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BBBA8: 488ED8B0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BBBB0 size=192
    let mut pc: u32 = 0x823BBBB0;
    'dispatch: loop {
        match pc {
            0x823BBBB0 => {
    //   block [0x823BBBB0..0x823BBC70)
	// 823BBBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BBBB4: 488ED859  bl 0x82ca940c
	ctx.lr = 0x823BBBB8;
	sub_82CA93D0(ctx, base);
	// 823BBBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BBBBC: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBBC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BBBC4: 3BEB79E8  addi r31, r11, 0x79e8
	ctx.r[31].s64 = ctx.r[11].s64 + 31208;
	// 823BBBC8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823BBBCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBBD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BBBD4: 419A0014  beq cr6, 0x823bbbe8
	if ctx.cr[6].eq {
	pc = 0x823BBBE8; continue 'dispatch;
	}
	// 823BBBD8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BBBDC: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 823BBBE0: 7D291671  srawi. r9, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 823BBBE4: 40820048  bne 0x823bbc2c
	if !ctx.cr[0].eq {
	pc = 0x823BBC2C; continue 'dispatch;
	}
	// 823BBBE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823BBBF0: 38EB7088  addi r7, r11, 0x7088
	ctx.r[7].s64 = ctx.r[11].s64 + 28808;
	// 823BBBF4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 823BBBF8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823BBBFC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BBC00: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823BBC04: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823BBC08: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823BBC0C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BBC10: 4082FFE8  bne 0x823bbbf8
	if !ctx.cr[0].eq {
	pc = 0x823BBBF8; continue 'dispatch;
	}
	// 823BBC14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BBC18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823BBC1C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 823BBC20: 48003451  bl 0x823bf070
	ctx.lr = 0x823BBC24;
	sub_823BF070(ctx, base);
	// 823BBC24: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BBC28: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BBC2C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 823BBC30: 41980038  blt cr6, 0x823bbc68
	if ctx.cr[6].lt {
	pc = 0x823BBC68; continue 'dispatch;
	}
	// 823BBC34: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 823BBC38: 40980030  bge cr6, 0x823bbc68
	if !ctx.cr[6].lt {
	pc = 0x823BBC68; continue 'dispatch;
	}
	// 823BBC3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BBC40: 419A0014  beq cr6, 0x823bbc54
	if ctx.cr[6].eq {
	pc = 0x823BBC54; continue 'dispatch;
	}
	// 823BBC44: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 823BBC48: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 823BBC4C: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823BBC50: 41980008  blt cr6, 0x823bbc58
	if ctx.cr[6].lt {
	pc = 0x823BBC58; continue 'dispatch;
	}
	// 823BBC54: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 823BBC58: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823BBC5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BBC60: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823BBC64: 4BEA953D  bl 0x822651a0
	ctx.lr = 0x823BBC68;
	sub_822651A0(ctx, base);
	// 823BBC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BBC6C: 488ED7F0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBC70 size=16
    let mut pc: u32 = 0x823BBC70;
    'dispatch: loop {
        match pc {
            0x823BBC70 => {
    //   block [0x823BBC70..0x823BBC80)
	// 823BBC70: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBC74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BBC78: 386B79F8  addi r3, r11, 0x79f8
	ctx.r[3].s64 = ctx.r[11].s64 + 31224;
	// 823BBC7C: 4BEA9524  b 0x822651a0
	sub_822651A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBC80 size=16
    let mut pc: u32 = 0x823BBC80;
    'dispatch: loop {
        match pc {
            0x823BBC80 => {
    //   block [0x823BBC80..0x823BBC90)
	// 823BBC80: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBC84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BBC88: 386B79FC  addi r3, r11, 0x79fc
	ctx.r[3].s64 = ctx.r[11].s64 + 31228;
	// 823BBC8C: 4BEA9514  b 0x822651a0
	sub_822651A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBC90 size=12
    let mut pc: u32 = 0x823BBC90;
    'dispatch: loop {
        match pc {
            0x823BBC90 => {
    //   block [0x823BBC90..0x823BBC9C)
	// 823BBC90: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBC94: 986B6B6D  stb r3, 0x6b6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27501 as u32), ctx.r[3].u8 ) };
	// 823BBC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBCA0 size=12
    let mut pc: u32 = 0x823BBCA0;
    'dispatch: loop {
        match pc {
            0x823BBCA0 => {
    //   block [0x823BBCA0..0x823BBCAC)
	// 823BBCA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBCA4: 986B6B6E  stb r3, 0x6b6e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27502 as u32), ctx.r[3].u8 ) };
	// 823BBCA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBCB0 size=12
    let mut pc: u32 = 0x823BBCB0;
    'dispatch: loop {
        match pc {
            0x823BBCB0 => {
    //   block [0x823BBCB0..0x823BBCBC)
	// 823BBCB0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBCB4: 986B6B6F  stb r3, 0x6b6f(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27503 as u32), ctx.r[3].u8 ) };
	// 823BBCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBCC0 size=12
    let mut pc: u32 = 0x823BBCC0;
    'dispatch: loop {
        match pc {
            0x823BBCC0 => {
    //   block [0x823BBCC0..0x823BBCCC)
	// 823BBCC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBCC4: 986B6B70  stb r3, 0x6b70(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27504 as u32), ctx.r[3].u8 ) };
	// 823BBCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBCD0 size=12
    let mut pc: u32 = 0x823BBCD0;
    'dispatch: loop {
        match pc {
            0x823BBCD0 => {
    //   block [0x823BBCD0..0x823BBCDC)
	// 823BBCD0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBCD4: 986B94A9  stb r3, -0x6b57(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27479 as u32), ctx.r[3].u8 ) };
	// 823BBCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBCE0 size=12
    let mut pc: u32 = 0x823BBCE0;
    'dispatch: loop {
        match pc {
            0x823BBCE0 => {
    //   block [0x823BBCE0..0x823BBCEC)
	// 823BBCE0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBCE4: 986B6B71  stb r3, 0x6b71(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27505 as u32), ctx.r[3].u8 ) };
	// 823BBCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBCF0 size=12
    let mut pc: u32 = 0x823BBCF0;
    'dispatch: loop {
        match pc {
            0x823BBCF0 => {
    //   block [0x823BBCF0..0x823BBCFC)
	// 823BBCF0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBCF4: D02B94C4  stfs f1, -0x6b3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27452 as u32), tmp.u32 ) };
	// 823BBCF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBD00 size=12
    let mut pc: u32 = 0x823BBD00;
    'dispatch: loop {
        match pc {
            0x823BBD00 => {
    //   block [0x823BBD00..0x823BBD0C)
	// 823BBD00: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBD04: D02B94C8  stfs f1, -0x6b38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27448 as u32), tmp.u32 ) };
	// 823BBD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD10 size=12
    let mut pc: u32 = 0x823BBD10;
    'dispatch: loop {
        match pc {
            0x823BBD10 => {
    //   block [0x823BBD10..0x823BBD1C)
	// 823BBD10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBD14: 986B6B72  stb r3, 0x6b72(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27506 as u32), ctx.r[3].u8 ) };
	// 823BBD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD20 size=12
    let mut pc: u32 = 0x823BBD20;
    'dispatch: loop {
        match pc {
            0x823BBD20 => {
    //   block [0x823BBD20..0x823BBD2C)
	// 823BBD20: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBD24: 986B6B73  stb r3, 0x6b73(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27507 as u32), ctx.r[3].u8 ) };
	// 823BBD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBD30 size=12
    let mut pc: u32 = 0x823BBD30;
    'dispatch: loop {
        match pc {
            0x823BBD30 => {
    //   block [0x823BBD30..0x823BBD3C)
	// 823BBD30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBD34: D02B6B74  stfs f1, 0x6b74(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(27508 as u32), tmp.u32 ) };
	// 823BBD38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD40 size=12
    let mut pc: u32 = 0x823BBD40;
    'dispatch: loop {
        match pc {
            0x823BBD40 => {
    //   block [0x823BBD40..0x823BBD4C)
	// 823BBD40: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBD44: 986B6B78  stb r3, 0x6b78(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27512 as u32), ctx.r[3].u8 ) };
	// 823BBD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD50 size=12
    let mut pc: u32 = 0x823BBD50;
    'dispatch: loop {
        match pc {
            0x823BBD50 => {
    //   block [0x823BBD50..0x823BBD5C)
	// 823BBD50: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBD54: 986B6B79  stb r3, 0x6b79(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27513 as u32), ctx.r[3].u8 ) };
	// 823BBD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD60 size=12
    let mut pc: u32 = 0x823BBD60;
    'dispatch: loop {
        match pc {
            0x823BBD60 => {
    //   block [0x823BBD60..0x823BBD6C)
	// 823BBD60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBD64: 986B6B7A  stb r3, 0x6b7a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27514 as u32), ctx.r[3].u8 ) };
	// 823BBD68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD70 size=12
    let mut pc: u32 = 0x823BBD70;
    'dispatch: loop {
        match pc {
            0x823BBD70 => {
    //   block [0x823BBD70..0x823BBD7C)
	// 823BBD70: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBD74: 906B94CC  stw r3, -0x6b34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27444 as u32), ctx.r[3].u32 ) };
	// 823BBD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD80 size=12
    let mut pc: u32 = 0x823BBD80;
    'dispatch: loop {
        match pc {
            0x823BBD80 => {
    //   block [0x823BBD80..0x823BBD8C)
	// 823BBD80: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBD84: 986B6B7B  stb r3, 0x6b7b(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(27515 as u32), ctx.r[3].u8 ) };
	// 823BBD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BBD90 size=12
    let mut pc: u32 = 0x823BBD90;
    'dispatch: loop {
        match pc {
            0x823BBD90 => {
    //   block [0x823BBD90..0x823BBD9C)
	// 823BBD90: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBD94: 986B94AA  stb r3, -0x6b56(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-27478 as u32), ctx.r[3].u8 ) };
	// 823BBD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBDA0 size=12
    let mut pc: u32 = 0x823BBDA0;
    'dispatch: loop {
        match pc {
            0x823BBDA0 => {
    //   block [0x823BBDA0..0x823BBDAC)
	// 823BBDA0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBDA4: D02B94D0  stfs f1, -0x6b30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27440 as u32), tmp.u32 ) };
	// 823BBDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBDB0 size=12
    let mut pc: u32 = 0x823BBDB0;
    'dispatch: loop {
        match pc {
            0x823BBDB0 => {
    //   block [0x823BBDB0..0x823BBDBC)
	// 823BBDB0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 823BBDB4: D02B6B7C  stfs f1, 0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(27516 as u32), tmp.u32 ) };
	// 823BBDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBDC0 size=12
    let mut pc: u32 = 0x823BBDC0;
    'dispatch: loop {
        match pc {
            0x823BBDC0 => {
    //   block [0x823BBDC0..0x823BBDCC)
	// 823BBDC0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBDC4: D02B94D4  stfs f1, -0x6b2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27436 as u32), tmp.u32 ) };
	// 823BBDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBDD0 size=12
    let mut pc: u32 = 0x823BBDD0;
    'dispatch: loop {
        match pc {
            0x823BBDD0 => {
    //   block [0x823BBDD0..0x823BBDDC)
	// 823BBDD0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBDD4: D02B94D8  stfs f1, -0x6b28(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27432 as u32), tmp.u32 ) };
	// 823BBDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBDE0 size=12
    let mut pc: u32 = 0x823BBDE0;
    'dispatch: loop {
        match pc {
            0x823BBDE0 => {
    //   block [0x823BBDE0..0x823BBDEC)
	// 823BBDE0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBDE4: D02B94DC  stfs f1, -0x6b24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27428 as u32), tmp.u32 ) };
	// 823BBDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BBDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823BBDF0 size=12
    let mut pc: u32 = 0x823BBDF0;
    'dispatch: loop {
        match pc {
            0x823BBDF0 => {
    //   block [0x823BBDF0..0x823BBDFC)
	// 823BBDF0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 823BBDF4: D02B94E0  stfs f1, -0x6b20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-27424 as u32), tmp.u32 ) };
	// 823BBDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


