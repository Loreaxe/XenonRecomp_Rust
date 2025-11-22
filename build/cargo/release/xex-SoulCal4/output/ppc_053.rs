pub fn sub_82436C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82436C28 size=160
    let mut pc: u32 = 0x82436C28;
    'dispatch: loop {
        match pc {
            0x82436C28 => {
    //   block [0x82436C28..0x82436CC8)
	// 82436C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436C30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436C34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436C38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436C3C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82436C40: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 82436C44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436C48: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436C4C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82436C50: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82436C54: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82436C58: 4BFFC389  bl 0x82432fe0
	ctx.lr = 0x82436C5C;
	sub_82432FE0(ctx, base);
	// 82436C5C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82436C60: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82436C64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436C68: 4BFFC379  bl 0x82432fe0
	ctx.lr = 0x82436C6C;
	sub_82432FE0(ctx, base);
	// 82436C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436C70: 4BFFFC91  bl 0x82436900
	ctx.lr = 0x82436C74;
	sub_82436900(ctx, base);
	// 82436C74: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82436C78: 419A002C  beq cr6, 0x82436ca4
	if ctx.cr[6].eq {
	pc = 0x82436CA4; continue 'dispatch;
	}
	// 82436C7C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82436C80: 419A0024  beq cr6, 0x82436ca4
	if ctx.cr[6].eq {
	pc = 0x82436CA4; continue 'dispatch;
	}
	// 82436C84: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82436C88: 419A0014  beq cr6, 0x82436c9c
	if ctx.cr[6].eq {
	pc = 0x82436C9C; continue 'dispatch;
	}
	// 82436C8C: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 82436C90: 419A000C  beq cr6, 0x82436c9c
	if ctx.cr[6].eq {
	pc = 0x82436C9C; continue 'dispatch;
	}
	// 82436C94: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82436C98: 48000010  b 0x82436ca8
	pc = 0x82436CA8; continue 'dispatch;
	// 82436C9C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82436CA0: 48000008  b 0x82436ca8
	pc = 0x82436CA8; continue 'dispatch;
	// 82436CA4: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82436CA8: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82436CAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436CB0: 48009329  bl 0x8243ffd8
	ctx.lr = 0x82436CB4;
	sub_8243FFD8(ctx, base);
	// 82436CB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436CC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82436CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436CC8 size=200
    let mut pc: u32 = 0x82436CC8;
    'dispatch: loop {
        match pc {
            0x82436CC8 => {
    //   block [0x82436CC8..0x82436D90)
	// 82436CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436CD4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82436CD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82436CDC: 386A59E0  addi r3, r10, 0x59e0
	ctx.r[3].s64 = ctx.r[10].s64 + 23008;
	// 82436CE0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82436CE4: 38803880  li r4, 0x3880
	ctx.r[4].s64 = 14464;
	// 82436CE8: E94A5674  ld r10, 0x5674(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(22132 as u32) ) };
	// 82436CEC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82436CF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82436CF4: 48010AF5  bl 0x824477e8
	ctx.lr = 0x82436CF8;
	sub_824477E8(ctx, base);
	// 82436CF8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82436CFC: 419A0024  beq cr6, 0x82436d20
	if ctx.cr[6].eq {
	pc = 0x82436D20; continue 'dispatch;
	}
	// 82436D00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436D04: 386B59AC  addi r3, r11, 0x59ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22956;
	// 82436D08: 480003C1  bl 0x824370c8
	ctx.lr = 0x82436D0C;
	sub_824370C8(ctx, base);
	// 82436D0C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82436D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D1C: 4E800020  blr
	return;
	// 82436D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82436D24: 48010D9D  bl 0x82447ac0
	ctx.lr = 0x82436D28;
	sub_82447AC0(ctx, base);
	// 82436D28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436D2C: 419A001C  beq cr6, 0x82436d48
	if ctx.cr[6].eq {
	pc = 0x82436D48; continue 'dispatch;
	}
	// 82436D30: 3860FED3  li r3, -0x12d
	ctx.r[3].s64 = -301;
	// 82436D34: 4BFFFC95  bl 0x824369c8
	ctx.lr = 0x82436D38;
	sub_824369C8(ctx, base);
	// 82436D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D44: 4E800020  blr
	return;
	// 82436D48: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82436D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82436D50: 388B69F8  addi r4, r11, 0x69f8
	ctx.r[4].s64 = ctx.r[11].s64 + 27128;
	// 82436D54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436D58: 48010C39  bl 0x82447990
	ctx.lr = 0x82436D5C;
	sub_82447990(ctx, base);
	// 82436D5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436D60: 419A001C  beq cr6, 0x82436d7c
	if ctx.cr[6].eq {
	pc = 0x82436D7C; continue 'dispatch;
	}
	// 82436D64: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 82436D68: 4BFFFC61  bl 0x824369c8
	ctx.lr = 0x82436D6C;
	sub_824369C8(ctx, base);
	// 82436D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D78: 4E800020  blr
	return;
	// 82436D7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82436D90 size=392
    let mut pc: u32 = 0x82436D90;
    'dispatch: loop {
        match pc {
            0x82436D90 => {
    //   block [0x82436D90..0x82436F18)
	// 82436D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436D98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82436D9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436DA0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82436DA4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436DAC: 409A0014  bne cr6, 0x82436dc0
	if !ctx.cr[6].eq {
	pc = 0x82436DC0; continue 'dispatch;
	}
	// 82436DB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436DB4: 386B5A2C  addi r3, r11, 0x5a2c
	ctx.r[3].s64 = ctx.r[11].s64 + 23084;
	// 82436DB8: 48000311  bl 0x824370c8
	ctx.lr = 0x82436DBC;
	sub_824370C8(ctx, base);
	// 82436DBC: 48000140  b 0x82436efc
	pc = 0x82436EFC; continue 'dispatch;
	// 82436DC0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82436DC4: C3E30000  lfs f31, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82436DC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82436DCC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82436DD0: 3D20828A  lis r9, -0x7d76
	ctx.r[9].s64 = -2104885248;
	// 82436DD4: 394A55E8  addi r10, r10, 0x55e8
	ctx.r[10].s64 = ctx.r[10].s64 + 21992;
	// 82436DD8: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 82436DDC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 82436DE0: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 82436DE4: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 82436DE8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436DEC: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82436DF0: 91499CD0  stw r10, -0x6330(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25392 as u32), ctx.r[10].u32 ) };
	// 82436DF4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82436DF8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82436DFC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82436E00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82436E04: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82436E08: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82436E0C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82436E10: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82436E14: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82436E18: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82436E1C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82436E20: 48000179  bl 0x82436f98
	ctx.lr = 0x82436E24;
	sub_82436F98(ctx, base);
	// 82436E24: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 82436E28: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82436E2C: 817E9DD8  lwz r11, -0x6228(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25128 as u32) ) } as u64;
	// 82436E30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82436E34: 409A00C0  bne cr6, 0x82436ef4
	if !ctx.cr[6].eq {
	pc = 0x82436EF4; continue 'dispatch;
	}
	// 82436E38: 48000319  bl 0x82437150
	ctx.lr = 0x82436E3C;
	sub_82437150(ctx, base);
	// 82436E3C: 4BFFE805  bl 0x82435640
	ctx.lr = 0x82436E40;
	sub_82435640(ctx, base);
	// 82436E40: 4BFE4EE1  bl 0x8241bd20
	ctx.lr = 0x82436E44;
	sub_8241BD20(ctx, base);
	// 82436E44: 4BFEBF25  bl 0x82422d68
	ctx.lr = 0x82436E48;
	sub_82422D68(ctx, base);
	// 82436E48: 4BFEE159  bl 0x82424fa0
	ctx.lr = 0x82436E4C;
	sub_82424FA0(ctx, base);
	// 82436E4C: 4BFF4565  bl 0x8242b3b0
	ctx.lr = 0x82436E50;
	sub_8242B3B0(ctx, base);
	// 82436E50: 480008B9  bl 0x82437708
	ctx.lr = 0x82436E54;
	sub_82437708(ctx, base);
	// 82436E54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436E58: 419A0018  beq cr6, 0x82436e70
	if ctx.cr[6].eq {
	pc = 0x82436E70; continue 'dispatch;
	}
	// 82436E5C: 3860FF9B  li r3, -0x65
	ctx.r[3].s64 = -101;
	// 82436E60: 4BFFFB69  bl 0x824369c8
	ctx.lr = 0x82436E64;
	sub_824369C8(ctx, base);
	// 82436E64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436E68: 386B5A04  addi r3, r11, 0x5a04
	ctx.r[3].s64 = ctx.r[11].s64 + 23044;
	// 82436E6C: 4800025D  bl 0x824370c8
	ctx.lr = 0x82436E70;
	sub_824370C8(ctx, base);
	// 82436E70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82436E74: 4BFFF995  bl 0x82436808
	ctx.lr = 0x82436E78;
	sub_82436808(ctx, base);
	// 82436E78: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82436E7C: C00B204C  lfs f0, 0x204c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436E80: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82436E84: 93EB06FC  stw r31, 0x6fc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1788 as u32), ctx.r[31].u32 ) };
	// 82436E88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82436E8C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82436E90: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82436E94: EC1F683A  fmadds f0, f31, f0, f13
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82436E98: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82436E9C: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82436EA0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82436EA4: 4BFFFE25  bl 0x82436cc8
	ctx.lr = 0x82436EA8;
	sub_82436CC8(ctx, base);
	// 82436EA8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436EAC: 419A0010  beq cr6, 0x82436ebc
	if ctx.cr[6].eq {
	pc = 0x82436EBC; continue 'dispatch;
	}
	// 82436EB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436EB4: 386B59E8  addi r3, r11, 0x59e8
	ctx.r[3].s64 = ctx.r[11].s64 + 23016;
	// 82436EB8: 48000211  bl 0x824370c8
	ctx.lr = 0x82436EBC;
	sub_824370C8(ctx, base);
	// 82436EBC: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82436EC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82436EC4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82436EC8: 916A9DDC  stw r11, -0x6224(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25124 as u32), ctx.r[11].u32 ) };
	// 82436ECC: 4BFFFD5D  bl 0x82436c28
	ctx.lr = 0x82436ED0;
	sub_82436C28(ctx, base);
	// 82436ED0: 4BFF3F61  bl 0x8242ae30
	ctx.lr = 0x82436ED4;
	sub_8242AE30(ctx, base);
	// 82436ED4: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82436ED8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436EDC: 386B6798  addi r3, r11, 0x6798
	ctx.r[3].s64 = ctx.r[11].s64 + 26520;
	// 82436EE0: 4BFF3E89  bl 0x8242ad68
	ctx.lr = 0x82436EE4;
	sub_8242AD68(ctx, base);
	// 82436EE4: 4BFFB30D  bl 0x824321f0
	ctx.lr = 0x82436EE8;
	sub_824321F0(ctx, base);
	// 82436EE8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82436EEC: 4BFFF8B5  bl 0x824367a0
	ctx.lr = 0x82436EF0;
	sub_824367A0(ctx, base);
	// 82436EF0: 817E9DD8  lwz r11, -0x6228(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25128 as u32) ) } as u64;
	// 82436EF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82436EF8: 917E9DD8  stw r11, -0x6228(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-25128 as u32), ctx.r[11].u32 ) };
	// 82436EFC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82436F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436F08: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82436F0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82436F10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82436F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436F18 size=104
    let mut pc: u32 = 0x82436F18;
    'dispatch: loop {
        match pc {
            0x82436F18 => {
    //   block [0x82436F18..0x82436F80)
	// 82436F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436F1C: 480FE195  bl 0x825350b0
	ctx.lr = 0x82436F20;
	sub_82535080(ctx, base);
	// 82436F20: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436F24: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82436F28: 83E300B8  lwz r31, 0xb8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 82436F2C: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82436F30: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82436F34: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82436F38: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82436F3C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82436F40: 4BFFB619  bl 0x82432558
	ctx.lr = 0x82436F44;
	sub_82432558(ctx, base);
	// 82436F44: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82436F48: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82436F4C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82436F50: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82436F54: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82436F58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82436F5C: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82436F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436F64: 480011BD  bl 0x82438120
	ctx.lr = 0x82436F68;
	sub_82438120(ctx, base);
	// 82436F68: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82436F6C: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82436F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436F74: 48010BB5  bl 0x82447b28
	ctx.lr = 0x82436F78;
	sub_82447B28(ctx, base);
	// 82436F78: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 82436F7C: 480FE184  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82436F80 size=20
    let mut pc: u32 = 0x82436F80;
    'dispatch: loop {
        match pc {
            0x82436F80 => {
    //   block [0x82436F80..0x82436F94)
	// 82436F80: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82436F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82436F88: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82436F90: 4BFFFF88  b 0x82436f18
	sub_82436F18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436F98 size=80
    let mut pc: u32 = 0x82436F98;
    'dispatch: loop {
        match pc {
            0x82436F98 => {
    //   block [0x82436F98..0x82436FE8)
	// 82436F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436FA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436FA8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82436FAC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82436FB0: 3BEB9F28  addi r31, r11, -0x60d8
	ctx.r[31].s64 = ctx.r[11].s64 + -24792;
	// 82436FB4: 387FFEDC  addi r3, r31, -0x124
	ctx.r[3].s64 = ctx.r[31].s64 + -292;
	// 82436FB8: 4BFF5EA1  bl 0x8242ce58
	ctx.lr = 0x82436FBC;
	sub_8242CE58(ctx, base);
	// 82436FBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436FC0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82436FC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82436FC8: 917FFEFC  stw r11, -0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-260 as u32), ctx.r[11].u32 ) };
	// 82436FCC: 917FFED8  stw r11, -0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-296 as u32), ctx.r[11].u32 ) };
	// 82436FD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82436FD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436FE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82436FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436FE8 size=72
    let mut pc: u32 = 0x82436FE8;
    'dispatch: loop {
        match pc {
            0x82436FE8 => {
    //   block [0x82436FE8..0x82437030)
	// 82436FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436FFC: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82437000: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82437004: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82437008: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243700C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82437010: 4BFEEF51  bl 0x82425f60
	ctx.lr = 0x82437014;
	sub_82425F60(ctx, base);
	// 82437014: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437018: 93EB9E24  stw r31, -0x61dc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25052 as u32), ctx.r[31].u32 ) };
	// 8243701C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243702C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437030 size=56
    let mut pc: u32 = 0x82437030;
    'dispatch: loop {
        match pc {
            0x82437030 => {
    //   block [0x82437030..0x82437068)
	// 82437030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243703C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82437040: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82437044: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82437048: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8243704C: 4BFEED8D  bl 0x82425dd8
	ctx.lr = 0x82437050;
	sub_82425DD8(ctx, base);
	// 82437050: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437054: 906B9E00  stw r3, -0x6200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25088 as u32), ctx.r[3].u32 ) };
	// 82437058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243705C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437068 size=56
    let mut pc: u32 = 0x82437068;
    'dispatch: loop {
        match pc {
            0x82437068 => {
    //   block [0x82437068..0x824370A0)
	// 82437068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243706C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437074: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82437078: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8243707C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82437080: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82437084: 4BFEED55  bl 0x82425dd8
	ctx.lr = 0x82437088;
	sub_82425DD8(ctx, base);
	// 82437088: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243708C: 906B9F28  stw r3, -0x60d8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24792 as u32), ctx.r[3].u32 ) };
	// 82437090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243709C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824370A0 size=12
    let mut pc: u32 = 0x824370A0;
    'dispatch: loop {
        match pc {
            0x824370A0 => {
    //   block [0x824370A0..0x824370AC)
	// 824370A0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824370A4: 806B9F34  lwz r3, -0x60cc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24780 as u32) ) } as u64;
	// 824370A8: 4BFF5F20  b 0x8242cfc8
	sub_8242CFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824370B0 size=12
    let mut pc: u32 = 0x824370B0;
    'dispatch: loop {
        match pc {
            0x824370B0 => {
    //   block [0x824370B0..0x824370BC)
	// 824370B0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824370B4: 806B9F34  lwz r3, -0x60cc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24780 as u32) ) } as u64;
	// 824370B8: 4BFF5FA8  b 0x8242d060
	sub_8242D060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824370C0 size=4
    let mut pc: u32 = 0x824370C0;
    'dispatch: loop {
        match pc {
            0x824370C0 => {
    //   block [0x824370C0..0x824370C4)
	// 824370C0: 4BFEEC98  b 0x82425d58
	sub_82425D58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824370C8 size=136
    let mut pc: u32 = 0x824370C8;
    'dispatch: loop {
        match pc {
            0x824370C8 => {
    //   block [0x824370C8..0x82437150)
	// 824370C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824370CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824370D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824370D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824370D8: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 824370DC: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 824370E0: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 824370E4: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 824370E8: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 824370EC: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 824370F0: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 824370F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824370F8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824370FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82437100: 3BEB9E28  addi r31, r11, -0x61d8
	ctx.r[31].s64 = ctx.r[11].s64 + -25048;
	// 82437104: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82437108: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243710C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437110: 480FE0C1  bl 0x825351d0
	ctx.lr = 0x82437114;
	sub_825351D0(ctx, base);
	// 82437114: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82437118: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 8243711C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82437120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437124: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82437128: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243712C: 480FBCED  bl 0x82532e18
	ctx.lr = 0x82437130;
	sub_82532E18(ctx, base);
	// 82437130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437134: 4BFF6645  bl 0x8242d778
	ctx.lr = 0x82437138;
	sub_8242D778(ctx, base);
	// 82437138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243713C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437144: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82437148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243714C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437150 size=80
    let mut pc: u32 = 0x82437150;
    'dispatch: loop {
        match pc {
            0x82437150 => {
    //   block [0x82437150..0x824371A0)
	// 82437150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243715C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437160: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437164: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82437168: 3BEB9DE0  addi r31, r11, -0x6220
	ctx.r[31].s64 = ctx.r[11].s64 + -25120;
	// 8243716C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437170: 4BFF5CE9  bl 0x8242ce58
	ctx.lr = 0x82437174;
	sub_8242CE58(ctx, base);
	// 82437174: 907F0154  stw r3, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[3].u32 ) };
	// 82437178: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243717C: 409A0010  bne cr6, 0x8243718c
	if !ctx.cr[6].eq {
	pc = 0x8243718C; continue 'dispatch;
	}
	// 82437180: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437184: 386B5A54  addi r3, r11, 0x5a54
	ctx.r[3].s64 = ctx.r[11].s64 + 23124;
	// 82437188: 4BFFFF41  bl 0x824370c8
	ctx.lr = 0x8243718C;
	sub_824370C8(ctx, base);
	// 8243718C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82437190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243719C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824371A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824371A0 size=12
    let mut pc: u32 = 0x824371A0;
    'dispatch: loop {
        match pc {
            0x824371A0 => {
    //   block [0x824371A0..0x824371AC)
	// 824371A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824371A4: 386B5A84  addi r3, r11, 0x5a84
	ctx.r[3].s64 = ctx.r[11].s64 + 23172;
	// 824371A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824371B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824371B0 size=20
    let mut pc: u32 = 0x824371B0;
    'dispatch: loop {
        match pc {
            0x824371B0 => {
    //   block [0x824371B0..0x824371C4)
	// 824371B0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824371B4: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 824371B8: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 824371BC: 908B000C  stw r4, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 824371C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824371C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824371C8 size=40
    let mut pc: u32 = 0x824371C8;
    'dispatch: loop {
        match pc {
            0x824371C8 => {
    //   block [0x824371C8..0x824371F0)
	// 824371C8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824371CC: 394B09A0  addi r10, r11, 0x9a0
	ctx.r[10].s64 = ctx.r[11].s64 + 2464;
	// 824371D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824371D4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824371D8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824371DC: 40990024  ble cr6, 0x82437200
	if !ctx.cr[6].gt {
		sub_824371F0(ctx, base);
		return;
	}
	// 824371E0: 386A0018  addi r3, r10, 0x18
	ctx.r[3].s64 = ctx.r[10].s64 + 24;
	// 824371E4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824371E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824371EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824371F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824371F0 size=24
    let mut pc: u32 = 0x824371F0;
    'dispatch: loop {
        match pc {
            0x824371F0 => {
    //   block [0x824371F0..0x82437208)
	// 824371F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824371F4: 3863009C  addi r3, r3, 0x9c
	ctx.r[3].s64 = ctx.r[3].s64 + 156;
	// 824371F8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824371FC: 4198FFE8  blt cr6, 0x824371e4
	if ctx.cr[6].lt {
		sub_824371C8(ctx, base);
		return;
	}
	// 82437200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437208 size=136
    let mut pc: u32 = 0x82437208;
    'dispatch: loop {
        match pc {
            0x82437208 => {
    //   block [0x82437208..0x82437290)
	// 82437208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243720C: 480FDEB1  bl 0x825350bc
	ctx.lr = 0x82437210;
	sub_82535080(ctx, base);
	// 82437210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437214: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82437218: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243721C: 38A0009C  li r5, 0x9c
	ctx.r[5].s64 = 156;
	// 82437220: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437228: 480FDFA9  bl 0x825351d0
	ctx.lr = 0x8243722C;
	sub_825351D0(ctx, base);
	// 8243722C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82437230: 395E001F  addi r10, r30, 0x1f
	ctx.r[10].s64 = ctx.r[30].s64 + 31;
	// 82437234: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82437238: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8243723C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82437240: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82437244: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82437248: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243724C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437250: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437254: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82437258: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8243725C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82437260: 396A0400  addi r11, r10, 0x400
	ctx.r[11].s64 = ctx.r[10].s64 + 1024;
	// 82437264: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82437268: 394B0400  addi r10, r11, 0x400
	ctx.r[10].s64 = ctx.r[11].s64 + 1024;
	// 8243726C: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82437270: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82437274: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82437278: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8243727C: 396A0400  addi r11, r10, 0x400
	ctx.r[11].s64 = ctx.r[10].s64 + 1024;
	// 82437280: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82437284: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82437288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243728C: 480FDE80  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437290 size=12
    let mut pc: u32 = 0x82437290;
    'dispatch: loop {
        match pc {
            0x82437290 => {
    //   block [0x82437290..0x8243729C)
	// 82437290: 2F03201F  cmpwi cr6, r3, 0x201f
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8223, &mut ctx.xer);
	// 82437294: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82437298: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243729C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243729C size=8
    let mut pc: u32 = 0x8243729C;
    'dispatch: loop {
        match pc {
            0x8243729C => {
    //   block [0x8243729C..0x824372A4)
	// 8243729C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824372A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824372A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824372A8 size=92
    let mut pc: u32 = 0x824372A8;
    'dispatch: loop {
        match pc {
            0x824372A8 => {
    //   block [0x824372A8..0x82437304)
	// 824372A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824372AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824372B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824372B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824372B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824372BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824372C0: 419A0030  beq cr6, 0x824372f0
	if ctx.cr[6].eq {
	pc = 0x824372F0; continue 'dispatch;
	}
	// 824372C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824372C8: 83EB0030  lwz r31, 0x30(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824372CC: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824372D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824372D4: 4800143D  bl 0x82438710
	ctx.lr = 0x824372D8;
	sub_82438710(ctx, base);
	// 824372D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824372DC: 480109C5  bl 0x82447ca0
	ctx.lr = 0x824372E0;
	sub_82447CA0(ctx, base);
	// 824372E0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824372E4: 814B09A0  lwz r10, 0x9a0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 824372E8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824372EC: 914B09A0  stw r10, 0x9a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2464 as u32), ctx.r[10].u32 ) };
	// 824372F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824372F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824372F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824372FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437308 size=32
    let mut pc: u32 = 0x82437308;
    'dispatch: loop {
        match pc {
            0x82437308 => {
    //   block [0x82437308..0x82437328)
	// 82437308: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243730C: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82437310: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82437314: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82437318: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8243731C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437320: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82437324: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437328 size=20
    let mut pc: u32 = 0x82437328;
    'dispatch: loop {
        match pc {
            0x82437328 => {
    //   block [0x82437328..0x8243733C)
	// 82437328: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243732C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82437330: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82437334: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82437338: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243733C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243733C size=4
    let mut pc: u32 = 0x8243733C;
    'dispatch: loop {
        match pc {
            0x8243733C => {
    //   block [0x8243733C..0x82437340)
	// 8243733C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437340 size=16
    let mut pc: u32 = 0x82437340;
    'dispatch: loop {
        match pc {
            0x82437340 => {
    //   block [0x82437340..0x82437350)
	// 82437340: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82437344: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82437348: 906B0014  stw r3, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8243734C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437350 size=16
    let mut pc: u32 = 0x82437350;
    'dispatch: loop {
        match pc {
            0x82437350 => {
    //   block [0x82437350..0x82437360)
	// 82437350: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82437354: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82437358: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437360 size=80
    let mut pc: u32 = 0x82437360;
    'dispatch: loop {
        match pc {
            0x82437360 => {
    //   block [0x82437360..0x824373B0)
	// 82437360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243736C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437370: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82437374: 38A00508  li r5, 0x508
	ctx.r[5].s64 = 1288;
	// 82437378: 3BEB09A0  addi r31, r11, 0x9a0
	ctx.r[31].s64 = ctx.r[11].s64 + 2464;
	// 8243737C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437384: 480FDE4D  bl 0x825351d0
	ctx.lr = 0x82437388;
	sub_825351D0(ctx, base);
	// 82437388: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8243738C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82437390: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82437394: 4BFFFFAD  bl 0x82437340
	ctx.lr = 0x82437398;
	sub_82437340(ctx, base);
	// 82437398: 48010829  bl 0x82447bc0
	ctx.lr = 0x8243739C;
	sub_82447BC0(ctx, base);
	// 8243739C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824373A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824373A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824373A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824373AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824373B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824373B0 size=296
    let mut pc: u32 = 0x824373B0;
    'dispatch: loop {
        match pc {
            0x824373B0 => {
    //   block [0x824373B0..0x824374D8)
	// 824373B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824373B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824373B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824373BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824373C0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 824373C4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 824373C8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824373CC: 4BFFFDFD  bl 0x824371c8
	ctx.lr = 0x824373D0;
	sub_824371C8(ctx, base);
	// 824373D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824373D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824373D8: 419A002C  beq cr6, 0x82437404
	if ctx.cr[6].eq {
	pc = 0x82437404; continue 'dispatch;
	}
	// 824373DC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824373E0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824373E4: 4BFFFEAD  bl 0x82437290
	ctx.lr = 0x824373E8;
	sub_82437290(ctx, base);
	// 824373E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824373EC: 419A0030  beq cr6, 0x8243741c
	if ctx.cr[6].eq {
	pc = 0x8243741C; continue 'dispatch;
	}
	// 824373F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824373F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824373F8: 38AB5AFC  addi r5, r11, 0x5afc
	ctx.r[5].s64 = ctx.r[11].s64 + 23292;
	// 824373FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437400: 4BFFFF09  bl 0x82437308
	ctx.lr = 0x82437404;
	sub_82437308(ctx, base);
	// 82437404: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243740C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437418: 4E800020  blr
	return;
	// 8243741C: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 82437420: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82437424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437428: 4BFFFDE1  bl 0x82437208
	ctx.lr = 0x8243742C;
	sub_82437208(ctx, base);
	// 8243742C: 4800179D  bl 0x82438bc8
	ctx.lr = 0x82437430;
	sub_82438BC8(ctx, base);
	// 82437430: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82437434: 409A0034  bne cr6, 0x82437468
	if !ctx.cr[6].eq {
	pc = 0x82437468; continue 'dispatch;
	}
	// 82437438: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243743C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437440: 38AB5ADC  addi r5, r11, 0x5adc
	ctx.r[5].s64 = ctx.r[11].s64 + 23260;
	// 82437444: 4BFFFEC5  bl 0x82437308
	ctx.lr = 0x82437448;
	sub_82437308(ctx, base);
	// 82437448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243744C: 4BFFFE5D  bl 0x824372a8
	ctx.lr = 0x82437450;
	sub_824372A8(ctx, base);
	// 82437450: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437454: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243745C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437464: 4E800020  blr
	return;
	// 82437468: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8243746C: 4801091D  bl 0x82447d88
	ctx.lr = 0x82437470;
	sub_82447D88(ctx, base);
	// 82437470: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82437474: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82437478: 409A0034  bne cr6, 0x824374ac
	if !ctx.cr[6].eq {
	pc = 0x824374AC; continue 'dispatch;
	}
	// 8243747C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437480: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437484: 38AB5ABC  addi r5, r11, 0x5abc
	ctx.r[5].s64 = ctx.r[11].s64 + 23228;
	// 82437488: 4BFFFE81  bl 0x82437308
	ctx.lr = 0x8243748C;
	sub_82437308(ctx, base);
	// 8243748C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437490: 4BFFFE19  bl 0x824372a8
	ctx.lr = 0x82437494;
	sub_824372A8(ctx, base);
	// 82437494: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243749C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824374A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824374A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824374A8: 4E800020  blr
	return;
	// 824374AC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824374B0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824374B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824374B8: 814B09A0  lwz r10, 0x9a0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 824374BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824374C0: 914B09A0  stw r10, 0x9a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2464 as u32), ctx.r[10].u32 ) };
	// 824374C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824374C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824374CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824374D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824374D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824374D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824374D8 size=104
    let mut pc: u32 = 0x824374D8;
    'dispatch: loop {
        match pc {
            0x824374D8 => {
    //   block [0x824374D8..0x82437540)
	// 824374D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824374DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824374E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824374E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824374E8: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 824374EC: 817F9F3C  lwz r11, -0x60c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24772 as u32) ) } as u64;
	// 824374F0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824374F4: 40980038  bge cr6, 0x8243752c
	if !ctx.cr[6].lt {
	pc = 0x8243752C; continue 'dispatch;
	}
	// 824374F8: 4BFFFCA9  bl 0x824371a0
	ctx.lr = 0x824374FC;
	sub_824371A0(ctx, base);
	// 824374FC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437500: 906B9F38  stw r3, -0x60c8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24776 as u32), ctx.r[3].u32 ) };
	// 82437504: 4BFFFE5D  bl 0x82437360
	ctx.lr = 0x82437508;
	sub_82437360(ctx, base);
	// 82437508: 480106D1  bl 0x82447bd8
	ctx.lr = 0x8243750C;
	sub_82447BD8(ctx, base);
	// 8243750C: 48001A5D  bl 0x82438f68
	ctx.lr = 0x82437510;
	sub_82438F68(ctx, base);
	// 82437510: 48010871  bl 0x82447d80
	ctx.lr = 0x82437514;
	sub_82447D80(ctx, base);
	// 82437514: 817F9F3C  lwz r11, -0x60c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24772 as u32) ) } as u64;
	// 82437518: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8243751C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437520: 917F9F3C  stw r11, -0x60c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-24772 as u32), ctx.r[11].u32 ) };
	// 82437524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82437528: 916A9F40  stw r11, -0x60c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24768 as u32), ctx.r[11].u32 ) };
	// 8243752C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243753C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437540 size=12
    let mut pc: u32 = 0x82437540;
    'dispatch: loop {
        match pc {
            0x82437540 => {
    //   block [0x82437540..0x8243754C)
	// 82437540: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82437544: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82437548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437550 size=8
    let mut pc: u32 = 0x82437550;
    'dispatch: loop {
        match pc {
            0x82437550 => {
    //   block [0x82437550..0x82437558)
	// 82437550: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82437554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437558 size=148
    let mut pc: u32 = 0x82437558;
    'dispatch: loop {
        match pc {
            0x82437558 => {
    //   block [0x82437558..0x824375EC)
	// 82437558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243755C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82437564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82437568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243756C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82437570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437574: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82437578: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8243757C: 38A948DC  addi r5, r9, 0x48dc
	ctx.r[5].s64 = ctx.r[9].s64 + 18652;
	// 82437580: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82437584: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82437588: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243758C: 38895B28  addi r4, r9, 0x5b28
	ctx.r[4].s64 = ctx.r[9].s64 + 23336;
	// 82437590: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82437594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82437598: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243759C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824375A0: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 824375A4: 4BFF1A6D  bl 0x82429010
	ctx.lr = 0x824375A8;
	sub_82429010(ctx, base);
	// 824375A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824375AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824375B0: 409A0010  bne cr6, 0x824375c0
	if !ctx.cr[6].eq {
	pc = 0x824375C0; continue 'dispatch;
	}
	// 824375B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824375B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824375BC: 4800000C  b 0x824375c8
	pc = 0x824375C8; continue 'dispatch;
	// 824375C0: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824375C4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824375C8: 48001659  bl 0x82438c20
	ctx.lr = 0x824375CC;
	sub_82438C20(ctx, base);
	// 824375CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824375D0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824375D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824375D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824375DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824375E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824375E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824375E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824375F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824375F0 size=28
    let mut pc: u32 = 0x824375F0;
    'dispatch: loop {
        match pc {
            0x824375F0 => {
    //   block [0x824375F0..0x8243760C)
	// 824375F0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824375F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824375F8: 419A0014  beq cr6, 0x8243760c
	if ctx.cr[6].eq {
		sub_8243760C(ctx, base);
		return;
	}
	// 824375FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82437600: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437604: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243760C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243760C size=20
    let mut pc: u32 = 0x8243760C;
    'dispatch: loop {
        match pc {
            0x8243760C => {
    //   block [0x8243760C..0x82437620)
	// 8243760C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82437610: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437614: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82437618: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243761C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437620 size=12
    let mut pc: u32 = 0x82437620;
    'dispatch: loop {
        match pc {
            0x82437620 => {
    //   block [0x82437620..0x8243762C)
	// 82437620: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437624: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82437628: 48001610  b 0x82438c38
	sub_82438C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437630 size=8
    let mut pc: u32 = 0x82437630;
    'dispatch: loop {
        match pc {
            0x82437630 => {
    //   block [0x82437630..0x82437638)
	// 82437630: 80630064  lwz r3, 0x64(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82437634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437638 size=40
    let mut pc: u32 = 0x82437638;
    'dispatch: loop {
        match pc {
            0x82437638 => {
    //   block [0x82437638..0x82437660)
	// 82437638: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243763C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82437640: 7D6B21D6  mullw r11, r11, r4
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[4].s32 as i64);
	// 82437644: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437648: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243764C: 7D444850  subf r10, r4, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 82437650: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82437654: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82437658: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82437660 size=132
    let mut pc: u32 = 0x82437660;
    'dispatch: loop {
        match pc {
            0x82437660 => {
    //   block [0x82437660..0x824376E4)
	// 82437660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243766C: 7CAB0E70  srawi r11, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 82437670: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82437674: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437678: 7C8A0E70  srawi r10, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 8243767C: 5567083C  slwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82437680: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437684: 38680004  addi r3, r8, 4
	ctx.r[3].s64 = ctx.r[8].s64 + 4;
	// 82437688: 5566083C  slwi r6, r11, 1
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8243768C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82437690: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82437694: 4BFFFFA5  bl 0x82437638
	ctx.lr = 0x82437698;
	sub_82437638(ctx, base);
	// 82437698: 7CCB0E70  srawi r11, r6, 1
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[6].s32 >> 1) as i64;
	// 8243769C: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 824376A0: 7CAB0194  addze r5, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[5].s64 = tmp.s64;
	// 824376A4: 7CEB0E70  srawi r11, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 824376A8: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 824376AC: 4BFFFF8D  bl 0x82437638
	ctx.lr = 0x824376B0;
	sub_82437638(ctx, base);
	// 824376B0: 38680024  addi r3, r8, 0x24
	ctx.r[3].s64 = ctx.r[8].s64 + 36;
	// 824376B4: 4BFFFF85  bl 0x82437638
	ctx.lr = 0x824376B8;
	sub_82437638(ctx, base);
	// 824376B8: 81680044  lwz r11, 0x44(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(68 as u32) ) } as u64;
	// 824376BC: 38680044  addi r3, r8, 0x44
	ctx.r[3].s64 = ctx.r[8].s64 + 68;
	// 824376C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824376C4: 419A0010  beq cr6, 0x824376d4
	if ctx.cr[6].eq {
	pc = 0x824376D4; continue 'dispatch;
	}
	// 824376C8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824376CC: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 824376D0: 4BFFFF69  bl 0x82437638
	ctx.lr = 0x824376D4;
	sub_82437638(ctx, base);
	// 824376D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824376D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824376DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824376E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824376E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824376E8 size=8
    let mut pc: u32 = 0x824376E8;
    'dispatch: loop {
        match pc {
            0x824376E8 => {
    //   block [0x824376E8..0x824376F0)
	// 824376E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824376EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824376F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824376F0 size=20
    let mut pc: u32 = 0x824376F0;
    'dispatch: loop {
        match pc {
            0x824376F0 => {
    //   block [0x824376F0..0x82437704)
	// 824376F0: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824376F4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824376F8: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 824376FC: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82437700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437708 size=8
    let mut pc: u32 = 0x82437708;
    'dispatch: loop {
        match pc {
            0x82437708 => {
    //   block [0x82437708..0x82437710)
	// 82437708: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243770C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437710 size=12
    let mut pc: u32 = 0x82437710;
    'dispatch: loop {
        match pc {
            0x82437710 => {
    //   block [0x82437710..0x8243771C)
	// 82437710: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82437714: 60840601  ori r4, r4, 0x601
	ctx.r[4].u64 = ctx.r[4].u64 | 1537;
	// 82437718: 480101F0  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437720 size=168
    let mut pc: u32 = 0x82437720;
    'dispatch: loop {
        match pc {
            0x82437720 => {
    //   block [0x82437720..0x824377C8)
	// 82437720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437724: 480FD995  bl 0x825350b8
	ctx.lr = 0x82437728;
	sub_82535080(ctx, base);
	// 82437728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243772C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82437734: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82437738: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8243773C: 48010105  bl 0x82447840
	ctx.lr = 0x82437740;
	sub_82447840(ctx, base);
	// 82437740: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82437744: 419A001C  beq cr6, 0x82437760
	if ctx.cr[6].eq {
	pc = 0x82437760; continue 'dispatch;
	}
	// 82437748: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243774C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437750: 60840191  ori r4, r4, 0x191
	ctx.r[4].u64 = ctx.r[4].u64 | 401;
	// 82437754: 480101B5  bl 0x82447908
	ctx.lr = 0x82437758;
	sub_82447908(ctx, base);
	// 82437758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243775C: 480FD9AC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82437760: 815F21E8  lwz r10, 0x21e8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8680 as u32) ) } as u64;
	// 82437764: 813F21E0  lwz r9, 0x21e0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8672 as u32) ) } as u64;
	// 82437768: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 8243776C: 409A001C  bne cr6, 0x82437788
	if !ctx.cr[6].eq {
	pc = 0x82437788; continue 'dispatch;
	}
	// 82437770: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82437774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437778: 60840602  ori r4, r4, 0x602
	ctx.r[4].u64 = ctx.r[4].u64 | 1538;
	// 8243777C: 4801018D  bl 0x82447908
	ctx.lr = 0x82437780;
	sub_82447908(ctx, base);
	// 82437780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82437784: 480FD984  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82437788: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243778C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82437790: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82437794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82437798: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8243779C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824377A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824377A4: 4BFFFF4D  bl 0x824376f0
	ctx.lr = 0x824377A8;
	sub_824376F0(ctx, base);
	// 824377A8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824377AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824377B0: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 824377B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824377B8: 48010831  bl 0x82447fe8
	ctx.lr = 0x824377BC;
	sub_82447FE8(ctx, base);
	// 824377BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824377C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824377C4: 480FD944  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824377C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824377C8 size=108
    let mut pc: u32 = 0x824377C8;
    'dispatch: loop {
        match pc {
            0x824377C8 => {
    //   block [0x824377C8..0x82437834)
	// 824377C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824377CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824377D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824377D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824377D8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824377DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824377E0: 48011789  bl 0x82448f68
	ctx.lr = 0x824377E4;
	sub_82448F68(ctx, base);
	// 824377E4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824377E8: 419A0038  beq cr6, 0x82437820
	if ctx.cr[6].eq {
	pc = 0x82437820; continue 'dispatch;
	}
	// 824377EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824377F0: 809F21E8  lwz r4, 0x21e8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8680 as u32) ) } as u64;
	// 824377F4: 48010C0D  bl 0x82448400
	ctx.lr = 0x824377F8;
	sub_82448400(ctx, base);
	// 824377F8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824377FC: 409A0024  bne cr6, 0x82437820
	if !ctx.cr[6].eq {
	pc = 0x82437820; continue 'dispatch;
	}
	// 82437800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437804: 4BFFFEE5  bl 0x824376e8
	ctx.lr = 0x82437808;
	sub_824376E8(ctx, base);
	// 82437808: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243780C: 419A0014  beq cr6, 0x82437820
	if ctx.cr[6].eq {
	pc = 0x82437820; continue 'dispatch;
	}
	// 82437810: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82437814: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82437818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243781C: 4801173D  bl 0x82448f58
	ctx.lr = 0x82437820;
	sub_82448F58(ctx, base);
	// 82437820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243782C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437838 size=108
    let mut pc: u32 = 0x82437838;
    'dispatch: loop {
        match pc {
            0x82437838 => {
    //   block [0x82437838..0x824378A4)
	// 82437838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243783C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82437844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437848: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8243784C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437850: 480116F9  bl 0x82448f48
	ctx.lr = 0x82437854;
	sub_82448F48(ctx, base);
	// 82437854: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437858: 419A0038  beq cr6, 0x82437890
	if ctx.cr[6].eq {
	pc = 0x82437890; continue 'dispatch;
	}
	// 8243785C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437860: 809F21E8  lwz r4, 0x21e8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8680 as u32) ) } as u64;
	// 82437864: 48010B65  bl 0x824483c8
	ctx.lr = 0x82437868;
	sub_824483C8(ctx, base);
	// 82437868: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243786C: 409A0024  bne cr6, 0x82437890
	if !ctx.cr[6].eq {
	pc = 0x82437890; continue 'dispatch;
	}
	// 82437870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437874: 4BFFFE75  bl 0x824376e8
	ctx.lr = 0x82437878;
	sub_824376E8(ctx, base);
	// 82437878: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243787C: 419A0014  beq cr6, 0x82437890
	if ctx.cr[6].eq {
	pc = 0x82437890; continue 'dispatch;
	}
	// 82437880: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82437884: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82437888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243788C: 480116AD  bl 0x82448f38
	ctx.lr = 0x82437890;
	sub_82448F38(ctx, base);
	// 82437890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243789C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824378A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824378A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824378A8 size=20
    let mut pc: u32 = 0x824378A8;
    'dispatch: loop {
        match pc {
            0x824378A8 => {
    //   block [0x824378A8..0x824378BC)
	// 824378A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824378AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824378B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824378B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824378B8: 4BFFFE38  b 0x824376f0
	sub_824376F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824378C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824378C0 size=60
    let mut pc: u32 = 0x824378C0;
    'dispatch: loop {
        match pc {
            0x824378C0 => {
    //   block [0x824378C0..0x824378FC)
	// 824378C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824378C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824378C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824378CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824378D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824378D4: 4BFFFEF5  bl 0x824377c8
	ctx.lr = 0x824378D8;
	sub_824377C8(ctx, base);
	// 824378D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824378DC: 4BFFFF5D  bl 0x82437838
	ctx.lr = 0x824378E0;
	sub_82437838(ctx, base);
	// 824378E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824378E4: 4BFFFE25  bl 0x82437708
	ctx.lr = 0x824378E8;
	sub_82437708(ctx, base);
	// 824378E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824378EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824378F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824378F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824378F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437900 size=84
    let mut pc: u32 = 0x82437900;
    'dispatch: loop {
        match pc {
            0x82437900 => {
    //   block [0x82437900..0x82437954)
	// 82437900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437904: 480FD7B5  bl 0x825350b8
	ctx.lr = 0x82437908;
	sub_82535080(ctx, base);
	// 82437908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243790C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82437910: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82437914: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82437918: 3BC40004  addi r30, r4, 4
	ctx.r[30].s64 = ctx.r[4].s64 + 4;
	// 8243791C: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82437920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82437924: 4BFFFF85  bl 0x824378a8
	ctx.lr = 0x82437928;
	sub_824378A8(ctx, base);
	// 82437928: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243792C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82437930: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82437934: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82437938: 480106B1  bl 0x82447fe8
	ctx.lr = 0x8243793C;
	sub_82447FE8(ctx, base);
	// 8243793C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82437940: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82437944: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 82437948: 4198FFD8  blt cr6, 0x82437920
	if ctx.cr[6].lt {
	pc = 0x82437920; continue 'dispatch;
	}
	// 8243794C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82437950: 480FD7B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437958 size=48
    let mut pc: u32 = 0x82437958;
    'dispatch: loop {
        match pc {
            0x82437958 => {
    //   block [0x82437958..0x82437988)
	// 82437958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243795C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437964: 38832620  addi r4, r3, 0x2620
	ctx.r[4].s64 = ctx.r[3].s64 + 9760;
	// 82437968: 80A321E8  lwz r5, 0x21e8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8680 as u32) ) } as u64;
	// 8243796C: 908321E0  stw r4, 0x21e0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8672 as u32), ctx.r[4].u32 ) };
	// 82437970: 4BFFFF91  bl 0x82437900
	ctx.lr = 0x82437974;
	sub_82437900(ctx, base);
	// 82437974: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437988 size=24
    let mut pc: u32 = 0x82437988;
    'dispatch: loop {
        match pc {
            0x82437988 => {
    //   block [0x82437988..0x824379A0)
	// 82437988: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8243798C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82437990: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82437994: 90E30044  stw r7, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 82437998: 91030048  stw r8, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 8243799C: 4BFFFCC4  b 0x82437660
	sub_82437660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824379A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824379A0 size=40
    let mut pc: u32 = 0x824379A0;
    'dispatch: loop {
        match pc {
            0x824379A0 => {
    //   block [0x824379A0..0x824379C8)
	// 824379A0: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824379A4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824379A8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824379AC: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824379B0: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 824379B4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824379B8: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 824379BC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824379C0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824379C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824379C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824379C8 size=428
    let mut pc: u32 = 0x824379C8;
    'dispatch: loop {
        match pc {
            0x824379C8 => {
    //   block [0x824379C8..0x82437B28)
	// 824379C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824379CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824379D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824379D4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824379D8: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 824379DC: 41990130  bgt cr6, 0x82437b0c
	if ctx.cr[6].gt {
	pc = 0x82437B0C; continue 'dispatch;
	}
	// 824379E0: 419A0180  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
	// 824379E4: 396BFFEF  addi r11, r11, -0x11
	ctx.r[11].s64 = ctx.r[11].s64 + -17;
	// 824379E8: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 824379EC: 41990160  bgt cr6, 0x82437b4c
	if ctx.cr[6].gt {
	pc = 0x82437B4C; continue 'dispatch;
	}
	// 824379F0: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 824379F4: 398C7A08  addi r12, r12, 0x7a08
	ctx.r[12].s64 = ctx.r[12].s64 + 31240;
	// 824379F8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 824379FC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82437A00: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82437A04: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		1 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		2 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		3 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		4 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		5 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		6 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		7 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		8 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		9 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		10 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		11 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		12 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		13 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		14 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		15 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		16 => {
	pc = 0x82437B28; continue 'dispatch;
		},
		17 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		18 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		19 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		20 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		21 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		22 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		23 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		24 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		25 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		26 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		27 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		28 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		29 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		30 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		31 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		32 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		33 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		34 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		35 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		36 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		37 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		38 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		39 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		40 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		41 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		42 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		43 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		44 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		45 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		46 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		47 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		48 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		49 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		50 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		51 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		52 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		53 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		54 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		55 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		56 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		57 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		58 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		59 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		60 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		61 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		62 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		63 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		64 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82437A08: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
	// 82437A0C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A10: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A14: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A18: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A1C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A20: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A24: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A28: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A2C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A30: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A34: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A38: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A3C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A40: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A44: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A48: 82437B28  lwz r18, 0x7b28(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31528 as u32) ) } as u64;
	// 82437A4C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A50: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A54: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A58: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A5C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A60: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A64: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A68: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A6C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A70: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A74: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A78: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A7C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A80: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A84: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A88: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
	// 82437A8C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A90: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A94: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A98: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A9C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AA0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AA4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AA8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AAC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AB0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AB4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AB8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437ABC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AC0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AC4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AC8: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
	// 82437ACC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AD0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AD4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AD8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437ADC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AE0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AE4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AE8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AEC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AF0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AF4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AF8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AFC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437B00: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437B04: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437B08: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
	// 82437B0C: 2F0B0101  cmpwi cr6, r11, 0x101
	ctx.cr[6].compare_i32(ctx.r[11].s32, 257, &mut ctx.xer);
	// 82437B10: 4199002C  bgt cr6, 0x82437b3c
	if ctx.cr[6].gt {
	pc = 0x82437B3C; continue 'dispatch;
	}
	// 82437B14: 419A0014  beq cr6, 0x82437b28
	if ctx.cr[6].eq {
	pc = 0x82437B28; continue 'dispatch;
	}
	// 82437B18: 2F0B0071  cmpwi cr6, r11, 0x71
	ctx.cr[6].compare_i32(ctx.r[11].s32, 113, &mut ctx.xer);
	// 82437B1C: 419A0044  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
	// 82437B20: 2F0B00F1  cmpwi cr6, r11, 0xf1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 241, &mut ctx.xer);
	// 82437B24: 48000024  b 0x82437b48
	pc = 0x82437B48; continue 'dispatch;
            }
            0x82437B28 => {
    //   block [0x82437B28..0x82437B4C)
	// 82437B28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82437B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437B38: 4E800020  blr
	return;
	// 82437B3C: 2F0B0111  cmpwi cr6, r11, 0x111
	ctx.cr[6].compare_i32(ctx.r[11].s32, 273, &mut ctx.xer);
	// 82437B40: 419A0020  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
	// 82437B44: 2F0B1001  cmpwi cr6, r11, 0x1001
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4097, &mut ctx.xer);
	// 82437B48: 419A0018  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
            }
            0x82437B4C => {
    //   block [0x82437B4C..0x82437B60)
	// 82437B4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437B50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437B54: 38AB5B68  addi r5, r11, 0x5b68
	ctx.r[5].s64 = ctx.r[11].s64 + 23400;
	// 82437B58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437B5C: 4BFFF7AD  bl 0x82437308
	ctx.lr = 0x82437B60;
	sub_82437308(ctx, base);
	pc = 0x82437B60; continue 'dispatch;
            }
            0x82437B60 => {
    //   block [0x82437B60..0x82437B74)
	// 82437B60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437B78 size=20
    let mut pc: u32 = 0x82437B78;
    'dispatch: loop {
        match pc {
            0x82437B78 => {
    //   block [0x82437B78..0x82437B8C)
	// 82437B78: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437B7C: 2F0B0051  cmpwi cr6, r11, 0x51
	ctx.cr[6].compare_i32(ctx.r[11].s32, 81, &mut ctx.xer);
	// 82437B80: 409A000C  bne cr6, 0x82437b8c
	if !ctx.cr[6].eq {
		sub_82437B8C(ctx, base);
		return;
	}
	// 82437B84: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82437B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437B8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437B8C size=24
    let mut pc: u32 = 0x82437B8C;
    'dispatch: loop {
        match pc {
            0x82437B8C => {
    //   block [0x82437B8C..0x82437BA4)
	// 82437B8C: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 82437B90: 419A0014  beq cr6, 0x82437ba4
	if ctx.cr[6].eq {
		sub_82437BA4(ctx, base);
		return;
	}
	// 82437B94: 81640094  lwz r11, 0x94(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) } as u64;
	// 82437B98: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82437B9C: 2F0B0051  cmpwi cr6, r11, 0x51
	ctx.cr[6].compare_i32(ctx.r[11].s32, 81, &mut ctx.xer);
	// 82437BA0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437BA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437BA4 size=8
    let mut pc: u32 = 0x82437BA4;
    'dispatch: loop {
        match pc {
            0x82437BA4 => {
    //   block [0x82437BA4..0x82437BAC)
	// 82437BA4: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82437BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437BB0 size=228
    let mut pc: u32 = 0x82437BB0;
    'dispatch: loop {
        match pc {
            0x82437BB0 => {
    //   block [0x82437BB0..0x82437C68)
	// 82437BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82437BBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437BC0: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82437BC4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82437BC8: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 82437BCC: 409A001C  bne cr6, 0x82437be8
	if !ctx.cr[6].eq {
	pc = 0x82437BE8; continue 'dispatch;
	}
	// 82437BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437BE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437BE4: 4E800020  blr
	return;
	// 82437BE8: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82437BEC: 409A0090  bne cr6, 0x82437c7c
	if !ctx.cr[6].eq {
	pc = 0x82437C7C; continue 'dispatch;
	}
	// 82437BF0: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82437BF4: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82437BF8: 41990084  bgt cr6, 0x82437c7c
	if ctx.cr[6].gt {
	pc = 0x82437C7C; continue 'dispatch;
	}
	// 82437BFC: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82437C00: 398C7C14  addi r12, r12, 0x7c14
	ctx.r[12].s64 = ctx.r[12].s64 + 31764;
	// 82437C04: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82437C08: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82437C0C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82437C10: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		1 => {
	pc = 0x82437C68; continue 'dispatch;
		},
		2 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		3 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		4 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		5 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		6 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		7 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		8 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		9 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		10 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		11 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		12 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		13 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		14 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		15 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		16 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		17 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		18 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		19 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		20 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82437C14: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
	// 82437C18: 82437C68  lwz r18, 0x7c68(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31848 as u32) ) } as u64;
	// 82437C1C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C20: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
	// 82437C24: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
	// 82437C28: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C2C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C30: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C34: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C38: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C3C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C40: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C44: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C48: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C4C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C50: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C54: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C58: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C5C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C60: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C64: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
            }
            0x82437C68 => {
    //   block [0x82437C68..0x82437C78)
	// 82437C68: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437C6C: 4BFF67BD  bl 0x8242e428
	ctx.lr = 0x82437C70;
	sub_8242E428(ctx, base);
	// 82437C70: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437C74: 419A0008  beq cr6, 0x82437c7c
	if ctx.cr[6].eq {
	pc = 0x82437C7C; continue 'dispatch;
	}
	pc = 0x82437C78; continue 'dispatch;
            }
            0x82437C78 => {
    //   block [0x82437C78..0x82437C7C)
	// 82437C78: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82437C7C; continue 'dispatch;
            }
            0x82437C7C => {
    //   block [0x82437C7C..0x82437C94)
	// 82437C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437C98 size=16
    let mut pc: u32 = 0x82437C98;
    'dispatch: loop {
        match pc {
            0x82437C98 => {
    //   block [0x82437C98..0x82437CA8)
	// 82437C98: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437C9C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437CA0: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82437CA4: 4800136C  b 0x82439010
	sub_82439010(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82437CA8 size=140
    let mut pc: u32 = 0x82437CA8;
    'dispatch: loop {
        match pc {
            0x82437CA8 => {
    //   block [0x82437CA8..0x82437D34)
	// 82437CA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82437CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82437CB0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82437CB4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82437CB8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82437CBC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437CC0: 4200FFF8  bdnz 0x82437cb8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82437CB8; continue 'dispatch;
	}
	// 82437CC4: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82437CC8: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82437CCC: 2123FFF0  subfic r9, r3, -0x10
	ctx.xer.ca = ctx.r[3].u32 <= -16 as u32;
	ctx.r[9].s64 = (-16 as i64) - ctx.r[3].s64;
	// 82437CD0: 394000DC  li r10, 0xdc
	ctx.r[10].s64 = 220;
	// 82437CD4: C0085B98  lfs f0, 0x5b98(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(23448 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82437CD8: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82437CDC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82437CE0: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82437CE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82437CE8: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 82437CEC: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437CF0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82437CF4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82437CF8: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82437CFC: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 82437D00: D9A1FFF8  stfd f13, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[13].u64 ) };
	// 82437D04: 8901FFFF  lbz r8, -1(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82437D08: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82437D0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437D10: 409AFFC8  bne cr6, 0x82437cd8
	if !ctx.cr[6].eq {
	pc = 0x82437CD8; continue 'dispatch;
	}
	// 82437D14: 396300EC  addi r11, r3, 0xec
	ctx.r[11].s64 = ctx.r[3].s64 + 236;
	// 82437D18: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 82437D1C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82437D20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82437D24: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82437D28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437D2C: 4200FFF8  bdnz 0x82437d24
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82437D24; continue 'dispatch;
	}
	// 82437D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D38 size=16
    let mut pc: u32 = 0x82437D38;
    'dispatch: loop {
        match pc {
            0x82437D38 => {
    //   block [0x82437D38..0x82437D48)
	// 82437D38: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D3C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437D40: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D44: 4800FF84  b 0x82447cc8
	sub_82447CC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D48 size=16
    let mut pc: u32 = 0x82437D48;
    'dispatch: loop {
        match pc {
            0x82437D48 => {
    //   block [0x82437D48..0x82437D58)
	// 82437D48: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D4C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437D50: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D54: 4800FFCC  b 0x82447d20
	sub_82447D20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D58 size=16
    let mut pc: u32 = 0x82437D58;
    'dispatch: loop {
        match pc {
            0x82437D58 => {
    //   block [0x82437D58..0x82437D68)
	// 82437D58: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D5C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437D60: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D64: 4800FFEC  b 0x82447d50
	sub_82447D50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D68 size=12
    let mut pc: u32 = 0x82437D68;
    'dispatch: loop {
        match pc {
            0x82437D68 => {
    //   block [0x82437D68..0x82437D74)
	// 82437D68: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82437D6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82437D70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D74 size=12
    let mut pc: u32 = 0x82437D74;
    'dispatch: loop {
        match pc {
            0x82437D74 => {
    //   block [0x82437D74..0x82437D80)
	// 82437D74: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82437D7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D80 size=4
    let mut pc: u32 = 0x82437D80;
    'dispatch: loop {
        match pc {
            0x82437D80 => {
    //   block [0x82437D80..0x82437D84)
	// 82437D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D88 size=8
    let mut pc: u32 = 0x82437D88;
    'dispatch: loop {
        match pc {
            0x82437D88 => {
    //   block [0x82437D88..0x82437D90)
	// 82437D88: 90830068  stw r4, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82437D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D90 size=8
    let mut pc: u32 = 0x82437D90;
    'dispatch: loop {
        match pc {
            0x82437D90 => {
    //   block [0x82437D90..0x82437D98)
	// 82437D90: 9083006C  stw r4, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82437D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D98 size=12
    let mut pc: u32 = 0x82437D98;
    'dispatch: loop {
        match pc {
            0x82437D98 => {
    //   block [0x82437D98..0x82437DA4)
	// 82437D98: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D9C: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82437DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DA8 size=12
    let mut pc: u32 = 0x82437DA8;
    'dispatch: loop {
        match pc {
            0x82437DA8 => {
    //   block [0x82437DA8..0x82437DB4)
	// 82437DA8: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437DAC: 908B0020  stw r4, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82437DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DB8 size=12
    let mut pc: u32 = 0x82437DB8;
    'dispatch: loop {
        match pc {
            0x82437DB8 => {
    //   block [0x82437DB8..0x82437DC4)
	// 82437DB8: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437DBC: 908B001C  stw r4, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82437DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DC8 size=8
    let mut pc: u32 = 0x82437DC8;
    'dispatch: loop {
        match pc {
            0x82437DC8 => {
    //   block [0x82437DC8..0x82437DD0)
	// 82437DC8: 90830070  stw r4, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[4].u32 ) };
	// 82437DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DD0 size=40
    let mut pc: u32 = 0x82437DD0;
    'dispatch: loop {
        match pc {
            0x82437DD0 => {
    //   block [0x82437DD0..0x82437DF8)
	// 82437DD0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82437DD4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82437DD8: 419A009C  beq cr6, 0x82437e74
	if ctx.cr[6].eq {
		sub_82437E74(ctx, base);
		return;
	}
	// 82437DDC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82437DE0: 419A0094  beq cr6, 0x82437e74
	if ctx.cr[6].eq {
		sub_82437E74(ctx, base);
		return;
	}
	// 82437DE4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82437DE8: 419A0010  beq cr6, 0x82437df8
	if ctx.cr[6].eq {
		sub_82437DF8(ctx, base);
		return;
	}
	// 82437DEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437DF0: 38AB5B9C  addi r5, r11, 0x5b9c
	ctx.r[5].s64 = ctx.r[11].s64 + 23452;
	// 82437DF4: 4BFFF514  b 0x82437308
	sub_82437308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82437DF8 size=124
    let mut pc: u32 = 0x82437DF8;
    'dispatch: loop {
        match pc {
            0x82437DF8 => {
    //   block [0x82437DF8..0x82437E74)
	// 82437DF8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82437DFC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437E00: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437E04: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82437E08: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E0C: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437E10: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82437E14: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437E18: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437E1C: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82437E20: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82437E24: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82437E28: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E2C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437E30: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437E34: 91650018  stw r11, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82437E38: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82437E3C: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82437E40: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82437E44: 91650020  stw r11, 0x20(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82437E48: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82437E4C: 91650024  stw r11, 0x24(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82437E50: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E54: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437E58: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437E5C: 91650028  stw r11, 0x28(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82437E60: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82437E64: 9165002C  stw r11, 0x2c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82437E68: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82437E6C: 91650030  stw r11, 0x30(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82437E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437E74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437E74 size=44
    let mut pc: u32 = 0x82437E74;
    'dispatch: loop {
        match pc {
            0x82437E74 => {
    //   block [0x82437E74..0x82437EA0)
	// 82437E74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82437E78: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437E7C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437E80: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82437E84: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E88: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437E8C: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82437E90: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437E94: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437E98: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82437E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82437EA0 size=280
    let mut pc: u32 = 0x82437EA0;
    'dispatch: loop {
        match pc {
            0x82437EA0 => {
    //   block [0x82437EA0..0x82437FB8)
	// 82437EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437EA4: 480FD219  bl 0x825350bc
	ctx.lr = 0x82437EA8;
	sub_82535080(ctx, base);
	// 82437EA8: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437EAC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82437EB0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82437EB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82437EB8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82437EBC: 4BFFFF15  bl 0x82437dd0
	ctx.lr = 0x82437EC0;
	sub_82437DD0(ctx, base);
	// 82437EC0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82437EC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82437EC8: 40990098  ble cr6, 0x82437f60
	if !ctx.cr[6].gt {
	pc = 0x82437F60; continue 'dispatch;
	}
	// 82437ECC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82437ED0: 409900A4  ble cr6, 0x82437f74
	if !ctx.cr[6].gt {
	pc = 0x82437F74; continue 'dispatch;
	}
	// 82437ED4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82437ED8: 409A0088  bne cr6, 0x82437f60
	if !ctx.cr[6].eq {
	pc = 0x82437F60; continue 'dispatch;
	}
	// 82437EDC: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82437EE0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82437EE4: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82437EE8: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82437EEC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437EF0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437EF4: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437EF8: 914100E4  stw r10, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 82437EFC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437F00: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82437F04: 914100E8  stw r10, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82437F08: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437F0C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82437F10: 914100EC  stw r10, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 82437F14: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437F18: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82437F1C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82437F20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82437F24: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82437F28: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82437F2C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82437F30: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82437F34: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82437F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82437F3C: 409A001C  bne cr6, 0x82437f58
	if !ctx.cr[6].eq {
	pc = 0x82437F58; continue 'dispatch;
	}
	// 82437F40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437F44: 38AB5C1C  addi r5, r11, 0x5c1c
	ctx.r[5].s64 = ctx.r[11].s64 + 23580;
	// 82437F48: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82437F4C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82437F50: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82437F54: 48000014  b 0x82437f68
	pc = 0x82437F68; continue 'dispatch;
	// 82437F58: 912100F0  stw r9, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[9].u32 ) };
	// 82437F5C: 48000018  b 0x82437f74
	pc = 0x82437F74; continue 'dispatch;
	// 82437F60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437F64: 38AB5BD8  addi r5, r11, 0x5bd8
	ctx.r[5].s64 = ctx.r[11].s64 + 23512;
	// 82437F68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82437F6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82437F70: 4BFFF399  bl 0x82437308
	ctx.lr = 0x82437F74;
	sub_82437308(ctx, base);
	// 82437F74: 4BFFF3DD  bl 0x82437350
	ctx.lr = 0x82437F78;
	sub_82437350(ctx, base);
	// 82437F78: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437F7C: 409A000C  bne cr6, 0x82437f88
	if !ctx.cr[6].eq {
	pc = 0x82437F88; continue 'dispatch;
	}
	// 82437F80: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437F84: 48000008  b 0x82437f8c
	pc = 0x82437F8C; continue 'dispatch;
	// 82437F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82437F8C: 916100C0  stw r11, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82437F90: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82437F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82437F98: 419A0018  beq cr6, 0x82437fb0
	if ctx.cr[6].eq {
	pc = 0x82437FB0; continue 'dispatch;
	}
	// 82437F9C: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82437FA0: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 82437FA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82437FA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82437FAC: 4E800421  bctrl
	ctx.lr = 0x82437FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82437FB0: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 82437FB4: 480FD158  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82437FB8 size=128
    let mut pc: u32 = 0x82437FB8;
    'dispatch: loop {
        match pc {
            0x82437FB8 => {
    //   block [0x82437FB8..0x82438038)
	// 82437FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437FBC: 480FD101  bl 0x825350bc
	ctx.lr = 0x82437FC0;
	sub_82535080(ctx, base);
	// 82437FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437FC4: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82437FC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82437FCC: 7FEB3214  add r31, r11, r6
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82437FD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82437FD4: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82437FD8: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437FDC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437FE0: 4BFFF9E9  bl 0x824379c8
	ctx.lr = 0x82437FE4;
	sub_824379C8(ctx, base);
	// 82437FE4: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82437FE8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437FEC: 409A000C  bne cr6, 0x82437ff8
	if !ctx.cr[6].eq {
	pc = 0x82437FF8; continue 'dispatch;
	}
	// 82437FF0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437FF4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437FF8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437FFC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82438000: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82438004: 409A0028  bne cr6, 0x8243802c
	if !ctx.cr[6].eq {
	pc = 0x8243802C; continue 'dispatch;
	}
	// 82438008: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243800C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438010: 83DE0044  lwz r30, 0x44(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82438014: 38AB5C5C  addi r5, r11, 0x5c5c
	ctx.r[5].s64 = ctx.r[11].s64 + 23644;
	// 82438018: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243801C: 4BFFF2ED  bl 0x82437308
	ctx.lr = 0x82438020;
	sub_82437308(ctx, base);
	// 82438020: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82438024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438028: 480FD0E4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243802C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82438030: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438034: 480FD0D8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438038 size=8
    let mut pc: u32 = 0x82438038;
    'dispatch: loop {
        match pc {
            0x82438038 => {
    //   block [0x82438038..0x82438040)
	// 82438038: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8243803C: 4BFFFC6C  b 0x82437ca8
	sub_82437CA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438040 size=164
    let mut pc: u32 = 0x82438040;
    'dispatch: loop {
        match pc {
            0x82438040 => {
    //   block [0x82438040..0x824380E4)
	// 82438040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438044: 480FD079  bl 0x825350bc
	ctx.lr = 0x82438048;
	sub_82535080(ctx, base);
	// 82438048: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243804C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438050: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438054: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82438058: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8243805C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82438060: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82438064: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82438068: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8243806C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82438070: 4200FFF8  bdnz 0x82438068
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438068; continue 'dispatch;
	}
	// 82438074: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82438078: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243807C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438080: 4BFFFD51  bl 0x82437dd0
	ctx.lr = 0x82438084;
	sub_82437DD0(ctx, base);
	// 82438084: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82438088: 409A000C  bne cr6, 0x82438094
	if !ctx.cr[6].eq {
	pc = 0x82438094; continue 'dispatch;
	}
	// 8243808C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82438090: 48000008  b 0x82438098
	pc = 0x82438098; continue 'dispatch;
	// 82438094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82438098: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243809C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824380A0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824380A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 824380A8: 4BFFF589  bl 0x82437630
	ctx.lr = 0x824380AC;
	sub_82437630(ctx, base);
	// 824380AC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824380B0: 409A000C  bne cr6, 0x824380bc
	if !ctx.cr[6].eq {
	pc = 0x824380BC; continue 'dispatch;
	}
	// 824380B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824380B8: 4BFFF8E9  bl 0x824379a0
	ctx.lr = 0x824380BC;
	sub_824379A0(ctx, base);
	// 824380BC: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 824380C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824380C4: 419A0018  beq cr6, 0x824380dc
	if ctx.cr[6].eq {
	pc = 0x824380DC; continue 'dispatch;
	}
	// 824380C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824380CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824380D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824380D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824380D8: 4E800421  bctrl
	ctx.lr = 0x824380DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824380DC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 824380E0: 480FD02C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824380E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824380E8 size=56
    let mut pc: u32 = 0x824380E8;
    'dispatch: loop {
        match pc {
            0x824380E8 => {
    //   block [0x824380E8..0x82438120)
	// 824380E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824380EC: 480FCFD1  bl 0x825350bc
	ctx.lr = 0x824380F0;
	sub_82535080(ctx, base);
	// 824380F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824380F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824380F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824380FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82438100: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438104: 4BFFFF3D  bl 0x82438040
	ctx.lr = 0x82438108;
	sub_82438040(ctx, base);
	// 82438108: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243810C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438114: 4BFFFD8D  bl 0x82437ea0
	ctx.lr = 0x82438118;
	sub_82437EA0(ctx, base);
	// 82438118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243811C: 480FCFF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438120 size=88
    let mut pc: u32 = 0x82438120;
    'dispatch: loop {
        match pc {
            0x82438120 => {
    //   block [0x82438120..0x82438178)
	// 82438120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438124: 480FCF95  bl 0x825350b8
	ctx.lr = 0x82438128;
	sub_82535080(ctx, base);
	// 82438128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243812C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82438130: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82438134: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82438138: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8243813C: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82438140: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82438144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438148: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243814C: 4BFFF83D  bl 0x82437988
	ctx.lr = 0x82438150;
	sub_82437988(ctx, base);
	// 82438150: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82438154: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82438158: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8243815C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438164: 4BFFFE55  bl 0x82437fb8
	ctx.lr = 0x82438168;
	sub_82437FB8(ctx, base);
	// 82438168: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243816C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438174: 480FCF94  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438178 size=312
    let mut pc: u32 = 0x82438178;
    'dispatch: loop {
        match pc {
            0x82438178 => {
    //   block [0x82438178..0x8243821C)
	// 82438178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243817C: 480FCF41  bl 0x825350bc
	ctx.lr = 0x82438180;
	sub_82535080(ctx, base);
	// 82438180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438184: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243818C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82438190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438194: 4BFFFA1D  bl 0x82437bb0
	ctx.lr = 0x82438198;
	sub_82437BB0(ctx, base);
	// 82438198: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243819C: 409A010C  bne cr6, 0x824382a8
	if !ctx.cr[6].eq {
	pc = 0x824382A8; continue 'dispatch;
	}
	// 824381A0: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 824381A4: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 824381A8: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 824381AC: 419900E8  bgt cr6, 0x82438294
	if ctx.cr[6].gt {
	pc = 0x82438294; continue 'dispatch;
	}
	// 824381B0: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 824381B4: 398C81C8  addi r12, r12, -0x7e38
	ctx.r[12].s64 = ctx.r[12].s64 + -32312;
	// 824381B8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 824381BC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 824381C0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 824381C4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82438280; continue 'dispatch;
		},
		1 => {
	pc = 0x82438230; continue 'dispatch;
		},
		2 => {
	pc = 0x82438294; continue 'dispatch;
		},
		3 => {
	pc = 0x82438244; continue 'dispatch;
		},
		4 => {
	pc = 0x82438258; continue 'dispatch;
		},
		5 => {
	pc = 0x82438294; continue 'dispatch;
		},
		6 => {
	pc = 0x82438294; continue 'dispatch;
		},
		7 => {
	pc = 0x82438294; continue 'dispatch;
		},
		8 => {
	pc = 0x82438294; continue 'dispatch;
		},
		9 => {
	pc = 0x82438294; continue 'dispatch;
		},
		10 => {
	pc = 0x8243821C; continue 'dispatch;
		},
		11 => {
	pc = 0x82438294; continue 'dispatch;
		},
		12 => {
	pc = 0x8243821C; continue 'dispatch;
		},
		13 => {
	pc = 0x82438294; continue 'dispatch;
		},
		14 => {
	pc = 0x82438294; continue 'dispatch;
		},
		15 => {
	pc = 0x82438294; continue 'dispatch;
		},
		16 => {
	pc = 0x82438294; continue 'dispatch;
		},
		17 => {
	pc = 0x82438294; continue 'dispatch;
		},
		18 => {
	pc = 0x82438294; continue 'dispatch;
		},
		19 => {
	pc = 0x82438294; continue 'dispatch;
		},
		20 => {
	pc = 0x8243826C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 824381C8: 82438280  lwz r18, -0x7d80(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32128 as u32) ) } as u64;
	// 824381CC: 82438230  lwz r18, -0x7dd0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32208 as u32) ) } as u64;
	// 824381D0: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381D4: 82438244  lwz r18, -0x7dbc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32188 as u32) ) } as u64;
	// 824381D8: 82438258  lwz r18, -0x7da8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32168 as u32) ) } as u64;
	// 824381DC: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381E0: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381E4: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381E8: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381EC: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381F0: 8243821C  lwz r18, -0x7de4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32228 as u32) ) } as u64;
	// 824381F4: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381F8: 8243821C  lwz r18, -0x7de4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32228 as u32) ) } as u64;
	// 824381FC: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438200: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438204: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438208: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 8243820C: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438210: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438214: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438218: 8243826C  lwz r18, -0x7d94(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32148 as u32) ) } as u64;
            }
            0x8243821C => {
    //   block [0x8243821C..0x82438230)
	// 8243821C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438224: 4BFFFA75  bl 0x82437c98
	ctx.lr = 0x82438228;
	sub_82437C98(ctx, base);
	// 82438228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243822C: 480FCEE0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438230 => {
    //   block [0x82438230..0x82438244)
	// 82438230: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438238: 4BFFFB01  bl 0x82437d38
	ctx.lr = 0x8243823C;
	sub_82437D38(ctx, base);
	// 8243823C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438240: 480FCECC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438244 => {
    //   block [0x82438244..0x82438258)
	// 82438244: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243824C: 4BFFFAFD  bl 0x82437d48
	ctx.lr = 0x82438250;
	sub_82437D48(ctx, base);
	// 82438250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438254: 480FCEB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438258 => {
    //   block [0x82438258..0x8243826C)
	// 82438258: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243825C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438260: 4BFFFAF9  bl 0x82437d58
	ctx.lr = 0x82438264;
	sub_82437D58(ctx, base);
	// 82438264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438268: 480FCEA4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8243826C => {
    //   block [0x8243826C..0x82438280)
	// 8243826C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438274: 4BFFFAF5  bl 0x82437d68
	ctx.lr = 0x82438278;
	sub_82437D68(ctx, base);
	// 82438278: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243827C: 480FCE90  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438280 => {
    //   block [0x82438280..0x82438294)
	// 82438280: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438288: 4BFFFDB1  bl 0x82438038
	ctx.lr = 0x8243828C;
	sub_82438038(ctx, base);
	// 8243828C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438290: 480FCE7C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438294 => {
    //   block [0x82438294..0x824382B0)
	// 82438294: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243829C: 38AB5C9C  addi r5, r11, 0x5c9c
	ctx.r[5].s64 = ctx.r[11].s64 + 23708;
	// 824382A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824382A4: 4BFFF065  bl 0x82437308
	ctx.lr = 0x824382A8;
	sub_82437308(ctx, base);
	// 824382A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824382AC: 480FCE60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824382B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824382B0 size=4
    let mut pc: u32 = 0x824382B0;
    'dispatch: loop {
        match pc {
            0x824382B0 => {
    //   block [0x824382B0..0x824382B4)
	// 824382B0: 4BFFFEC8  b 0x82438178
	sub_82438178(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824382B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824382B8 size=484
    let mut pc: u32 = 0x824382B8;
    'dispatch: loop {
        match pc {
            0x824382B8 => {
    //   block [0x824382B8..0x824383E4)
	// 824382B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824382BC: 480FCE01  bl 0x825350bc
	ctx.lr = 0x824382C0;
	sub_82535080(ctx, base);
	// 824382C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824382C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824382C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824382CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824382D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824382D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824382D8: 409A0018  bne cr6, 0x824382f0
	if !ctx.cr[6].eq {
	pc = 0x824382F0; continue 'dispatch;
	}
	// 824382DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824382E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824382E4: 388B5D0C  addi r4, r11, 0x5d0c
	ctx.r[4].s64 = ctx.r[11].s64 + 23820;
	// 824382E8: 480110C1  bl 0x824493a8
	ctx.lr = 0x824382EC;
	sub_824493A8(ctx, base);
	// 824382EC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 824382F0: 2F030051  cmpwi cr6, r3, 0x51
	ctx.cr[6].compare_i32(ctx.r[3].s32, 81, &mut ctx.xer);
	// 824382F4: 41990134  bgt cr6, 0x82438428
	if ctx.cr[6].gt {
	pc = 0x82438428; continue 'dispatch;
	}
	// 824382F8: 419A016C  beq cr6, 0x82438464
	if ctx.cr[6].eq {
	pc = 0x82438464; continue 'dispatch;
	}
	// 824382FC: 3963FFEF  addi r11, r3, -0x11
	ctx.r[11].s64 = ctx.r[3].s64 + -17;
	// 82438300: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82438304: 4199013C  bgt cr6, 0x82438440
	if ctx.cr[6].gt {
	pc = 0x82438440; continue 'dispatch;
	}
	// 82438308: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 8243830C: 398C8320  addi r12, r12, -0x7ce0
	ctx.r[12].s64 = ctx.r[12].s64 + -31968;
	// 82438310: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82438314: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82438318: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8243831C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x824383E4; continue 'dispatch;
		},
		1 => {
	pc = 0x82438440; continue 'dispatch;
		},
		2 => {
	pc = 0x82438440; continue 'dispatch;
		},
		3 => {
	pc = 0x82438440; continue 'dispatch;
		},
		4 => {
	pc = 0x82438440; continue 'dispatch;
		},
		5 => {
	pc = 0x82438440; continue 'dispatch;
		},
		6 => {
	pc = 0x82438440; continue 'dispatch;
		},
		7 => {
	pc = 0x82438440; continue 'dispatch;
		},
		8 => {
	pc = 0x82438440; continue 'dispatch;
		},
		9 => {
	pc = 0x82438440; continue 'dispatch;
		},
		10 => {
	pc = 0x82438440; continue 'dispatch;
		},
		11 => {
	pc = 0x82438440; continue 'dispatch;
		},
		12 => {
	pc = 0x82438440; continue 'dispatch;
		},
		13 => {
	pc = 0x82438440; continue 'dispatch;
		},
		14 => {
	pc = 0x82438440; continue 'dispatch;
		},
		15 => {
	pc = 0x82438440; continue 'dispatch;
		},
		16 => {
	pc = 0x824383F8; continue 'dispatch;
		},
		17 => {
	pc = 0x82438440; continue 'dispatch;
		},
		18 => {
	pc = 0x82438440; continue 'dispatch;
		},
		19 => {
	pc = 0x82438440; continue 'dispatch;
		},
		20 => {
	pc = 0x82438440; continue 'dispatch;
		},
		21 => {
	pc = 0x82438440; continue 'dispatch;
		},
		22 => {
	pc = 0x82438440; continue 'dispatch;
		},
		23 => {
	pc = 0x82438440; continue 'dispatch;
		},
		24 => {
	pc = 0x82438440; continue 'dispatch;
		},
		25 => {
	pc = 0x82438440; continue 'dispatch;
		},
		26 => {
	pc = 0x82438440; continue 'dispatch;
		},
		27 => {
	pc = 0x82438440; continue 'dispatch;
		},
		28 => {
	pc = 0x82438440; continue 'dispatch;
		},
		29 => {
	pc = 0x82438440; continue 'dispatch;
		},
		30 => {
	pc = 0x82438440; continue 'dispatch;
		},
		31 => {
	pc = 0x82438440; continue 'dispatch;
		},
		32 => {
	pc = 0x82438420; continue 'dispatch;
		},
		33 => {
	pc = 0x82438440; continue 'dispatch;
		},
		34 => {
	pc = 0x82438440; continue 'dispatch;
		},
		35 => {
	pc = 0x82438440; continue 'dispatch;
		},
		36 => {
	pc = 0x82438440; continue 'dispatch;
		},
		37 => {
	pc = 0x82438440; continue 'dispatch;
		},
		38 => {
	pc = 0x82438440; continue 'dispatch;
		},
		39 => {
	pc = 0x82438440; continue 'dispatch;
		},
		40 => {
	pc = 0x82438440; continue 'dispatch;
		},
		41 => {
	pc = 0x82438440; continue 'dispatch;
		},
		42 => {
	pc = 0x82438440; continue 'dispatch;
		},
		43 => {
	pc = 0x82438440; continue 'dispatch;
		},
		44 => {
	pc = 0x82438440; continue 'dispatch;
		},
		45 => {
	pc = 0x82438440; continue 'dispatch;
		},
		46 => {
	pc = 0x82438440; continue 'dispatch;
		},
		47 => {
	pc = 0x82438440; continue 'dispatch;
		},
		48 => {
	pc = 0x82438464; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82438320: 824383E4  lwz r18, -0x7c1c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31772 as u32) ) } as u64;
	// 82438324: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438328: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243832C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438330: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438334: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438338: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243833C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438340: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438344: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438348: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243834C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438350: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438354: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438358: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243835C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438360: 824383F8  lwz r18, -0x7c08(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31752 as u32) ) } as u64;
	// 82438364: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438368: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243836C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438370: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438374: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438378: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243837C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438380: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438384: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438388: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243838C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438390: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438394: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438398: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243839C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383A0: 82438420  lwz r18, -0x7be0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31712 as u32) ) } as u64;
	// 824383A4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383A8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383AC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383B0: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383B4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383B8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383BC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383C0: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383C4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383C8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383CC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383D0: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383D4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383D8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383DC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383E0: 82438464  lwz r18, -0x7b9c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31644 as u32) ) } as u64;
            }
            0x824383E4 => {
    //   block [0x824383E4..0x824383F8)
	// 824383E4: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824383E8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824383EC: 419A0070  beq cr6, 0x8243845c
	if ctx.cr[6].eq {
	pc = 0x8243845C; continue 'dispatch;
	}
	// 824383F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824383F4: 48000090  b 0x82438484
	pc = 0x82438484; continue 'dispatch;
            }
            0x824383F8 => {
    //   block [0x824383F8..0x82438420)
	// 824383F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824383FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438404: 4BFFFEAD  bl 0x824382b0
	ctx.lr = 0x82438408;
	sub_824382B0(ctx, base);
	// 82438408: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243840C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438414: 4BFFFCD5  bl 0x824380e8
	ctx.lr = 0x82438418;
	sub_824380E8(ctx, base);
	// 82438418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243841C: 480FCCF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438420 => {
    //   block [0x82438420..0x82438440)
	// 82438420: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82438424: 48000050  b 0x82438474
	pc = 0x82438474; continue 'dispatch;
	// 82438428: 2F030061  cmpwi cr6, r3, 0x61
	ctx.cr[6].compare_i32(ctx.r[3].s32, 97, &mut ctx.xer);
	// 8243842C: 419A0038  beq cr6, 0x82438464
	if ctx.cr[6].eq {
	pc = 0x82438464; continue 'dispatch;
	}
	// 82438430: 2F030101  cmpwi cr6, r3, 0x101
	ctx.cr[6].compare_i32(ctx.r[3].s32, 257, &mut ctx.xer);
	// 82438434: 419AFFBC  beq cr6, 0x824383f0
	if ctx.cr[6].eq {
	pc = 0x824383F0; continue 'dispatch;
	}
	// 82438438: 2F031001  cmpwi cr6, r3, 0x1001
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4097, &mut ctx.xer);
	// 8243843C: 419A0020  beq cr6, 0x8243845c
	if ctx.cr[6].eq {
	pc = 0x8243845C; continue 'dispatch;
	}
            }
            0x82438440 => {
    //   block [0x82438440..0x82438464)
	// 82438440: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438444: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438448: 38AB5CD0  addi r5, r11, 0x5cd0
	ctx.r[5].s64 = ctx.r[11].s64 + 23760;
	// 8243844C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438450: 4BFFEEB9  bl 0x82437308
	ctx.lr = 0x82438454;
	sub_82437308(ctx, base);
	// 82438454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438458: 480FCCB4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243845C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82438460: 48000014  b 0x82438474
	pc = 0x82438474; continue 'dispatch;
            }
            0x82438464 => {
    //   block [0x82438464..0x8243849C)
	// 82438464: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243846C: 4BFFF70D  bl 0x82437b78
	ctx.lr = 0x82438470;
	sub_82437B78(ctx, base);
	// 82438470: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82438474: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243847C: 4BFFFE35  bl 0x824382b0
	ctx.lr = 0x82438480;
	sub_824382B0(ctx, base);
	// 82438480: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82438484: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82438488: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243848C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438490: 4BFFFBB1  bl 0x82438040
	ctx.lr = 0x82438494;
	sub_82438040(ctx, base);
	// 82438494: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438498: 480FCC74  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824384A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824384A0 size=12
    let mut pc: u32 = 0x824384A0;
    'dispatch: loop {
        match pc {
            0x824384A0 => {
    //   block [0x824384A0..0x824384AC)
	// 824384A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824384A4: 386B5D14  addi r3, r11, 0x5d14
	ctx.r[3].s64 = ctx.r[11].s64 + 23828;
	// 824384A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824384B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824384B0 size=92
    let mut pc: u32 = 0x824384B0;
    'dispatch: loop {
        match pc {
            0x824384B0 => {
    //   block [0x824384B0..0x8243850C)
	// 824384B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824384B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824384B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824384BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824384C0: 419A0038  beq cr6, 0x824384f8
	if ctx.cr[6].eq {
	pc = 0x824384F8; continue 'dispatch;
	}
	// 824384C4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824384C8: 41980030  blt cr6, 0x824384f8
	if ctx.cr[6].lt {
	pc = 0x824384F8; continue 'dispatch;
	}
	// 824384CC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 824384D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824384D4: 388B4EB8  addi r4, r11, 0x4eb8
	ctx.r[4].s64 = ctx.r[11].s64 + 20152;
	// 824384D8: 38630012  addi r3, r3, 0x12
	ctx.r[3].s64 = ctx.r[3].s64 + 18;
	// 824384DC: 480FACB5  bl 0x82533190
	ctx.lr = 0x824384E0;
	sub_82533190(ctx, base);
	// 824384E0: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824384E4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824384E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824384EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824384F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824384F4: 4E800020  blr
	return;
	// 824384F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824384FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438510 size=92
    let mut pc: u32 = 0x82438510;
    'dispatch: loop {
        match pc {
            0x82438510 => {
    //   block [0x82438510..0x8243856C)
	// 82438510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243851C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438520: 419A0038  beq cr6, 0x82438558
	if ctx.cr[6].eq {
	pc = 0x82438558; continue 'dispatch;
	}
	// 82438524: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82438528: 41980030  blt cr6, 0x82438558
	if ctx.cr[6].lt {
	pc = 0x82438558; continue 'dispatch;
	}
	// 8243852C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438530: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82438534: 388B5D54  addi r4, r11, 0x5d54
	ctx.r[4].s64 = ctx.r[11].s64 + 23892;
	// 82438538: 38630013  addi r3, r3, 0x13
	ctx.r[3].s64 = ctx.r[3].s64 + 19;
	// 8243853C: 480FAC55  bl 0x82533190
	ctx.lr = 0x82438540;
	sub_82533190(ctx, base);
	// 82438540: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82438544: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82438548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243854C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438554: 4E800020  blr
	return;
	// 82438558: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243855C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82438570 size=16
    let mut pc: u32 = 0x82438570;
    'dispatch: loop {
        match pc {
            0x82438570 => {
    //   block [0x82438570..0x82438580)
	// 82438570: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 82438574: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82438578: 71630023  andi. r3, r11, 0x23
	ctx.r[3].u64 = ctx.r[11].u64 & 35;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243857C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438580 size=64
    let mut pc: u32 = 0x82438580;
    'dispatch: loop {
        match pc {
            0x82438580 => {
    //   block [0x82438580..0x824385C0)
	// 82438580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243858C: 3D00828A  lis r8, -0x7d76
	ctx.r[8].s64 = -2104885248;
	// 82438590: 81489F48  lwz r10, -0x60b8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24760 as u32) ) } as u64;
	// 82438594: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82438598: 40980018  bge cr6, 0x824385b0
	if !ctx.cr[6].lt {
	pc = 0x824385B0; continue 'dispatch;
	}
	// 8243859C: 4BFFFF05  bl 0x824384a0
	ctx.lr = 0x824385A0;
	sub_824384A0(ctx, base);
	// 824385A0: 3D20828A  lis r9, -0x7d76
	ctx.r[9].s64 = -2104885248;
	// 824385A4: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 824385A8: 90699F44  stw r3, -0x60bc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-24764 as u32), ctx.r[3].u32 ) };
	// 824385AC: 91689F48  stw r11, -0x60b8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-24760 as u32), ctx.r[11].u32 ) };
	// 824385B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824385B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824385B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824385BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824385C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824385C0 size=228
    let mut pc: u32 = 0x824385C0;
    'dispatch: loop {
        match pc {
            0x824385C0 => {
    //   block [0x824385C0..0x824386A4)
	// 824385C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824385C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824385C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824385CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824385D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824385D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824385D8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 824385DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824385E0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824385E4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824385E8: 419A00A4  beq cr6, 0x8243868c
	if ctx.cr[6].eq {
	pc = 0x8243868C; continue 'dispatch;
	}
	// 824385EC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824385F0: 4099009C  ble cr6, 0x8243868c
	if !ctx.cr[6].gt {
	pc = 0x8243868C; continue 'dispatch;
	}
	// 824385F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824385F8: 3BEB5D60  addi r31, r11, 0x5d60
	ctx.r[31].s64 = ctx.r[11].s64 + 23904;
	// 824385FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438600: 3BCB5D6C  addi r30, r11, 0x5d6c
	ctx.r[30].s64 = ctx.r[11].s64 + 23916;
	// 82438604: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82438608: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8243860C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82438610: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438614: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438618: 7D234851  subf. r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8243861C: 40820014  bne 0x82438630
	if !ctx.cr[0].eq {
	pc = 0x82438630; continue 'dispatch;
	}
	// 82438620: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438624: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82438628: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8243862C: 409AFFE4  bne cr6, 0x82438610
	if !ctx.cr[6].eq {
	pc = 0x82438610; continue 'dispatch;
	}
	// 82438630: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82438634: 409A0048  bne cr6, 0x8243867c
	if !ctx.cr[6].eq {
	pc = 0x8243867C; continue 'dispatch;
	}
	// 82438638: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8243863C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82438640: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 82438644: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438648: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243864C: 7D234851  subf. r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82438650: 40820014  bne 0x82438664
	if !ctx.cr[0].eq {
	pc = 0x82438664; continue 'dispatch;
	}
	// 82438654: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438658: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8243865C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82438660: 409AFFE4  bne cr6, 0x82438644
	if !ctx.cr[6].eq {
	pc = 0x82438644; continue 'dispatch;
	}
	// 82438664: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82438668: 409A0014  bne cr6, 0x8243867c
	if !ctx.cr[6].eq {
	pc = 0x8243867C; continue 'dispatch;
	}
	// 8243866C: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82438670: 90E50000  stw r7, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438674: 4BFFFEFD  bl 0x82438570
	ctx.lr = 0x82438678;
	sub_82438570(ctx, base);
	// 82438678: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8243867C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 82438680: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82438684: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82438688: 409AFF7C  bne cr6, 0x82438604
	if !ctx.cr[6].eq {
	pc = 0x82438604; continue 'dispatch;
	}
	// 8243868C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438698: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243869C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824386A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824386A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824386A8 size=40
    let mut pc: u32 = 0x824386A8;
    'dispatch: loop {
        match pc {
            0x824386A8 => {
    //   block [0x824386A8..0x824386D0)
	// 824386A8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824386AC: 394B0720  addi r10, r11, 0x720
	ctx.r[10].s64 = ctx.r[11].s64 + 1824;
	// 824386B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824386B4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824386B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824386BC: 40990024  ble cr6, 0x824386e0
	if !ctx.cr[6].gt {
		sub_824386D0(ctx, base);
		return;
	}
	// 824386C0: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 824386C4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824386C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824386CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824386D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824386D0 size=24
    let mut pc: u32 = 0x824386D0;
    'dispatch: loop {
        match pc {
            0x824386D0 => {
    //   block [0x824386D0..0x824386E8)
	// 824386D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824386D4: 3863004C  addi r3, r3, 0x4c
	ctx.r[3].s64 = ctx.r[3].s64 + 76;
	// 824386D8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824386DC: 4198FFE8  blt cr6, 0x824386c4
	if ctx.cr[6].lt {
		sub_824386A8(ctx, base);
		return;
	}
	// 824386E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824386E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824386E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824386E8 size=36
    let mut pc: u32 = 0x824386E8;
    'dispatch: loop {
        match pc {
            0x824386E8 => {
    //   block [0x824386E8..0x8243870C)
	// 824386E8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824386EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824386F0: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824386F4: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 824386F8: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824386FC: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82438700: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82438704: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82438708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438710 size=8
    let mut pc: u32 = 0x82438710;
    'dispatch: loop {
        match pc {
            0x82438710 => {
    //   block [0x82438710..0x82438718)
	// 82438710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438714: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438718 size=28
    let mut pc: u32 = 0x82438718;
    'dispatch: loop {
        match pc {
            0x82438718 => {
    //   block [0x82438718..0x82438734)
	// 82438718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243871C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438720: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438724: 814B0720  lwz r10, 0x720(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 82438728: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8243872C: 914B0720  stw r10, 0x720(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1824 as u32), ctx.r[10].u32 ) };
	// 82438730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438738 size=188
    let mut pc: u32 = 0x82438738;
    'dispatch: loop {
        match pc {
            0x82438738 => {
    //   block [0x82438738..0x824387F4)
	// 82438738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243873C: 480FC981  bl 0x825350bc
	ctx.lr = 0x82438740;
	sub_82535080(ctx, base);
	// 82438740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438748: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243874C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438750: 409A002C  bne cr6, 0x8243877c
	if !ctx.cr[6].eq {
	pc = 0x8243877C; continue 'dispatch;
	}
	// 82438754: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82438758: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8243875C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82438760: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82438764: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82438768: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8243876C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82438770: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82438774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438778: 480FC994  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243877C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438780: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82438784: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82438788: 3BCB48DC  addi r30, r11, 0x48dc
	ctx.r[30].s64 = ctx.r[11].s64 + 18652;
	// 8243878C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82438790: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82438794: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8243879C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824387A0: 388B5D78  addi r4, r11, 0x5d78
	ctx.r[4].s64 = ctx.r[11].s64 + 23928;
	// 824387A4: 4BFF086D  bl 0x82429010
	ctx.lr = 0x824387A8;
	sub_82429010(ctx, base);
	// 824387A8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824387AC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824387B0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 824387B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824387B8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824387BC: 388B5D70  addi r4, r11, 0x5d70
	ctx.r[4].s64 = ctx.r[11].s64 + 23920;
	// 824387C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824387C4: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 824387C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824387CC: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 824387D0: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824387D4: 4BFF083D  bl 0x82429010
	ctx.lr = 0x824387D8;
	sub_82429010(ctx, base);
	// 824387D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824387DC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824387E0: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 824387E4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824387E8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 824387EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824387F0: 480FC91C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824387F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824387F8 size=132
    let mut pc: u32 = 0x824387F8;
    'dispatch: loop {
        match pc {
            0x824387F8 => {
    //   block [0x824387F8..0x8243887C)
	// 824387F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824387FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438800: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82438804: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82438808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243880C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82438810: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82438814: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82438818: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243881C: 409A003C  bne cr6, 0x82438858
	if !ctx.cr[6].eq {
	pc = 0x82438858; continue 'dispatch;
	}
	// 82438820: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82438824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438828: 419A0030  beq cr6, 0x82438858
	if ctx.cr[6].eq {
	pc = 0x82438858; continue 'dispatch;
	}
	// 8243882C: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82438830: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82438834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82438838: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243883C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82438840: 4BFF07D1  bl 0x82429010
	ctx.lr = 0x82438844;
	sub_82429010(ctx, base);
	// 82438844: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82438848: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243884C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438850: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438854: 48000010  b 0x82438864
	pc = 0x82438864; continue 'dispatch;
	// 82438858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243885C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438860: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243886C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438870: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82438874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82438878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438880 size=128
    let mut pc: u32 = 0x82438880;
    'dispatch: loop {
        match pc {
            0x82438880 => {
    //   block [0x82438880..0x82438900)
	// 82438880: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82438884: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82438888: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8243888C: 6127FFFF  ori r7, r9, 0xffff
	ctx.r[7].u64 = ctx.r[9].u64 | 65535;
	// 82438890: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82438894: 409A0008  bne cr6, 0x8243889c
	if !ctx.cr[6].eq {
	pc = 0x8243889C; continue 'dispatch;
	}
	// 82438898: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 8243889C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824388A0: 409A0008  bne cr6, 0x824388a8
	if !ctx.cr[6].eq {
	pc = 0x824388A8; continue 'dispatch;
	}
	// 824388A4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 824388A8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 824388AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824388B0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 824388B4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 824388B8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824388BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824388C0: 4200FFF8  bdnz 0x824388b8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824388B8; continue 'dispatch;
	}
	// 824388C4: 39660024  addi r11, r6, 0x24
	ctx.r[11].s64 = ctx.r[6].s64 + 36;
	// 824388C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 824388CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824388D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824388D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824388D8: 4200FFF8  bdnz 0x824388d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824388D0; continue 'dispatch;
	}
	// 824388DC: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 824388E0: 409A0020  bne cr6, 0x82438900
	if !ctx.cr[6].eq {
		sub_82438900(ctx, base);
		return;
	}
	// 824388E4: 39660044  addi r11, r6, 0x44
	ctx.r[11].s64 = ctx.r[6].s64 + 68;
	// 824388E8: 392000CF  li r9, 0xcf
	ctx.r[9].s64 = 207;
	// 824388EC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824388F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824388F4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824388F8: 4200FFF8  bdnz 0x824388f0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824388F0; continue 'dispatch;
	}
	// 824388FC: 4800003C  b 0x82438938
	sub_82438900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438900 size=112
    let mut pc: u32 = 0x82438900;
    'dispatch: loop {
        match pc {
            0x82438900 => {
    //   block [0x82438900..0x82438970)
	// 82438900: 3D202793  lis r9, 0x2793
	ctx.r[9].s64 = 663945216;
	// 82438904: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82438908: 61282B49  ori r8, r9, 0x2b49
	ctx.r[8].u64 = ctx.r[9].u64 | 11081;
	// 8243890C: 39260044  addi r9, r6, 0x44
	ctx.r[9].s64 = ctx.r[6].s64 + 68;
	// 82438910: 7D6B4016  mulhwu r11, r11, r8
	ctx.r[11].u64 = ((ctx.r[11].u32 as u64 * ctx.r[8].u32 as u64) >> 32);
	// 82438914: 5568D97E  srwi r8, r11, 5
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82438918: 396000CF  li r11, 0xcf
	ctx.r[11].s64 = 207;
	// 8243891C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82438920: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82438924: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82438928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243892C: 90890000  stw r4, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82438930: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82438934: 409AFFE8  bne cr6, 0x8243891c
	if !ctx.cr[6].eq {
	pc = 0x8243891C; continue 'dispatch;
	}
	// 82438938: 39660380  addi r11, r6, 0x380
	ctx.r[11].s64 = ctx.r[6].s64 + 896;
	// 8243893C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82438940: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82438944: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82438948: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8243894C: 4200FFF8  bdnz 0x82438944
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438944; continue 'dispatch;
	}
	// 82438950: 396603C0  addi r11, r6, 0x3c0
	ctx.r[11].s64 = ctx.r[6].s64 + 960;
	// 82438954: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82438958: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8243895C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82438960: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82438964: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438968: 4200FFF8  bdnz 0x82438960
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438960; continue 'dispatch;
	}
	// 8243896C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82438970 size=488
    let mut pc: u32 = 0x82438970;
    'dispatch: loop {
        match pc {
            0x82438970 => {
    //   block [0x82438970..0x82438B58)
	// 82438970: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82438974: 39060400  addi r8, r6, 0x400
	ctx.r[8].s64 = ctx.r[6].s64 + 1024;
	// 82438978: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8243897C: 3BE80400  addi r31, r8, 0x400
	ctx.r[31].s64 = ctx.r[8].s64 + 1024;
	// 82438980: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82438984: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82438988: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8243898C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82438990: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82438994: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438998: 4200FFF8  bdnz 0x82438990
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438990; continue 'dispatch;
	}
	// 8243899C: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 824389A0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 824389A4: 20FFFFF0  subfic r7, r31, -0x10
	ctx.xer.ca = ctx.r[31].u32 <= -16 as u32;
	ctx.r[7].s64 = (-16 as i64) - ctx.r[31].s64;
	// 824389A8: 392000DC  li r9, 0xdc
	ctx.r[9].s64 = 220;
	// 824389AC: C0035B98  lfs f0, 0x5b98(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(23448 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824389B0: 7C875A14  add r4, r7, r11
	ctx.r[4].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 824389B4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824389B8: 7C8407B4  extsw r4, r4
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 824389BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824389C0: F881FFE0  std r4, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[4].u64 ) };
	// 824389C4: C9A1FFE0  lfd f13, -0x20(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824389C8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824389CC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824389D0: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824389D4: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 824389D8: D9A1FFE8  stfd f13, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[13].u64 ) };
	// 824389DC: 8881FFEF  lbz r4, -0x11(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-17 as u32) ) } as u64;
	// 824389E0: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 824389E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824389E8: 409AFFC8  bne cr6, 0x824389b0
	if !ctx.cr[6].eq {
	pc = 0x824389B0; continue 'dispatch;
	}
	// 824389EC: 397F00EC  addi r11, r31, 0xec
	ctx.r[11].s64 = ctx.r[31].s64 + 236;
	// 824389F0: 38E000FF  li r7, 0xff
	ctx.r[7].s64 = 255;
	// 824389F4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 824389F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824389FC: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82438A00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438A04: 4200FFF8  bdnz 0x824389fc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824389FC; continue 'dispatch;
	}
	// 82438A08: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82438A0C: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82438A10: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82438A14: 6124FFFF  ori r4, r9, 0xffff
	ctx.r[4].u64 = ctx.r[9].u64 | 65535;
	// 82438A18: 409A0008  bne cr6, 0x82438a20
	if !ctx.cr[6].eq {
	pc = 0x82438A20; continue 'dispatch;
	}
	// 82438A1C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82438A20: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82438A24: 409A0008  bne cr6, 0x82438a2c
	if !ctx.cr[6].eq {
	pc = 0x82438A2C; continue 'dispatch;
	}
	// 82438A28: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82438A2C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82438A30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82438A34: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82438A38: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82438A3C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438A40: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438A44: 4200FFF8  bdnz 0x82438a3c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438A3C; continue 'dispatch;
	}
	// 82438A48: 39680024  addi r11, r8, 0x24
	ctx.r[11].s64 = ctx.r[8].s64 + 36;
	// 82438A4C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82438A50: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82438A54: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438A58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438A5C: 4200FFF8  bdnz 0x82438a54
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438A54; continue 'dispatch;
	}
	// 82438A60: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82438A64: 409A0020  bne cr6, 0x82438a84
	if !ctx.cr[6].eq {
	pc = 0x82438A84; continue 'dispatch;
	}
	// 82438A68: 39680044  addi r11, r8, 0x44
	ctx.r[11].s64 = ctx.r[8].s64 + 68;
	// 82438A6C: 392000CF  li r9, 0xcf
	ctx.r[9].s64 = 207;
	// 82438A70: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82438A74: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438A78: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438A7C: 4200FFF8  bdnz 0x82438a74
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438A74; continue 'dispatch;
	}
	// 82438A80: 4800003C  b 0x82438abc
	pc = 0x82438ABC; continue 'dispatch;
	// 82438A84: 3D202793  lis r9, 0x2793
	ctx.r[9].s64 = 663945216;
	// 82438A88: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82438A8C: 61272B49  ori r7, r9, 0x2b49
	ctx.r[7].u64 = ctx.r[9].u64 | 11081;
	// 82438A90: 39280044  addi r9, r8, 0x44
	ctx.r[9].s64 = ctx.r[8].s64 + 68;
	// 82438A94: 7D6B3816  mulhwu r11, r11, r7
	ctx.r[11].u64 = ((ctx.r[11].u32 as u64 * ctx.r[7].u32 as u64) >> 32);
	// 82438A98: 5567D97E  srwi r7, r11, 5
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82438A9C: 396000CF  li r11, 0xcf
	ctx.r[11].s64 = 207;
	// 82438AA0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82438AA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82438AA8: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82438AAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438AB0: 90690000  stw r3, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82438AB4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82438AB8: 409AFFE8  bne cr6, 0x82438aa0
	if !ctx.cr[6].eq {
	pc = 0x82438AA0; continue 'dispatch;
	}
	// 82438ABC: 39680380  addi r11, r8, 0x380
	ctx.r[11].s64 = ctx.r[8].s64 + 896;
	// 82438AC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82438AC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82438AC8: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82438ACC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438AD0: 4200FFF8  bdnz 0x82438ac8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438AC8; continue 'dispatch;
	}
	// 82438AD4: 396803C0  addi r11, r8, 0x3c0
	ctx.r[11].s64 = ctx.r[8].s64 + 960;
	// 82438AD8: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82438ADC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82438AE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82438AE4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82438AE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438AEC: 4200FFF8  bdnz 0x82438ae4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438AE4; continue 'dispatch;
	}
	// 82438AF0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82438AF4: 39460008  addi r10, r6, 8
	ctx.r[10].s64 = ctx.r[6].s64 + 8;
	// 82438AF8: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82438AFC: 88EBFFFF  lbz r7, -1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82438B00: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82438B04: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B08: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82438B0C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B10: 90EAFFF8  stw r7, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 82438B14: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438B18: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B1C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B20: 90EAFFFC  stw r7, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 82438B24: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82438B28: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B2C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B30: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438B34: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82438B38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438B3C: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B40: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B44: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82438B48: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82438B4C: 409AFFB0  bne cr6, 0x82438afc
	if !ctx.cr[6].eq {
	pc = 0x82438AFC; continue 'dispatch;
	}
	// 82438B50: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82438B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438B58 size=16
    let mut pc: u32 = 0x82438B58;
    'dispatch: loop {
        match pc {
            0x82438B58 => {
    //   block [0x82438B58..0x82438B68)
	// 82438B58: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438B5C: 396B0720  addi r11, r11, 0x720
	ctx.r[11].s64 = ctx.r[11].s64 + 1824;
	// 82438B60: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82438B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438B68 size=16
    let mut pc: u32 = 0x82438B68;
    'dispatch: loop {
        match pc {
            0x82438B68 => {
    //   block [0x82438B68..0x82438B78)
	// 82438B68: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438B6C: 396B0720  addi r11, r11, 0x720
	ctx.r[11].s64 = ctx.r[11].s64 + 1824;
	// 82438B70: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82438B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438B78 size=76
    let mut pc: u32 = 0x82438B78;
    'dispatch: loop {
        match pc {
            0x82438B78 => {
    //   block [0x82438B78..0x82438BC4)
	// 82438B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82438B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438B88: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438B8C: 38A0026C  li r5, 0x26c
	ctx.r[5].s64 = 620;
	// 82438B90: 3BEB0720  addi r31, r11, 0x720
	ctx.r[31].s64 = ctx.r[11].s64 + 1824;
	// 82438B94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82438B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438B9C: 480FC635  bl 0x825351d0
	ctx.lr = 0x82438BA0;
	sub_825351D0(ctx, base);
	// 82438BA0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82438BA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82438BA8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82438BAC: 4BFFFFAD  bl 0x82438b58
	ctx.lr = 0x82438BB0;
	sub_82438B58(ctx, base);
	// 82438BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82438BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438BC8 size=84
    let mut pc: u32 = 0x82438BC8;
    'dispatch: loop {
        match pc {
            0x82438BC8 => {
    //   block [0x82438BC8..0x82438C1C)
	// 82438BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438BD4: 4BFFFAD5  bl 0x824386a8
	ctx.lr = 0x82438BD8;
	sub_824386A8(ctx, base);
	// 82438BD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438BDC: 409A0014  bne cr6, 0x82438bf0
	if !ctx.cr[6].eq {
	pc = 0x82438BF0; continue 'dispatch;
	}
	// 82438BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438BEC: 4E800020  blr
	return;
	// 82438BF0: 4BFFFAF9  bl 0x824386e8
	ctx.lr = 0x82438BF4;
	sub_824386E8(ctx, base);
	// 82438BF4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82438BFC: 814B0720  lwz r10, 0x720(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 82438C00: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82438C04: 914B0720  stw r10, 0x720(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1824 as u32), ctx.r[10].u32 ) };
	// 82438C08: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82438C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438C20 size=20
    let mut pc: u32 = 0x82438C20;
    'dispatch: loop {
        match pc {
            0x82438C20 => {
    //   block [0x82438C20..0x82438C34)
	// 82438C20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82438C24: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82438C28: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82438C2C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82438C30: 4BFFFB08  b 0x82438738
	sub_82438738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438C38 size=236
    let mut pc: u32 = 0x82438C38;
    'dispatch: loop {
        match pc {
            0x82438C38 => {
    //   block [0x82438C38..0x82438D24)
	// 82438C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438C3C: 480FC479  bl 0x825350b4
	ctx.lr = 0x82438C40;
	sub_82535080(ctx, base);
	// 82438C40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438C44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438C48: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82438C4C: 3BCB48DC  addi r30, r11, 0x48dc
	ctx.r[30].s64 = ctx.r[11].s64 + 18652;
	// 82438C50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438C54: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438C58: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82438C5C: 388B5D98  addi r4, r11, 0x5d98
	ctx.r[4].s64 = ctx.r[11].s64 + 23960;
	// 82438C60: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82438C64: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82438C68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438C6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438C70: 4BFFFB89  bl 0x824387f8
	ctx.lr = 0x82438C74;
	sub_824387F8(ctx, base);
	// 82438C74: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82438C78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438C7C: 409A0020  bne cr6, 0x82438c9c
	if !ctx.cr[6].eq {
	pc = 0x82438C9C; continue 'dispatch;
	}
	// 82438C80: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82438C84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82438C88: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82438C8C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438C90: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438C94: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82438C98: 480FC46C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82438C9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438CA0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82438CA4: 388B5D94  addi r4, r11, 0x5d94
	ctx.r[4].s64 = ctx.r[11].s64 + 23956;
	// 82438CA8: 480FEF59  bl 0x82537c00
	ctx.lr = 0x82438CAC;
	sub_82537C00(ctx, base);
	// 82438CAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438CB0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82438CB4: 388B5D8C  addi r4, r11, 0x5d8c
	ctx.r[4].s64 = ctx.r[11].s64 + 23948;
	// 82438CB8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82438CBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438CC4: 4BFFFB35  bl 0x824387f8
	ctx.lr = 0x82438CC8;
	sub_824387F8(ctx, base);
	// 82438CC8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82438CCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438CD0: 409A0018  bne cr6, 0x82438ce8
	if !ctx.cr[6].eq {
	pc = 0x82438CE8; continue 'dispatch;
	}
	// 82438CD4: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82438CD8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82438CDC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438CE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82438CE4: 480FC420  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82438CE8: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82438CEC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82438CF0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82438CF4: 7D4AD9D6  mullw r10, r10, r27
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[27].s32 as i64);
	// 82438CF8: 38895D80  addi r4, r9, 0x5d80
	ctx.r[4].s64 = ctx.r[9].s64 + 23936;
	// 82438CFC: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82438D00: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82438D04: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82438D08: 480FEEF9  bl 0x82537c00
	ctx.lr = 0x82438D0C;
	sub_82537C00(ctx, base);
	// 82438D0C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82438D10: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82438D14: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438D18: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438D1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82438D20: 480FC3E4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438D28 size=80
    let mut pc: u32 = 0x82438D28;
    'dispatch: loop {
        match pc {
            0x82438D28 => {
    //   block [0x82438D28..0x82438D78)
	// 82438D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438D2C: 480FC38D  bl 0x825350b8
	ctx.lr = 0x82438D30;
	sub_82535080(ctx, base);
	// 82438D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438D38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82438D3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438D40: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82438D44: 4BFFE60D  bl 0x82437350
	ctx.lr = 0x82438D48;
	sub_82437350(ctx, base);
	// 82438D48: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82438D4C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82438D50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82438D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438D5C: 409A0010  bne cr6, 0x82438d6c
	if !ctx.cr[6].eq {
	pc = 0x82438D6C; continue 'dispatch;
	}
	// 82438D60: 4BFFFC11  bl 0x82438970
	ctx.lr = 0x82438D64;
	sub_82438970(ctx, base);
	// 82438D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438D68: 480FC3A0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82438D6C: 4BFFFB15  bl 0x82438880
	ctx.lr = 0x82438D70;
	sub_82438880(ctx, base);
	// 82438D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438D74: 480FC394  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82438D78 size=252
    let mut pc: u32 = 0x82438D78;
    'dispatch: loop {
        match pc {
            0x82438D78 => {
    //   block [0x82438D78..0x82438E74)
	// 82438D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438D80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438D84: C1A3003C  lfs f13, 0x3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82438D88: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82438D8C: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82438D90: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82438D94: 4BFFFDD5  bl 0x82438b68
	ctx.lr = 0x82438D98;
	sub_82438B68(ctx, base);
	// 82438D98: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82438D9C: 409A0038  bne cr6, 0x82438dd4
	if !ctx.cr[6].eq {
	pc = 0x82438DD4; continue 'dispatch;
	}
	// 82438DA0: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 82438DA4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438DA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82438DAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82438DB0: 55088BFE  srwi r8, r8, 0xf
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82438DB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438DB8: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82438DBC: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82438DC0: 409AFFE4  bne cr6, 0x82438da4
	if !ctx.cr[6].eq {
	pc = 0x82438DA4; continue 'dispatch;
	}
	// 82438DC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438DD0: 4E800020  blr
	return;
	// 82438DD4: FD606828  fsub f11, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82438DD8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82438DDC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82438DE0: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 82438DE4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82438DE8: C9882000  lfd f12, 0x2000(r8)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8192 as u32) ) };
	// 82438DEC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82438DF0: FD8C5824  fdiv f12, f12, f11
	ctx.f[12].f64 = ctx.f[12].f64 / ctx.f[11].f64;
	// 82438DF4: FD8C0032  fmul f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 82438DF8: FD6C0372  fmul f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438DFC: C9A85DA8  lfd f13, 0x5da8(r8)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(23976 as u32) ) };
	// 82438E00: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82438E04: FD8C0372  fmul f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438E08: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 82438E0C: C9685DA0  lfd f11, 0x5da0(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(23968 as u32) ) };
	// 82438E10: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438E14: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82438E18: 409A0008  bne cr6, 0x82438e20
	if !ctx.cr[6].eq {
	pc = 0x82438E20; continue 'dispatch;
	}
	// 82438E1C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438E20: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438E24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82438E28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438E2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438E30: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82438E34: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82438E38: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82438E3C: FD4A0032  fmul f10, f10, f0
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64;
	// 82438E40: FD4A02F2  fmul f10, f10, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[11].f64;
	// 82438E44: FD4D5024  fdiv f10, f13, f10
	ctx.f[10].f64 = ctx.f[13].f64 / ctx.f[10].f64;
	// 82438E48: FD4C5028  fsub f10, f12, f10
	ctx.f[10].f64 = ctx.f[12].f64 - ctx.f[10].f64;
	// 82438E4C: FD40565E  fctidz f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 82438E50: D9410058  stfd f10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[10].u64 ) };
	// 82438E54: A101005E  lhz r8, 0x5e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 82438E58: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82438E5C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82438E60: 409AFFB0  bne cr6, 0x82438e10
	if !ctx.cr[6].eq {
	pc = 0x82438E10; continue 'dispatch;
	}
	// 82438E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82438E78 size=240
    let mut pc: u32 = 0x82438E78;
    'dispatch: loop {
        match pc {
            0x82438E78 => {
    //   block [0x82438E78..0x82438F68)
	// 82438E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438E84: C1A3003C  lfs f13, 0x3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82438E88: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82438E8C: 4BFFFCDD  bl 0x82438b68
	ctx.lr = 0x82438E90;
	sub_82438B68(ctx, base);
	// 82438E90: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82438E94: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 82438E98: 409A0038  bne cr6, 0x82438ed0
	if !ctx.cr[6].eq {
	pc = 0x82438ED0; continue 'dispatch;
	}
	// 82438E9C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82438EA0: 7D252050  subf r9, r5, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82438EA4: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82438EA8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82438EAC: 5508082E  rlwinm r8, r8, 1, 0, 0x17
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x7FFFFFFFu64;
	// 82438EB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438EB4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82438EB8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438EBC: 409AFFE8  bne cr6, 0x82438ea4
	if !ctx.cr[6].eq {
	pc = 0x82438EA4; continue 'dispatch;
	}
	// 82438EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438ECC: 4E800020  blr
	return;
	// 82438ED0: FD606828  fsub f11, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82438ED4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82438ED8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82438EDC: 7D042850  subf r8, r4, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 82438EE0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82438EE4: C9892000  lfd f12, 0x2000(r9)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8192 as u32) ) };
	// 82438EE8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82438EEC: FD8C5824  fdiv f12, f12, f11
	ctx.f[12].f64 = ctx.f[12].f64 / ctx.f[11].f64;
	// 82438EF0: FD8C0032  fmul f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 82438EF4: FD6C0372  fmul f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438EF8: C9A95DB0  lfd f13, 0x5db0(r9)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(23984 as u32) ) };
	// 82438EFC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82438F00: FD8C0372  fmul f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438F04: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 82438F08: C9695DA0  lfd f11, 0x5da0(r9)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(23968 as u32) ) };
	// 82438F0C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438F10: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82438F14: 409A0008  bne cr6, 0x82438f1c
	if !ctx.cr[6].eq {
	pc = 0x82438F1C; continue 'dispatch;
	}
	// 82438F18: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438F1C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438F20: 7CC85A14  add r6, r8, r11
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82438F24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82438F28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438F2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438F30: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82438F34: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82438F38: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82438F3C: FD4A0032  fmul f10, f10, f0
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64;
	// 82438F40: FD4A02F2  fmul f10, f10, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[11].f64;
	// 82438F44: FD4D5024  fdiv f10, f13, f10
	ctx.f[10].f64 = ctx.f[13].f64 / ctx.f[10].f64;
	// 82438F48: FD4C5028  fsub f10, f12, f10
	ctx.f[10].f64 = ctx.f[12].f64 - ctx.f[10].f64;
	// 82438F4C: FD40565E  fctidz f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 82438F50: 7D4037AE  stfiwx f10, 0, r6
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32, tmp.u32) };
	// 82438F54: 409AFFB8  bne cr6, 0x82438f0c
	if !ctx.cr[6].eq {
	pc = 0x82438F0C; continue 'dispatch;
	}
	// 82438F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438F68 size=4
    let mut pc: u32 = 0x82438F68;
    'dispatch: loop {
        match pc {
            0x82438F68 => {
    //   block [0x82438F68..0x82438F6C)
	// 82438F68: 4BFFFC10  b 0x82438b78
	sub_82438B78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82438F70 size=160
    let mut pc: u32 = 0x82438F70;
    'dispatch: loop {
        match pc {
            0x82438F70 => {
    //   block [0x82438F70..0x82439010)
	// 82438F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438F74: 480FC141  bl 0x825350b4
	ctx.lr = 0x82438F78;
	sub_82535080(ctx, base);
	// 82438F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438F7C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82438F80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438F84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82438F88: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82438F8C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 82438F90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82438F94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82438F98: 3BBE0400  addi r29, r30, 0x400
	ctx.r[29].s64 = ctx.r[30].s64 + 1024;
	// 82438F9C: 480FC235  bl 0x825351d0
	ctx.lr = 0x82438FA0;
	sub_825351D0(ctx, base);
	// 82438FA0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82438FA4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82438FA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82438FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438FB0: 4BFFFD79  bl 0x82438d28
	ctx.lr = 0x82438FB4;
	sub_82438D28(ctx, base);
	// 82438FB4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82438FB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438FBC: 409A0034  bne cr6, 0x82438ff0
	if !ctx.cr[6].eq {
	pc = 0x82438FF0; continue 'dispatch;
	}
	// 82438FC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82438FC4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438FC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82438FCC: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82438FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438FD4: 409A0010  bne cr6, 0x82438fe4
	if !ctx.cr[6].eq {
	pc = 0x82438FE4; continue 'dispatch;
	}
	// 82438FD8: 4BFFFDA1  bl 0x82438d78
	ctx.lr = 0x82438FDC;
	sub_82438D78(ctx, base);
	// 82438FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438FE0: 480FC124  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82438FE4: 4BFFFE95  bl 0x82438e78
	ctx.lr = 0x82438FE8;
	sub_82438E78(ctx, base);
	// 82438FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438FEC: 480FC118  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82438FF0: C05F0040  lfs f2, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82438FF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82438FF8: C03F003C  lfs f1, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82438FFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82439000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82439004: 4E800421  bctrl
	ctx.lr = 0x82439008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82439008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243900C: 480FC0F8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439010 size=84
    let mut pc: u32 = 0x82439010;
    'dispatch: loop {
        match pc {
            0x82439010 => {
    //   block [0x82439010..0x82439064)
	// 82439010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243901C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439024: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82439028: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243902C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82439030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439034: 4BFFFC05  bl 0x82438c38
	ctx.lr = 0x82439038;
	sub_82438C38(ctx, base);
	// 82439038: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243903C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439040: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82439044: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82439048: 4BFFFF29  bl 0x82438f70
	ctx.lr = 0x8243904C;
	sub_82438F70(ctx, base);
	// 8243904C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439058: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243905C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439068 size=56
    let mut pc: u32 = 0x82439068;
    'dispatch: loop {
        match pc {
            0x82439068 => {
    //   block [0x82439068..0x824390A0)
	// 82439068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439074: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82439078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8243907C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82439080: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82439084: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82439088: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243908C: 4800FE6D  bl 0x82448ef8
	ctx.lr = 0x82439090;
	sub_82448EF8(ctx, base);
	// 82439090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243909C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824390A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824390A0 size=20
    let mut pc: u32 = 0x824390A0;
    'dispatch: loop {
        match pc {
            0x824390A0 => {
    //   block [0x824390A0..0x824390B4)
	// 824390A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824390A4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 824390A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824390AC: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 824390B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824390B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824390B8 size=104
    let mut pc: u32 = 0x824390B8;
    'dispatch: loop {
        match pc {
            0x824390B8 => {
    //   block [0x824390B8..0x82439120)
	// 824390B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824390BC: 480FC001  bl 0x825350bc
	ctx.lr = 0x824390C0;
	sub_82535080(ctx, base);
	// 824390C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824390C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824390C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824390CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824390D0: 4800E771  bl 0x82447840
	ctx.lr = 0x824390D4;
	sub_82447840(ctx, base);
	// 824390D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824390D8: 419A001C  beq cr6, 0x824390f4
	if ctx.cr[6].eq {
	pc = 0x824390F4; continue 'dispatch;
	}
	// 824390DC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824390E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824390E4: 60840144  ori r4, r4, 0x144
	ctx.r[4].u64 = ctx.r[4].u64 | 324;
	// 824390E8: 4800E821  bl 0x82447908
	ctx.lr = 0x824390EC;
	sub_82447908(ctx, base);
	// 824390EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824390F0: 480FC01C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824390F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824390F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824390FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439100: 48006119  bl 0x8243f218
	ctx.lr = 0x82439104;
	sub_8243F218(ctx, base);
	// 82439104: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82439108: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243910C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439110: 4800C9D1  bl 0x82445ae0
	ctx.lr = 0x82439114;
	sub_82445AE0(ctx, base);
	// 82439114: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243911C: 480FBFF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439120 size=104
    let mut pc: u32 = 0x82439120;
    'dispatch: loop {
        match pc {
            0x82439120 => {
    //   block [0x82439120..0x82439188)
	// 82439120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243912C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439138: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243913C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82439140: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82439144: 419A0014  beq cr6, 0x82439158
	if ctx.cr[6].eq {
	pc = 0x82439158; continue 'dispatch;
	}
	// 82439148: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8243914C: 419A000C  beq cr6, 0x82439158
	if ctx.cr[6].eq {
	pc = 0x82439158; continue 'dispatch;
	}
	// 82439150: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439154: 4800001C  b 0x82439170
	pc = 0x82439170; continue 'dispatch;
	// 82439158: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243915C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439160: 48006789  bl 0x8243f8e8
	ctx.lr = 0x82439164;
	sub_8243F8E8(ctx, base);
	// 82439164: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82439168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243916C: 4BFFFEFD  bl 0x82439068
	ctx.lr = 0x82439170;
	sub_82439068(ctx, base);
	// 82439170: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243917C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439188 size=96
    let mut pc: u32 = 0x82439188;
    'dispatch: loop {
        match pc {
            0x82439188 => {
    //   block [0x82439188..0x824391E8)
	// 82439188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243918C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243919C: 4800E6A5  bl 0x82447840
	ctx.lr = 0x824391A0;
	sub_82447840(ctx, base);
	// 824391A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824391A4: 419A0028  beq cr6, 0x824391cc
	if ctx.cr[6].eq {
	pc = 0x824391CC; continue 'dispatch;
	}
	// 824391A8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824391AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824391B0: 60840143  ori r4, r4, 0x143
	ctx.r[4].u64 = ctx.r[4].u64 | 323;
	// 824391B4: 4800E755  bl 0x82447908
	ctx.lr = 0x824391B8;
	sub_82447908(ctx, base);
	// 824391B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824391BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824391C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824391C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824391C8: 4E800020  blr
	return;
	// 824391CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824391D0: 4BFFFED1  bl 0x824390a0
	ctx.lr = 0x824391D4;
	sub_824390A0(ctx, base);
	// 824391D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824391D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824391DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824391E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824391E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824391E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824391E8 size=28
    let mut pc: u32 = 0x824391E8;
    'dispatch: loop {
        match pc {
            0x824391E8 => {
    //   block [0x824391E8..0x82439204)
	// 824391E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824391EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824391F0: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 824391F4: 41980048  blt cr6, 0x8243923c
	if ctx.cr[6].lt {
		sub_8243923C(ctx, base);
		return;
	}
	// 824391F8: 419A0024  beq cr6, 0x8243921c
	if ctx.cr[6].eq {
		sub_8243921C(ctx, base);
		return;
	}
	// 824391FC: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82439200: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439204(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439204 size=12
    let mut pc: u32 = 0x82439204;
    'dispatch: loop {
        match pc {
            0x82439204 => {
    //   block [0x82439204..0x82439210)
	// 82439204: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82439208: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8243920C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439210 size=12
    let mut pc: u32 = 0x82439210;
    'dispatch: loop {
        match pc {
            0x82439210 => {
    //   block [0x82439210..0x8243921C)
	// 82439210: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82439214: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82439218: 4BFFFF08  b 0x82439120
	sub_82439120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243921C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243921C size=20
    let mut pc: u32 = 0x8243921C;
    'dispatch: loop {
        match pc {
            0x8243921C => {
    //   block [0x8243921C..0x82439230)
	// 8243921C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82439220: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82439224: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82439228: 912B0054  stw r9, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8243922C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439230 size=12
    let mut pc: u32 = 0x82439230;
    'dispatch: loop {
        match pc {
            0x82439230 => {
    //   block [0x82439230..0x8243923C)
	// 82439230: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82439234: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82439238: 4BFFFEE8  b 0x82439120
	sub_82439120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243923C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243923C size=16
    let mut pc: u32 = 0x8243923C;
    'dispatch: loop {
        match pc {
            0x8243923C => {
    //   block [0x8243923C..0x8243924C)
	// 8243923C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82439240: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82439244: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82439248: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243924C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243924C size=12
    let mut pc: u32 = 0x8243924C;
    'dispatch: loop {
        match pc {
            0x8243924C => {
    //   block [0x8243924C..0x82439258)
	// 8243924C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82439250: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82439254: 4BFFFECC  b 0x82439120
	sub_82439120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439258 size=4
    let mut pc: u32 = 0x82439258;
    'dispatch: loop {
        match pc {
            0x82439258 => {
    //   block [0x82439258..0x8243925C)
	// 82439258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439260 size=156
    let mut pc: u32 = 0x82439260;
    'dispatch: loop {
        match pc {
            0x82439260 => {
    //   block [0x82439260..0x824392FC)
	// 82439260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243926C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439278: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243927C: 4800E5C5  bl 0x82447840
	ctx.lr = 0x82439280;
	sub_82447840(ctx, base);
	// 82439280: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439284: 419A0018  beq cr6, 0x8243929c
	if ctx.cr[6].eq {
	pc = 0x8243929C; continue 'dispatch;
	}
	// 82439288: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243928C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439290: 60840142  ori r4, r4, 0x142
	ctx.r[4].u64 = ctx.r[4].u64 | 322;
	// 82439294: 4800E675  bl 0x82447908
	ctx.lr = 0x82439298;
	sub_82447908(ctx, base);
	// 82439298: 4800004C  b 0x824392e4
	pc = 0x824392E4; continue 'dispatch;
	// 8243929C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824392A0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824392A4: 409A001C  bne cr6, 0x824392c0
	if !ctx.cr[6].eq {
	pc = 0x824392C0; continue 'dispatch;
	}
	// 824392A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824392AC: 409A000C  bne cr6, 0x824392b8
	if !ctx.cr[6].eq {
	pc = 0x824392B8; continue 'dispatch;
	}
	// 824392B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824392B4: 48000030  b 0x824392e4
	pc = 0x824392E4; continue 'dispatch;
	// 824392B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824392BC: 48000014  b 0x824392d0
	pc = 0x824392D0; continue 'dispatch;
	// 824392C0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824392C4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824392C8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824392CC: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 824392D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824392D4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 824392D8: 4BFFFF11  bl 0x824391e8
	ctx.lr = 0x824392DC;
	sub_824391E8(ctx, base);
	// 824392DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824392E0: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824392E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824392E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824392EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824392F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824392F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824392F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439300 size=16
    let mut pc: u32 = 0x82439300;
    'dispatch: loop {
        match pc {
            0x82439300 => {
    //   block [0x82439300..0x82439310)
	// 82439300: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439304: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439308: 60840201  ori r4, r4, 0x201
	ctx.r[4].u64 = ctx.r[4].u64 | 513;
	// 8243930C: 4800E5FC  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439310 size=4
    let mut pc: u32 = 0x82439310;
    'dispatch: loop {
        match pc {
            0x82439310 => {
    //   block [0x82439310..0x82439314)
	// 82439310: 48006C08  b 0x8243ff18
	sub_8243FF18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439318 size=44
    let mut pc: u32 = 0x82439318;
    'dispatch: loop {
        match pc {
            0x82439318 => {
    //   block [0x82439318..0x82439344)
	// 82439318: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243931C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439320: 419A0024  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		sub_82439344(ctx, base);
		return;
	}
	// 82439324: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82439328: 419A001C  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		sub_82439344(ctx, base);
		return;
	}
	// 8243932C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82439330: 419A0014  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		sub_82439344(ctx, base);
		return;
	}
	// 82439334: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82439338: 419A000C  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		sub_82439344(ctx, base);
		return;
	}
	// 8243933C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82439340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439344 size=16
    let mut pc: u32 = 0x82439344;
    'dispatch: loop {
        match pc {
            0x82439344 => {
    //   block [0x82439344..0x82439354)
	// 82439344: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82439348: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243934C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82439350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439358 size=8
    let mut pc: u32 = 0x82439358;
    'dispatch: loop {
        match pc {
            0x82439358 => {
    //   block [0x82439358..0x82439360)
	// 82439358: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8243935C: 4800FB2C  b 0x82448e88
	sub_82448E88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439360 size=16
    let mut pc: u32 = 0x82439360;
    'dispatch: loop {
        match pc {
            0x82439360 => {
    //   block [0x82439360..0x82439370)
	// 82439360: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82439364: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82439368: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243936C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439370 size=16
    let mut pc: u32 = 0x82439370;
    'dispatch: loop {
        match pc {
            0x82439370 => {
    //   block [0x82439370..0x82439380)
	// 82439370: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82439374: 4099000C  ble cr6, 0x82439380
	if !ctx.cr[6].gt {
		sub_82439380(ctx, base);
		return;
	}
	// 82439378: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8243937C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439380 size=8
    let mut pc: u32 = 0x82439380;
    'dispatch: loop {
        match pc {
            0x82439380 => {
    //   block [0x82439380..0x82439388)
	// 82439380: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82439384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439388 size=164
    let mut pc: u32 = 0x82439388;
    'dispatch: loop {
        match pc {
            0x82439388 => {
    //   block [0x82439388..0x8243942C)
	// 82439388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243938C: 480FBD31  bl 0x825350bc
	ctx.lr = 0x82439390;
	sub_82535080(ctx, base);
	// 82439390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439394: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82439398: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243939C: 4800336D  bl 0x8243c708
	ctx.lr = 0x824393A0;
	sub_8243C708(ctx, base);
	// 824393A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824393A4: 409A000C  bne cr6, 0x824393b0
	if !ctx.cr[6].eq {
	pc = 0x824393B0; continue 'dispatch;
	}
	// 824393A8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 824393AC: 48000024  b 0x824393d0
	pc = 0x824393D0; continue 'dispatch;
	// 824393B0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824393B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393B8: 4800FB91  bl 0x82448f48
	ctx.lr = 0x824393BC;
	sub_82448F48(ctx, base);
	// 824393BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824393C0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824393C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393C8: 4800FBA1  bl 0x82448f68
	ctx.lr = 0x824393CC;
	sub_82448F68(ctx, base);
	// 824393CC: 7C7DF378  or r29, r3, r30
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 824393D0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824393D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393D8: 48003331  bl 0x8243c708
	ctx.lr = 0x824393DC;
	sub_8243C708(ctx, base);
	// 824393DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824393E0: 409A000C  bne cr6, 0x824393ec
	if !ctx.cr[6].eq {
	pc = 0x824393EC; continue 'dispatch;
	}
	// 824393E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824393E8: 48000024  b 0x8243940c
	pc = 0x8243940C; continue 'dispatch;
	// 824393EC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 824393F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393F4: 4800FB55  bl 0x82448f48
	ctx.lr = 0x824393F8;
	sub_82448F48(ctx, base);
	// 824393F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824393FC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82439400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439404: 4800FB65  bl 0x82448f68
	ctx.lr = 0x82439408;
	sub_82448F68(ctx, base);
	// 82439408: 7C6BF378  or r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 8243940C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82439410: 419A0010  beq cr6, 0x82439420
	if ctx.cr[6].eq {
	pc = 0x82439420; continue 'dispatch;
	}
	// 82439414: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82439418: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243941C: 409A0008  bne cr6, 0x82439424
	if !ctx.cr[6].eq {
	pc = 0x82439424; continue 'dispatch;
	}
	// 82439420: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439428: 480FBCE4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439430 size=176
    let mut pc: u32 = 0x82439430;
    'dispatch: loop {
        match pc {
            0x82439430 => {
    //   block [0x82439430..0x824394E0)
	// 82439430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243943C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439448: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243944C: 817F0A20  lwz r11, 0xa20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 82439450: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439454: 409A002C  bne cr6, 0x82439480
	if !ctx.cr[6].eq {
	pc = 0x82439480; continue 'dispatch;
	}
	// 82439458: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243945C: 4800EDED  bl 0x82448248
	ctx.lr = 0x82439460;
	sub_82448248(ctx, base);
	// 82439460: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439464: 409A001C  bne cr6, 0x82439480
	if !ctx.cr[6].eq {
	pc = 0x82439480; continue 'dispatch;
	}
	// 82439468: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243946C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439470: 4800EDC9  bl 0x82448238
	ctx.lr = 0x82439474;
	sub_82448238(ctx, base);
	// 82439474: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439478: 409A0008  bne cr6, 0x82439480
	if !ctx.cr[6].eq {
	pc = 0x82439480; continue 'dispatch;
	}
	// 8243947C: 93DF0A20  stw r30, 0xa20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2592 as u32), ctx.r[30].u32 ) };
	// 82439480: 817F0A24  lwz r11, 0xa24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82439484: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439488: 409A0030  bne cr6, 0x824394b8
	if !ctx.cr[6].eq {
	pc = 0x824394B8; continue 'dispatch;
	}
	// 8243948C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82439490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439494: 4800EDB5  bl 0x82448248
	ctx.lr = 0x82439498;
	sub_82448248(ctx, base);
	// 82439498: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243949C: 409A001C  bne cr6, 0x824394b8
	if !ctx.cr[6].eq {
	pc = 0x824394B8; continue 'dispatch;
	}
	// 824394A0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824394A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824394A8: 4800ED91  bl 0x82448238
	ctx.lr = 0x824394AC;
	sub_82448238(ctx, base);
	// 824394AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824394B0: 409A0008  bne cr6, 0x824394b8
	if !ctx.cr[6].eq {
	pc = 0x824394B8; continue 'dispatch;
	}
	// 824394B4: 93DF0A24  stw r30, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[30].u32 ) };
	// 824394B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824394BC: 80BF0A24  lwz r5, 0xa24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824394C0: 809F0A20  lwz r4, 0xa20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 824394C4: 4800FF65  bl 0x82449428
	ctx.lr = 0x824394C8;
	sub_82449428(ctx, base);
	// 824394C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824394CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824394D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824394D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824394D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824394DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824394E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824394E0 size=160
    let mut pc: u32 = 0x824394E0;
    'dispatch: loop {
        match pc {
            0x824394E0 => {
    //   block [0x824394E0..0x82439580)
	// 824394E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824394E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824394E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824394EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824394F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824394F4: 817F0A24  lwz r11, 0xa24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824394F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824394FC: 409A001C  bne cr6, 0x82439518
	if !ctx.cr[6].eq {
	pc = 0x82439518; continue 'dispatch;
	}
	// 82439500: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 82439504: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82439508: 409A0010  bne cr6, 0x82439518
	if !ctx.cr[6].eq {
	pc = 0x82439518; continue 'dispatch;
	}
	// 8243950C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82439510: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82439514: 4800323D  bl 0x8243c750
	ctx.lr = 0x82439518;
	sub_8243C750(ctx, base);
	// 82439518: 817F0A20  lwz r11, 0xa20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 8243951C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82439520: 409A0020  bne cr6, 0x82439540
	if !ctx.cr[6].eq {
	pc = 0x82439540; continue 'dispatch;
	}
	// 82439524: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 82439528: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243952C: 409A0014  bne cr6, 0x82439540
	if !ctx.cr[6].eq {
	pc = 0x82439540; continue 'dispatch;
	}
	// 82439530: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82439534: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82439538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243953C: 48003215  bl 0x8243c750
	ctx.lr = 0x82439540;
	sub_8243C750(ctx, base);
	// 82439540: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 82439544: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439548: 409A0024  bne cr6, 0x8243956c
	if !ctx.cr[6].eq {
	pc = 0x8243956C; continue 'dispatch;
	}
	// 8243954C: 817F106C  lwz r11, 0x106c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4204 as u32) ) } as u64;
	// 82439550: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82439554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243955C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82439560: 409A0008  bne cr6, 0x82439568
	if !ctx.cr[6].eq {
	pc = 0x82439568; continue 'dispatch;
	}
	// 82439564: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82439568: 480031E9  bl 0x8243c750
	ctx.lr = 0x8243956C;
	sub_8243C750(ctx, base);
	// 8243956C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439578: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243957C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439580 size=200
    let mut pc: u32 = 0x82439580;
    'dispatch: loop {
        match pc {
            0x82439580 => {
    //   block [0x82439580..0x82439648)
	// 82439580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243958C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82439598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243959C: 815E0A24  lwz r10, 0xa24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824395A0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824395A4: 409A0008  bne cr6, 0x824395ac
	if !ctx.cr[6].eq {
	pc = 0x824395AC; continue 'dispatch;
	}
	// 824395A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824395AC: 815E0A20  lwz r10, 0xa20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2592 as u32) ) } as u64;
	// 824395B0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824395B4: 409A0008  bne cr6, 0x824395bc
	if !ctx.cr[6].eq {
	pc = 0x824395BC; continue 'dispatch;
	}
	// 824395B8: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 824395BC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824395C0: 419A005C  beq cr6, 0x8243961c
	if ctx.cr[6].eq {
	pc = 0x8243961C; continue 'dispatch;
	}
	// 824395C4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824395C8: 419A004C  beq cr6, 0x82439614
	if ctx.cr[6].eq {
	pc = 0x82439614; continue 'dispatch;
	}
	// 824395CC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824395D0: 409A003C  bne cr6, 0x8243960c
	if !ctx.cr[6].eq {
	pc = 0x8243960C; continue 'dispatch;
	}
	// 824395D4: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 824395D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824395DC: 4800312D  bl 0x8243c708
	ctx.lr = 0x824395E0;
	sub_8243C708(ctx, base);
	// 824395E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824395E4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824395E8: 409A0038  bne cr6, 0x82439620
	if !ctx.cr[6].eq {
	pc = 0x82439620; continue 'dispatch;
	}
	// 824395EC: 480104CD  bl 0x82449ab8
	ctx.lr = 0x824395F0;
	sub_82449AB8(ctx, base);
	// 824395F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824395F4: 409A0018  bne cr6, 0x8243960c
	if !ctx.cr[6].eq {
	pc = 0x8243960C; continue 'dispatch;
	}
	// 824395F8: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 824395FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439600: 48003109  bl 0x8243c708
	ctx.lr = 0x82439604;
	sub_8243C708(ctx, base);
	// 82439604: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439608: 409A0018  bne cr6, 0x82439620
	if !ctx.cr[6].eq {
	pc = 0x82439620; continue 'dispatch;
	}
	// 8243960C: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 82439610: 48000010  b 0x82439620
	pc = 0x82439620; continue 'dispatch;
	// 82439614: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 82439618: 48000008  b 0x82439620
	pc = 0x82439620; continue 'dispatch;
	// 8243961C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82439620: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82439624: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 82439628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243962C: 48003125  bl 0x8243c750
	ctx.lr = 0x82439630;
	sub_8243C750(ctx, base);
	// 82439630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243963C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439648 size=8
    let mut pc: u32 = 0x82439648;
    'dispatch: loop {
        match pc {
            0x82439648 => {
    //   block [0x82439648..0x82439650)
	// 82439648: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243964C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439650 size=168
    let mut pc: u32 = 0x82439650;
    'dispatch: loop {
        match pc {
            0x82439650 => {
    //   block [0x82439650..0x824396F8)
	// 82439650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243965C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439664: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82439668: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243966C: 4800309D  bl 0x8243c708
	ctx.lr = 0x82439670;
	sub_8243C708(ctx, base);
	// 82439670: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439674: 419A0020  beq cr6, 0x82439694
	if ctx.cr[6].eq {
	pc = 0x82439694; continue 'dispatch;
	}
	// 82439678: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243967C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439680: 4800F8E9  bl 0x82448f68
	ctx.lr = 0x82439684;
	sub_82448F68(ctx, base);
	// 82439684: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439688: 419A000C  beq cr6, 0x82439694
	if ctx.cr[6].eq {
	pc = 0x82439694; continue 'dispatch;
	}
	// 8243968C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82439690: 48000050  b 0x824396e0
	pc = 0x824396E0; continue 'dispatch;
	// 82439694: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82439698: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243969C: 4800306D  bl 0x8243c708
	ctx.lr = 0x824396A0;
	sub_8243C708(ctx, base);
	// 824396A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824396A4: 419A0018  beq cr6, 0x824396bc
	if ctx.cr[6].eq {
	pc = 0x824396BC; continue 'dispatch;
	}
	// 824396A8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 824396AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824396B0: 4800F8B9  bl 0x82448f68
	ctx.lr = 0x824396B4;
	sub_82448F68(ctx, base);
	// 824396B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824396B8: 409AFFD4  bne cr6, 0x8243968c
	if !ctx.cr[6].eq {
	pc = 0x8243968C; continue 'dispatch;
	}
	// 824396BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824396C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824396C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824396C8: 4800ED39  bl 0x82448400
	ctx.lr = 0x824396CC;
	sub_82448400(ctx, base);
	// 824396CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824396D0: 409AFFBC  bne cr6, 0x8243968c
	if !ctx.cr[6].eq {
	pc = 0x8243968C; continue 'dispatch;
	}
	// 824396D4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824396D8: 2F1F0008  cmpwi cr6, r31, 8
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8, &mut ctx.xer);
	// 824396DC: 4198FFE4  blt cr6, 0x824396c0
	if ctx.cr[6].lt {
	pc = 0x824396C0; continue 'dispatch;
	}
	// 824396E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824396E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824396E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824396EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824396F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824396F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824396F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824396F8 size=164
    let mut pc: u32 = 0x824396F8;
    'dispatch: loop {
        match pc {
            0x824396F8 => {
    //   block [0x824396F8..0x8243979C)
	// 824396F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824396FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439700: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82439704: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243970C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439710: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82439714: 817F2050  lwz r11, 0x2050(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82439718: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 8243971C: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82439720: 807E13AC  lwz r3, 0x13ac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5036 as u32) ) } as u64;
	// 82439724: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82439728: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243972C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82439730: 4E800421  bctrl
	ctx.lr = 0x82439734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82439734: 3D4051EB  lis r10, 0x51eb
	ctx.r[10].s64 = 1374355456;
	// 82439738: 817E13B4  lwz r11, 0x13b4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5044 as u32) ) } as u64;
	// 8243973C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82439740: 6149851F  ori r9, r10, 0x851f
	ctx.r[9].u64 = ctx.r[10].u64 | 34079;
	// 82439744: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82439748: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243974C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82439750: 7D6B4896  mulhw r11, r11, r9
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) >> 32);
	// 82439754: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82439758: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243975C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82439760: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82439764: 4098001C  bge cr6, 0x82439780
	if !ctx.cr[6].lt {
	pc = 0x82439780; continue 'dispatch;
	}
	// 82439768: 38800046  li r4, 0x46
	ctx.r[4].s64 = 70;
	// 8243976C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439770: 48002F99  bl 0x8243c708
	ctx.lr = 0x82439774;
	sub_8243C708(ctx, base);
	// 82439774: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82439778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243977C: 41980008  blt cr6, 0x82439784
	if ctx.cr[6].lt {
	pc = 0x82439784; continue 'dispatch;
	}
	// 82439780: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82439784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243978C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824397A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824397A0 size=128
    let mut pc: u32 = 0x824397A0;
    'dispatch: loop {
        match pc {
            0x824397A0 => {
    //   block [0x824397A0..0x82439820)
	// 824397A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824397A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824397A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824397AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824397B0: 81632094  lwz r11, 0x2094(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8340 as u32) ) } as u64;
	// 824397B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824397B8: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 824397BC: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824397C0: 807F13AC  lwz r3, 0x13ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5036 as u32) ) } as u64;
	// 824397C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824397C8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824397CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824397D0: 4E800421  bctrl
	ctx.lr = 0x824397D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824397D4: 3D4051EB  lis r10, 0x51eb
	ctx.r[10].s64 = 1374355456;
	// 824397D8: 817F13B4  lwz r11, 0x13b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5044 as u32) ) } as u64;
	// 824397DC: 6149851F  ori r9, r10, 0x851f
	ctx.r[9].u64 = ctx.r[10].u64 | 34079;
	// 824397E0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824397E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824397E8: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824397EC: 7D6B4896  mulhw r11, r11, r9
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) >> 32);
	// 824397F0: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 824397F4: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824397F8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824397FC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82439800: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82439804: 40980008  bge cr6, 0x8243980c
	if !ctx.cr[6].lt {
	pc = 0x8243980C; continue 'dispatch;
	}
	// 82439808: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243980C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243981C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439820 size=116
    let mut pc: u32 = 0x82439820;
    'dispatch: loop {
        match pc {
            0x82439820 => {
    //   block [0x82439820..0x82439894)
	// 82439820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243982C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439834: 83E30A5C  lwz r31, 0xa5c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2652 as u32) ) } as u64;
	// 82439838: 83C30A60  lwz r30, 0xa60(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2656 as u32) ) } as u64;
	// 8243983C: 2F1FFFFC  cmpwi cr6, r31, -4
	ctx.cr[6].compare_i32(ctx.r[31].s32, -4, &mut ctx.xer);
	// 82439840: 409A000C  bne cr6, 0x8243984c
	if !ctx.cr[6].eq {
	pc = 0x8243984C; continue 'dispatch;
	}
	// 82439844: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439848: 48000034  b 0x8243987c
	pc = 0x8243987C; continue 'dispatch;
	// 8243984C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82439850: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82439854: 4800503D  bl 0x8243e890
	ctx.lr = 0x82439858;
	sub_8243E890(ctx, base);
	// 82439858: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243985C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439860: 4198FFE4  blt cr6, 0x82439844
	if ctx.cr[6].lt {
	pc = 0x82439844; continue 'dispatch;
	}
	// 82439864: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82439868: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243986C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82439870: 48010351  bl 0x82449bc0
	ctx.lr = 0x82439874;
	sub_82449BC0(ctx, base);
	// 82439874: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82439878: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243987C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243988C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439898 size=228
    let mut pc: u32 = 0x82439898;
    'dispatch: loop {
        match pc {
            0x82439898 => {
    //   block [0x82439898..0x8243993C)
	// 82439898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243989C: 480FB819  bl 0x825350b4
	ctx.lr = 0x824398A0;
	sub_82535080(ctx, base);
	// 824398A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824398A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824398A8: 817E0A24  lwz r11, 0xa24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824398AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824398B0: 409A001C  bne cr6, 0x824398cc
	if !ctx.cr[6].eq {
	pc = 0x824398CC; continue 'dispatch;
	}
	// 824398B4: 817E0A20  lwz r11, 0xa20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2592 as u32) ) } as u64;
	// 824398B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824398BC: 409A0010  bne cr6, 0x824398cc
	if !ctx.cr[6].eq {
	pc = 0x824398CC; continue 'dispatch;
	}
	// 824398C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824398C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824398C8: 480FB83C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 824398CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824398D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824398D4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824398D8: 4800F691  bl 0x82448f68
	ctx.lr = 0x824398DC;
	sub_82448F68(ctx, base);
	// 824398DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824398E0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 824398E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824398E8: 4800F681  bl 0x82448f68
	ctx.lr = 0x824398EC;
	sub_82448F68(ctx, base);
	// 824398EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824398F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824398F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824398F8: 4800F671  bl 0x82448f68
	ctx.lr = 0x824398FC;
	sub_82448F68(ctx, base);
	// 824398FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82439900: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 82439904: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439908: 48002E01  bl 0x8243c708
	ctx.lr = 0x8243990C;
	sub_8243C708(ctx, base);
	// 8243990C: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82439910: 41990048  bgt cr6, 0x82439958
	if ctx.cr[6].gt {
	pc = 0x82439958; continue 'dispatch;
	}
	// 82439914: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 82439918: 398C992C  addi r12, r12, -0x66d4
	ctx.r[12].s64 = ctx.r[12].s64 + -26324;
	// 8243991C: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82439920: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82439924: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82439928: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x82439954; continue 'dispatch;
		},
		1 => {
	pc = 0x8243993C; continue 'dispatch;
		},
		2 => {
	pc = 0x82439944; continue 'dispatch;
		},
		3 => {
	pc = 0x8243994C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8243992C: 82439954  lwz r18, -0x66ac(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26284 as u32) ) } as u64;
	// 82439930: 8243993C  lwz r18, -0x66c4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26308 as u32) ) } as u64;
	// 82439934: 82439944  lwz r18, -0x66bc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 82439938: 8243994C  lwz r18, -0x66b4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26292 as u32) ) } as u64;
            }
            0x8243993C => {
    //   block [0x8243993C..0x82439944)
	// 8243993C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82439940: 48000018  b 0x82439958
	pc = 0x82439958; continue 'dispatch;
            }
            0x82439944 => {
    //   block [0x82439944..0x8243994C)
	// 82439944: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82439948: 48000010  b 0x82439958
	pc = 0x82439958; continue 'dispatch;
            }
            0x8243994C => {
    //   block [0x8243994C..0x82439954)
	// 8243994C: 7F9FEB78  or r31, r28, r29
	ctx.r[31].u64 = ctx.r[28].u64 | ctx.r[29].u64;
	// 82439950: 48000008  b 0x82439958
	pc = 0x82439958; continue 'dispatch;
            }
            0x82439954 => {
    //   block [0x82439954..0x8243997C)
	// 82439954: 7F9FE838  and r31, r28, r29
	ctx.r[31].u64 = ctx.r[28].u64 & ctx.r[29].u64;
	// 82439958: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243995C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439960: 4800F619  bl 0x82448f78
	ctx.lr = 0x82439964;
	sub_82448F78(ctx, base);
	// 82439964: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439968: 419A0008  beq cr6, 0x82439970
	if ctx.cr[6].eq {
	pc = 0x82439970; continue 'dispatch;
	}
	// 8243996C: 7F7FF838  and r31, r27, r31
	ctx.r[31].u64 = ctx.r[27].u64 & ctx.r[31].u64;
	// 82439970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82439978: 480FB78C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439980 size=100
    let mut pc: u32 = 0x82439980;
    'dispatch: loop {
        match pc {
            0x82439980 => {
    //   block [0x82439980..0x824399E4)
	// 82439980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243998C: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82439990: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82439994: 409A003C  bne cr6, 0x824399d0
	if !ctx.cr[6].eq {
	pc = 0x824399D0; continue 'dispatch;
	}
	// 82439998: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243999C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824399A0: 419A0030  beq cr6, 0x824399d0
	if ctx.cr[6].eq {
	pc = 0x824399D0; continue 'dispatch;
	}
	// 824399A4: 81630970  lwz r11, 0x970(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2416 as u32) ) } as u64;
	// 824399A8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824399AC: 419A0024  beq cr6, 0x824399d0
	if ctx.cr[6].eq {
	pc = 0x824399D0; continue 'dispatch;
	}
	// 824399B0: 48005CC9  bl 0x8243f678
	ctx.lr = 0x824399B4;
	sub_8243F678(ctx, base);
	// 824399B4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824399B8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824399BC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 824399C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824399C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824399C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824399CC: 4E800020  blr
	return;
	// 824399D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824399D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824399D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824399DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824399E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824399E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824399E8 size=192
    let mut pc: u32 = 0x824399E8;
    'dispatch: loop {
        match pc {
            0x824399E8 => {
    //   block [0x824399E8..0x82439AA8)
	// 824399E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824399EC: 480FB6D1  bl 0x825350bc
	ctx.lr = 0x824399F0;
	sub_82535080(ctx, base);
	// 824399F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824399F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824399F8: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 824399FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82439A00: 48002D09  bl 0x8243c708
	ctx.lr = 0x82439A04;
	sub_8243C708(ctx, base);
	// 82439A04: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82439A08: 409A0098  bne cr6, 0x82439aa0
	if !ctx.cr[6].eq {
	pc = 0x82439AA0; continue 'dispatch;
	}
	// 82439A0C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82439A10: 419A0090  beq cr6, 0x82439aa0
	if ctx.cr[6].eq {
	pc = 0x82439AA0; continue 'dispatch;
	}
	// 82439A14: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82439A18: 617FFFFF  ori r31, r11, 0xffff
	ctx.r[31].u64 = ctx.r[11].u64 | 65535;
	// 82439A1C: 817D0DC4  lwz r11, 0xdc4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(3524 as u32) ) } as u64;
	// 82439A20: 7D5FF3D6  divw r10, r31, r30
	ctx.r[10].s32 = ctx.r[31].s32 / ctx.r[30].s32;
	// 82439A24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82439A28: 1D6A03E8  mulli r11, r10, 0x3e8
	ctx.r[11].s64 = ctx.r[10].s64 * 1000;
	// 82439A2C: 3BCBFC18  addi r30, r11, -0x3e8
	ctx.r[30].s64 = ctx.r[11].s64 + -1000;
	// 82439A30: 419A0028  beq cr6, 0x82439a58
	if ctx.cr[6].eq {
	pc = 0x82439A58; continue 'dispatch;
	}
	// 82439A34: 817D0DEC  lwz r11, 0xdec(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(3564 as u32) ) } as u64;
	// 82439A38: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439A3C: 419A001C  beq cr6, 0x82439a58
	if ctx.cr[6].eq {
	pc = 0x82439A58; continue 'dispatch;
	}
	// 82439A40: 7D7F5BD6  divw r11, r31, r11
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 82439A44: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 82439A48: 396BFC18  addi r11, r11, -0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 + -1000;
	// 82439A4C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82439A50: 40990008  ble cr6, 0x82439a58
	if !ctx.cr[6].gt {
	pc = 0x82439A58; continue 'dispatch;
	}
	// 82439A54: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82439A58: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82439A5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82439A60: 48002CA9  bl 0x8243c708
	ctx.lr = 0x82439A64;
	sub_8243C708(ctx, base);
	// 82439A64: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82439A68: 409A0028  bne cr6, 0x82439a90
	if !ctx.cr[6].eq {
	pc = 0x82439A90; continue 'dispatch;
	}
	// 82439A6C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82439A70: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 82439A74: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 82439A78: 7D7F5BD6  divw r11, r31, r11
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[11].s32;
	// 82439A7C: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 82439A80: 396BFC18  addi r11, r11, -0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 + -1000;
	// 82439A84: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82439A88: 40990008  ble cr6, 0x82439a90
	if !ctx.cr[6].gt {
	pc = 0x82439A90; continue 'dispatch;
	}
	// 82439A8C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82439A90: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82439A94: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 82439A98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82439A9C: 48002CB5  bl 0x8243c750
	ctx.lr = 0x82439AA0;
	sub_8243C750(ctx, base);
	// 82439AA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439AA4: 480FB668  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439AA8 size=28
    let mut pc: u32 = 0x82439AA8;
    'dispatch: loop {
        match pc {
            0x82439AA8 => {
    //   block [0x82439AA8..0x82439AC4)
	// 82439AA8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82439AAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82439AB0: 409A0014  bne cr6, 0x82439ac4
	if !ctx.cr[6].eq {
		sub_82439AC4(ctx, base);
		return;
	}
	// 82439AB4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439AB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439ABC: 60840204  ori r4, r4, 0x204
	ctx.r[4].u64 = ctx.r[4].u64 | 516;
	// 82439AC0: 4800DE48  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439AC4 size=16
    let mut pc: u32 = 0x82439AC4;
    'dispatch: loop {
        match pc {
            0x82439AC4 => {
    //   block [0x82439AC4..0x82439AD4)
	// 82439AC4: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82439AC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439ACC: 2B0B38A0  cmplwi cr6, r11, 0x38a0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14496 as u32, &mut ctx.xer);
	// 82439AD0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439AD4 size=12
    let mut pc: u32 = 0x82439AD4;
    'dispatch: loop {
        match pc {
            0x82439AD4 => {
    //   block [0x82439AD4..0x82439AE0)
	// 82439AD4: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439AD8: 60840205  ori r4, r4, 0x205
	ctx.r[4].u64 = ctx.r[4].u64 | 517;
	// 82439ADC: 4800DE2C  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439AE0 size=4
    let mut pc: u32 = 0x82439AE0;
    'dispatch: loop {
        match pc {
            0x82439AE0 => {
    //   block [0x82439AE0..0x82439AE4)
	// 82439AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439AE8 size=28
    let mut pc: u32 = 0x82439AE8;
    'dispatch: loop {
        match pc {
            0x82439AE8 => {
    //   block [0x82439AE8..0x82439B04)
	// 82439AE8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82439AEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439AF0: 394B04A0  addi r10, r11, 0x4a0
	ctx.r[10].s64 = ctx.r[11].s64 + 1184;
	// 82439AF4: 396A020C  addi r11, r10, 0x20c
	ctx.r[11].s64 = ctx.r[10].s64 + 524;
	// 82439AF8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82439AFC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82439B00: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439B04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439B04 size=28
    let mut pc: u32 = 0x82439B04;
    'dispatch: loop {
        match pc {
            0x82439B04 => {
    //   block [0x82439B04..0x82439B20)
	// 82439B04: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82439B08: 392A022C  addi r9, r10, 0x22c
	ctx.r[9].s64 = ctx.r[10].s64 + 556;
	// 82439B0C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82439B10: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82439B14: 4198FFE4  blt cr6, 0x82439af8
	if ctx.cr[6].lt {
		sub_82439AE8(ctx, base);
		return;
	}
	// 82439B18: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82439B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439B20 size=112
    let mut pc: u32 = 0x82439B20;
    'dispatch: loop {
        match pc {
            0x82439B20 => {
    //   block [0x82439B20..0x82439B90)
	// 82439B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439B28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439B2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439B30: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82439B34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82439B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439B3C: 480100B5  bl 0x82449bf0
	ctx.lr = 0x82439B40;
	sub_82449BF0(ctx, base);
	// 82439B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82439B44: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82439B48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82439B4C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82439B50: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82439B54: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82439B58: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82439B5C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82439B60: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82439B64: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82439B68: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82439B6C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82439B70: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82439B74: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82439B78: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82439B7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439B88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439B90 size=20
    let mut pc: u32 = 0x82439B90;
    'dispatch: loop {
        match pc {
            0x82439B90 => {
    //   block [0x82439B90..0x82439BA4)
	// 82439B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82439B94: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82439B98: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82439B9C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82439BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82439BA8 size=108
    let mut pc: u32 = 0x82439BA8;
    'dispatch: loop {
        match pc {
            0x82439BA8 => {
    //   block [0x82439BA8..0x82439C14)
	// 82439BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439BAC: 480FB511  bl 0x825350bc
	ctx.lr = 0x82439BB0;
	sub_82535080(ctx, base);
	// 82439BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439BB4: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 82439BB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82439BBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439BC0: 48010031  bl 0x82449bf0
	ctx.lr = 0x82439BC4;
	sub_82449BF0(ctx, base);
	// 82439BC4: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82439BC8: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 82439BCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82439BD0: 48010201  bl 0x82449dd0
	ctx.lr = 0x82439BD4;
	sub_82449DD0(ctx, base);
	// 82439BD4: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82439BD8: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 82439BDC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82439BE0: 409AFFEC  bne cr6, 0x82439bcc
	if !ctx.cr[6].eq {
	pc = 0x82439BCC; continue 'dispatch;
	}
	// 82439BE4: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82439BE8: 480101E9  bl 0x82449dd0
	ctx.lr = 0x82439BEC;
	sub_82449DD0(ctx, base);
	// 82439BEC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82439BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82439BF4: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82439BF8: D01F00DC  stfs f0, 0xdc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 82439BFC: F97F00C0  std r11, 0xc0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u64 ) };
	// 82439C00: F97F00C8  std r11, 0xc8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u64 ) };
	// 82439C04: F97F00D0  std r11, 0xd0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u64 ) };
	// 82439C08: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 82439C0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439C10: 480FB4FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C18 size=36
    let mut pc: u32 = 0x82439C18;
    'dispatch: loop {
        match pc {
            0x82439C18 => {
    //   block [0x82439C18..0x82439C3C)
	// 82439C18: 81230950  lwz r9, 0x950(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2384 as u32) ) } as u64;
	// 82439C1C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82439C20: 81630D5C  lwz r11, 0xd5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3420 as u32) ) } as u64;
	// 82439C24: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82439C28: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82439C2C: 38A30950  addi r5, r3, 0x950
	ctx.r[5].s64 = ctx.r[3].s64 + 2384;
	// 82439C30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82439C34: 91430950  stw r10, 0x950(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(2384 as u32), ctx.r[10].u32 ) };
	// 82439C38: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C3C size=12
    let mut pc: u32 = 0x82439C3C;
    'dispatch: loop {
        match pc {
            0x82439C3C => {
    //   block [0x82439C3C..0x82439C48)
	// 82439C3C: 80630D60  lwz r3, 0xd60(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3424 as u32) ) } as u64;
	// 82439C40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82439C44: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C48 size=4
    let mut pc: u32 = 0x82439C48;
    'dispatch: loop {
        match pc {
            0x82439C48 => {
    //   block [0x82439C48..0x82439C4C)
	// 82439C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C50 size=36
    let mut pc: u32 = 0x82439C50;
    'dispatch: loop {
        match pc {
            0x82439C50 => {
    //   block [0x82439C50..0x82439C74)
	// 82439C50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82439C54: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82439C58: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82439C5C: 810B0954  lwz r8, 0x954(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2388 as u32) ) } as u64;
	// 82439C60: 814B0D64  lwz r10, 0xd64(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3428 as u32) ) } as u64;
	// 82439C64: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82439C68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82439C6C: 912B0954  stw r9, 0x954(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2388 as u32), ctx.r[9].u32 ) };
	// 82439C70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C74 size=16
    let mut pc: u32 = 0x82439C74;
    'dispatch: loop {
        match pc {
            0x82439C74 => {
    //   block [0x82439C74..0x82439C84)
	// 82439C74: 806B0D68  lwz r3, 0xd68(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3432 as u32) ) } as u64;
	// 82439C78: 38AB0950  addi r5, r11, 0x950
	ctx.r[5].s64 = ctx.r[11].s64 + 2384;
	// 82439C7C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82439C80: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C84 size=4
    let mut pc: u32 = 0x82439C84;
    'dispatch: loop {
        match pc {
            0x82439C84 => {
    //   block [0x82439C84..0x82439C88)
	// 82439C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C88 size=8
    let mut pc: u32 = 0x82439C88;
    'dispatch: loop {
        match pc {
            0x82439C88 => {
    //   block [0x82439C88..0x82439C90)
	// 82439C88: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82439C8C: 4800F1FC  b 0x82448e88
	sub_82448E88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439C90 size=20
    let mut pc: u32 = 0x82439C90;
    'dispatch: loop {
        match pc {
            0x82439C90 => {
    //   block [0x82439C90..0x82439CA4)
	// 82439C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82439C94: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82439C98: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82439C9C: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82439CA0: 4800F1E8  b 0x82448e88
	sub_82448E88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439CA8 size=20
    let mut pc: u32 = 0x82439CA8;
    'dispatch: loop {
        match pc {
            0x82439CA8 => {
    //   block [0x82439CA8..0x82439CBC)
	// 82439CA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82439CAC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82439CB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439CB4: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82439CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439CC0 size=20
    let mut pc: u32 = 0x82439CC0;
    'dispatch: loop {
        match pc {
            0x82439CC0 => {
    //   block [0x82439CC0..0x82439CD4)
	// 82439CC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82439CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82439CC8: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82439CCC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82439CD0: 4800F228  b 0x82448ef8
	sub_82448EF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439CD8 size=16
    let mut pc: u32 = 0x82439CD8;
    'dispatch: loop {
        match pc {
            0x82439CD8 => {
    //   block [0x82439CD8..0x82439CE8)
	// 82439CD8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82439CDC: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 82439CE0: 906B0204  stw r3, 0x204(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(516 as u32), ctx.r[3].u32 ) };
	// 82439CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439CE8 size=16
    let mut pc: u32 = 0x82439CE8;
    'dispatch: loop {
        match pc {
            0x82439CE8 => {
    //   block [0x82439CE8..0x82439CF8)
	// 82439CE8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82439CEC: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 82439CF0: 806B0204  lwz r3, 0x204(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(516 as u32) ) } as u64;
	// 82439CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439CF8 size=96
    let mut pc: u32 = 0x82439CF8;
    'dispatch: loop {
        match pc {
            0x82439CF8 => {
    //   block [0x82439CF8..0x82439D58)
	// 82439CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439D00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439D04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439D08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439D0C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82439D10: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82439D14: 409A0020  bne cr6, 0x82439d34
	if !ctx.cr[6].eq {
	pc = 0x82439D34; continue 'dispatch;
	}
	// 82439D18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82439D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82439D20: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82439D24: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82439D28: 4800F1D1  bl 0x82448ef8
	ctx.lr = 0x82439D2C;
	sub_82448EF8(ctx, base);
	// 82439D2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439D30: 409A0014  bne cr6, 0x82439d44
	if !ctx.cr[6].eq {
	pc = 0x82439D44; continue 'dispatch;
	}
	// 82439D34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82439D38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439D3C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82439D40: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82439D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439D50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439D58 size=180
    let mut pc: u32 = 0x82439D58;
    'dispatch: loop {
        match pc {
            0x82439D58 => {
    //   block [0x82439D58..0x82439E0C)
	// 82439D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439D6C: 4800DAD5  bl 0x82447840
	ctx.lr = 0x82439D70;
	sub_82447840(ctx, base);
	// 82439D70: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439D74: 419A0028  beq cr6, 0x82439d9c
	if ctx.cr[6].eq {
	pc = 0x82439D9C; continue 'dispatch;
	}
	// 82439D78: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439D7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439D80: 6084013C  ori r4, r4, 0x13c
	ctx.r[4].u64 = ctx.r[4].u64 | 316;
	// 82439D84: 4800DB85  bl 0x82447908
	ctx.lr = 0x82439D88;
	sub_82447908(ctx, base);
	// 82439D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439D94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439D98: 4E800020  blr
	return;
	// 82439D9C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82439DA0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439DA4: 419A0054  beq cr6, 0x82439df8
	if ctx.cr[6].eq {
	pc = 0x82439DF8; continue 'dispatch;
	}
	// 82439DA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82439DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82439DB0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82439DB4: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82439DB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82439DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439DC0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82439DC4: 4800F135  bl 0x82448ef8
	ctx.lr = 0x82439DC8;
	sub_82448EF8(ctx, base);
	// 82439DC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82439DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82439DD0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82439DD4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82439DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439DDC: 4800F11D  bl 0x82448ef8
	ctx.lr = 0x82439DE0;
	sub_82448EF8(ctx, base);
	// 82439DE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82439DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82439DE8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82439DEC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82439DF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439DF4: 4800F105  bl 0x82448ef8
	ctx.lr = 0x82439DF8;
	sub_82448EF8(ctx, base);
	// 82439DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439E10 size=108
    let mut pc: u32 = 0x82439E10;
    'dispatch: loop {
        match pc {
            0x82439E10 => {
    //   block [0x82439E10..0x82439E7C)
	// 82439E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439E18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82439E1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439E28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82439E2C: 4800DA15  bl 0x82447840
	ctx.lr = 0x82439E30;
	sub_82447840(ctx, base);
	// 82439E30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439E34: 419A0018  beq cr6, 0x82439e4c
	if ctx.cr[6].eq {
	pc = 0x82439E4C; continue 'dispatch;
	}
	// 82439E38: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439E3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439E40: 60840134  ori r4, r4, 0x134
	ctx.r[4].u64 = ctx.r[4].u64 | 308;
	// 82439E44: 4800DAC5  bl 0x82447908
	ctx.lr = 0x82439E48;
	sub_82447908(ctx, base);
	// 82439E48: 4800001C  b 0x82439e64
	pc = 0x82439E64; continue 'dispatch;
	// 82439E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82439E50: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82439E54: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82439E58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82439E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439E60: 4800F099  bl 0x82448ef8
	ctx.lr = 0x82439E64;
	sub_82448EF8(ctx, base);
	// 82439E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439E70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439E80 size=120
    let mut pc: u32 = 0x82439E80;
    'dispatch: loop {
        match pc {
            0x82439E80 => {
    //   block [0x82439E80..0x82439EF8)
	// 82439E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439E88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439E8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439E90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439E94: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82439E98: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82439E9C: 4800D9A5  bl 0x82447840
	ctx.lr = 0x82439EA0;
	sub_82447840(ctx, base);
	// 82439EA0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439EA4: 419A0028  beq cr6, 0x82439ecc
	if ctx.cr[6].eq {
	pc = 0x82439ECC; continue 'dispatch;
	}
	// 82439EA8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439EAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439EB0: 60840135  ori r4, r4, 0x135
	ctx.r[4].u64 = ctx.r[4].u64 | 309;
	// 82439EB4: 4800DA55  bl 0x82447908
	ctx.lr = 0x82439EB8;
	sub_82447908(ctx, base);
	// 82439EB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439EC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439EC8: 4E800020  blr
	return;
	// 82439ECC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82439ED0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82439ED4: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82439ED8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82439EDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439EE0: 4800F019  bl 0x82448ef8
	ctx.lr = 0x82439EE4;
	sub_82448EF8(ctx, base);
	// 82439EE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439EF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439EF8 size=132
    let mut pc: u32 = 0x82439EF8;
    'dispatch: loop {
        match pc {
            0x82439EF8 => {
    //   block [0x82439EF8..0x82439F7C)
	// 82439EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439F00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82439F04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439F0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439F10: 4800D931  bl 0x82447840
	ctx.lr = 0x82439F14;
	sub_82447840(ctx, base);
	// 82439F14: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439F18: 419A0018  beq cr6, 0x82439f30
	if ctx.cr[6].eq {
	pc = 0x82439F30; continue 'dispatch;
	}
	// 82439F1C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439F20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439F24: 60840135  ori r4, r4, 0x135
	ctx.r[4].u64 = ctx.r[4].u64 | 309;
	// 82439F28: 4800D9E1  bl 0x82447908
	ctx.lr = 0x82439F2C;
	sub_82447908(ctx, base);
	// 82439F2C: 48000038  b 0x82439f64
	pc = 0x82439F64; continue 'dispatch;
	// 82439F30: 83DF1FCC  lwz r30, 0x1fcc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8140 as u32) ) } as u64;
	// 82439F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439F38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82439F3C: 4800E4C5  bl 0x82448400
	ctx.lr = 0x82439F40;
	sub_82448400(ctx, base);
	// 82439F40: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82439F44: 419A001C  beq cr6, 0x82439f60
	if ctx.cr[6].eq {
	pc = 0x82439F60; continue 'dispatch;
	}
	// 82439F48: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82439F4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82439F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439F54: 4800E495  bl 0x824483e8
	ctx.lr = 0x82439F58;
	sub_824483E8(ctx, base);
	// 82439F58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82439F5C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82439F60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439F64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439F70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439F74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82439F80 size=132
    let mut pc: u32 = 0x82439F80;
    'dispatch: loop {
        match pc {
            0x82439F80 => {
    //   block [0x82439F80..0x8243A004)
	// 82439F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439F88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439F8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439F94: 4800FD25  bl 0x82449cb8
	ctx.lr = 0x82439F98;
	sub_82449CB8(ctx, base);
	// 82439F98: F87F2750  std r3, 0x2750(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(10064 as u32), ctx.r[3].u64 ) };
	// 82439F9C: 4800FDF5  bl 0x82449d90
	ctx.lr = 0x82439FA0;
	sub_82449D90(ctx, base);
	// 82439FA0: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 82439FA4: E95F2750  ld r10, 0x2750(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(10064 as u32) ) };
	// 82439FA8: E93F2748  ld r9, 0x2748(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(10056 as u32) ) };
	// 82439FAC: F87F2758  std r3, 0x2758(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(10072 as u32), ctx.r[3].u64 ) };
	// 82439FB0: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82439FB4: 917F2760  stw r11, 0x2760(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10080 as u32), ctx.r[11].u32 ) };
	// 82439FB8: 2F2A0000  cmpdi cr6, r10, 0
	ctx.cr[6].compare_i64(ctx.r[10].s64, 0, &mut ctx.xer);
	// 82439FBC: 419A0034  beq cr6, 0x82439ff0
	if ctx.cr[6].eq {
	pc = 0x82439FF0; continue 'dispatch;
	}
	// 82439FC0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82439FC4: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82439FC8: 7D6B19D2  mulld r11, r11, r3
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[3].s64;
	// 82439FCC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82439FD0: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82439FD4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82439FD8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82439FDC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82439FE0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82439FE4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82439FE8: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82439FEC: D01F2764  stfs f0, 0x2764(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10084 as u32), tmp.u32 ) };
	// 82439FF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439FFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243A008 size=28
    let mut pc: u32 = 0x8243A008;
    'dispatch: loop {
        match pc {
            0x8243A008 => {
    //   block [0x8243A008..0x8243A024)
	// 8243A008: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8243A00C: 816A0058  lwz r11, 0x58(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243A010: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A014: 409A0010  bne cr6, 0x8243a024
	if !ctx.cr[6].eq {
		sub_8243A024(ctx, base);
		return;
	}
	// 8243A018: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A01C: 908A0058  stw r4, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8243A020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A024(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243A024 size=16
    let mut pc: u32 = 0x8243A024;
    'dispatch: loop {
        match pc {
            0x8243A024 => {
    //   block [0x8243A024..0x8243A034)
	// 8243A024: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8243A028: 409A000C  bne cr6, 0x8243a034
	if !ctx.cr[6].eq {
		sub_8243A034(ctx, base);
		return;
	}
	// 8243A02C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243A034 size=16
    let mut pc: u32 = 0x8243A034;
    'dispatch: loop {
        match pc {
            0x8243A034 => {
    //   block [0x8243A034..0x8243A044)
	// 8243A034: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243A038: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8243A03C: 60840207  ori r4, r4, 0x207
	ctx.r[4].u64 = ctx.r[4].u64 | 519;
	// 8243A040: 4800D8C8  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A048 size=96
    let mut pc: u32 = 0x8243A048;
    'dispatch: loop {
        match pc {
            0x8243A048 => {
    //   block [0x8243A048..0x8243A0A8)
	// 8243A048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A050: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243A054: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A05C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243A064: 4800D7DD  bl 0x82447840
	ctx.lr = 0x8243A068;
	sub_82447840(ctx, base);
	// 8243A068: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A06C: 419A0018  beq cr6, 0x8243a084
	if ctx.cr[6].eq {
	pc = 0x8243A084; continue 'dispatch;
	}
	// 8243A070: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243A074: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A078: 60840139  ori r4, r4, 0x139
	ctx.r[4].u64 = ctx.r[4].u64 | 313;
	// 8243A07C: 4800D88D  bl 0x82447908
	ctx.lr = 0x8243A080;
	sub_82447908(ctx, base);
	// 8243A080: 48000010  b 0x8243a090
	pc = 0x8243A090; continue 'dispatch;
	// 8243A084: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243A088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A08C: 4800EC5D  bl 0x82448ce8
	ctx.lr = 0x8243A090;
	sub_82448CE8(ctx, base);
	// 8243A090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A09C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243A0A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A0A8 size=44
    let mut pc: u32 = 0x8243A0A8;
    'dispatch: loop {
        match pc {
            0x8243A0A8 => {
    //   block [0x8243A0A8..0x8243A0D4)
	// 8243A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A0B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A0B4: 4BFFF24D  bl 0x82439300
	ctx.lr = 0x8243A0B8;
	sub_82439300(ctx, base);
	// 8243A0B8: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8243A0BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243A0C0: 916A070C  stw r11, 0x70c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1804 as u32), ctx.r[11].u32 ) };
	// 8243A0C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A0D8 size=52
    let mut pc: u32 = 0x8243A0D8;
    'dispatch: loop {
        match pc {
            0x8243A0D8 => {
    //   block [0x8243A0D8..0x8243A10C)
	// 8243A0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A0E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A0E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A0EC: 4BFFF26D  bl 0x82439358
	ctx.lr = 0x8243A0F0;
	sub_82439358(ctx, base);
	// 8243A0F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A0F4: 4800F8F5  bl 0x824499e8
	ctx.lr = 0x8243A0F8;
	sub_824499E8(ctx, base);
	// 8243A0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A110 size=60
    let mut pc: u32 = 0x8243A110;
    'dispatch: loop {
        match pc {
            0x8243A110 => {
    //   block [0x8243A110..0x8243A14C)
	// 8243A110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A11C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A124: 4BFFF30D  bl 0x82439430
	ctx.lr = 0x8243A128;
	sub_82439430(ctx, base);
	// 8243A128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A12C: 4BFFF3B5  bl 0x824394e0
	ctx.lr = 0x8243A130;
	sub_824394E0(ctx, base);
	// 8243A130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A134: 4BFFF44D  bl 0x82439580
	ctx.lr = 0x8243A138;
	sub_82439580(ctx, base);
	// 8243A138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A150 size=116
    let mut pc: u32 = 0x8243A150;
    'dispatch: loop {
        match pc {
            0x8243A150 => {
    //   block [0x8243A150..0x8243A1C4)
	// 8243A150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A15C: 81630A44  lwz r11, 0xa44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2628 as u32) ) } as u64;
	// 8243A160: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A164: 409A0018  bne cr6, 0x8243a17c
	if !ctx.cr[6].eq {
	pc = 0x8243A17C; continue 'dispatch;
	}
	// 8243A168: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243A16C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A178: 4E800020  blr
	return;
	// 8243A17C: 81630A20  lwz r11, 0xa20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2592 as u32) ) } as u64;
	// 8243A180: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A184: 419AFFE4  beq cr6, 0x8243a168
	if ctx.cr[6].eq {
	pc = 0x8243A168; continue 'dispatch;
	}
	// 8243A188: 81631040  lwz r11, 0x1040(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4160 as u32) ) } as u64;
	// 8243A18C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A190: 409AFFD8  bne cr6, 0x8243a168
	if !ctx.cr[6].eq {
	pc = 0x8243A168; continue 'dispatch;
	}
	// 8243A194: 8163105C  lwz r11, 0x105c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4188 as u32) ) } as u64;
	// 8243A198: 81430AC0  lwz r10, 0xac0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2752 as u32) ) } as u64;
	// 8243A19C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243A1A0: 4098FFC8  bge cr6, 0x8243a168
	if !ctx.cr[6].lt {
	pc = 0x8243A168; continue 'dispatch;
	}
	// 8243A1A4: 4BFFF6F5  bl 0x82439898
	ctx.lr = 0x8243A1A8;
	sub_82439898(ctx, base);
	// 8243A1A8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243A1AC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243A1B0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8243A1B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A1C8 size=380
    let mut pc: u32 = 0x8243A1C8;
    'dispatch: loop {
        match pc {
            0x8243A1C8 => {
    //   block [0x8243A1C8..0x8243A344)
	// 8243A1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A1D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243A1D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A1D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A1DC: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 8243A1E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A1E4: 48002525  bl 0x8243c708
	ctx.lr = 0x8243A1E8;
	sub_8243C708(ctx, base);
	// 8243A1E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A1EC: 419A013C  beq cr6, 0x8243a328
	if ctx.cr[6].eq {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A1F0: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8243A1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A1F8: 48002511  bl 0x8243c708
	ctx.lr = 0x8243A1FC;
	sub_8243C708(ctx, base);
	// 8243A1FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A200: 419A0128  beq cr6, 0x8243a328
	if ctx.cr[6].eq {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A204: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243A208: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A20C: 409A011C  bne cr6, 0x8243a328
	if !ctx.cr[6].eq {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A210: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243A214: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8243A218: 409A0110  bne cr6, 0x8243a328
	if !ctx.cr[6].eq {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A21C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A220: 4BFFF431  bl 0x82439650
	ctx.lr = 0x8243A224;
	sub_82439650(ctx, base);
	// 8243A224: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A228: 409A0100  bne cr6, 0x8243a328
	if !ctx.cr[6].eq {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A22C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8243A230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A234: 480024D5  bl 0x8243c708
	ctx.lr = 0x8243A238;
	sub_8243C708(ctx, base);
	// 8243A238: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243A23C: 409A0010  bne cr6, 0x8243a24c
	if !ctx.cr[6].eq {
	pc = 0x8243A24C; continue 'dispatch;
	}
	// 8243A240: 817F0978  lwz r11, 0x978(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2424 as u32) ) } as u64;
	// 8243A244: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A248: 419A00E0  beq cr6, 0x8243a328
	if ctx.cr[6].eq {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A24C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243A250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A254: 480024B5  bl 0x8243c708
	ctx.lr = 0x8243A258;
	sub_8243C708(ctx, base);
	// 8243A258: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243A25C: 409A0018  bne cr6, 0x8243a274
	if !ctx.cr[6].eq {
	pc = 0x8243A274; continue 'dispatch;
	}
	// 8243A260: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8243A264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A268: 4800E939  bl 0x82448ba0
	ctx.lr = 0x8243A26C;
	sub_82448BA0(ctx, base);
	// 8243A26C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A270: 419900B8  bgt cr6, 0x8243a328
	if ctx.cr[6].gt {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A274: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243A278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A27C: 4800ECFD  bl 0x82448f78
	ctx.lr = 0x8243A280;
	sub_82448F78(ctx, base);
	// 8243A280: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A284: 419A0018  beq cr6, 0x8243a29c
	if ctx.cr[6].eq {
	pc = 0x8243A29C; continue 'dispatch;
	}
	// 8243A288: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243A28C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A290: 4800E911  bl 0x82448ba0
	ctx.lr = 0x8243A294;
	sub_82448BA0(ctx, base);
	// 8243A294: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A298: 41990090  bgt cr6, 0x8243a328
	if ctx.cr[6].gt {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A29C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8243A2A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A2A4: 48002465  bl 0x8243c708
	ctx.lr = 0x8243A2A8;
	sub_8243C708(ctx, base);
	// 8243A2A8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243A2AC: 409A0014  bne cr6, 0x8243a2c0
	if !ctx.cr[6].eq {
	pc = 0x8243A2C0; continue 'dispatch;
	}
	// 8243A2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A2B4: 4BFFF445  bl 0x824396f8
	ctx.lr = 0x8243A2B8;
	sub_824396F8(ctx, base);
	// 8243A2B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A2BC: 409A006C  bne cr6, 0x8243a328
	if !ctx.cr[6].eq {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A2C0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243A2C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243A2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A2CC: 480045C5  bl 0x8243e890
	ctx.lr = 0x8243A2D0;
	sub_8243E890(ctx, base);
	// 8243A2D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A2D4: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 8243A2D8: 83DF1010  lwz r30, 0x1010(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4112 as u32) ) } as u64;
	// 8243A2DC: 83FF1014  lwz r31, 0x1014(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4116 as u32) ) } as u64;
	// 8243A2E0: 48002429  bl 0x8243c708
	ctx.lr = 0x8243A2E4;
	sub_8243C708(ctx, base);
	// 8243A2E4: 3CA0000F  lis r5, 0xf
	ctx.r[5].s64 = 983040;
	// 8243A2E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243A2EC: 60A54240  ori r5, r5, 0x4240
	ctx.r[5].u64 = ctx.r[5].u64 | 16960;
	// 8243A2F0: 4800A289  bl 0x82444578
	ctx.lr = 0x8243A2F4;
	sub_82444578(ctx, base);
	// 8243A2F4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243A2F8: 7CA3F050  subf r5, r3, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 8243A2FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A300: 40990028  ble cr6, 0x8243a328
	if !ctx.cr[6].gt {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A304: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8243A308: 40990020  ble cr6, 0x8243a328
	if !ctx.cr[6].gt {
	pc = 0x8243A328; continue 'dispatch;
	}
	// 8243A30C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8243A310: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243A314: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8243A318: 4800F8A9  bl 0x82449bc0
	ctx.lr = 0x8243A31C;
	sub_82449BC0(ctx, base);
	// 8243A31C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243A320: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243A324: 48000008  b 0x8243a32c
	pc = 0x8243A32C; continue 'dispatch;
	// 8243A328: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A32C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A338: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243A33C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A348 size=224
    let mut pc: u32 = 0x8243A348;
    'dispatch: loop {
        match pc {
            0x8243A348 => {
    //   block [0x8243A348..0x8243A428)
	// 8243A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243A354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A360: 4BFFF2F1  bl 0x82439650
	ctx.lr = 0x8243A364;
	sub_82439650(ctx, base);
	// 8243A364: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A368: 419A000C  beq cr6, 0x8243a374
	if ctx.cr[6].eq {
	pc = 0x8243A374; continue 'dispatch;
	}
	// 8243A36C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243A370: 480000A0  b 0x8243a410
	pc = 0x8243A410; continue 'dispatch;
	// 8243A374: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8243A378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A37C: 4800238D  bl 0x8243c708
	ctx.lr = 0x8243A380;
	sub_8243C708(ctx, base);
	// 8243A380: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243A384: 409A0014  bne cr6, 0x8243a398
	if !ctx.cr[6].eq {
	pc = 0x8243A398; continue 'dispatch;
	}
	// 8243A388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A38C: 4BFFF36D  bl 0x824396f8
	ctx.lr = 0x8243A390;
	sub_824396F8(ctx, base);
	// 8243A390: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A394: 409AFFD8  bne cr6, 0x8243a36c
	if !ctx.cr[6].eq {
	pc = 0x8243A36C; continue 'dispatch;
	}
	// 8243A398: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243A39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A3A0: 48002369  bl 0x8243c708
	ctx.lr = 0x8243A3A4;
	sub_8243C708(ctx, base);
	// 8243A3A4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243A3A8: 409A0014  bne cr6, 0x8243a3bc
	if !ctx.cr[6].eq {
	pc = 0x8243A3BC; continue 'dispatch;
	}
	// 8243A3AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A3B0: 4BFFF3F1  bl 0x824397a0
	ctx.lr = 0x8243A3B4;
	sub_824397A0(ctx, base);
	// 8243A3B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A3B8: 409AFFB4  bne cr6, 0x8243a36c
	if !ctx.cr[6].eq {
	pc = 0x8243A36C; continue 'dispatch;
	}
	// 8243A3BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243A3C0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8243A3C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A3C8: 480044C9  bl 0x8243e890
	ctx.lr = 0x8243A3CC;
	sub_8243E890(ctx, base);
	// 8243A3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A3D0: 38800045  li r4, 0x45
	ctx.r[4].s64 = 69;
	// 8243A3D4: 83DF1010  lwz r30, 0x1010(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4112 as u32) ) } as u64;
	// 8243A3D8: 83FF1014  lwz r31, 0x1014(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4116 as u32) ) } as u64;
	// 8243A3DC: 4800232D  bl 0x8243c708
	ctx.lr = 0x8243A3E0;
	sub_8243C708(ctx, base);
	// 8243A3E0: 3CA0000F  lis r5, 0xf
	ctx.r[5].s64 = 983040;
	// 8243A3E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243A3E8: 60A54240  ori r5, r5, 0x4240
	ctx.r[5].u64 = ctx.r[5].u64 | 16960;
	// 8243A3EC: 4800A18D  bl 0x82444578
	ctx.lr = 0x8243A3F0;
	sub_82444578(ctx, base);
	// 8243A3F0: 7CA3F050  subf r5, r3, r30
	ctx.r[5].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 8243A3F4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8243A3F8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243A3FC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243A400: 4800F7C1  bl 0x82449bc0
	ctx.lr = 0x8243A404;
	sub_82449BC0(ctx, base);
	// 8243A404: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243A408: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243A40C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8243A410: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A41C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243A420: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A428 size=196
    let mut pc: u32 = 0x8243A428;
    'dispatch: loop {
        match pc {
            0x8243A428 => {
    //   block [0x8243A428..0x8243A4EC)
	// 8243A428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A430: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A434: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A43C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243A440: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8243A444: 409A0090  bne cr6, 0x8243a4d4
	if !ctx.cr[6].eq {
	pc = 0x8243A4D4; continue 'dispatch;
	}
	// 8243A448: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243A44C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243A450: 419A0084  beq cr6, 0x8243a4d4
	if ctx.cr[6].eq {
	pc = 0x8243A4D4; continue 'dispatch;
	}
	// 8243A454: 817F0970  lwz r11, 0x970(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2416 as u32) ) } as u64;
	// 8243A458: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243A45C: 419A0078  beq cr6, 0x8243a4d4
	if ctx.cr[6].eq {
	pc = 0x8243A4D4; continue 'dispatch;
	}
	// 8243A460: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243A464: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8243A468: 48005791  bl 0x8243fbf8
	ctx.lr = 0x8243A46C;
	sub_8243FBF8(ctx, base);
	// 8243A46C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A470: 409A0064  bne cr6, 0x8243a4d4
	if !ctx.cr[6].eq {
	pc = 0x8243A4D4; continue 'dispatch;
	}
	// 8243A474: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243A478: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A47C: 41980058  blt cr6, 0x8243a4d4
	if ctx.cr[6].lt {
	pc = 0x8243A4D4; continue 'dispatch;
	}
	// 8243A480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A484: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243A488: 4BFFF561  bl 0x824399e8
	ctx.lr = 0x8243A48C;
	sub_824399E8(ctx, base);
	// 8243A48C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8243A490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A494: 480050C5  bl 0x8243f558
	ctx.lr = 0x8243A498;
	sub_8243F558(ctx, base);
	// 8243A498: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243A49C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8243A4A0: 419A0034  beq cr6, 0x8243a4d4
	if ctx.cr[6].eq {
	pc = 0x8243A4D4; continue 'dispatch;
	}
	// 8243A4A4: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 8243A4A8: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243A4AC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243A4B0: 4800F711  bl 0x82449bc0
	ctx.lr = 0x8243A4B4;
	sub_82449BC0(ctx, base);
	// 8243A4B4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243A4B8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243A4BC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8243A4C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A4CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A4D0: 4E800020  blr
	return;
	// 8243A4D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A4D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A4E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A4F0 size=72
    let mut pc: u32 = 0x8243A4F0;
    'dispatch: loop {
        match pc {
            0x8243A4F0 => {
    //   block [0x8243A4F0..0x8243A538)
	// 8243A4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A4F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A4FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A504: 4BFFF7F5  bl 0x82439cf8
	ctx.lr = 0x8243A508;
	sub_82439CF8(ctx, base);
	// 8243A508: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A50C: 409A0018  bne cr6, 0x8243a524
	if !ctx.cr[6].eq {
	pc = 0x8243A524; continue 'dispatch;
	}
	// 8243A510: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8243A514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A518: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8243A51C: 4BFFFA65  bl 0x82439f80
	ctx.lr = 0x8243A520;
	sub_82439F80(ctx, base);
	// 8243A520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A538 size=136
    let mut pc: u32 = 0x8243A538;
    'dispatch: loop {
        match pc {
            0x8243A538 => {
    //   block [0x8243A538..0x8243A5C0)
	// 8243A538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A548: 38A0002A  li r5, 0x2a
	ctx.r[5].s64 = 42;
	// 8243A54C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243A550: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A554: 4800F69D  bl 0x82449bf0
	ctx.lr = 0x8243A558;
	sub_82449BF0(ctx, base);
	// 8243A558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243A55C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8243A560: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243A564: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243A568: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8243A56C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8243A570: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8243A574: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8243A578: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8243A57C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8243A580: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8243A584: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8243A588: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8243A58C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8243A590: 4BFFF601  bl 0x82439b90
	ctx.lr = 0x8243A594;
	sub_82439B90(ctx, base);
	// 8243A594: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8243A598: 4BFFF5F9  bl 0x82439b90
	ctx.lr = 0x8243A59C;
	sub_82439B90(ctx, base);
	// 8243A59C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8243A5A0: 4BFFF5F1  bl 0x82439b90
	ctx.lr = 0x8243A5A4;
	sub_82439B90(ctx, base);
	// 8243A5A4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8243A5A8: 4BFFF5E9  bl 0x82439b90
	ctx.lr = 0x8243A5AC;
	sub_82439B90(ctx, base);
	// 8243A5AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A5B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A5C0 size=132
    let mut pc: u32 = 0x8243A5C0;
    'dispatch: loop {
        match pc {
            0x8243A5C0 => {
    //   block [0x8243A5C0..0x8243A644)
	// 8243A5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A5C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A5CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A5D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A5D4: 4800D26D  bl 0x82447840
	ctx.lr = 0x8243A5D8;
	sub_82447840(ctx, base);
	// 8243A5D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A5DC: 419A0028  beq cr6, 0x8243a604
	if ctx.cr[6].eq {
	pc = 0x8243A604; continue 'dispatch;
	}
	// 8243A5E0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243A5E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A5E8: 60840132  ori r4, r4, 0x132
	ctx.r[4].u64 = ctx.r[4].u64 | 306;
	// 8243A5EC: 4800D31D  bl 0x82447908
	ctx.lr = 0x8243A5F0;
	sub_82447908(ctx, base);
	// 8243A5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A5FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A600: 4E800020  blr
	return;
	// 8243A604: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 8243A608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A60C: 480020FD  bl 0x8243c708
	ctx.lr = 0x8243A610;
	sub_8243C708(ctx, base);
	// 8243A610: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243A614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A618: 409A000C  bne cr6, 0x8243a624
	if !ctx.cr[6].eq {
	pc = 0x8243A624; continue 'dispatch;
	}
	// 8243A61C: 4BFFEA85  bl 0x824390a0
	ctx.lr = 0x8243A620;
	sub_824390A0(ctx, base);
	// 8243A620: 48000008  b 0x8243a628
	pc = 0x8243A628; continue 'dispatch;
	// 8243A624: 4BFFF685  bl 0x82439ca8
	ctx.lr = 0x8243A628;
	sub_82439CA8(ctx, base);
	// 8243A628: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243A62C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8243A630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A63C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A648 size=216
    let mut pc: u32 = 0x8243A648;
    'dispatch: loop {
        match pc {
            0x8243A648 => {
    //   block [0x8243A648..0x8243A720)
	// 8243A648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A64C: 480FAA71  bl 0x825350bc
	ctx.lr = 0x8243A650;
	sub_82535080(ctx, base);
	// 8243A650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A654: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243A658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243A65C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A660: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243A664: 4800D1DD  bl 0x82447840
	ctx.lr = 0x8243A668;
	sub_82447840(ctx, base);
	// 8243A668: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A66C: 419A001C  beq cr6, 0x8243a688
	if ctx.cr[6].eq {
	pc = 0x8243A688; continue 'dispatch;
	}
	// 8243A670: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243A674: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A678: 60840136  ori r4, r4, 0x136
	ctx.r[4].u64 = ctx.r[4].u64 | 310;
	// 8243A67C: 4800D28D  bl 0x82447908
	ctx.lr = 0x8243A680;
	sub_82447908(ctx, base);
	// 8243A680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A684: 480FAA88  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243A688: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243A68C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A690: 4BFFF979  bl 0x8243a008
	ctx.lr = 0x8243A694;
	sub_8243A008(ctx, base);
	// 8243A694: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A698: 409A0080  bne cr6, 0x8243a718
	if !ctx.cr[6].eq {
	pc = 0x8243A718; continue 'dispatch;
	}
	// 8243A69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8243A6A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243A6A4: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8243A6A8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243A6AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A6B0: 4800E849  bl 0x82448ef8
	ctx.lr = 0x8243A6B4;
	sub_82448EF8(ctx, base);
	// 8243A6B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243A6B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243A6BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243A6C0: 419A0030  beq cr6, 0x8243a6f0
	if ctx.cr[6].eq {
	pc = 0x8243A6F0; continue 'dispatch;
	}
	// 8243A6C4: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 8243A6C8: 815F096C  lwz r10, 0x96c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2412 as u32) ) } as u64;
	// 8243A6CC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243A6D0: 409A0020  bne cr6, 0x8243a6f0
	if !ctx.cr[6].eq {
	pc = 0x8243A6F0; continue 'dispatch;
	}
	// 8243A6D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A6D8: 409A000C  bne cr6, 0x8243a6e4
	if !ctx.cr[6].eq {
	pc = 0x8243A6E4; continue 'dispatch;
	}
	// 8243A6DC: 4800F5DD  bl 0x82449cb8
	ctx.lr = 0x8243A6E0;
	sub_82449CB8(ctx, base);
	// 8243A6E0: F87F2748  std r3, 0x2748(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(10056 as u32), ctx.r[3].u64 ) };
	// 8243A6E4: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 8243A6E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243A6EC: 917F0968  stw r11, 0x968(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2408 as u32), ctx.r[11].u32 ) };
	// 8243A6F0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243A6F4: 814B070C  lwz r10, 0x70c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1804 as u32) ) } as u64;
	// 8243A6F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243A6FC: 419A0018  beq cr6, 0x8243a714
	if ctx.cr[6].eq {
	pc = 0x8243A714; continue 'dispatch;
	}
	// 8243A700: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243A704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A708: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243A70C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243A710: 4E800421  bctrl
	ctx.lr = 0x8243A714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243A714: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243A718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A71C: 480FA9F0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A720 size=152
    let mut pc: u32 = 0x8243A720;
    'dispatch: loop {
        match pc {
            0x8243A720 => {
    //   block [0x8243A720..0x8243A7B8)
	// 8243A720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243A72C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A734: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A738: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243A73C: 4800D105  bl 0x82447840
	ctx.lr = 0x8243A740;
	sub_82447840(ctx, base);
	// 8243A740: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A744: 419A0018  beq cr6, 0x8243a75c
	if ctx.cr[6].eq {
	pc = 0x8243A75C; continue 'dispatch;
	}
	// 8243A748: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243A74C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243A750: 60840137  ori r4, r4, 0x137
	ctx.r[4].u64 = ctx.r[4].u64 | 311;
	// 8243A754: 4800D1B5  bl 0x82447908
	ctx.lr = 0x8243A758;
	sub_82447908(ctx, base);
	// 8243A758: 48000048  b 0x8243a7a0
	pc = 0x8243A7A0; continue 'dispatch;
	// 8243A75C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243A760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A764: 4BFFF8A5  bl 0x8243a008
	ctx.lr = 0x8243A768;
	sub_8243A008(ctx, base);
	// 8243A768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A76C: 409A0034  bne cr6, 0x8243a7a0
	if !ctx.cr[6].eq {
	pc = 0x8243A7A0; continue 'dispatch;
	}
	// 8243A770: 817F096C  lwz r11, 0x96c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2412 as u32) ) } as u64;
	// 8243A774: 815F0968  lwz r10, 0x968(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 8243A778: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243A77C: 4098000C  bge cr6, 0x8243a788
	if !ctx.cr[6].lt {
	pc = 0x8243A788; continue 'dispatch;
	}
	// 8243A780: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243A784: 917F096C  stw r11, 0x96c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2412 as u32), ctx.r[11].u32 ) };
	// 8243A788: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8243A78C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243A790: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8243A794: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243A798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A79C: 4800E75D  bl 0x82448ef8
	ctx.lr = 0x8243A7A0;
	sub_82448EF8(ctx, base);
	// 8243A7A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A7AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243A7B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A7B8 size=172
    let mut pc: u32 = 0x8243A7B8;
    'dispatch: loop {
        match pc {
            0x8243A7B8 => {
    //   block [0x8243A7B8..0x8243A81C)
	// 8243A7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A7BC: 480FA901  bl 0x825350bc
	ctx.lr = 0x8243A7C0;
	sub_82535080(ctx, base);
	// 8243A7C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A7C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A7C8: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243A7CC: 83BF004C  lwz r29, 0x4c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243A7D0: 4BFFEBB9  bl 0x82439388
	ctx.lr = 0x8243A7D4;
	sub_82439388(ctx, base);
	// 8243A7D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A7D8: 419A0080  beq cr6, 0x8243a858
	if ctx.cr[6].eq {
	pc = 0x8243A858; continue 'dispatch;
	}
	// 8243A7DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A7E0: 4BFFF931  bl 0x8243a110
	ctx.lr = 0x8243A7E4;
	sub_8243A110(ctx, base);
	// 8243A7E4: 397DFFFE  addi r11, r29, -2
	ctx.r[11].s64 = ctx.r[29].s64 + -2;
	// 8243A7E8: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8243A7EC: 4199006C  bgt cr6, 0x8243a858
	if ctx.cr[6].gt {
	pc = 0x8243A858; continue 'dispatch;
	}
	// 8243A7F0: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 8243A7F4: 398CA808  addi r12, r12, -0x57f8
	ctx.r[12].s64 = ctx.r[12].s64 + -22520;
	// 8243A7F8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8243A7FC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8243A800: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8243A804: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8243A81C; continue 'dispatch;
		},
		1 => {
	pc = 0x8243A854; continue 'dispatch;
		},
		2 => {
	pc = 0x8243A82C; continue 'dispatch;
		},
		3 => {
	pc = 0x8243A858; continue 'dispatch;
		},
		4 => {
	pc = 0x8243A82C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8243A808: 8243A81C  lwz r18, -0x57e4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22500 as u32) ) } as u64;
	// 8243A80C: 8243A854  lwz r18, -0x57ac(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22444 as u32) ) } as u64;
	// 8243A810: 8243A82C  lwz r18, -0x57d4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22484 as u32) ) } as u64;
	// 8243A814: 8243A858  lwz r18, -0x57a8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22440 as u32) ) } as u64;
	// 8243A818: 8243A82C  lwz r18, -0x57d4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22484 as u32) ) } as u64;
            }
            0x8243A81C => {
    //   block [0x8243A81C..0x8243A82C)
	// 8243A81C: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8243A820: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243A824: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A828: 480FA8E4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8243A82C => {
    //   block [0x8243A82C..0x8243A854)
	// 8243A82C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A830: 4BFFF921  bl 0x8243a150
	ctx.lr = 0x8243A834;
	sub_8243A150(ctx, base);
	// 8243A834: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A838: 419A001C  beq cr6, 0x8243a854
	if ctx.cr[6].eq {
	pc = 0x8243A854; continue 'dispatch;
	}
	// 8243A83C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A840: 4BFFF481  bl 0x82439cc0
	ctx.lr = 0x8243A844;
	sub_82439CC0(ctx, base);
	// 8243A844: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8243A848: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243A84C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A850: 480FA8BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8243A854 => {
    //   block [0x8243A854..0x8243A858)
	// 8243A854: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	pc = 0x8243A858; continue 'dispatch;
            }
            0x8243A858 => {
    //   block [0x8243A858..0x8243A864)
	// 8243A858: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243A85C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A860: 480FA8AC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A868 size=160
    let mut pc: u32 = 0x8243A868;
    'dispatch: loop {
        match pc {
            0x8243A868 => {
    //   block [0x8243A868..0x8243A8C0)
	// 8243A868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243A874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A87C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A880: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243A884: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243A888: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 8243A88C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8243A890: 4199005C  bgt cr6, 0x8243a8ec
	if ctx.cr[6].gt {
	pc = 0x8243A8EC; continue 'dispatch;
	}
	// 8243A894: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 8243A898: 398CA8AC  addi r12, r12, -0x5754
	ctx.r[12].s64 = ctx.r[12].s64 + -22356;
	// 8243A89C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8243A8A0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8243A8A4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8243A8A8: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8243A8C0; continue 'dispatch;
		},
		1 => {
	pc = 0x8243A8C8; continue 'dispatch;
		},
		2 => {
	pc = 0x8243A8D0; continue 'dispatch;
		},
		3 => {
	pc = 0x8243A8EC; continue 'dispatch;
		},
		4 => {
	pc = 0x8243A8D0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8243A8AC: 8243A8C0  lwz r18, -0x5740(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22336 as u32) ) } as u64;
	// 8243A8B0: 8243A8C8  lwz r18, -0x5738(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22328 as u32) ) } as u64;
	// 8243A8B4: 8243A8D0  lwz r18, -0x5730(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22320 as u32) ) } as u64;
	// 8243A8B8: 8243A8EC  lwz r18, -0x5714(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22292 as u32) ) } as u64;
	// 8243A8BC: 8243A8D0  lwz r18, -0x5730(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-22320 as u32) ) } as u64;
            }
            0x8243A8C0 => {
    //   block [0x8243A8C0..0x8243A8C8)
	// 8243A8C0: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8243A8C4: 48000028  b 0x8243a8ec
	pc = 0x8243A8EC; continue 'dispatch;
            }
            0x8243A8C8 => {
    //   block [0x8243A8C8..0x8243A8D0)
	// 8243A8C8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 8243A8CC: 48000020  b 0x8243a8ec
	pc = 0x8243A8EC; continue 'dispatch;
            }
            0x8243A8D0 => {
    //   block [0x8243A8D0..0x8243A8EC)
	// 8243A8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A8D4: 4BFFF87D  bl 0x8243a150
	ctx.lr = 0x8243A8D8;
	sub_8243A150(ctx, base);
	// 8243A8D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A8DC: 419A0010  beq cr6, 0x8243a8ec
	if ctx.cr[6].eq {
	pc = 0x8243A8EC; continue 'dispatch;
	}
	// 8243A8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A8E4: 4BFFF3DD  bl 0x82439cc0
	ctx.lr = 0x8243A8E8;
	sub_82439CC0(ctx, base);
	// 8243A8E8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	pc = 0x8243A8EC; continue 'dispatch;
            }
            0x8243A8EC => {
    //   block [0x8243A8EC..0x8243A908)
	// 8243A8EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243A8F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243A8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243A8FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243A900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243A904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A908 size=140
    let mut pc: u32 = 0x8243A908;
    'dispatch: loop {
        match pc {
            0x8243A908 => {
    //   block [0x8243A908..0x8243A994)
	// 8243A908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A90C: 480FA7B1  bl 0x825350bc
	ctx.lr = 0x8243A910;
	sub_82535080(ctx, base);
	// 8243A910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A914: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243A918: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243A91C: 4800CFC5  bl 0x824478e0
	ctx.lr = 0x8243A920;
	sub_824478E0(ctx, base);
	// 8243A920: 3BFE0950  addi r31, r30, 0x950
	ctx.r[31].s64 = ctx.r[30].s64 + 2384;
	// 8243A924: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8243A928: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243A92C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243A930: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A934: 409A002C  bne cr6, 0x8243a960
	if !ctx.cr[6].eq {
	pc = 0x8243A960; continue 'dispatch;
	}
	// 8243A938: 4BFFF891  bl 0x8243a1c8
	ctx.lr = 0x8243A93C;
	sub_8243A1C8(ctx, base);
	// 8243A93C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A940: 419A0040  beq cr6, 0x8243a980
	if ctx.cr[6].eq {
	pc = 0x8243A980; continue 'dispatch;
	}
	// 8243A944: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243A948: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243A94C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243A950: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243A954: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8243A958: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8243A95C: 48000018  b 0x8243a974
	pc = 0x8243A974; continue 'dispatch;
	// 8243A960: 4BFFF9E9  bl 0x8243a348
	ctx.lr = 0x8243A964;
	sub_8243A348(ctx, base);
	// 8243A964: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A968: 419A0018  beq cr6, 0x8243a980
	if ctx.cr[6].eq {
	pc = 0x8243A980; continue 'dispatch;
	}
	// 8243A96C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243A970: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 8243A974: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243A978: 4BFFE871  bl 0x824391e8
	ctx.lr = 0x8243A97C;
	sub_824391E8(ctx, base);
	// 8243A97C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243A980: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243A984: 4800CF6D  bl 0x824478f0
	ctx.lr = 0x8243A988;
	sub_824478F0(ctx, base);
	// 8243A988: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243A98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243A990: 480FA77C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243A998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243A998 size=140
    let mut pc: u32 = 0x8243A998;
    'dispatch: loop {
        match pc {
            0x8243A998 => {
    //   block [0x8243A998..0x8243AA24)
	// 8243A998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243A99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243A9A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243A9A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243A9A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243A9AC: 4BFFEE75  bl 0x82439820
	ctx.lr = 0x8243A9B0;
	sub_82439820(ctx, base);
	// 8243A9B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A9B4: 409A0054  bne cr6, 0x8243aa08
	if !ctx.cr[6].eq {
	pc = 0x8243AA08; continue 'dispatch;
	}
	// 8243A9B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A9BC: 4BFFEEDD  bl 0x82439898
	ctx.lr = 0x8243A9C0;
	sub_82439898(ctx, base);
	// 8243A9C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A9C4: 409A0044  bne cr6, 0x8243aa08
	if !ctx.cr[6].eq {
	pc = 0x8243AA08; continue 'dispatch;
	}
	// 8243A9C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A9CC: 4BFFEFB5  bl 0x82439980
	ctx.lr = 0x8243A9D0;
	sub_82439980(ctx, base);
	// 8243A9D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A9D4: 409A0034  bne cr6, 0x8243aa08
	if !ctx.cr[6].eq {
	pc = 0x8243AA08; continue 'dispatch;
	}
	// 8243A9D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243A9DC: 4BFFFA4D  bl 0x8243a428
	ctx.lr = 0x8243A9E0;
	sub_8243A428(ctx, base);
	// 8243A9E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243A9E4: 409A0024  bne cr6, 0x8243aa08
	if !ctx.cr[6].eq {
	pc = 0x8243AA08; continue 'dispatch;
	}
	// 8243A9E8: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8243A9EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243A9F0: 409A0018  bne cr6, 0x8243aa08
	if !ctx.cr[6].eq {
	pc = 0x8243AA08; continue 'dispatch;
	}
	// 8243A9F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243A9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243A9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243AA00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243AA04: 4E800020  blr
	return;
	// 8243AA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AA0C: 4BFFFAE5  bl 0x8243a4f0
	ctx.lr = 0x8243AA10;
	sub_8243A4F0(ctx, base);
	// 8243AA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243AA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243AA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243AA1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243AA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243AA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243AA28 size=404
    let mut pc: u32 = 0x8243AA28;
    'dispatch: loop {
        match pc {
            0x8243AA28 => {
    //   block [0x8243AA28..0x8243ABBC)
	// 8243AA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243AA2C: 480FA689  bl 0x825350b4
	ctx.lr = 0x8243AA30;
	sub_82535080(ctx, base);
	// 8243AA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243AA34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243AA38: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8243AA3C: 83FD003C  lwz r31, 0x3c(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 8243AA40: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 8243AA44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243AA48: 5565F0BE  srwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8243AA4C: 419A0164  beq cr6, 0x8243abb0
	if ctx.cr[6].eq {
	pc = 0x8243ABB0; continue 'dispatch;
	}
	// 8243AA50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243AA54: 4099015C  ble cr6, 0x8243abb0
	if !ctx.cr[6].gt {
	pc = 0x8243ABB0; continue 'dispatch;
	}
	// 8243AA58: 2B0B7140  cmplwi cr6, r11, 0x7140
	ctx.cr[6].compare_u32(ctx.r[11].u32, 28992 as u32, &mut ctx.xer);
	// 8243AA5C: 41990154  bgt cr6, 0x8243abb0
	if ctx.cr[6].gt {
	pc = 0x8243ABB0; continue 'dispatch;
	}
	// 8243AA60: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8243AA64: 812A0708  lwz r9, 0x708(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1800 as u32) ) } as u64;
	// 8243AA68: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8243AA6C: 419A000C  beq cr6, 0x8243aa78
	if ctx.cr[6].eq {
	pc = 0x8243AA78; continue 'dispatch;
	}
	// 8243AA70: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243AA74: 409A013C  bne cr6, 0x8243abb0
	if !ctx.cr[6].eq {
	pc = 0x8243ABB0; continue 'dispatch;
	}
	// 8243AA78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243AA7C: 916A0708  stw r11, 0x708(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1800 as u32), ctx.r[11].u32 ) };
	// 8243AA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AA84: 4800F16D  bl 0x82449bf0
	ctx.lr = 0x8243AA88;
	sub_82449BF0(ctx, base);
	// 8243AA88: 397F001F  addi r11, r31, 0x1f
	ctx.r[11].s64 = ctx.r[31].s64 + 31;
	// 8243AA8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243AA90: 557F0034  rlwinm r31, r11, 0, 0, 0x1a
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8243AA94: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 8243AA98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243AA9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AAA0: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 8243AAA4: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 8243AAA8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243AAAC: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 8243AAB0: 556B0030  rlwinm r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8243AAB4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243AAB8: 480FA099  bl 0x82534b50
	ctx.lr = 0x8243AABC;
	sub_82534B50(ctx, base);
	// 8243AABC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8243AAC0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8243AAC4: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8243AAC8: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8243AACC: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8243AAD0: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8243AAD4: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8243AAD8: 939F0044  stw r28, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[28].u32 ) };
	// 8243AADC: 4800F375  bl 0x82449e50
	ctx.lr = 0x8243AAE0;
	sub_82449E50(ctx, base);
	// 8243AAE0: 387F090C  addi r3, r31, 0x90c
	ctx.r[3].s64 = ctx.r[31].s64 + 2316;
	// 8243AAE4: 4BFFF03D  bl 0x82439b20
	ctx.lr = 0x8243AAE8;
	sub_82439B20(ctx, base);
	// 8243AAE8: 387F0950  addi r3, r31, 0x950
	ctx.r[3].s64 = ctx.r[31].s64 + 2384;
	// 8243AAEC: 4BFFFA4D  bl 0x8243a538
	ctx.lr = 0x8243AAF0;
	sub_8243A538(ctx, base);
	// 8243AAF0: 387F2688  addi r3, r31, 0x2688
	ctx.r[3].s64 = ctx.r[31].s64 + 9864;
	// 8243AAF4: 4BFFF0B5  bl 0x82439ba8
	ctx.lr = 0x8243AAF8;
	sub_82439BA8(ctx, base);
	// 8243AAF8: 387F09F8  addi r3, r31, 0x9f8
	ctx.r[3].s64 = ctx.r[31].s64 + 2552;
	// 8243AAFC: 4801539D  bl 0x8244fe98
	ctx.lr = 0x8243AB00;
	sub_8244FE98(ctx, base);
	// 8243AB00: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243AB04: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 8243AB08: 3BCB04A0  addi r30, r11, 0x4a0
	ctx.r[30].s64 = ctx.r[11].s64 + 1184;
	// 8243AB0C: 387F0A0C  addi r3, r31, 0xa0c
	ctx.r[3].s64 = ctx.r[31].s64 + 2572;
	// 8243AB10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243AB14: 4800F325  bl 0x82449e38
	ctx.lr = 0x8243AB18;
	sub_82449E38(ctx, base);
	// 8243AB18: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 8243AB1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243AB20: 387F0B9C  addi r3, r31, 0xb9c
	ctx.r[3].s64 = ctx.r[31].s64 + 2972;
	// 8243AB24: 4800F315  bl 0x82449e38
	ctx.lr = 0x8243AB28;
	sub_82449E38(ctx, base);
	// 8243AB28: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 8243AB2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243AB30: 387F0D2C  addi r3, r31, 0xd2c
	ctx.r[3].s64 = ctx.r[31].s64 + 3372;
	// 8243AB34: 480FA69D  bl 0x825351d0
	ctx.lr = 0x8243AB38;
	sub_825351D0(ctx, base);
	// 8243AB38: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8243AB3C: 389F0D88  addi r4, r31, 0xd88
	ctx.r[4].s64 = ctx.r[31].s64 + 3464;
	// 8243AB40: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8243AB44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AB48: 917F0D7C  stw r11, 0xd7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3452 as u32), ctx.r[11].u32 ) };
	// 8243AB4C: 48004ECD  bl 0x8243fa18
	ctx.lr = 0x8243AB50;
	sub_8243FA18(ctx, base);
	// 8243AB50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243AB54: 389F1398  addi r4, r31, 0x1398
	ctx.r[4].s64 = ctx.r[31].s64 + 5016;
	// 8243AB58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AB5C: 4800E075  bl 0x82448bd0
	ctx.lr = 0x8243AB60;
	sub_82448BD0(ctx, base);
	// 8243AB60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243AB64: 409A004C  bne cr6, 0x8243abb0
	if !ctx.cr[6].eq {
	pc = 0x8243ABB0; continue 'dispatch;
	}
	// 8243AB68: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8243AB6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243AB70: 389F1FB8  addi r4, r31, 0x1fb8
	ctx.r[4].s64 = ctx.r[31].s64 + 8120;
	// 8243AB74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AB78: 4800E799  bl 0x82449310
	ctx.lr = 0x8243AB7C;
	sub_82449310(ctx, base);
	// 8243AB7C: 387F2668  addi r3, r31, 0x2668
	ctx.r[3].s64 = ctx.r[31].s64 + 9832;
	// 8243AB80: 48009571  bl 0x824440f0
	ctx.lr = 0x8243AB84;
	sub_824440F0(ctx, base);
	// 8243AB84: 387F2658  addi r3, r31, 0x2658
	ctx.r[3].s64 = ctx.r[31].s64 + 9816;
	// 8243AB88: 4800E829  bl 0x824493b0
	ctx.lr = 0x8243AB8C;
	sub_824493B0(ctx, base);
	// 8243AB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AB90: 4BFFF0F9  bl 0x82439c88
	ctx.lr = 0x8243AB94;
	sub_82439C88(ctx, base);
	// 8243AB94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243AB98: 409A0018  bne cr6, 0x8243abb0
	if !ctx.cr[6].eq {
	pc = 0x8243ABB0; continue 'dispatch;
	}
	// 8243AB9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243ABA0: 939F004C  stw r28, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[28].u32 ) };
	// 8243ABA4: 939F0048  stw r28, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 8243ABA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243ABAC: 480FA558  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8243ABB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243ABB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243ABB8: 480FA54C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243ABC0 size=884
    let mut pc: u32 = 0x8243ABC0;
    'dispatch: loop {
        match pc {
            0x8243ABC0 => {
    //   block [0x8243ABC0..0x8243AF34)
	// 8243ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243ABC4: 480FA4BD  bl 0x82535080
	ctx.lr = 0x8243ABC8;
	sub_82535080(ctx, base);
	// 8243ABC8: 9421FC40  stwu r1, -0x3c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-960 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243ABCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243ABD0: 3A000000  li r16, 0
	ctx.r[16].s64 = 0;
	// 8243ABD4: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 8243ABD8: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 8243ABDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243ABE0: 7E118378  mr r17, r16
	ctx.r[17].u64 = ctx.r[16].u64;
	// 8243ABE4: 480F9F6D  bl 0x82534b50
	ctx.lr = 0x8243ABE8;
	sub_82534B50(ctx, base);
	// 8243ABE8: 82BF0A2C  lwz r21, 0xa2c(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2604 as u32) ) } as u64;
	// 8243ABEC: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8243ABF0: 419A0014  beq cr6, 0x8243ac04
	if ctx.cr[6].eq {
	pc = 0x8243AC04; continue 'dispatch;
	}
	// 8243ABF4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8243ABF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243ABFC: 4BFFF215  bl 0x82439e10
	ctx.lr = 0x8243AC00;
	sub_82439E10(ctx, base);
	// 8243AC00: 82210094  lwz r17, 0x94(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8243AC04: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8243AC08: 4800F269  bl 0x82449e70
	ctx.lr = 0x8243AC0C;
	sub_82449E70(ctx, base);
	// 8243AC0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AC10: 4800D971  bl 0x82448580
	ctx.lr = 0x8243AC14;
	sub_82448580(ctx, base);
	// 8243AC14: 389F0D2C  addi r4, r31, 0xd2c
	ctx.r[4].s64 = ctx.r[31].s64 + 3372;
	// 8243AC18: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 8243AC1C: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 8243AC20: 480F9F31  bl 0x82534b50
	ctx.lr = 0x8243AC24;
	sub_82534B50(ctx, base);
	// 8243AC24: 393F1358  addi r9, r31, 0x1358
	ctx.r[9].s64 = ctx.r[31].s64 + 4952;
	// 8243AC28: 395F1364  addi r10, r31, 0x1364
	ctx.r[10].s64 = ctx.r[31].s64 + 4964;
	// 8243AC2C: 83DF2658  lwz r30, 0x2658(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9816 as u32) ) } as u64;
	// 8243AC30: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8243AC34: 839F09F8  lwz r28, 0x9f8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2552 as u32) ) } as u64;
	// 8243AC38: 837F09FC  lwz r27, 0x9fc(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2556 as u32) ) } as u64;
	// 8243AC3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243AC40: 835F1064  lwz r26, 0x1064(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4196 as u32) ) } as u64;
	// 8243AC44: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243AC48: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243AC4C: 833F1068  lwz r25, 0x1068(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4200 as u32) ) } as u64;
	// 8243AC50: 831F106C  lwz r24, 0x106c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4204 as u32) ) } as u64;
	// 8243AC54: 82FF107C  lwz r23, 0x107c(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4220 as u32) ) } as u64;
	// 8243AC58: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 8243AC5C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243AC60: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243AC64: 82DF1080  lwz r22, 0x1080(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4224 as u32) ) } as u64;
	// 8243AC68: 829F0DA0  lwz r20, 0xda0(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3488 as u32) ) } as u64;
	// 8243AC6C: 827F1038  lwz r19, 0x1038(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4152 as u32) ) } as u64;
	// 8243AC70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8243AC74: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8243AC78: 813F1374  lwz r9, 0x1374(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4980 as u32) ) } as u64;
	// 8243AC7C: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243AC80: 825F103C  lwz r18, 0x103c(r31)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4156 as u32) ) } as u64;
	// 8243AC84: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 8243AC88: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243AC8C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243AC90: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8243AC94: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8243AC98: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8243AC9C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8243ACA0: 419A0024  beq cr6, 0x8243acc4
	if ctx.cr[6].eq {
	pc = 0x8243ACC4; continue 'dispatch;
	}
	// 8243ACA4: 817E0DD0  lwz r11, 0xdd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3536 as u32) ) } as u64;
	// 8243ACA8: 81DE0DC8  lwz r14, 0xdc8(r30)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3528 as u32) ) } as u64;
	// 8243ACAC: 81FE0DCC  lwz r15, 0xdcc(r30)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3532 as u32) ) } as u64;
	// 8243ACB0: 821E0DD4  lwz r16, 0xdd4(r30)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3540 as u32) ) } as u64;
	// 8243ACB4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243ACB8: 817E0DC4  lwz r11, 0xdc4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3524 as u32) ) } as u64;
	// 8243ACBC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8243ACC0: 48000014  b 0x8243acd4
	pc = 0x8243ACD4; continue 'dispatch;
	// 8243ACC4: 7E0F8378  mr r15, r16
	ctx.r[15].u64 = ctx.r[16].u64;
	// 8243ACC8: 92010054  stw r16, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[16].u32 ) };
	// 8243ACCC: 7E0E8378  mr r14, r16
	ctx.r[14].u64 = ctx.r[16].u64;
	// 8243ACD0: 92010050  stw r16, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[16].u32 ) };
	// 8243ACD4: 817F1448  lwz r11, 0x1448(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5192 as u32) ) } as u64;
	// 8243ACD8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8243ACDC: 815F1444  lwz r10, 0x1444(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5188 as u32) ) } as u64;
	// 8243ACE0: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 8243ACE4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243ACE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243ACEC: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 8243ACF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8243ACF4: 4800537D  bl 0x82440070
	ctx.lr = 0x8243ACF8;
	sub_82440070(ctx, base);
	// 8243ACF8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243ACFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AD00: 4BFFEF91  bl 0x82439c90
	ctx.lr = 0x8243AD04;
	sub_82439C90(ctx, base);
	// 8243AD04: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243AD08: 409A0224  bne cr6, 0x8243af2c
	if !ctx.cr[6].eq {
	pc = 0x8243AF2C; continue 'dispatch;
	}
	// 8243AD0C: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 8243AD10: 389F0B9C  addi r4, r31, 0xb9c
	ctx.r[4].s64 = ctx.r[31].s64 + 2972;
	// 8243AD14: 38610190  addi r3, r1, 0x190
	ctx.r[3].s64 = ctx.r[1].s64 + 400;
	// 8243AD18: 4800F121  bl 0x82449e38
	ctx.lr = 0x8243AD1C;
	sub_82449E38(ctx, base);
	// 8243AD1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243AD20: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 8243AD24: 4BFFFD05  bl 0x8243aa28
	ctx.lr = 0x8243AD28;
	sub_8243AA28(ctx, base);
	// 8243AD28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243AD2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243AD30: 409A0018  bne cr6, 0x8243ad48
	if !ctx.cr[6].eq {
	pc = 0x8243AD48; continue 'dispatch;
	}
	// 8243AD34: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243AD38: 60840202  ori r4, r4, 0x202
	ctx.r[4].u64 = ctx.r[4].u64 | 514;
	// 8243AD3C: 4800CBCD  bl 0x82447908
	ctx.lr = 0x8243AD40;
	sub_82447908(ctx, base);
	// 8243AD40: 382103C0  addi r1, r1, 0x3c0
	ctx.r[1].s64 = ctx.r[1].s64 + 960;
	// 8243AD44: 480FA38C  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
	// 8243AD48: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 8243AD4C: 38810190  addi r4, r1, 0x190
	ctx.r[4].s64 = ctx.r[1].s64 + 400;
	// 8243AD50: 387F0A0C  addi r3, r31, 0xa0c
	ctx.r[3].s64 = ctx.r[31].s64 + 2572;
	// 8243AD54: 4800F0E5  bl 0x82449e38
	ctx.lr = 0x8243AD58;
	sub_82449E38(ctx, base);
	// 8243AD58: 38A00190  li r5, 0x190
	ctx.r[5].s64 = 400;
	// 8243AD5C: 38810190  addi r4, r1, 0x190
	ctx.r[4].s64 = ctx.r[1].s64 + 400;
	// 8243AD60: 387F0B9C  addi r3, r31, 0xb9c
	ctx.r[3].s64 = ctx.r[31].s64 + 2972;
	// 8243AD64: 4800F0D5  bl 0x82449e38
	ctx.lr = 0x8243AD68;
	sub_82449E38(ctx, base);
	// 8243AD68: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243AD6C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 8243AD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AD74: 48005375  bl 0x824400e8
	ctx.lr = 0x8243AD78;
	sub_824400E8(ctx, base);
	// 8243AD78: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8243AD7C: 419A0038  beq cr6, 0x8243adb4
	if ctx.cr[6].eq {
	pc = 0x8243ADB4; continue 'dispatch;
	}
	// 8243AD80: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8243AD84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AD88: 4BFFF089  bl 0x82439e10
	ctx.lr = 0x8243AD8C;
	sub_82439E10(ctx, base);
	// 8243AD8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243AD90: 409A019C  bne cr6, 0x8243af2c
	if !ctx.cr[6].eq {
	pc = 0x8243AF2C; continue 'dispatch;
	}
	// 8243AD94: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8243AD98: 80A10094  lwz r5, 0x94(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8243AD9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243ADA0: 4BFFF0E1  bl 0x82439e80
	ctx.lr = 0x8243ADA4;
	sub_82439E80(ctx, base);
	// 8243ADA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243ADA8: 409A0184  bne cr6, 0x8243af2c
	if !ctx.cr[6].eq {
	pc = 0x8243AF2C; continue 'dispatch;
	}
	// 8243ADAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243ADB0: 4BFFF149  bl 0x82439ef8
	ctx.lr = 0x8243ADB4;
	sub_82439EF8(ctx, base);
	// 8243ADB4: 387F0D2C  addi r3, r31, 0xd2c
	ctx.r[3].s64 = ctx.r[31].s64 + 3372;
	// 8243ADB8: 38810130  addi r4, r1, 0x130
	ctx.r[4].s64 = ctx.r[1].s64 + 304;
	// 8243ADBC: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 8243ADC0: 480F9D91  bl 0x82534b50
	ctx.lr = 0x8243ADC4;
	sub_82534B50(ctx, base);
	// 8243ADC4: 80BF0D74  lwz r5, 0xd74(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3444 as u32) ) } as u64;
	// 8243ADC8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8243ADCC: 419A0014  beq cr6, 0x8243ade0
	if ctx.cr[6].eq {
	pc = 0x8243ADE0; continue 'dispatch;
	}
	// 8243ADD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243ADD4: 80DF0D78  lwz r6, 0xd78(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3448 as u32) ) } as u64;
	// 8243ADD8: 809F0D7C  lwz r4, 0xd7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3452 as u32) ) } as u64;
	// 8243ADDC: 48005365  bl 0x82440140
	ctx.lr = 0x8243ADE0;
	sub_82440140(ctx, base);
	// 8243ADE0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8243ADE4: 419A0014  beq cr6, 0x8243adf8
	if ctx.cr[6].eq {
	pc = 0x8243ADF8; continue 'dispatch;
	}
	// 8243ADE8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8243ADEC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8243ADF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243ADF4: 4800CB9D  bl 0x82447990
	ctx.lr = 0x8243ADF8;
	sub_82447990(ctx, base);
	// 8243ADF8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8243ADFC: 419A0014  beq cr6, 0x8243ae10
	if ctx.cr[6].eq {
	pc = 0x8243AE10; continue 'dispatch;
	}
	// 8243AE00: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8243AE04: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8243AE08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AE0C: 48003B45  bl 0x8243e950
	ctx.lr = 0x8243AE10;
	sub_8243E950(ctx, base);
	// 8243AE10: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8243AE14: 419A0018  beq cr6, 0x8243ae2c
	if ctx.cr[6].eq {
	pc = 0x8243AE2C; continue 'dispatch;
	}
	// 8243AE18: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8243AE1C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8243AE20: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8243AE24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AE28: 48003B79  bl 0x8243e9a0
	ctx.lr = 0x8243AE2C;
	sub_8243E9A0(ctx, base);
	// 8243AE2C: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 8243AE30: 419A0010  beq cr6, 0x8243ae40
	if ctx.cr[6].eq {
	pc = 0x8243AE40; continue 'dispatch;
	}
	// 8243AE34: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8243AE38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AE3C: 48003ABD  bl 0x8243e8f8
	ctx.lr = 0x8243AE40;
	sub_8243E8F8(ctx, base);
	// 8243AE40: 7F139000  cmpw cr6, r19, r18
	ctx.cr[6].compare_i32(ctx.r[19].s32, ctx.r[18].s32, &mut ctx.xer);
	// 8243AE44: 419A0014  beq cr6, 0x8243ae58
	if ctx.cr[6].eq {
	pc = 0x8243AE58; continue 'dispatch;
	}
	// 8243AE48: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8243AE4C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8243AE50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AE54: 4BFFE265  bl 0x824390b8
	ctx.lr = 0x8243AE58;
	sub_824390B8(ctx, base);
	// 8243AE58: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243AE5C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8243AE60: 419A003C  beq cr6, 0x8243ae9c
	if ctx.cr[6].eq {
	pc = 0x8243AE9C; continue 'dispatch;
	}
	// 8243AE64: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243AE68: 391F1358  addi r8, r31, 0x1358
	ctx.r[8].s64 = ctx.r[31].s64 + 4952;
	// 8243AE6C: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8243AE70: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243AE74: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243AE78: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8243AE7C: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8243AE80: 814B0710  lwz r10, 0x710(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1808 as u32) ) } as u64;
	// 8243AE84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243AE88: 419A0014  beq cr6, 0x8243ae9c
	if ctx.cr[6].eq {
	pc = 0x8243AE9C; continue 'dispatch;
	}
	// 8243AE8C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8243AE90: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243AE94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243AE98: 4E800421  bctrl
	ctx.lr = 0x8243AE9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243AE9C: 80A10074  lwz r5, 0x74(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8243AEA0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8243AEA4: 419A0010  beq cr6, 0x8243aeb4
	if ctx.cr[6].eq {
	pc = 0x8243AEB4; continue 'dispatch;
	}
	// 8243AEA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AEAC: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8243AEB0: 48004391  bl 0x8243f240
	ctx.lr = 0x8243AEB4;
	sub_8243F240(ctx, base);
	// 8243AEB4: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8243AEB8: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 8243AEBC: 419A000C  beq cr6, 0x8243aec8
	if ctx.cr[6].eq {
	pc = 0x8243AEC8; continue 'dispatch;
	}
	// 8243AEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AEC4: 4800468D  bl 0x8243f550
	ctx.lr = 0x8243AEC8;
	sub_8243F550(ctx, base);
	// 8243AEC8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243AECC: 419A0044  beq cr6, 0x8243af10
	if ctx.cr[6].eq {
	pc = 0x8243AF10; continue 'dispatch;
	}
	// 8243AED0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243AED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AED8: 4800E4F9  bl 0x824493d0
	ctx.lr = 0x8243AEDC;
	sub_824493D0(ctx, base);
	// 8243AEDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AEE0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243AEE4: 4800E9CD  bl 0x824498b0
	ctx.lr = 0x8243AEE8;
	sub_824498B0(ctx, base);
	// 8243AEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AEEC: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243AEF0: 4800E8E9  bl 0x824497d8
	ctx.lr = 0x8243AEF4;
	sub_824497D8(ctx, base);
	// 8243AEF4: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 8243AEF8: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8243AEFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AF00: 4800E949  bl 0x82449848
	ctx.lr = 0x8243AF04;
	sub_82449848(ctx, base);
	// 8243AF04: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8243AF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AF0C: 4800E64D  bl 0x82449558
	ctx.lr = 0x8243AF10;
	sub_82449558(ctx, base);
	// 8243AF10: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8243AF14: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8243AF18: 419A0010  beq cr6, 0x8243af28
	if ctx.cr[6].eq {
	pc = 0x8243AF28; continue 'dispatch;
	}
	// 8243AF1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AF20: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8243AF24: 4800FB05  bl 0x8244aa28
	ctx.lr = 0x8243AF28;
	sub_8244AA28(ctx, base);
	// 8243AF28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243AF2C: 382103C0  addi r1, r1, 0x3c0
	ctx.r[1].s64 = ctx.r[1].s64 + 960;
	// 8243AF30: 480FA1A0  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243AF38 size=112
    let mut pc: u32 = 0x8243AF38;
    'dispatch: loop {
        match pc {
            0x8243AF38 => {
    //   block [0x8243AF38..0x8243AFA8)
	// 8243AF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243AF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243AF40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243AF44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243AF48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243AF4C: 4BFFFA4D  bl 0x8243a998
	ctx.lr = 0x8243AF50;
	sub_8243A998(ctx, base);
	// 8243AF50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243AF54: 419A001C  beq cr6, 0x8243af70
	if ctx.cr[6].eq {
	pc = 0x8243AF70; continue 'dispatch;
	}
	// 8243AF58: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243AF5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243AF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243AF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243AF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243AF6C: 4E800020  blr
	return;
	// 8243AF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AF74: 4BFFF995  bl 0x8243a908
	ctx.lr = 0x8243AF78;
	sub_8243A908(ctx, base);
	// 8243AF78: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243AF7C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243AF80: 409A0014  bne cr6, 0x8243af94
	if !ctx.cr[6].eq {
	pc = 0x8243AF94; continue 'dispatch;
	}
	// 8243AF84: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243AF88: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8243AF8C: 409A0008  bne cr6, 0x8243af94
	if !ctx.cr[6].eq {
	pc = 0x8243AF94; continue 'dispatch;
	}
	// 8243AF90: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8243AF94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243AF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243AF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243AFA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243AFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243AFA8 size=116
    let mut pc: u32 = 0x8243AFA8;
    'dispatch: loop {
        match pc {
            0x8243AFA8 => {
    //   block [0x8243AFA8..0x8243B01C)
	// 8243AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243AFAC: 480FA111  bl 0x825350bc
	ctx.lr = 0x8243AFB0;
	sub_82535080(ctx, base);
	// 8243AFB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243AFB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243AFB8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243AFBC: 4BFFEAED  bl 0x82439aa8
	ctx.lr = 0x8243AFC0;
	sub_82439AA8(ctx, base);
	// 8243AFC0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243AFC4: 409A0024  bne cr6, 0x8243afe8
	if !ctx.cr[6].eq {
	pc = 0x8243AFE8; continue 'dispatch;
	}
	// 8243AFC8: 4BFFEB21  bl 0x82439ae8
	ctx.lr = 0x8243AFCC;
	sub_82439AE8(ctx, base);
	// 8243AFCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243AFD0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 8243AFD4: 409A0020  bne cr6, 0x8243aff4
	if !ctx.cr[6].eq {
	pc = 0x8243AFF4; continue 'dispatch;
	}
	// 8243AFD8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243AFDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243AFE0: 60840206  ori r4, r4, 0x206
	ctx.r[4].u64 = ctx.r[4].u64 | 518;
	// 8243AFE4: 4800C925  bl 0x82447908
	ctx.lr = 0x8243AFE8;
	sub_82447908(ctx, base);
	// 8243AFE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243AFEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243AFF0: 480FA11C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243AFF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243AFF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243AFFC: 4BFFFA2D  bl 0x8243aa28
	ctx.lr = 0x8243B000;
	sub_8243AA28(ctx, base);
	// 8243B000: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243B004: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243B008: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243B00C: 396B020C  addi r11, r11, 0x20c
	ctx.r[11].s64 = ctx.r[11].s64 + 524;
	// 8243B010: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 8243B014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243B018: 480FA0F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B020 size=136
    let mut pc: u32 = 0x8243B020;
    'dispatch: loop {
        match pc {
            0x8243B020 => {
    //   block [0x8243B020..0x8243B0A8)
	// 8243B020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243B02C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B034: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243B038: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243B03C: 409A001C  bne cr6, 0x8243b058
	if !ctx.cr[6].eq {
	pc = 0x8243B058; continue 'dispatch;
	}
	// 8243B040: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B054: 4E800020  blr
	return;
	// 8243B058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B05C: 4BFFEC9D  bl 0x82439cf8
	ctx.lr = 0x8243B060;
	sub_82439CF8(ctx, base);
	// 8243B060: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243B064: 409A0030  bne cr6, 0x8243b094
	if !ctx.cr[6].eq {
	pc = 0x8243B094; continue 'dispatch;
	}
	// 8243B068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243B06C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243B070: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8243B074: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8243B078: 4BFFEC61  bl 0x82439cd8
	ctx.lr = 0x8243B07C;
	sub_82439CD8(ctx, base);
	// 8243B07C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B080: 4BFFFB41  bl 0x8243abc0
	ctx.lr = 0x8243B084;
	sub_8243ABC0(ctx, base);
	// 8243B084: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8243B088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B08C: 4BFFEC4D  bl 0x82439cd8
	ctx.lr = 0x8243B090;
	sub_82439CD8(ctx, base);
	// 8243B090: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8243B094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B0A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B0A8 size=264
    let mut pc: u32 = 0x8243B0A8;
    'dispatch: loop {
        match pc {
            0x8243B0A8 => {
    //   block [0x8243B0A8..0x8243B158)
	// 8243B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B0AC: 480FA011  bl 0x825350bc
	ctx.lr = 0x8243B0B0;
	sub_82535080(ctx, base);
	// 8243B0B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B0B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B0B8: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243B0BC: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8243B0C0: 419A001C  beq cr6, 0x8243b0dc
	if ctx.cr[6].eq {
	pc = 0x8243B0DC; continue 'dispatch;
	}
	// 8243B0C4: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 8243B0C8: 419A0014  beq cr6, 0x8243b0dc
	if ctx.cr[6].eq {
	pc = 0x8243B0DC; continue 'dispatch;
	}
	// 8243B0CC: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 8243B0D0: 419A000C  beq cr6, 0x8243b0dc
	if ctx.cr[6].eq {
	pc = 0x8243B0DC; continue 'dispatch;
	}
	// 8243B0D4: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 8243B0D8: 409A00D0  bne cr6, 0x8243b1a8
	if !ctx.cr[6].eq {
	pc = 0x8243B1A8; continue 'dispatch;
	}
	// 8243B0DC: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8243B0E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243B0E4: 419A00C4  beq cr6, 0x8243b1a8
	if ctx.cr[6].eq {
	pc = 0x8243B1A8; continue 'dispatch;
	}
	// 8243B0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243B0EC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8243B0F0: 4800EBC9  bl 0x82449cb8
	ctx.lr = 0x8243B0F4;
	sub_82449CB8(ctx, base);
	// 8243B0F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243B0F8: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 8243B0FC: 419A0014  beq cr6, 0x8243b110
	if ctx.cr[6].eq {
	pc = 0x8243B110; continue 'dispatch;
	}
	// 8243B100: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 8243B104: 419A000C  beq cr6, 0x8243b110
	if ctx.cr[6].eq {
	pc = 0x8243B110; continue 'dispatch;
	}
	// 8243B108: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 8243B10C: 409A000C  bne cr6, 0x8243b118
	if !ctx.cr[6].eq {
	pc = 0x8243B118; continue 'dispatch;
	}
	// 8243B110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B114: 4BFFEFC5  bl 0x8243a0d8
	ctx.lr = 0x8243B118;
	sub_8243A0D8(ctx, base);
	// 8243B118: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243B11C: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8243B120: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8243B124: 4199006C  bgt cr6, 0x8243b190
	if ctx.cr[6].gt {
	pc = 0x8243B190; continue 'dispatch;
	}
	// 8243B128: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 8243B12C: 398CB140  addi r12, r12, -0x4ec0
	ctx.r[12].s64 = ctx.r[12].s64 + -20160;
	// 8243B130: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8243B134: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8243B138: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8243B13C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8243B158; continue 'dispatch;
		},
		1 => {
	pc = 0x8243B164; continue 'dispatch;
		},
		2 => {
	pc = 0x8243B170; continue 'dispatch;
		},
		3 => {
	pc = 0x8243B17C; continue 'dispatch;
		},
		4 => {
	pc = 0x8243B190; continue 'dispatch;
		},
		5 => {
	pc = 0x8243B188; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8243B140: 8243B158  lwz r18, -0x4ea8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20136 as u32) ) } as u64;
	// 8243B144: 8243B164  lwz r18, -0x4e9c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20124 as u32) ) } as u64;
	// 8243B148: 8243B170  lwz r18, -0x4e90(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20112 as u32) ) } as u64;
	// 8243B14C: 8243B17C  lwz r18, -0x4e84(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20100 as u32) ) } as u64;
	// 8243B150: 8243B190  lwz r18, -0x4e70(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20080 as u32) ) } as u64;
	// 8243B154: 8243B188  lwz r18, -0x4e78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-20088 as u32) ) } as u64;
            }
            0x8243B158 => {
    //   block [0x8243B158..0x8243B164)
	// 8243B158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B15C: 4BFFE205  bl 0x82439360
	ctx.lr = 0x8243B160;
	sub_82439360(ctx, base);
	// 8243B160: 48000030  b 0x8243b190
	pc = 0x8243B190; continue 'dispatch;
            }
            0x8243B164 => {
    //   block [0x8243B164..0x8243B170)
	// 8243B164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B168: 4BFFF651  bl 0x8243a7b8
	ctx.lr = 0x8243B16C;
	sub_8243A7B8(ctx, base);
	// 8243B16C: 48000024  b 0x8243b190
	pc = 0x8243B190; continue 'dispatch;
            }
            0x8243B170 => {
    //   block [0x8243B170..0x8243B17C)
	// 8243B170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B174: 4BFFF6F5  bl 0x8243a868
	ctx.lr = 0x8243B178;
	sub_8243A868(ctx, base);
	// 8243B178: 48000018  b 0x8243b190
	pc = 0x8243B190; continue 'dispatch;
            }
            0x8243B17C => {
    //   block [0x8243B17C..0x8243B188)
	// 8243B17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B180: 4BFFFDB9  bl 0x8243af38
	ctx.lr = 0x8243B184;
	sub_8243AF38(ctx, base);
	// 8243B184: 4800000C  b 0x8243b190
	pc = 0x8243B190; continue 'dispatch;
            }
            0x8243B188 => {
    //   block [0x8243B188..0x8243B190)
	// 8243B188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B18C: 4BFFE4BD  bl 0x82439648
	ctx.lr = 0x8243B190;
	sub_82439648(ctx, base);
	pc = 0x8243B190; continue 'dispatch;
            }
            0x8243B190 => {
    //   block [0x8243B190..0x8243B1B0)
	// 8243B190: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 8243B194: 4800EB25  bl 0x82449cb8
	ctx.lr = 0x8243B198;
	sub_82449CB8(ctx, base);
	// 8243B198: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243B19C: 387F2728  addi r3, r31, 0x2728
	ctx.r[3].s64 = ctx.r[31].s64 + 10024;
	// 8243B1A0: 7C9D5850  subf r4, r29, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8243B1A4: 4800EC4D  bl 0x82449df0
	ctx.lr = 0x8243B1A8;
	sub_82449DF0(ctx, base);
	// 8243B1A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243B1AC: 480F9F60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B1B0 size=168
    let mut pc: u32 = 0x8243B1B0;
    'dispatch: loop {
        match pc {
            0x8243B1B0 => {
    //   block [0x8243B1B0..0x8243B258)
	// 8243B1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B1B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243B1BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B1C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B1C4: 4800C67D  bl 0x82447840
	ctx.lr = 0x8243B1C8;
	sub_82447840(ctx, base);
	// 8243B1C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243B1CC: 419A0028  beq cr6, 0x8243b1f4
	if ctx.cr[6].eq {
	pc = 0x8243B1F4; continue 'dispatch;
	}
	// 8243B1D0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243B1D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B1D8: 60840131  ori r4, r4, 0x131
	ctx.r[4].u64 = ctx.r[4].u64 | 305;
	// 8243B1DC: 4800C72D  bl 0x82447908
	ctx.lr = 0x8243B1E0;
	sub_82447908(ctx, base);
	// 8243B1E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B1EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B1F0: 4E800020  blr
	return;
	// 8243B1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B1F8: 4BFFFE29  bl 0x8243b020
	ctx.lr = 0x8243B1FC;
	sub_8243B020(ctx, base);
	// 8243B1FC: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8243B200: 4800EC71  bl 0x82449e70
	ctx.lr = 0x8243B204;
	sub_82449E70(ctx, base);
	// 8243B204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B208: 4800D379  bl 0x82448580
	ctx.lr = 0x8243B20C;
	sub_82448580(ctx, base);
	// 8243B20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B210: 4BFFEA81  bl 0x82439c90
	ctx.lr = 0x8243B214;
	sub_82439C90(ctx, base);
	// 8243B214: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243B218: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8243B21C: 394B04A0  addi r10, r11, 0x4a0
	ctx.r[10].s64 = ctx.r[11].s64 + 1184;
	// 8243B220: 396A020C  addi r11, r10, 0x20c
	ctx.r[11].s64 = ctx.r[10].s64 + 524;
	// 8243B224: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B228: 7F08F840  cmplw cr6, r8, r31
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8243B22C: 409A0008  bne cr6, 0x8243b234
	if !ctx.cr[6].eq {
	pc = 0x8243B234; continue 'dispatch;
	}
	// 8243B230: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8243B234: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8243B238: 390A022C  addi r8, r10, 0x22c
	ctx.r[8].s64 = ctx.r[10].s64 + 556;
	// 8243B23C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8243B240: 4198FFE4  blt cr6, 0x8243b224
	if ctx.cr[6].lt {
	pc = 0x8243B224; continue 'dispatch;
	}
	// 8243B244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B258 size=104
    let mut pc: u32 = 0x8243B258;
    'dispatch: loop {
        match pc {
            0x8243B258 => {
    //   block [0x8243B258..0x8243B2C0)
	// 8243B258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243B264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B26C: 4800C5D5  bl 0x82447840
	ctx.lr = 0x8243B270;
	sub_82447840(ctx, base);
	// 8243B270: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243B274: 419A0028  beq cr6, 0x8243b29c
	if ctx.cr[6].eq {
	pc = 0x8243B29C; continue 'dispatch;
	}
	// 8243B278: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243B27C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B280: 60840133  ori r4, r4, 0x133
	ctx.r[4].u64 = ctx.r[4].u64 | 307;
	// 8243B284: 4800C685  bl 0x82447908
	ctx.lr = 0x8243B288;
	sub_82447908(ctx, base);
	// 8243B288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B298: 4E800020  blr
	return;
	// 8243B29C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B2A0: 4BFFFD81  bl 0x8243b020
	ctx.lr = 0x8243B2A4;
	sub_8243B020(ctx, base);
	// 8243B2A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243B2A8: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8243B2AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B2B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B2C0 size=100
    let mut pc: u32 = 0x8243B2C0;
    'dispatch: loop {
        match pc {
            0x8243B2C0 => {
    //   block [0x8243B2C0..0x8243B324)
	// 8243B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B2C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243B2CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B2D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B2D4: 4800C56D  bl 0x82447840
	ctx.lr = 0x8243B2D8;
	sub_82447840(ctx, base);
	// 8243B2D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243B2DC: 419A0028  beq cr6, 0x8243b304
	if ctx.cr[6].eq {
	pc = 0x8243B304; continue 'dispatch;
	}
	// 8243B2E0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243B2E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B2E8: 60840138  ori r4, r4, 0x138
	ctx.r[4].u64 = ctx.r[4].u64 | 312;
	// 8243B2EC: 4800C61D  bl 0x82447908
	ctx.lr = 0x8243B2F0;
	sub_82447908(ctx, base);
	// 8243B2F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B2FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B300: 4E800020  blr
	return;
	// 8243B304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B308: 4BFFFDA1  bl 0x8243b0a8
	ctx.lr = 0x8243B30C;
	sub_8243B0A8(ctx, base);
	// 8243B30C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B31C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B328 size=44
    let mut pc: u32 = 0x8243B328;
    'dispatch: loop {
        match pc {
            0x8243B328 => {
    //   block [0x8243B328..0x8243B354)
	// 8243B328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B334: 4BFE654D  bl 0x82421880
	ctx.lr = 0x8243B338;
	sub_82421880(ctx, base);
	// 8243B338: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 8243B33C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243B340: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243B344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B358 size=8
    let mut pc: u32 = 0x8243B358;
    'dispatch: loop {
        match pc {
            0x8243B358 => {
    //   block [0x8243B358..0x8243B360)
	// 8243B358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243B35C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B360 size=8
    let mut pc: u32 = 0x8243B360;
    'dispatch: loop {
        match pc {
            0x8243B360 => {
    //   block [0x8243B360..0x8243B368)
	// 8243B360: 4BFE6D20  b 0x82422080
	sub_82422080(ctx, base);
	return;
	// 8243B364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B368 size=4
    let mut pc: u32 = 0x8243B368;
    'dispatch: loop {
        match pc {
            0x8243B368 => {
    //   block [0x8243B368..0x8243B36C)
	// 8243B368: 4BFE62F8  b 0x82421660
	sub_82421660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B370 size=8
    let mut pc: u32 = 0x8243B370;
    'dispatch: loop {
        match pc {
            0x8243B370 => {
    //   block [0x8243B370..0x8243B378)
	// 8243B370: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243B374: 4BFE6E3C  b 0x824221b0
	sub_824221B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B378 size=4
    let mut pc: u32 = 0x8243B378;
    'dispatch: loop {
        match pc {
            0x8243B378 => {
    //   block [0x8243B378..0x8243B37C)
	// 8243B378: 4BFE72F8  b 0x82422670
	sub_82422670(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B380 size=84
    let mut pc: u32 = 0x8243B380;
    'dispatch: loop {
        match pc {
            0x8243B380 => {
    //   block [0x8243B380..0x8243B3D4)
	// 8243B380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B384: 480F9D31  bl 0x825350b4
	ctx.lr = 0x8243B388;
	sub_82535080(ctx, base);
	// 8243B388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B390: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243B394: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8243B398: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8243B39C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8243B3A0: 4BFE7201  bl 0x824225a0
	ctx.lr = 0x8243B3A4;
	sub_824225A0(ctx, base);
	// 8243B3A4: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 8243B3A8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8243B3AC: 79675D24  sldi r7, r11, 0xb
	ctx.r[7].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 8243B3B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8243B3B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243B3B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B3BC: 4BFE6E45  bl 0x82422200
	ctx.lr = 0x8243B3C0;
	sub_82422200(ctx, base);
	// 8243B3C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243B3C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B3C8: 4BFE65D1  bl 0x82421998
	ctx.lr = 0x8243B3CC;
	sub_82421998(ctx, base);
	// 8243B3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243B3D0: 480F9D34  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B3D8 size=4
    let mut pc: u32 = 0x8243B3D8;
    'dispatch: loop {
        match pc {
            0x8243B3D8 => {
    //   block [0x8243B3D8..0x8243B3DC)
	// 8243B3D8: 4BFE6E90  b 0x82422268
	sub_82422268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B3E0 size=52
    let mut pc: u32 = 0x8243B3E0;
    'dispatch: loop {
        match pc {
            0x8243B3E0 => {
    //   block [0x8243B3E0..0x8243B414)
	// 8243B3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B3E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243B3EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B3F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B3F4: 4BFE6F7D  bl 0x82422370
	ctx.lr = 0x8243B3F8;
	sub_82422370(ctx, base);
	// 8243B3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B3FC: 4BFE71A5  bl 0x824225a0
	ctx.lr = 0x8243B400;
	sub_824225A0(ctx, base);
	// 8243B400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B40C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B418 size=4
    let mut pc: u32 = 0x8243B418;
    'dispatch: loop {
        match pc {
            0x8243B418 => {
    //   block [0x8243B418..0x8243B41C)
	// 8243B418: 4BFE6468  b 0x82421880
	sub_82421880(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B420 size=40
    let mut pc: u32 = 0x8243B420;
    'dispatch: loop {
        match pc {
            0x8243B420 => {
    //   block [0x8243B420..0x8243B448)
	// 8243B420: 39640025  addi r11, r4, 0x25
	ctx.r[11].s64 = ctx.r[4].s64 + 37;
	// 8243B424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8243B428: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243B42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8243B430: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243B434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8243B438: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243B43C: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8243B440: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 8243B444: 4BFF7F8C  b 0x824333d0
	sub_824333D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B448 size=24
    let mut pc: u32 = 0x8243B448;
    'dispatch: loop {
        match pc {
            0x8243B448 => {
    //   block [0x8243B448..0x8243B460)
	// 8243B448: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243B44C: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8243B450: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243B454: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8243B458: 806B0538  lwz r3, 0x538(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1336 as u32) ) } as u64;
	// 8243B45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B460 size=20
    let mut pc: u32 = 0x8243B460;
    'dispatch: loop {
        match pc {
            0x8243B460 => {
    //   block [0x8243B460..0x8243B474)
	// 8243B460: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243B464: 546A1838  slwi r10, r3, 3
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243B468: 396B9F4C  addi r11, r11, -0x60b4
	ctx.r[11].s64 = ctx.r[11].s64 + -24756;
	// 8243B46C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243B470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243B478 size=20
    let mut pc: u32 = 0x8243B478;
    'dispatch: loop {
        match pc {
            0x8243B478 => {
    //   block [0x8243B478..0x8243B48C)
	// 8243B478: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243B47C: 546A1838  slwi r10, r3, 3
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243B480: 396B9F4C  addi r11, r11, -0x60b4
	ctx.r[11].s64 = ctx.r[11].s64 + -24756;
	// 8243B484: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8243B488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B490 size=96
    let mut pc: u32 = 0x8243B490;
    'dispatch: loop {
        match pc {
            0x8243B490 => {
    //   block [0x8243B490..0x8243B4F0)
	// 8243B490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B49C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8243B4A0: 80690004  lwz r3, 4(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243B4A4: 4BFFFFD5  bl 0x8243b478
	ctx.lr = 0x8243B4A8;
	sub_8243B478(ctx, base);
	// 8243B4A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243B4AC: 409A0018  bne cr6, 0x8243b4c4
	if !ctx.cr[6].eq {
	pc = 0x8243B4C4; continue 'dispatch;
	}
	// 8243B4B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B4B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B4C0: 4E800020  blr
	return;
	// 8243B4C4: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B4C8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243B4CC: 409AFFE4  bne cr6, 0x8243b4b0
	if !ctx.cr[6].eq {
	pc = 0x8243B4B0; continue 'dispatch;
	}
	// 8243B4D0: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243B4D4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243B4D8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243B4DC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8243B4E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B4F0 size=92
    let mut pc: u32 = 0x8243B4F0;
    'dispatch: loop {
        match pc {
            0x8243B4F0 => {
    //   block [0x8243B4F0..0x8243B54C)
	// 8243B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B4F4: 480F9BC5  bl 0x825350b8
	ctx.lr = 0x8243B4F8;
	sub_82535080(ctx, base);
	// 8243B4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B4FC: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243B500: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243B504: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8243B508: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243B50C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243B510: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8243B514: 839F052C  lwz r28, 0x52c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1324 as u32) ) } as u64;
	// 8243B518: 4BFFFF09  bl 0x8243b420
	ctx.lr = 0x8243B51C;
	sub_8243B420(ctx, base);
	// 8243B51C: 817F0530  lwz r11, 0x530(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) } as u64;
	// 8243B520: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243B524: 409A001C  bne cr6, 0x8243b540
	if !ctx.cr[6].eq {
	pc = 0x8243B540; continue 'dispatch;
	}
	// 8243B528: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8243B52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8243B530: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8243B534: 389D00C0  addi r4, r29, 0xc0
	ctx.r[4].s64 = ctx.r[29].s64 + 192;
	// 8243B538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243B53C: 4BFF7E95  bl 0x824333d0
	ctx.lr = 0x8243B540;
	sub_824333D0(ctx, base);
	// 8243B540: 93BF0534  stw r29, 0x534(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1332 as u32), ctx.r[29].u32 ) };
	// 8243B544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243B548: 480F9BC0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B550 size=100
    let mut pc: u32 = 0x8243B550;
    'dispatch: loop {
        match pc {
            0x8243B550 => {
    //   block [0x8243B550..0x8243B5B4)
	// 8243B550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B55C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8243B560: 4BFFFF01  bl 0x8243b460
	ctx.lr = 0x8243B564;
	sub_8243B460(ctx, base);
	// 8243B564: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8243B568: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8243B56C: 4BFFFF0D  bl 0x8243b478
	ctx.lr = 0x8243B570;
	sub_8243B478(ctx, base);
	// 8243B570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243B574: 419A0030  beq cr6, 0x8243b5a4
	if ctx.cr[6].eq {
	pc = 0x8243B5A4; continue 'dispatch;
	}
	// 8243B578: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243B57C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243B580: 419A0024  beq cr6, 0x8243b5a4
	if ctx.cr[6].eq {
	pc = 0x8243B5A4; continue 'dispatch;
	}
	// 8243B584: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243B588: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243B58C: 40820018  bne 0x8243b5a4
	if !ctx.cr[0].eq {
	pc = 0x8243B5A4; continue 'dispatch;
	}
	// 8243B590: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243B594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B598: 419A000C  beq cr6, 0x8243b5a4
	if ctx.cr[6].eq {
	pc = 0x8243B5A4; continue 'dispatch;
	}
	// 8243B59C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B5A0: 4E800421  bctrl
	ctx.lr = 0x8243B5A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B5A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B5B8 size=72
    let mut pc: u32 = 0x8243B5B8;
    'dispatch: loop {
        match pc {
            0x8243B5B8 => {
    //   block [0x8243B5B8..0x8243B600)
	// 8243B5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B5C4: 4BFFFEB5  bl 0x8243b478
	ctx.lr = 0x8243B5C8;
	sub_8243B478(ctx, base);
	// 8243B5C8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8243B5CC: 419A0024  beq cr6, 0x8243b5f0
	if ctx.cr[6].eq {
	pc = 0x8243B5F0; continue 'dispatch;
	}
	// 8243B5D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243B5D4: 419A001C  beq cr6, 0x8243b5f0
	if ctx.cr[6].eq {
	pc = 0x8243B5F0; continue 'dispatch;
	}
	// 8243B5D8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243B5DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B5E0: 419A0010  beq cr6, 0x8243b5f0
	if ctx.cr[6].eq {
	pc = 0x8243B5F0; continue 'dispatch;
	}
	// 8243B5E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8243B5E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B5EC: 4E800421  bctrl
	ctx.lr = 0x8243B5F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B600 size=84
    let mut pc: u32 = 0x8243B600;
    'dispatch: loop {
        match pc {
            0x8243B600 => {
    //   block [0x8243B600..0x8243B654)
	// 8243B600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B60C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8243B610: 4BFFFE81  bl 0x8243b490
	ctx.lr = 0x8243B614;
	sub_8243B490(ctx, base);
	// 8243B614: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243B618: 409A002C  bne cr6, 0x8243b644
	if !ctx.cr[6].eq {
	pc = 0x8243B644; continue 'dispatch;
	}
	// 8243B61C: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243B620: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B624: 419A0020  beq cr6, 0x8243b644
	if ctx.cr[6].eq {
	pc = 0x8243B644; continue 'dispatch;
	}
	// 8243B628: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243B62C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B630: 419A0014  beq cr6, 0x8243b644
	if ctx.cr[6].eq {
	pc = 0x8243B644; continue 'dispatch;
	}
	// 8243B634: 80880014  lwz r4, 0x14(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243B638: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243B63C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B640: 4E800421  bctrl
	ctx.lr = 0x8243B644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B658 size=88
    let mut pc: u32 = 0x8243B658;
    'dispatch: loop {
        match pc {
            0x8243B658 => {
    //   block [0x8243B658..0x8243B6B0)
	// 8243B658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B664: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8243B668: 4BFFFE29  bl 0x8243b490
	ctx.lr = 0x8243B66C;
	sub_8243B490(ctx, base);
	// 8243B66C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243B670: 409A0030  bne cr6, 0x8243b6a0
	if !ctx.cr[6].eq {
	pc = 0x8243B6A0; continue 'dispatch;
	}
	// 8243B674: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243B678: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243B67C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243B680: 419A0020  beq cr6, 0x8243b6a0
	if ctx.cr[6].eq {
	pc = 0x8243B6A0; continue 'dispatch;
	}
	// 8243B684: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B688: 419A0018  beq cr6, 0x8243b6a0
	if ctx.cr[6].eq {
	pc = 0x8243B6A0; continue 'dispatch;
	}
	// 8243B68C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243B690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B694: 419A000C  beq cr6, 0x8243b6a0
	if ctx.cr[6].eq {
	pc = 0x8243B6A0; continue 'dispatch;
	}
	// 8243B698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B69C: 4E800421  bctrl
	ctx.lr = 0x8243B6A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B6B0 size=112
    let mut pc: u32 = 0x8243B6B0;
    'dispatch: loop {
        match pc {
            0x8243B6B0 => {
    //   block [0x8243B6B0..0x8243B720)
	// 8243B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B6BC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8243B6C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8243B6C4: 4BFFFDCD  bl 0x8243b490
	ctx.lr = 0x8243B6C8;
	sub_8243B490(ctx, base);
	// 8243B6C8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243B6CC: 419A0018  beq cr6, 0x8243b6e4
	if ctx.cr[6].eq {
	pc = 0x8243B6E4; continue 'dispatch;
	}
	// 8243B6D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243B6D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B6E0: 4E800020  blr
	return;
	// 8243B6E4: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243B6E8: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243B6EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B6F0: 419A001C  beq cr6, 0x8243b70c
	if ctx.cr[6].eq {
	pc = 0x8243B70C; continue 'dispatch;
	}
	// 8243B6F4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8243B6F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B6FC: 419A0010  beq cr6, 0x8243b70c
	if ctx.cr[6].eq {
	pc = 0x8243B70C; continue 'dispatch;
	}
	// 8243B700: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B704: 4E800421  bctrl
	ctx.lr = 0x8243B708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B708: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8243B70C: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 8243B710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B720 size=80
    let mut pc: u32 = 0x8243B720;
    'dispatch: loop {
        match pc {
            0x8243B720 => {
    //   block [0x8243B720..0x8243B770)
	// 8243B720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B72C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8243B730: 4BFFFD61  bl 0x8243b490
	ctx.lr = 0x8243B734;
	sub_8243B490(ctx, base);
	// 8243B734: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243B738: 409A0028  bne cr6, 0x8243b760
	if !ctx.cr[6].eq {
	pc = 0x8243B760; continue 'dispatch;
	}
	// 8243B73C: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243B740: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B744: 419A001C  beq cr6, 0x8243b760
	if ctx.cr[6].eq {
	pc = 0x8243B760; continue 'dispatch;
	}
	// 8243B748: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243B74C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B750: 419A0010  beq cr6, 0x8243b760
	if ctx.cr[6].eq {
	pc = 0x8243B760; continue 'dispatch;
	}
	// 8243B754: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243B758: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B75C: 4E800421  bctrl
	ctx.lr = 0x8243B760;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B770 size=80
    let mut pc: u32 = 0x8243B770;
    'dispatch: loop {
        match pc {
            0x8243B770 => {
    //   block [0x8243B770..0x8243B7C0)
	// 8243B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B77C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8243B780: 4BFFFD11  bl 0x8243b490
	ctx.lr = 0x8243B784;
	sub_8243B490(ctx, base);
	// 8243B784: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243B788: 409A0028  bne cr6, 0x8243b7b0
	if !ctx.cr[6].eq {
	pc = 0x8243B7B0; continue 'dispatch;
	}
	// 8243B78C: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243B790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B794: 419A001C  beq cr6, 0x8243b7b0
	if ctx.cr[6].eq {
	pc = 0x8243B7B0; continue 'dispatch;
	}
	// 8243B798: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243B79C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B7A0: 419A0010  beq cr6, 0x8243b7b0
	if ctx.cr[6].eq {
	pc = 0x8243B7B0; continue 'dispatch;
	}
	// 8243B7A4: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243B7A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B7AC: 4E800421  bctrl
	ctx.lr = 0x8243B7B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B7C0 size=104
    let mut pc: u32 = 0x8243B7C0;
    'dispatch: loop {
        match pc {
            0x8243B7C0 => {
    //   block [0x8243B7C0..0x8243B828)
	// 8243B7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B7C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243B7CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B7D0: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243B7D4: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8243B7D8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243B7DC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8243B7E0: 390B0518  addi r8, r11, 0x518
	ctx.r[8].s64 = ctx.r[11].s64 + 1304;
	// 8243B7E4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8243B7E8: 83E80014  lwz r31, 0x14(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243B7EC: 4BFFFCA5  bl 0x8243b490
	ctx.lr = 0x8243B7F0;
	sub_8243B490(ctx, base);
	// 8243B7F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243B7F4: 409A0020  bne cr6, 0x8243b814
	if !ctx.cr[6].eq {
	pc = 0x8243B814; continue 'dispatch;
	}
	// 8243B7F8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8243B7FC: 4BFFFE5D  bl 0x8243b658
	ctx.lr = 0x8243B800;
	sub_8243B658(ctx, base);
	// 8243B800: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B808: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243B80C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B810: 4E800421  bctrl
	ctx.lr = 0x8243B814;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243B818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243B81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243B820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243B824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B828 size=120
    let mut pc: u32 = 0x8243B828;
    'dispatch: loop {
        match pc {
            0x8243B828 => {
    //   block [0x8243B828..0x8243B8A0)
	// 8243B828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B82C: 480F9889  bl 0x825350b4
	ctx.lr = 0x8243B830;
	sub_82535080(ctx, base);
	// 8243B830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B838: 4BFFFC59  bl 0x8243b490
	ctx.lr = 0x8243B83C;
	sub_8243B490(ctx, base);
	// 8243B83C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243B840: 409A0058  bne cr6, 0x8243b898
	if !ctx.cr[6].eq {
	pc = 0x8243B898; continue 'dispatch;
	}
	// 8243B844: 839F0020  lwz r28, 0x20(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243B848: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243B84C: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243B850: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8243B854: 419A0044  beq cr6, 0x8243b898
	if ctx.cr[6].eq {
	pc = 0x8243B898; continue 'dispatch;
	}
	// 8243B858: 4BFF36F9  bl 0x8242ef50
	ctx.lr = 0x8243B85C;
	sub_8242EF50(ctx, base);
	// 8243B85C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B860: 4BFFFDF9  bl 0x8243b658
	ctx.lr = 0x8243B864;
	sub_8243B658(ctx, base);
	// 8243B864: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8243B868: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8243B86C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243B870: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8243B874: 4BFFFD45  bl 0x8243b5b8
	ctx.lr = 0x8243B878;
	sub_8243B5B8(ctx, base);
	// 8243B878: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B87C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243B880: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243B884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B888: 4E800421  bctrl
	ctx.lr = 0x8243B88C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B88C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243B890: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 8243B894: 4BFFFCBD  bl 0x8243b550
	ctx.lr = 0x8243B898;
	sub_8243B550(ctx, base);
	// 8243B898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243B89C: 480F9868  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B8A0 size=276
    let mut pc: u32 = 0x8243B8A0;
    'dispatch: loop {
        match pc {
            0x8243B8A0 => {
    //   block [0x8243B8A0..0x8243B9B4)
	// 8243B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B8A4: 480F9819  bl 0x825350bc
	ctx.lr = 0x8243B8A8;
	sub_82535080(ctx, base);
	// 8243B8A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B8AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B8B0: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243B8B4: 48000D95  bl 0x8243c648
	ctx.lr = 0x8243B8B8;
	sub_8243C648(ctx, base);
	// 8243B8B8: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8243B8BC: 409A00F0  bne cr6, 0x8243b9ac
	if !ctx.cr[6].eq {
	pc = 0x8243B9AC; continue 'dispatch;
	}
	// 8243B8C0: 3BBF0518  addi r29, r31, 0x518
	ctx.r[29].s64 = ctx.r[31].s64 + 1304;
	// 8243B8C4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B8C8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243B8CC: 409A0034  bne cr6, 0x8243b900
	if !ctx.cr[6].eq {
	pc = 0x8243B900; continue 'dispatch;
	}
	// 8243B8D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243B8D4: 4BFFFDDD  bl 0x8243b6b0
	ctx.lr = 0x8243B8D8;
	sub_8243B6B0(ctx, base);
	// 8243B8D8: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8243B8DC: 419A0024  beq cr6, 0x8243b900
	if ctx.cr[6].eq {
	pc = 0x8243B900; continue 'dispatch;
	}
	// 8243B8E0: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243B8E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243B8E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B8EC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243B8F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B8F4: 4E800421  bctrl
	ctx.lr = 0x8243B8F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B8F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243B8FC: 409A00B0  bne cr6, 0x8243b9ac
	if !ctx.cr[6].eq {
	pc = 0x8243B9AC; continue 'dispatch;
	}
	// 8243B900: 3BDF053C  addi r30, r31, 0x53c
	ctx.r[30].s64 = ctx.r[31].s64 + 1340;
	// 8243B904: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B908: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243B90C: 409A0034  bne cr6, 0x8243b940
	if !ctx.cr[6].eq {
	pc = 0x8243B940; continue 'dispatch;
	}
	// 8243B910: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243B914: 4BFFFD9D  bl 0x8243b6b0
	ctx.lr = 0x8243B918;
	sub_8243B6B0(ctx, base);
	// 8243B918: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8243B91C: 419A0024  beq cr6, 0x8243b940
	if ctx.cr[6].eq {
	pc = 0x8243B940; continue 'dispatch;
	}
	// 8243B920: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243B924: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243B928: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243B92C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243B930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243B934: 4E800421  bctrl
	ctx.lr = 0x8243B938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243B938: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243B93C: 409A0070  bne cr6, 0x8243b9ac
	if !ctx.cr[6].eq {
	pc = 0x8243B9AC; continue 'dispatch;
	}
	// 8243B940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B944: 4BFF6DA5  bl 0x824326e8
	ctx.lr = 0x8243B948;
	sub_824326E8(ctx, base);
	// 8243B948: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 8243B94C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B950: 409A0010  bne cr6, 0x8243b960
	if !ctx.cr[6].eq {
	pc = 0x8243B960; continue 'dispatch;
	}
	// 8243B954: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243B958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B95C: 4BFF6FE5  bl 0x82432940
	ctx.lr = 0x8243B960;
	sub_82432940(ctx, base);
	// 8243B960: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8243B964: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8243B968: 409A0020  bne cr6, 0x8243b988
	if !ctx.cr[6].eq {
	pc = 0x8243B988; continue 'dispatch;
	}
	// 8243B96C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243B970: 480088F1  bl 0x82444260
	ctx.lr = 0x8243B974;
	sub_82444260(ctx, base);
	// 8243B974: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243B978: 419A0010  beq cr6, 0x8243b988
	if ctx.cr[6].eq {
	pc = 0x8243B988; continue 'dispatch;
	}
	// 8243B97C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243B980: 386B5F4C  addi r3, r11, 0x5f4c
	ctx.r[3].s64 = ctx.r[11].s64 + 24396;
	// 8243B984: 4BFFB745  bl 0x824370c8
	ctx.lr = 0x8243B988;
	sub_824370C8(ctx, base);
	// 8243B988: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 8243B98C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B990: 409A001C  bne cr6, 0x8243b9ac
	if !ctx.cr[6].eq {
	pc = 0x8243B9AC; continue 'dispatch;
	}
	// 8243B994: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243B998: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243B99C: 4BFFFD85  bl 0x8243b720
	ctx.lr = 0x8243B9A0;
	sub_8243B720(ctx, base);
	// 8243B9A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243B9A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243B9A8: 4BFFFD79  bl 0x8243b720
	ctx.lr = 0x8243B9AC;
	sub_8243B720(ctx, base);
	// 8243B9AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243B9B0: 480F975C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243B9B8 size=128
    let mut pc: u32 = 0x8243B9B8;
    'dispatch: loop {
        match pc {
            0x8243B9B8 => {
    //   block [0x8243B9B8..0x8243BA38)
	// 8243B9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243B9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243B9C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243B9C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243B9C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243B9CC: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243B9D0: 48000C79  bl 0x8243c648
	ctx.lr = 0x8243B9D4;
	sub_8243C648(ctx, base);
	// 8243B9D4: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 8243B9D8: 409A004C  bne cr6, 0x8243ba24
	if !ctx.cr[6].eq {
	pc = 0x8243BA24; continue 'dispatch;
	}
	// 8243B9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B9E0: 4BFF6D09  bl 0x824326e8
	ctx.lr = 0x8243B9E4;
	sub_824326E8(ctx, base);
	// 8243B9E4: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 8243B9E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243B9EC: 409A0010  bne cr6, 0x8243b9fc
	if !ctx.cr[6].eq {
	pc = 0x8243B9FC; continue 'dispatch;
	}
	// 8243B9F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243B9F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243B9F8: 4BFF6F49  bl 0x82432940
	ctx.lr = 0x8243B9FC;
	sub_82432940(ctx, base);
	// 8243B9FC: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8243BA00: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8243BA04: 409A0020  bne cr6, 0x8243ba24
	if !ctx.cr[6].eq {
	pc = 0x8243BA24; continue 'dispatch;
	}
	// 8243BA08: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243BA0C: 48008855  bl 0x82444260
	ctx.lr = 0x8243BA10;
	sub_82444260(ctx, base);
	// 8243BA10: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243BA14: 419A0010  beq cr6, 0x8243ba24
	if ctx.cr[6].eq {
	pc = 0x8243BA24; continue 'dispatch;
	}
	// 8243BA18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243BA1C: 386B5F4C  addi r3, r11, 0x5f4c
	ctx.r[3].s64 = ctx.r[11].s64 + 24396;
	// 8243BA20: 4BFFB6A9  bl 0x824370c8
	ctx.lr = 0x8243BA24;
	sub_824370C8(ctx, base);
	// 8243BA24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BA30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BA38 size=236
    let mut pc: u32 = 0x8243BA38;
    'dispatch: loop {
        match pc {
            0x8243BA38 => {
    //   block [0x8243BA38..0x8243BB24)
	// 8243BA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BA40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BA44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BA48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BA4C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243BA50: 4BFFF9C9  bl 0x8243b418
	ctx.lr = 0x8243BA54;
	sub_8243B418(ctx, base);
	// 8243BA54: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8243BA58: 409A001C  bne cr6, 0x8243ba74
	if !ctx.cr[6].eq {
	pc = 0x8243BA74; continue 'dispatch;
	}
	// 8243BA5C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243BA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BA70: 4E800020  blr
	return;
	// 8243BA74: 817F05AC  lwz r11, 0x5ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8243BA78: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243BA7C: 419A0090  beq cr6, 0x8243bb0c
	if ctx.cr[6].eq {
	pc = 0x8243BB0C; continue 'dispatch;
	}
	// 8243BA80: 807F0450  lwz r3, 0x450(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1104 as u32) ) } as u64;
	// 8243BA84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243BA88: 419A0014  beq cr6, 0x8243ba9c
	if ctx.cr[6].eq {
	pc = 0x8243BA9C; continue 'dispatch;
	}
	// 8243BA8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243BA90: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243BA94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243BA98: 4E800421  bctrl
	ctx.lr = 0x8243BA9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243BA9C: 80FF044C  lwz r7, 0x44c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1100 as u32) ) } as u64;
	// 8243BAA0: 80DF0448  lwz r6, 0x448(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1096 as u32) ) } as u64;
	// 8243BAA4: 80BF0444  lwz r5, 0x444(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1092 as u32) ) } as u64;
	// 8243BAA8: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 8243BAAC: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243BAB0: 4BFFF8D1  bl 0x8243b380
	ctx.lr = 0x8243BAB4;
	sub_8243B380(ctx, base);
	// 8243BAB4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243BAB8: 4BFFF921  bl 0x8243b3d8
	ctx.lr = 0x8243BABC;
	sub_8243B3D8(ctx, base);
	// 8243BABC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8243BAC0: 409A0044  bne cr6, 0x8243bb04
	if !ctx.cr[6].eq {
	pc = 0x8243BB04; continue 'dispatch;
	}
	// 8243BAC4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8243BAC8: 3860FF9A  li r3, -0x66
	ctx.r[3].s64 = -102;
	// 8243BACC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243BAD0: 4BFFAEF9  bl 0x824369c8
	ctx.lr = 0x8243BAD4;
	sub_824369C8(ctx, base);
	// 8243BAD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243BAD8: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 8243BADC: 386B5F78  addi r3, r11, 0x5f78
	ctx.r[3].s64 = ctx.r[11].s64 + 24440;
	// 8243BAE0: 4BFFB5E9  bl 0x824370c8
	ctx.lr = 0x8243BAE4;
	sub_824370C8(ctx, base);
	// 8243BAE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243BAE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243BAEC: 917F0440  stw r11, 0x440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1088 as u32), ctx.r[11].u32 ) };
	// 8243BAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BAFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BB00: 4E800020  blr
	return;
	// 8243BB04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243BB08: 4BFF9529  bl 0x82435030
	ctx.lr = 0x8243BB0C;
	sub_82435030(ctx, base);
	// 8243BB0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243BB10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BB1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BB28 size=152
    let mut pc: u32 = 0x8243BB28;
    'dispatch: loop {
        match pc {
            0x8243BB28 => {
    //   block [0x8243BB28..0x8243BBC0)
	// 8243BB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BB2C: 480F958D  bl 0x825350b8
	ctx.lr = 0x8243BB30;
	sub_82535080(ctx, base);
	// 8243BB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BB34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BB38: 817F05AC  lwz r11, 0x5ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8243BB3C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243BB40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243BB44: 409A0074  bne cr6, 0x8243bbb8
	if !ctx.cr[6].eq {
	pc = 0x8243BBB8; continue 'dispatch;
	}
	// 8243BB48: 48000B01  bl 0x8243c648
	ctx.lr = 0x8243BB4C;
	sub_8243C648(ctx, base);
	// 8243BB4C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8243BB50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243BB54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243BB58: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8243BB5C: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 8243BB60: 4BFFF8E9  bl 0x8243b448
	ctx.lr = 0x8243BB64;
	sub_8243B448(ctx, base);
	// 8243BB64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243BB68: 419A0010  beq cr6, 0x8243bb78
	if ctx.cr[6].eq {
	pc = 0x8243BB78; continue 'dispatch;
	}
	// 8243BB6C: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 8243BB70: 4BFFFB41  bl 0x8243b6b0
	ctx.lr = 0x8243BB74;
	sub_8243B6B0(ctx, base);
	// 8243BB74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243BB78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243BB7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243BB80: 4BFFF8C9  bl 0x8243b448
	ctx.lr = 0x8243BB84;
	sub_8243B448(ctx, base);
	// 8243BB84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243BB88: 419A0010  beq cr6, 0x8243bb98
	if ctx.cr[6].eq {
	pc = 0x8243BB98; continue 'dispatch;
	}
	// 8243BB8C: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 8243BB90: 4BFFFB21  bl 0x8243b6b0
	ctx.lr = 0x8243BB94;
	sub_8243B6B0(ctx, base);
	// 8243BB94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243BB98: 2F1C0006  cmpwi cr6, r28, 6
	ctx.cr[6].compare_i32(ctx.r[28].s32, 6, &mut ctx.xer);
	// 8243BB9C: 409A001C  bne cr6, 0x8243bbb8
	if !ctx.cr[6].eq {
	pc = 0x8243BBB8; continue 'dispatch;
	}
	// 8243BBA0: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 8243BBA4: 419A0014  beq cr6, 0x8243bbb8
	if ctx.cr[6].eq {
	pc = 0x8243BBB8; continue 'dispatch;
	}
	// 8243BBA8: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8243BBAC: 419A000C  beq cr6, 0x8243bbb8
	if ctx.cr[6].eq {
	pc = 0x8243BBB8; continue 'dispatch;
	}
	// 8243BBB0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8243BBB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243BBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243BBBC: 480F954C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BBC0 size=108
    let mut pc: u32 = 0x8243BBC0;
    'dispatch: loop {
        match pc {
            0x8243BBC0 => {
    //   block [0x8243BBC0..0x8243BC2C)
	// 8243BBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BBC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243BBCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BBD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BBD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BBD8: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8243BBDC: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243BBE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243BBE4: 419A0014  beq cr6, 0x8243bbf8
	if ctx.cr[6].eq {
	pc = 0x8243BBF8; continue 'dispatch;
	}
	// 8243BBE8: 4BFFF741  bl 0x8243b328
	ctx.lr = 0x8243BBEC;
	sub_8243B328(ctx, base);
	// 8243BBEC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243BBF0: 419A0008  beq cr6, 0x8243bbf8
	if ctx.cr[6].eq {
	pc = 0x8243BBF8; continue 'dispatch;
	}
	// 8243BBF4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8243BBF8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243BBFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243BC00: 419A0014  beq cr6, 0x8243bc14
	if ctx.cr[6].eq {
	pc = 0x8243BC14; continue 'dispatch;
	}
	// 8243BC04: 4BFF7AFD  bl 0x82433700
	ctx.lr = 0x8243BC08;
	sub_82433700(ctx, base);
	// 8243BC08: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243BC0C: 409A0008  bne cr6, 0x8243bc14
	if !ctx.cr[6].eq {
	pc = 0x8243BC14; continue 'dispatch;
	}
	// 8243BC10: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8243BC14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243BC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BC20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243BC24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BC30 size=152
    let mut pc: u32 = 0x8243BC30;
    'dispatch: loop {
        match pc {
            0x8243BC30 => {
    //   block [0x8243BC30..0x8243BCC8)
	// 8243BC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BC38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BC3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BC40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BC44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243BC48: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243BC4C: 419A000C  beq cr6, 0x8243bc58
	if ctx.cr[6].eq {
	pc = 0x8243BC58; continue 'dispatch;
	}
	// 8243BC50: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243BC54: 409A0060  bne cr6, 0x8243bcb4
	if !ctx.cr[6].eq {
	pc = 0x8243BCB4; continue 'dispatch;
	}
	// 8243BC58: 897F0081  lbz r11, 0x81(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(129 as u32) ) } as u64;
	// 8243BC5C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8243BC60: 409A0024  bne cr6, 0x8243bc84
	if !ctx.cr[6].eq {
	pc = 0x8243BC84; continue 'dispatch;
	}
	// 8243BC64: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243BC68: 4BFEBC49  bl 0x824278b0
	ctx.lr = 0x8243BC6C;
	sub_824278B0(ctx, base);
	// 8243BC6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243BC70: 409A0014  bne cr6, 0x8243bc84
	if !ctx.cr[6].eq {
	pc = 0x8243BC84; continue 'dispatch;
	}
	// 8243BC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243BC78: 4BFF6AC9  bl 0x82432740
	ctx.lr = 0x8243BC7C;
	sub_82432740(ctx, base);
	// 8243BC7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243BC80: 997F0081  stb r11, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 8243BC84: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8243BC88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243BC8C: 409A0028  bne cr6, 0x8243bcb4
	if !ctx.cr[6].eq {
	pc = 0x8243BCB4; continue 'dispatch;
	}
	// 8243BC90: 897F0081  lbz r11, 0x81(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(129 as u32) ) } as u64;
	// 8243BC94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243BC98: 409A001C  bne cr6, 0x8243bcb4
	if !ctx.cr[6].eq {
	pc = 0x8243BCB4; continue 'dispatch;
	}
	// 8243BC9C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243BCA0: 4BFEBC11  bl 0x824278b0
	ctx.lr = 0x8243BCA4;
	sub_824278B0(ctx, base);
	// 8243BCA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243BCA8: 409A000C  bne cr6, 0x8243bcb4
	if !ctx.cr[6].eq {
	pc = 0x8243BCB4; continue 'dispatch;
	}
	// 8243BCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243BCB0: 4BFF6E01  bl 0x82432ab0
	ctx.lr = 0x8243BCB4;
	sub_82432AB0(ctx, base);
	// 8243BCB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BCC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BCC8 size=36
    let mut pc: u32 = 0x8243BCC8;
    'dispatch: loop {
        match pc {
            0x8243BCC8 => {
    //   block [0x8243BCC8..0x8243BCEC)
	// 8243BCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BCD4: 4BFFAAB5  bl 0x82436788
	ctx.lr = 0x8243BCD8;
	sub_82436788(ctx, base);
	// 8243BCD8: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243BCDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BCF0 size=140
    let mut pc: u32 = 0x8243BCF0;
    'dispatch: loop {
        match pc {
            0x8243BCF0 => {
    //   block [0x8243BCF0..0x8243BD7C)
	// 8243BCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BCF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243BCFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BD00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BD04: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243BD08: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 8243BD0C: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8243BD10: 812B9F5C  lwz r9, -0x60a4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24740 as u32) ) } as u64;
	// 8243BD14: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8243BD18: 912B9F5C  stw r9, -0x60a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24740 as u32), ctx.r[9].u32 ) };
	// 8243BD1C: 817F9DDC  lwz r11, -0x6224(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 8243BD20: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243BD24: 816A06FC  lwz r11, 0x6fc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1788 as u32) ) } as u64;
	// 8243BD28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243BD2C: 916A06FC  stw r11, 0x6fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1788 as u32), ctx.r[11].u32 ) };
	// 8243BD30: 409A0034  bne cr6, 0x8243bd64
	if !ctx.cr[6].eq {
	pc = 0x8243BD64; continue 'dispatch;
	}
	// 8243BD34: 4BFFAA55  bl 0x82436788
	ctx.lr = 0x8243BD38;
	sub_82436788(ctx, base);
	// 8243BD38: 3BC3005C  addi r30, r3, 0x5c
	ctx.r[30].s64 = ctx.r[3].s64 + 92;
	// 8243BD3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243BD40: 4BFFB381  bl 0x824370c0
	ctx.lr = 0x8243BD44;
	sub_824370C0(ctx, base);
	// 8243BD44: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243BD48: 409A001C  bne cr6, 0x8243bd64
	if !ctx.cr[6].eq {
	pc = 0x8243BD64; continue 'dispatch;
	}
	// 8243BD4C: 817F9DDC  lwz r11, -0x6224(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 8243BD50: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243BD54: 409A0008  bne cr6, 0x8243bd5c
	if !ctx.cr[6].eq {
	pc = 0x8243BD5C; continue 'dispatch;
	}
	// 8243BD58: 4BFFD5B9  bl 0x82439310
	ctx.lr = 0x8243BD5C;
	sub_82439310(ctx, base);
	// 8243BD5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243BD60: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243BD64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243BD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BD70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243BD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BD80 size=56
    let mut pc: u32 = 0x8243BD80;
    'dispatch: loop {
        match pc {
            0x8243BD80 => {
    //   block [0x8243BD80..0x8243BDB8)
	// 8243BD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BD8C: 4BFFA9FD  bl 0x82436788
	ctx.lr = 0x8243BD90;
	sub_82436788(ctx, base);
	// 8243BD90: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8243BD94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243BD98: 419A0010  beq cr6, 0x8243bda8
	if ctx.cr[6].eq {
	pc = 0x8243BDA8; continue 'dispatch;
	}
	// 8243BD9C: 80630044  lwz r3, 0x44(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 8243BDA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243BDA4: 4E800421  bctrl
	ctx.lr = 0x8243BDA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243BDA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BDB8 size=56
    let mut pc: u32 = 0x8243BDB8;
    'dispatch: loop {
        match pc {
            0x8243BDB8 => {
    //   block [0x8243BDB8..0x8243BDF0)
	// 8243BDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BDC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BDC4: 4BFFA9C5  bl 0x82436788
	ctx.lr = 0x8243BDC8;
	sub_82436788(ctx, base);
	// 8243BDC8: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243BDCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243BDD0: 419A0010  beq cr6, 0x8243bde0
	if ctx.cr[6].eq {
	pc = 0x8243BDE0; continue 'dispatch;
	}
	// 8243BDD4: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243BDD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243BDDC: 4E800421  bctrl
	ctx.lr = 0x8243BDE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243BDE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BDF0 size=56
    let mut pc: u32 = 0x8243BDF0;
    'dispatch: loop {
        match pc {
            0x8243BDF0 => {
    //   block [0x8243BDF0..0x8243BE28)
	// 8243BDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BDF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BDFC: 4BFFA98D  bl 0x82436788
	ctx.lr = 0x8243BE00;
	sub_82436788(ctx, base);
	// 8243BE00: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243BE04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243BE08: 419A0010  beq cr6, 0x8243be18
	if ctx.cr[6].eq {
	pc = 0x8243BE18; continue 'dispatch;
	}
	// 8243BE0C: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243BE10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243BE14: 4E800421  bctrl
	ctx.lr = 0x8243BE18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243BE18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BE28 size=48
    let mut pc: u32 = 0x8243BE28;
    'dispatch: loop {
        match pc {
            0x8243BE28 => {
    //   block [0x8243BE28..0x8243BE58)
	// 8243BE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BE30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BE34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BE38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BE3C: 4BFFA94D  bl 0x82436788
	ctx.lr = 0x8243BE40;
	sub_82436788(ctx, base);
	// 8243BE40: 93E30058  stw r31, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8243BE44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BE50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243BE58 size=8
    let mut pc: u32 = 0x8243BE58;
    'dispatch: loop {
        match pc {
            0x8243BE58 => {
    //   block [0x8243BE58..0x8243BE60)
	// 8243BE58: 8063006C  lwz r3, 0x6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8243BE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BE60 size=112
    let mut pc: u32 = 0x8243BE60;
    'dispatch: loop {
        match pc {
            0x8243BE60 => {
    //   block [0x8243BE60..0x8243BED0)
	// 8243BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BE68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BE6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BE70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BE74: 4BFF71ED  bl 0x82433060
	ctx.lr = 0x8243BE78;
	sub_82433060(ctx, base);
	// 8243BE78: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243BE7C: 419A0028  beq cr6, 0x8243bea4
	if ctx.cr[6].eq {
	pc = 0x8243BEA4; continue 'dispatch;
	}
	// 8243BE80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243BE84: 386B5FA4  addi r3, r11, 0x5fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 24484;
	// 8243BE88: 4BFFB241  bl 0x824370c8
	ctx.lr = 0x8243BE8C;
	sub_824370C8(ctx, base);
	// 8243BE8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243BE90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BE9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BEA0: 4E800020  blr
	return;
	// 8243BEA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243BEA8: 4BFFFFB1  bl 0x8243be58
	ctx.lr = 0x8243BEAC;
	sub_8243BE58(ctx, base);
	// 8243BEAC: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8243BEB0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243BEB4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243BEB8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8243BEBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BEC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BED0 size=72
    let mut pc: u32 = 0x8243BED0;
    'dispatch: loop {
        match pc {
            0x8243BED0 => {
    //   block [0x8243BED0..0x8243BF18)
	// 8243BED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243BEDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BEE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BEE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243BEE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8243BEEC: 4BFFA89D  bl 0x82436788
	ctx.lr = 0x8243BEF0;
	sub_82436788(ctx, base);
	// 8243BEF0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243BEF4: 419A0008  beq cr6, 0x8243befc
	if ctx.cr[6].eq {
	pc = 0x8243BEFC; continue 'dispatch;
	}
	// 8243BEF8: 93FE0068  stw r31, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 8243BEFC: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8243BF00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243BF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BF0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243BF10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BF18 size=36
    let mut pc: u32 = 0x8243BF18;
    'dispatch: loop {
        match pc {
            0x8243BF18 => {
    //   block [0x8243BF18..0x8243BF3C)
	// 8243BF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BF24: 4BFFA865  bl 0x82436788
	ctx.lr = 0x8243BF28;
	sub_82436788(ctx, base);
	// 8243BF28: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243BF2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BF40 size=60
    let mut pc: u32 = 0x8243BF40;
    'dispatch: loop {
        match pc {
            0x8243BF40 => {
    //   block [0x8243BF40..0x8243BF7C)
	// 8243BF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BF48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BF4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BF50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243BF54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BF58: 4BFF77F1  bl 0x82433748
	ctx.lr = 0x8243BF5C;
	sub_82433748(ctx, base);
	// 8243BF5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243BF60: 4BFF70B1  bl 0x82433010
	ctx.lr = 0x8243BF64;
	sub_82433010(ctx, base);
	// 8243BF64: 4BFFF47D  bl 0x8243b3e0
	ctx.lr = 0x8243BF68;
	sub_8243B3E0(ctx, base);
	// 8243BF68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BF74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BF80 size=52
    let mut pc: u32 = 0x8243BF80;
    'dispatch: loop {
        match pc {
            0x8243BF80 => {
    //   block [0x8243BF80..0x8243BFB4)
	// 8243BF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BF88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BF8C: 4BFF7085  bl 0x82433010
	ctx.lr = 0x8243BF90;
	sub_82433010(ctx, base);
	// 8243BF90: 4BFFF3D9  bl 0x8243b368
	ctx.lr = 0x8243BF94;
	sub_8243B368(ctx, base);
	// 8243BF94: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8243BF98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243BF9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243BFA0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8243BFA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BFB8 size=48
    let mut pc: u32 = 0x8243BFB8;
    'dispatch: loop {
        match pc {
            0x8243BFB8 => {
    //   block [0x8243BFB8..0x8243BFE8)
	// 8243BFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BFC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243BFC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BFC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243BFCC: 4BFFA7BD  bl 0x82436788
	ctx.lr = 0x8243BFD0;
	sub_82436788(ctx, base);
	// 8243BFD0: 93E32E0C  stw r31, 0x2e0c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11788 as u32), ctx.r[31].u32 ) };
	// 8243BFD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243BFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243BFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243BFE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243BFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243BFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243BFE8 size=36
    let mut pc: u32 = 0x8243BFE8;
    'dispatch: loop {
        match pc {
            0x8243BFE8 => {
    //   block [0x8243BFE8..0x8243C00C)
	// 8243BFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243BFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243BFF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243BFF4: 4BFFA795  bl 0x82436788
	ctx.lr = 0x8243BFF8;
	sub_82436788(ctx, base);
	// 8243BFF8: 80632E0C  lwz r3, 0x2e0c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11788 as u32) ) } as u64;
	// 8243BFFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C010 size=120
    let mut pc: u32 = 0x8243C010;
    'dispatch: loop {
        match pc {
            0x8243C010 => {
    //   block [0x8243C010..0x8243C088)
	// 8243C010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243C01C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C024: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243C028: 816B9DDC  lwz r11, -0x6224(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 8243C02C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C030: 409A0040  bne cr6, 0x8243c070
	if !ctx.cr[6].eq {
	pc = 0x8243C070; continue 'dispatch;
	}
	// 8243C034: 4BFFA755  bl 0x82436788
	ctx.lr = 0x8243C038;
	sub_82436788(ctx, base);
	// 8243C038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C03C: 4BFFFFAD  bl 0x8243bfe8
	ctx.lr = 0x8243C040;
	sub_8243BFE8(ctx, base);
	// 8243C040: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C044: 419A002C  beq cr6, 0x8243c070
	if ctx.cr[6].eq {
	pc = 0x8243C070; continue 'dispatch;
	}
	// 8243C048: 3BFF006C  addi r31, r31, 0x6c
	ctx.r[31].s64 = ctx.r[31].s64 + 108;
	// 8243C04C: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8243C050: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243C054: 419A000C  beq cr6, 0x8243c060
	if ctx.cr[6].eq {
	pc = 0x8243C060; continue 'dispatch;
	}
	// 8243C058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C05C: 4BFF7D05  bl 0x82433d60
	ctx.lr = 0x8243C060;
	sub_82433D60(ctx, base);
	// 8243C060: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8243C064: 3BFF05B4  addi r31, r31, 0x5b4
	ctx.r[31].s64 = ctx.r[31].s64 + 1460;
	// 8243C068: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243C06C: 409AFFE4  bne cr6, 0x8243c050
	if !ctx.cr[6].eq {
	pc = 0x8243C050; continue 'dispatch;
	}
	// 8243C070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C07C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243C080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C088 size=160
    let mut pc: u32 = 0x8243C088;
    'dispatch: loop {
        match pc {
            0x8243C088 => {
    //   block [0x8243C088..0x8243C128)
	// 8243C088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C08C: 480F9031  bl 0x825350bc
	ctx.lr = 0x8243C090;
	sub_82535080(ctx, base);
	// 8243C090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C094: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243C098: 816B9DDC  lwz r11, -0x6224(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 8243C09C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C0A0: 409A0080  bne cr6, 0x8243c120
	if !ctx.cr[6].eq {
	pc = 0x8243C120; continue 'dispatch;
	}
	// 8243C0A4: 4BFFA6E5  bl 0x82436788
	ctx.lr = 0x8243C0A8;
	sub_82436788(ctx, base);
	// 8243C0A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C0AC: 4BFFFF3D  bl 0x8243bfe8
	ctx.lr = 0x8243C0B0;
	sub_8243BFE8(ctx, base);
	// 8243C0B0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C0B4: 419A006C  beq cr6, 0x8243c120
	if ctx.cr[6].eq {
	pc = 0x8243C120; continue 'dispatch;
	}
	// 8243C0B8: 3BFF006C  addi r31, r31, 0x6c
	ctx.r[31].s64 = ctx.r[31].s64 + 108;
	// 8243C0BC: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8243C0C0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8243C0C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243C0C8: 419A0048  beq cr6, 0x8243c110
	if ctx.cr[6].eq {
	pc = 0x8243C110; continue 'dispatch;
	}
	// 8243C0CC: 817F0440  lwz r11, 0x440(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1088 as u32) ) } as u64;
	// 8243C0D0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C0D4: 409A0018  bne cr6, 0x8243c0ec
	if !ctx.cr[6].eq {
	pc = 0x8243C0EC; continue 'dispatch;
	}
	// 8243C0D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C0DC: 4BFFF95D  bl 0x8243ba38
	ctx.lr = 0x8243C0E0;
	sub_8243BA38(ctx, base);
	// 8243C0E0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C0E4: 409A0008  bne cr6, 0x8243c0ec
	if !ctx.cr[6].eq {
	pc = 0x8243C0EC; continue 'dispatch;
	}
	// 8243C0E8: 93BF0440  stw r29, 0x440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1088 as u32), ctx.r[29].u32 ) };
	// 8243C0EC: 817F05AC  lwz r11, 0x5ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8243C0F0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C0F4: 409A001C  bne cr6, 0x8243c110
	if !ctx.cr[6].eq {
	pc = 0x8243C110; continue 'dispatch;
	}
	// 8243C0F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C0FC: 4BFF6BBD  bl 0x82432cb8
	ctx.lr = 0x8243C100;
	sub_82432CB8(ctx, base);
	// 8243C100: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C104: 409A000C  bne cr6, 0x8243c110
	if !ctx.cr[6].eq {
	pc = 0x8243C110; continue 'dispatch;
	}
	// 8243C108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C10C: 4BFF6DCD  bl 0x82432ed8
	ctx.lr = 0x8243C110;
	sub_82432ED8(ctx, base);
	// 8243C110: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8243C114: 3BFF05B4  addi r31, r31, 0x5b4
	ctx.r[31].s64 = ctx.r[31].s64 + 1460;
	// 8243C118: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243C11C: 409AFFA8  bne cr6, 0x8243c0c4
	if !ctx.cr[6].eq {
	pc = 0x8243C0C4; continue 'dispatch;
	}
	// 8243C120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C124: 480F8FE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C128 size=104
    let mut pc: u32 = 0x8243C128;
    'dispatch: loop {
        match pc {
            0x8243C128 => {
    //   block [0x8243C128..0x8243C190)
	// 8243C128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243C134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C13C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243C140: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8243C144: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243C148: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C14C: 4BFFFD85  bl 0x8243bed0
	ctx.lr = 0x8243C150;
	sub_8243BED0(ctx, base);
	// 8243C150: 4BFE0511  bl 0x8241c660
	ctx.lr = 0x8243C154;
	sub_8241C660(ctx, base);
	// 8243C154: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243C158: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C15C: 4BFFFD75  bl 0x8243bed0
	ctx.lr = 0x8243C160;
	sub_8243BED0(ctx, base);
	// 8243C160: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 8243C164: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C168: 409A0010  bne cr6, 0x8243c178
	if !ctx.cr[6].eq {
	pc = 0x8243C178; continue 'dispatch;
	}
	// 8243C16C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8243C170: 2F1F000A  cmpwi cr6, r31, 0xa
	ctx.cr[6].compare_i32(ctx.r[31].s32, 10, &mut ctx.xer);
	// 8243C174: 4198FFD0  blt cr6, 0x8243c144
	if ctx.cr[6].lt {
	pc = 0x8243C144; continue 'dispatch;
	}
	// 8243C178: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C184: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243C188: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C190 size=28
    let mut pc: u32 = 0x8243C190;
    'dispatch: loop {
        match pc {
            0x8243C190 => {
    //   block [0x8243C190..0x8243C1AC)
	// 8243C190: 81630518  lwz r11, 0x518(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1304 as u32) ) } as u64;
	// 8243C194: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C198: 419A0014  beq cr6, 0x8243c1ac
	if ctx.cr[6].eq {
		sub_8243C1AC(ctx, base);
		return;
	}
	// 8243C19C: 8163053C  lwz r11, 0x53c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1340 as u32) ) } as u64;
	// 8243C1A0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C1A4: 419A0008  beq cr6, 0x8243c1ac
	if ctx.cr[6].eq {
		sub_8243C1AC(ctx, base);
		return;
	}
	// 8243C1A8: 4BFFF810  b 0x8243b9b8
	sub_8243B9B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


