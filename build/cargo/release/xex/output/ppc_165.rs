pub fn sub_82DB8C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB8C00 size=148
    let mut pc: u32 = 0x82DB8C00;
    'dispatch: loop {
        match pc {
            0x82DB8C00 => {
    //   block [0x82DB8C00..0x82DB8C94)
	// 82DB8C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB8C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB8C0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB8C14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8C18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB8C1C: 396B09D8  addi r11, r11, 0x9d8
	ctx.r[11].s64 = ctx.r[11].s64 + 2520;
	// 82DB8C20: 394A09B4  addi r10, r10, 0x9b4
	ctx.r[10].s64 = ctx.r[10].s64 + 2484;
	// 82DB8C24: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB8C28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB8C2C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB8C30: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB8C34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB8C38: 419A0030  beq cr6, 0x82db8c68
	if ctx.cr[6].eq {
	pc = 0x82DB8C68; continue 'dispatch;
	}
	// 82DB8C3C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB8C40: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB8C44: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DB8C48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DB8C4C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB8C50: 409A0018  bne cr6, 0x82db8c68
	if !ctx.cr[6].eq {
	pc = 0x82DB8C68; continue 'dispatch;
	}
	// 82DB8C54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8C58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DB8C5C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8C60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8C64: 4E800421  bctrl
	ctx.lr = 0x82DB8C68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB8C68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8C6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DB8C70: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB8C74: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DB8C78: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB8C7C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB8C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB8C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB8C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB8C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB8C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DB8C98 size=32
    let mut pc: u32 = 0x82DB8C98;
    'dispatch: loop {
        match pc {
            0x82DB8C98 => {
    //   block [0x82DB8C98..0x82DB8CB8)
	// 82DB8C98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DB8C9C: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB8CA0: C00B001C  lfs f0, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB8CA4: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DB8CA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8CAC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB8CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8CB4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB8CB8 size=612
    let mut pc: u32 = 0x82DB8CB8;
    'dispatch: loop {
        match pc {
            0x82DB8CB8 => {
    //   block [0x82DB8CB8..0x82DB8F1C)
	// 82DB8CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8CBC: 4BEF0741  bl 0x82ca93fc
	ctx.lr = 0x82DB8CC0;
	sub_82CA93D0(ctx, base);
	// 82DB8CC0: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 82DB8CC4: 4BEF5015  bl 0x82cadcd8
	ctx.lr = 0x82DB8CC8;
	sub_82CADCA0(ctx, base);
	// 82DB8CC8: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 82DB8CCC: 4824DD09  bl 0x830069d4
	ctx.lr = 0x82DB8CD0;
	sub_83006760(ctx, base);
	// 82DB8CD0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8CD4: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82DB8CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB8CDC: 573DFC7E  rlwinm r29, r25, 0x1f, 0x11, 0x1f
	ctx.r[29].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	// 82DB8CE0: 573C843E  srwi r28, r25, 0x10
	ctx.r[28].u32 = ctx.r[25].u32.wrapping_shr(16);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DB8CE4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DB8CE8: 419A0058  beq cr6, 0x82db8d40
	if ctx.cr[6].eq {
	pc = 0x82DB8D40; continue 'dispatch;
	}
	// 82DB8CEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB8CF0: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB8CF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DB8CF8: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DB8CFC: 392A4CA4  addi r9, r10, 0x4ca4
	ctx.r[9].s64 = ctx.r[10].s64 + 19620;
	// 82DB8D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DB8D04: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82DB8D08: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82DB8D0C: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DB8D10: B1050006  sth r8, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DB8D14: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB8D18: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DB8D1C: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DB8D20: 90E5000C  stw r7, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB8D24: B1450014  sth r10, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82DB8D28: 98C50016  stb r6, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[6].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8F20 size=12
    let mut pc: u32 = 0x82DB8F20;
    'dispatch: loop {
        match pc {
            0x82DB8F20 => {
    //   block [0x82DB8F20..0x82DB8F2C)
	// 82DB8F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DB8F24: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DB8F28: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8F2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8F2C size=8
    let mut pc: u32 = 0x82DB8F2C;
    'dispatch: loop {
        match pc {
            0x82DB8F2C => {
    //   block [0x82DB8F2C..0x82DB8F34)
	// 82DB8F2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DB8F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB8F38 size=96
    let mut pc: u32 = 0x82DB8F38;
    'dispatch: loop {
        match pc {
            0x82DB8F38 => {
    //   block [0x82DB8F38..0x82DB8F98)
	// 82DB8F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB8F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB8F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8F48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DB8F4C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8F50: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DB8F54: 388B2B50  addi r4, r11, 0x2b50
	ctx.r[4].s64 = ctx.r[11].s64 + 11088;
	// 82DB8F58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DB8F5C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB8F64: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB8F68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8F6C: 4E800421  bctrl
	ctx.lr = 0x82DB8F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB8F70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8F74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB8F78: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB8F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB8F80: 4E800421  bctrl
	ctx.lr = 0x82DB8F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB8F84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB8F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB8F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB8F90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB8F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB8F98 size=64
    let mut pc: u32 = 0x82DB8F98;
    'dispatch: loop {
        match pc {
            0x82DB8F98 => {
    //   block [0x82DB8F98..0x82DB8FD8)
	// 82DB8F98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DB8F9C: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82DB8FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB8FA4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB8FA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DB8FAC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DB8FB0: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DB8FB4: 394A0218  addi r10, r10, 0x218
	ctx.r[10].s64 = ctx.r[10].s64 + 536;
	// 82DB8FB8: 392901F4  addi r9, r9, 0x1f4
	ctx.r[9].s64 = ctx.r[9].s64 + 500;
	// 82DB8FBC: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DB8FC0: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DB8FC4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DB8FC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DB8FCC: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DB8FD0: 99030014  stb r8, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82DB8FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB8FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB8FD8 size=560
    let mut pc: u32 = 0x82DB8FD8;
    'dispatch: loop {
        match pc {
            0x82DB8FD8 => {
    //   block [0x82DB8FD8..0x82DB9208)
	// 82DB8FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB8FDC: 4BEF0415  bl 0x82ca93f0
	ctx.lr = 0x82DB8FE0;
	sub_82CA93D0(ctx, base);
	// 82DB8FE0: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB8FE4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB8FE8: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DB8FEC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DB8FF0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DB8FF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DB8FF8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DB8FFC: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB9000: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9004: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9008: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB900C: 40980020  bge cr6, 0x82db902c
	if !ctx.cr[6].lt {
	pc = 0x82DB902C; continue 'dispatch;
	}
	// 82DB9010: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB9014: 39292B5C  addi r9, r9, 0x2b5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11100;
	// 82DB9018: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB901C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB9020: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DB9024: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB9028: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB902C: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB9030: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 82DB9034: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 82DB9038: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB903C: 7EFABB78  mr r26, r23
	ctx.r[26].u64 = ctx.r[23].u64;
	// 82DB9040: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9044: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DB9048: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB904C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB9050: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9054: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB9058: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB905C: 409A0088  bne cr6, 0x82db90e4
	if !ctx.cr[6].eq {
	pc = 0x82DB90E4; continue 'dispatch;
	}
	// 82DB9060: 4E800421  bctrl
	ctx.lr = 0x82DB9064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9068: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB906C: 419A012C  beq cr6, 0x82db9198
	if ctx.cr[6].eq {
	pc = 0x82DB9198; continue 'dispatch;
	}
	// 82DB9070: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9074: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB9078: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB907C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9080: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9084: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9088: 4E800421  bctrl
	ctx.lr = 0x82DB908C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB908C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DB9090: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DB9094: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB9098: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82DB909C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB90A0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB90A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB90A8: 4E800421  bctrl
	ctx.lr = 0x82DB90AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB90AC: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82DB90B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB90B4: 419A0008  beq cr6, 0x82db90bc
	if ctx.cr[6].eq {
	pc = 0x82DB90BC; continue 'dispatch;
	}
	// 82DB90B8: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DB90BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB90C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB90C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB90C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB90CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB90D0: 4E800421  bctrl
	ctx.lr = 0x82DB90D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB90D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB90D8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB90DC: 409AFF94  bne cr6, 0x82db9070
	if !ctx.cr[6].eq {
	pc = 0x82DB9070; continue 'dispatch;
	}
	// 82DB90E0: 480000B8  b 0x82db9198
	pc = 0x82DB9198; continue 'dispatch;
	// 82DB90E4: 4E800421  bctrl
	ctx.lr = 0x82DB90E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB90E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB90EC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB90F0: 419A00A8  beq cr6, 0x82db9198
	if ctx.cr[6].eq {
	pc = 0x82DB9198; continue 'dispatch;
	}
	// 82DB90F4: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB90F8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DB90FC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82DB9100: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DB9104: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB9108: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82DB910C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9110: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9118: 4E800421  bctrl
	ctx.lr = 0x82DB911C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB911C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB9124: 419A0050  beq cr6, 0x82db9174
	if ctx.cr[6].eq {
	pc = 0x82DB9174; continue 'dispatch;
	}
	// 82DB9128: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB912C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB9130: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9134: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9138: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB913C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9140: 4E800421  bctrl
	ctx.lr = 0x82DB9144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9144: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DB9148: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DB914C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB9150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB9154: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9158: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB915C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9160: 4E800421  bctrl
	ctx.lr = 0x82DB9164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9164: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DB9168: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB916C: 419A0008  beq cr6, 0x82db9174
	if ctx.cr[6].eq {
	pc = 0x82DB9174; continue 'dispatch;
	}
	// 82DB9170: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DB9174: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9178: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB917C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9180: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9188: 4E800421  bctrl
	ctx.lr = 0x82DB918C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB918C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9190: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9194: 409AFF60  bne cr6, 0x82db90f4
	if !ctx.cr[6].eq {
	pc = 0x82DB90F4; continue 'dispatch;
	}
	// 82DB9198: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DB919C: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 82DB91A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DB91A4: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DB91A8: 419A0010  beq cr6, 0x82db91b8
	if ctx.cr[6].eq {
	pc = 0x82DB91B8; continue 'dispatch;
	}
	// 82DB91AC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DB91B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DB91B4: 7F4BE12E  stwx r26, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u32) };
	// 82DB91B8: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB91BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB91C0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB91C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB91C8: 40980020  bge cr6, 0x82db91e8
	if !ctx.cr[6].lt {
	pc = 0x82DB91E8; continue 'dispatch;
	}
	// 82DB91CC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB91D0: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB91D4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB91D8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB91DC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB91E0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB91E4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB91E8: 7D7AB850  subf r11, r26, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[26].s64;
	// 82DB91EC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DB91F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DB91F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DB91F8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DB91FC: 99760000  stb r11, 0(r22)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DB9200: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DB9204: 4BEF023C  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB9208 size=368
    let mut pc: u32 = 0x82DB9208;
    'dispatch: loop {
        match pc {
            0x82DB9208 => {
    //   block [0x82DB9208..0x82DB9378)
	// 82DB9208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB920C: 4BEF01F1  bl 0x82ca93fc
	ctx.lr = 0x82DB9210;
	sub_82CA93D0(ctx, base);
	// 82DB9210: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82DB9214: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB9218: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB921C: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DB9220: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DB9224: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DB9228: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DB922C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DB9230: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DB9234: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9238: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB923C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DB9240: 40980020  bge cr6, 0x82db9260
	if !ctx.cr[6].lt {
	pc = 0x82DB9260; continue 'dispatch;
	}
	// 82DB9244: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DB9248: 39082B6C  addi r8, r8, 0x2b6c
	ctx.r[8].s64 = ctx.r[8].s64 + 11116;
	// 82DB924C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DB9250: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DB9254: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 82DB9258: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB925C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DB9260: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB9264: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 82DB9268: 3BC90010  addi r30, r9, 0x10
	ctx.r[30].s64 = ctx.r[9].s64 + 16;
	// 82DB926C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9270: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DB9274: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB9278: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DB9378 size=276
    let mut pc: u32 = 0x82DB9378;
    'dispatch: loop {
        match pc {
            0x82DB9378 => {
    //   block [0x82DB9378..0x82DB948C)
	// 82DB9378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB937C: 4BEF0089  bl 0x82ca9404
	ctx.lr = 0x82DB9380;
	sub_82CA93D0(ctx, base);
	// 82DB9380: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DB9384: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB9388: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB938C: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DB9390: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DB9394: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DB9398: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB939C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB93A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB93A4: 40980020  bge cr6, 0x82db93c4
	if !ctx.cr[6].lt {
	pc = 0x82DB93C4; continue 'dispatch;
	}
	// 82DB93A8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB93AC: 39292B8C  addi r9, r9, 0x2b8c
	ctx.r[9].s64 = ctx.r[9].s64 + 11148;
	// 82DB93B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB93B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB93B8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB93BC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB93C0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB93C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB93C8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB93CC: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 82DB93D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB93D4: C3EB0BE4  lfs f31, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DB93D8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB93DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB93E0: 4E800421  bctrl
	ctx.lr = 0x82DB93E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB93E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB93E8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB93EC: 419A0060  beq cr6, 0x82db944c
	if ctx.cr[6].eq {
	pc = 0x82DB944C; continue 'dispatch;
	}
	// 82DB93F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB93F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DB93F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB93FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9400: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9408: 4E800421  bctrl
	ctx.lr = 0x82DB940C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB940C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9410: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB9414: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB941C: 4E800421  bctrl
	ctx.lr = 0x82DB9420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9420: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DB9424: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9428: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB942C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9430: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9434: FFE00FEE  fsel f31, f0, f31, f1
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[1].f64 };
	// 82DB9438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB943C: 4E800421  bctrl
	ctx.lr = 0x82DB9440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9444: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9448: 409AFFA8  bne cr6, 0x82db93f0
	if !ctx.cr[6].eq {
	pc = 0x82DB93F0; continue 'dispatch;
	}
	// 82DB944C: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DB9450: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9454: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9458: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB945C: 40980020  bge cr6, 0x82db947c
	if !ctx.cr[6].lt {
	pc = 0x82DB947C; continue 'dispatch;
	}
	// 82DB9460: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB9464: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB9468: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB946C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB9470: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB9474: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB9478: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB947C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DB9480: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82DB9484: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DB9488: 4BEEFFCC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB9490 size=484
    let mut pc: u32 = 0x82DB9490;
    'dispatch: loop {
        match pc {
            0x82DB9490 => {
    //   block [0x82DB9490..0x82DB9674)
	// 82DB9490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB9494: 4BEEFF65  bl 0x82ca93f8
	ctx.lr = 0x82DB9498;
	sub_82CA93D0(ctx, base);
	// 82DB9498: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB949C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB94A0: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DB94A4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DB94A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DB94AC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DB94B0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DB94B4: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB94B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB94BC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB94C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB94C4: 40980020  bge cr6, 0x82db94e4
	if !ctx.cr[6].lt {
	pc = 0x82DB94E4; continue 'dispatch;
	}
	// 82DB94C8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DB94CC: 39292B5C  addi r9, r9, 0x2b5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11100;
	// 82DB94D0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB94D4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB94D8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DB94DC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB94E0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB94E4: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB94E8: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 82DB94EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB94F0: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB94F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB94F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB94FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9500: 409A0088  bne cr6, 0x82db9588
	if !ctx.cr[6].eq {
	pc = 0x82DB9588; continue 'dispatch;
	}
	// 82DB9504: 4E800421  bctrl
	ctx.lr = 0x82DB9508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB950C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9510: 419A012C  beq cr6, 0x82db963c
	if ctx.cr[6].eq {
	pc = 0x82DB963C; continue 'dispatch;
	}
	// 82DB9514: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9518: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DB951C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9524: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB952C: 4E800421  bctrl
	ctx.lr = 0x82DB9530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9530: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB9534: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82DB9538: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DB953C: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82DB9540: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DB9544: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DB9548: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB954C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DB9550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9554: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9558: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB955C: 4E800421  bctrl
	ctx.lr = 0x82DB9560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9560: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9564: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB956C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9574: 4E800421  bctrl
	ctx.lr = 0x82DB9578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB957C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9580: 409AFF94  bne cr6, 0x82db9514
	if !ctx.cr[6].eq {
	pc = 0x82DB9514; continue 'dispatch;
	}
	// 82DB9584: 480000B8  b 0x82db963c
	pc = 0x82DB963C; continue 'dispatch;
	// 82DB9588: 4E800421  bctrl
	ctx.lr = 0x82DB958C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB958C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9590: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9594: 419A00A8  beq cr6, 0x82db963c
	if ctx.cr[6].eq {
	pc = 0x82DB963C; continue 'dispatch;
	}
	// 82DB9598: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DB959C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82DB95A0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82DB95A4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DB95A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DB95AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DB95B0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB95BC: 4E800421  bctrl
	ctx.lr = 0x82DB95C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB95C0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB95C8: 419A0050  beq cr6, 0x82db9618
	if ctx.cr[6].eq {
	pc = 0x82DB9618; continue 'dispatch;
	}
	// 82DB95CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB95D0: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DB95D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB95D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB95DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB95E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB95E4: 4E800421  bctrl
	ctx.lr = 0x82DB95E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB95E8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DB95EC: 9381007C  stw r28, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 82DB95F0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DB95F4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82DB95F8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DB95FC: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 82DB9600: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DB9604: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82DB9608: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB960C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB9614: 4E800421  bctrl
	ctx.lr = 0x82DB9618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9618: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB961C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DB9620: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DB9624: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB962C: 4E800421  bctrl
	ctx.lr = 0x82DB9630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB9630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB9634: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DB9638: 409AFF60  bne cr6, 0x82db9598
	if !ctx.cr[6].eq {
	pc = 0x82DB9598; continue 'dispatch;
	}
	// 82DB963C: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DB9640: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB9644: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DB9648: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DB964C: 40980020  bge cr6, 0x82db966c
	if !ctx.cr[6].lt {
	pc = 0x82DB966C; continue 'dispatch;
	}
	// 82DB9650: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DB9654: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DB9658: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DB965C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DB9660: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DB9664: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DB9668: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DB966C: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82DB9670: 4BEEFDD8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB9678 size=24
    let mut pc: u32 = 0x82DB9678;
    'dispatch: loop {
        match pc {
            0x82DB9678 => {
    //   block [0x82DB9678..0x82DB9690)
	// 82DB9678: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB967C: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB9680: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9684: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DB9688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB968C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB9690 size=20
    let mut pc: u32 = 0x82DB9690;
    'dispatch: loop {
        match pc {
            0x82DB9690 => {
    //   block [0x82DB9690..0x82DB96A4)
	// 82DB9690: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB9694: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB9698: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DB969C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB96A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB96A8 size=68
    let mut pc: u32 = 0x82DB96A8;
    'dispatch: loop {
        match pc {
            0x82DB96A8 => {
    //   block [0x82DB96A8..0x82DB96EC)
	// 82DB96A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB96AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB96B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DB96B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB96B8: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DB96BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DB96C0: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DB96C4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DB96C8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DB96CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DB96D0: 4E800421  bctrl
	ctx.lr = 0x82DB96D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DB96D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DB96D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DB96DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DB96E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DB96E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DB96E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB96F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB96F0 size=204
    let mut pc: u32 = 0x82DB96F0;
    'dispatch: loop {
        match pc {
            0x82DB96F0 => {
    //   block [0x82DB96F0..0x82DB97BC)
	// 82DB96F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB96F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DB96F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB96FC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB97C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DB97C0 size=84
    let mut pc: u32 = 0x82DB97C0;
    'dispatch: loop {
        match pc {
            0x82DB97C0 => {
    //   block [0x82DB97C0..0x82DB9814)
	// 82DB97C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DB97C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DB97C8: 396B27DC  addi r11, r11, 0x27dc
	ctx.r[11].s64 = ctx.r[11].s64 + 10204;
	// 82DB97CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DB97D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DB97D4: 38E00015  li r7, 0x15
	ctx.r[7].s64 = 21;
	// 82DB97D8: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DB97DC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DB97E0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DB97E4: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DB97E8: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DB97EC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DB97F0: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DB97F4: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DB97F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DB97FC: 419A0010  beq cr6, 0x82db980c
	if ctx.cr[6].eq {
	pc = 0x82DB980C; continue 'dispatch;
	}
	// 82DB9800: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DB9804: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DB9808: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DB980C: 98A30018  stb r5, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u8 ) };
	// 82DB9810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB9818 size=960
    let mut pc: u32 = 0x82DB9818;
    'dispatch: loop {
        match pc {
            0x82DB9818 => {
    //   block [0x82DB9818..0x82DB9BD8)
	// 82DB9818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB981C: 4BEEFBC1  bl 0x82ca93dc
	ctx.lr = 0x82DB9820;
	sub_82CA93D0(ctx, base);
	// 82DB9820: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82DB9824: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82DB9828: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB982C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB9830: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DB9BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DB9BD8 size=1136
    let mut pc: u32 = 0x82DB9BD8;
    'dispatch: loop {
        match pc {
            0x82DB9BD8 => {
    //   block [0x82DB9BD8..0x82DBA048)
	// 82DB9BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DB9BDC: 4BEEF805  bl 0x82ca93e0
	ctx.lr = 0x82DB9BE0;
	sub_82CA93D0(ctx, base);
	// 82DB9BE0: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82DB9BE4: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82DB9BE8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DB9BEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DB9BF0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBA048 size=96
    let mut pc: u32 = 0x82DBA048;
    'dispatch: loop {
        match pc {
            0x82DBA048 => {
    //   block [0x82DBA048..0x82DBA0A8)
	// 82DBA048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBA04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBA050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBA054: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBA058: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBA05C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBA060: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DBA064: 388B2BD0  addi r4, r11, 0x2bd0
	ctx.r[4].s64 = ctx.r[11].s64 + 11216;
	// 82DBA068: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBA06C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBA074: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBA078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBA07C: 4E800421  bctrl
	ctx.lr = 0x82DBA080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBA080: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBA088: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBA08C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBA090: 4E800421  bctrl
	ctx.lr = 0x82DBA094;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBA094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBA098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBA09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBA0A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBA0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA0A8 size=28
    let mut pc: u32 = 0x82DBA0A8;
    'dispatch: loop {
        match pc {
            0x82DBA0A8 => {
    //   block [0x82DBA0A8..0x82DBA0C4)
	// 82DBA0A8: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBA0AC: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DBA0B0: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82DBA0B4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DBA0B8: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBA0BC: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBA0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA0C8 size=20
    let mut pc: u32 = 0x82DBA0C8;
    'dispatch: loop {
        match pc {
            0x82DBA0C8 => {
    //   block [0x82DBA0C8..0x82DBA0DC)
	// 82DBA0C8: 89630017  lbz r11, 0x17(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(23 as u32) ) } as u64;
	// 82DBA0CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBA0D0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBA0D4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBA0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA0E0 size=16
    let mut pc: u32 = 0x82DBA0E0;
    'dispatch: loop {
        match pc {
            0x82DBA0E0 => {
    //   block [0x82DBA0E0..0x82DBA0F0)
	// 82DBA0E0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBA0F0 size=72
    let mut pc: u32 = 0x82DBA0F0;
    'dispatch: loop {
        match pc {
            0x82DBA0F0 => {
    //   block [0x82DBA0F0..0x82DBA138)
	// 82DBA0F0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBA0F4: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82DBA0F8: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82DBA0FC: C00A0CA8  lfs f0, 0xca8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3240 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBA100: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA138 size=108
    let mut pc: u32 = 0x82DBA138;
    'dispatch: loop {
        match pc {
            0x82DBA138 => {
    //   block [0x82DBA138..0x82DBA1A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA1A8 size=224
    let mut pc: u32 = 0x82DBA1A8;
    'dispatch: loop {
        match pc {
            0x82DBA1A8 => {
    //   block [0x82DBA1A8..0x82DBA288)
	// 82DBA1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBA1AC: 4BEEF261  bl 0x82ca940c
	ctx.lr = 0x82DBA1B0;
	sub_82CA93D0(ctx, base);
	// 82DBA1B0: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA288 size=92
    let mut pc: u32 = 0x82DBA288;
    'dispatch: loop {
        match pc {
            0x82DBA288 => {
    //   block [0x82DBA288..0x82DBA2E4)
	// 82DBA288: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBA28C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBA290: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82DBA294: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 82DBA298: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82DBA29C: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA2E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA2E4 size=88
    let mut pc: u32 = 0x82DBA2E4;
    'dispatch: loop {
        match pc {
            0x82DBA2E4 => {
    //   block [0x82DBA2E4..0x82DBA33C)
	// 82DBA2E4: 394B0050  addi r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA340 size=96
    let mut pc: u32 = 0x82DBA340;
    'dispatch: loop {
        match pc {
            0x82DBA340 => {
    //   block [0x82DBA340..0x82DBA3A0)
	// 82DBA340: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBA3A0 size=112
    let mut pc: u32 = 0x82DBA3A0;
    'dispatch: loop {
        match pc {
            0x82DBA3A0 => {
    //   block [0x82DBA3A0..0x82DBA410)
	// 82DBA3A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DBA3A4: FC006890  fmr f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DBA3A8: C1A1FFF0  lfs f13, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBA3AC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DBA3B0: 40990008  ble cr6, 0x82dba3b8
	if !ctx.cr[6].gt {
	pc = 0x82DBA3B8; continue 'dispatch;
	}
	// 82DBA3B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBA3B8: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82DBA3BC: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82DBA3C0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBA3C4: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA410 size=12
    let mut pc: u32 = 0x82DBA410;
    'dispatch: loop {
        match pc {
            0x82DBA410 => {
    //   block [0x82DBA410..0x82DBA41C)
	// 82DBA410: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82DBA414: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBA418: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA41C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA41C size=112
    let mut pc: u32 = 0x82DBA41C;
    'dispatch: loop {
        match pc {
            0x82DBA41C => {
    //   block [0x82DBA41C..0x82DBA48C)
	// 82DBA41C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DBA420: 38E30050  addi r7, r3, 0x50
	ctx.r[7].s64 = ctx.r[3].s64 + 80;
	// 82DBA424: 392AB340  addi r9, r10, -0x4cc0
	ctx.r[9].s64 = ctx.r[10].s64 + -19648;
	// 82DBA428: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DBA42C: 390AB36C  addi r8, r10, -0x4c94
	ctx.r[8].s64 = ctx.r[10].s64 + -19604;
	// 82DBA430: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA434: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBA438: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DBA43C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBA440: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DBA444: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DBA448: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA490 size=1272
    let mut pc: u32 = 0x82DBA490;
    'dispatch: loop {
        match pc {
            0x82DBA490 => {
    //   block [0x82DBA490..0x82DBA988)
	// 82DBA490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBA494: 4BEEEF5D  bl 0x82ca93f0
	ctx.lr = 0x82DBA498;
	sub_82CA93D0(ctx, base);
	// 82DBA498: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82DBA49C: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBA988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBA988 size=724
    let mut pc: u32 = 0x82DBA988;
    'dispatch: loop {
        match pc {
            0x82DBA988 => {
    //   block [0x82DBA988..0x82DBAC5C)
	// 82DBA988: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DBA98C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DBA990: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA994: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DBA998: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBA99C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBA9A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBA9A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBA9A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBA9AC: 40980020  bge cr6, 0x82dba9cc
	if !ctx.cr[6].lt {
	pc = 0x82DBA9CC; continue 'dispatch;
	}
	// 82DBA9B0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBA9B4: 39292BDC  addi r9, r9, 0x2bdc
	ctx.r[9].s64 = ctx.r[9].s64 + 11228;
	// 82DBA9B8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBA9BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBA9C0: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DBA9C4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBA9C8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DBA9CC: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 82DBA9D0: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82DBA9D4: 39240040  addi r9, r4, 0x40
	ctx.r[9].s64 = ctx.r[4].s64 + 64;
	// 82DBA9D8: 38E1FFD0  addi r7, r1, -0x30
	ctx.r[7].s64 = ctx.r[1].s64 + -48;
	// 82DBA9DC: 3881FFD4  addi r4, r1, -0x2c
	ctx.r[4].s64 = ctx.r[1].s64 + -44;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAC60 size=112
    let mut pc: u32 = 0x82DBAC60;
    'dispatch: loop {
        match pc {
            0x82DBAC60 => {
    //   block [0x82DBAC60..0x82DBACD0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBACD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBACD0 size=232
    let mut pc: u32 = 0x82DBACD0;
    'dispatch: loop {
        match pc {
            0x82DBACD0 => {
    //   block [0x82DBACD0..0x82DBADB8)
	// 82DBACD0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBACD4: C1AB0A64  lfs f13, 0xa64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2660 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBACD8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBACDC: 40990014  ble cr6, 0x82dbacf0
	if !ctx.cr[6].gt {
	pc = 0x82DBACF0; continue 'dispatch;
	}
	// 82DBACE0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBACE4: C1AB0AD4  lfs f13, 0xad4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2772 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBACE8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBACEC: 4198FFD8  blt cr6, 0x82dbacc4
	if ctx.cr[6].lt {
		sub_82DBAC60(ctx, base);
		return;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBADB8 size=224
    let mut pc: u32 = 0x82DBADB8;
    'dispatch: loop {
        match pc {
            0x82DBADB8 => {
    //   block [0x82DBADB8..0x82DBAE98)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAE98 size=8
    let mut pc: u32 = 0x82DBAE98;
    'dispatch: loop {
        match pc {
            0x82DBAE98 => {
    //   block [0x82DBAE98..0x82DBAEA0)
	// 82DBAE98: 88630040  lbz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DBAE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAEA0 size=8
    let mut pc: u32 = 0x82DBAEA0;
    'dispatch: loop {
        match pc {
            0x82DBAEA0 => {
    //   block [0x82DBAEA0..0x82DBAEA8)
	// 82DBAEA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBAEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAEA8 size=8
    let mut pc: u32 = 0x82DBAEA8;
    'dispatch: loop {
        match pc {
            0x82DBAEA8 => {
    //   block [0x82DBAEA8..0x82DBAEB0)
	// 82DBAEA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBAEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAEB0 size=12
    let mut pc: u32 = 0x82DBAEB0;
    'dispatch: loop {
        match pc {
            0x82DBAEB0 => {
    //   block [0x82DBAEB0..0x82DBAEBC)
	// 82DBAEB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DBAEB4: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DBAEB8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAEBC size=8
    let mut pc: u32 = 0x82DBAEBC;
    'dispatch: loop {
        match pc {
            0x82DBAEBC => {
    //   block [0x82DBAEBC..0x82DBAEC4)
	// 82DBAEBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBAEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBAEC8 size=144
    let mut pc: u32 = 0x82DBAEC8;
    'dispatch: loop {
        match pc {
            0x82DBAEC8 => {
    //   block [0x82DBAEC8..0x82DBAF58)
	// 82DBAEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBAECC: 4BEEE539  bl 0x82ca9404
	ctx.lr = 0x82DBAED0;
	sub_82CA93D0(ctx, base);
	// 82DBAED0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBAED4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DBAED8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DBAEDC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBAEE0: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DBAEE4: 4099006C  ble cr6, 0x82dbaf50
	if !ctx.cr[6].gt {
	pc = 0x82DBAF50; continue 'dispatch;
	}
	// 82DBAEE8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBAEEC: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAEF0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DBAEF4: 815B0044  lwz r10, 0x44(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBAEF8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBAEFC: 557CC23E  srwi r28, r11, 8
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DBAF00: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DBAF04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DBAF08: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 82DBAF0C: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBAF10: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBAF14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAF18: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBAF1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBAF20: 4E800421  bctrl
	ctx.lr = 0x82DBAF24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBAF24: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBAF28: 578A402E  slwi r10, r28, 8
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBAF2C: 556B0202  rlwinm r11, r11, 0, 8, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBAF30: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DBAF34: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBAF38: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82DBAF3C: 656B3F00  oris r11, r11, 0x3f00
	ctx.r[11].u64 = ctx.r[11].u64 | 1056964608;
	// 82DBAF40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DBAF44: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBAF48: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82DBAF4C: 409AFFA0  bne cr6, 0x82dbaeec
	if !ctx.cr[6].eq {
	pc = 0x82DBAEEC; continue 'dispatch;
	}
	// 82DBAF50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DBAF54: 4BEEE500  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBAF58 size=24
    let mut pc: u32 = 0x82DBAF58;
    'dispatch: loop {
        match pc {
            0x82DBAF58 => {
    //   block [0x82DBAF58..0x82DBAF70)
	// 82DBAF58: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBAF5C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAF60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAF64: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBAF68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBAF6C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBAF70 size=100
    let mut pc: u32 = 0x82DBAF70;
    'dispatch: loop {
        match pc {
            0x82DBAF70 => {
    //   block [0x82DBAF70..0x82DBAFD4)
	// 82DBAF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBAF74: 4BEEE495  bl 0x82ca9408
	ctx.lr = 0x82DBAF78;
	sub_82CA93D0(ctx, base);
	// 82DBAF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBAF7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBAF80: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DBAF84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBAF88: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBAF8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBAF90: 40990038  ble cr6, 0x82dbafc8
	if !ctx.cr[6].gt {
	pc = 0x82DBAFC8; continue 'dispatch;
	}
	// 82DBAF94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DBAF98: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBAF9C: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DBAFA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBAFA4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBAFA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBAFAC: 4E800421  bctrl
	ctx.lr = 0x82DBAFB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBAFB0: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBAFB4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBAFB8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DBAFBC: 7F83E214  add r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 82DBAFC0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBAFC4: 4198FFD4  blt cr6, 0x82dbaf98
	if ctx.cr[6].lt {
	pc = 0x82DBAF98; continue 'dispatch;
	}
	// 82DBAFC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DBAFCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBAFD0: 4BEEE488  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBAFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBAFD8 size=136
    let mut pc: u32 = 0x82DBAFD8;
    'dispatch: loop {
        match pc {
            0x82DBAFD8 => {
    //   block [0x82DBAFD8..0x82DBB060)
	// 82DBAFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBAFDC: 4BEEE425  bl 0x82ca9400
	ctx.lr = 0x82DBAFE0;
	sub_82CA93D0(ctx, base);
	// 82DBAFE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBAFE4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DBAFE8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DBAFEC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DBAFF0: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82DBAFF4: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBAFF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBAFFC: 40990058  ble cr6, 0x82dbb054
	if !ctx.cr[6].gt {
	pc = 0x82DBB054; continue 'dispatch;
	}
	// 82DBB000: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DBB004: 817B0044  lwz r11, 0x44(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB008: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DBB00C: 7FFC582E  lwzx r31, r28, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DBB010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB014: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB018: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBB01C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB020: 4E800421  bctrl
	ctx.lr = 0x82DBB024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB02C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBB030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB034: 4E800421  bctrl
	ctx.lr = 0x82DBB038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB038: 815B0048  lwz r10, 0x48(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB03C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DBB040: 546B2036  slwi r11, r3, 4
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB044: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DBB048: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DBB04C: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DBB050: 4198FFB4  blt cr6, 0x82dbb004
	if ctx.cr[6].lt {
	pc = 0x82DBB004; continue 'dispatch;
	}
	// 82DBB054: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DBB058: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DBB05C: 4BEEE3F4  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB060 size=328
    let mut pc: u32 = 0x82DBB060;
    'dispatch: loop {
        match pc {
            0x82DBB060 => {
    //   block [0x82DBB060..0x82DBB1A8)
	// 82DBB060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB064: 4BEEE3A5  bl 0x82ca9408
	ctx.lr = 0x82DBB068;
	sub_82CA93D0(ctx, base);
	// 82DBB068: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82DBB06C: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB1A8 size=8
    let mut pc: u32 = 0x82DBB1A8;
    'dispatch: loop {
        match pc {
            0x82DBB1A8 => {
    //   block [0x82DBB1A8..0x82DBB1B0)
	// 82DBB1A8: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBB1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB1B0 size=20
    let mut pc: u32 = 0x82DBB1B0;
    'dispatch: loop {
        match pc {
            0x82DBB1B0 => {
    //   block [0x82DBB1B0..0x82DBB1C4)
	// 82DBB1B0: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBB1B4: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82DBB1B8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DBB1BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DBB1C0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB1C4 size=8
    let mut pc: u32 = 0x82DBB1C4;
    'dispatch: loop {
        match pc {
            0x82DBB1C4 => {
    //   block [0x82DBB1C4..0x82DBB1CC)
	// 82DBB1C4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DBB1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB1D0 size=16
    let mut pc: u32 = 0x82DBB1D0;
    'dispatch: loop {
        match pc {
            0x82DBB1D0 => {
    //   block [0x82DBB1D0..0x82DBB1E0)
	// 82DBB1D0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBB1D4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBB1D8: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBB1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB1E0 size=220
    let mut pc: u32 = 0x82DBB1E0;
    'dispatch: loop {
        match pc {
            0x82DBB1E0 => {
    //   block [0x82DBB1E0..0x82DBB2BC)
	// 82DBB1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB1E4: 4BEEE229  bl 0x82ca940c
	ctx.lr = 0x82DBB1E8;
	sub_82CA93D0(ctx, base);
	// 82DBB1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB1EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB1F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB1F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBB1F8: 396B3734  addi r11, r11, 0x3734
	ctx.r[11].s64 = ctx.r[11].s64 + 14132;
	// 82DBB1FC: 394A24EC  addi r10, r10, 0x24ec
	ctx.r[10].s64 = ctx.r[10].s64 + 9452;
	// 82DBB200: 813F0048  lwz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB204: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DBB208: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DBB20C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBB210: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBB214: 4099005C  ble cr6, 0x82dbb270
	if !ctx.cr[6].gt {
	pc = 0x82DBB270; continue 'dispatch;
	}
	// 82DBB218: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DBB21C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB220: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB224: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB22C: 419A0030  beq cr6, 0x82dbb25c
	if ctx.cr[6].eq {
	pc = 0x82DBB25C; continue 'dispatch;
	}
	// 82DBB230: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBB234: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBB238: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DBB23C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBB240: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DBB244: 409A0018  bne cr6, 0x82dbb25c
	if !ctx.cr[6].eq {
	pc = 0x82DBB25C; continue 'dispatch;
	}
	// 82DBB248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB24C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DBB250: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB258: 4E800421  bctrl
	ctx.lr = 0x82DBB25C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB25C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB260: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DBB264: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DBB268: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB26C: 4198FFB0  blt cr6, 0x82dbb21c
	if ctx.cr[6].lt {
	pc = 0x82DBB21C; continue 'dispatch;
	}
	// 82DBB270: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DBB274: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBB278: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBB27C: 409A0020  bne cr6, 0x82dbb29c
	if !ctx.cr[6].eq {
	pc = 0x82DBB29C; continue 'dispatch;
	}
	// 82DBB280: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB284: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DBB288: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DBB28C: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB290: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DBB294: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBB298: 4BF9A031  bl 0x82d552c8
	ctx.lr = 0x82DBB29C;
	sub_82D552C8(ctx, base);
	// 82DBB29C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBB2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBB2A4: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DBB2A8: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DBB2AC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DBB2B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBB2B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBB2B8: 4BEEE1A4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB2C0 size=200
    let mut pc: u32 = 0x82DBB2C0;
    'dispatch: loop {
        match pc {
            0x82DBB2C0 => {
    //   block [0x82DBB2C0..0x82DBB388)
	// 82DBB2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB2C4: 4BEEE139  bl 0x82ca93fc
	ctx.lr = 0x82DBB2C8;
	sub_82CA93D0(ctx, base);
	// 82DBB2C8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82DBB2CC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB2D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB2D4: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB2D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DBB2DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DBB2E0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DBB2E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBB2E8: C3EB0BE4  lfs f31, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DBB2EC: 3B230048  addi r25, r3, 0x48
	ctx.r[25].s64 = ctx.r[3].s64 + 72;
	// 82DBB2F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBB2F4: 40990070  ble cr6, 0x82dbb364
	if !ctx.cr[6].gt {
	pc = 0x82DBB364; continue 'dispatch;
	}
	// 82DBB2F8: 3B430044  addi r26, r3, 0x44
	ctx.r[26].s64 = ctx.r[3].s64 + 68;
	// 82DBB2FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DBB300: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB304: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBB308: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DBB30C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB314: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBB318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB31C: 4E800421  bctrl
	ctx.lr = 0x82DBB320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB320: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB388 size=168
    let mut pc: u32 = 0x82DBB388;
    'dispatch: loop {
        match pc {
            0x82DBB388 => {
    //   block [0x82DBB388..0x82DBB430)
	// 82DBB388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB38C: 4BEEE07D  bl 0x82ca9408
	ctx.lr = 0x82DBB390;
	sub_82CA93D0(ctx, base);
	// 82DBB390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB394: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DBB398: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DBB39C: 3BBC0044  addi r29, r28, 0x44
	ctx.r[29].s64 = ctx.r[28].s64 + 68;
	// 82DBB3A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBB3A4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB3A8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBB3AC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBB3B0: 40980024  bge cr6, 0x82dbb3d4
	if !ctx.cr[6].lt {
	pc = 0x82DBB3D4; continue 'dispatch;
	}
	// 82DBB3B4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB3B8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB3BC: 41980008  blt cr6, 0x82dbb3c4
	if ctx.cr[6].lt {
	pc = 0x82DBB3C4; continue 'dispatch;
	}
	// 82DBB3C0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DBB3C4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DBB3C8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBB3CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DBB3D0: 4BF9BB41  bl 0x82d56f10
	ctx.lr = 0x82DBB3D4;
	sub_82D56F10(ctx, base);
	// 82DBB3D4: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBB3D8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DBB3DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB3E0: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB3E4: D01C0010  stfs f0, 0x10(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBB3E8: 40990040  ble cr6, 0x82dbb428
	if !ctx.cr[6].gt {
	pc = 0x82DBB428; continue 'dispatch;
	}
	// 82DBB3EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBB3F0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB3F4: 7D2BF02E  lwzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB3F8: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82DBB3FC: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB400: A12A0004  lhz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB404: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DBB408: 419A0010  beq cr6, 0x82dbb418
	if ctx.cr[6].eq {
	pc = 0x82DBB418; continue 'dispatch;
	}
	// 82DBB40C: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBB410: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBB414: B12A0006  sth r9, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DBB418: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82DBB41C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DBB420: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DBB424: 409AFFCC  bne cr6, 0x82dbb3f0
	if !ctx.cr[6].eq {
	pc = 0x82DBB3F0; continue 'dispatch;
	}
	// 82DBB428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBB42C: 4BEEE02C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB430 size=252
    let mut pc: u32 = 0x82DBB430;
    'dispatch: loop {
        match pc {
            0x82DBB430 => {
    //   block [0x82DBB430..0x82DBB52C)
	// 82DBB430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB434: 4BEEDFCD  bl 0x82ca9400
	ctx.lr = 0x82DBB438;
	sub_82CA93D0(ctx, base);
	// 82DBB438: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82DBB43C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB444: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DBB448: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DBB44C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DBB450: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DBB454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB458: 419A002C  beq cr6, 0x82dbb484
	if ctx.cr[6].eq {
	pc = 0x82DBB484; continue 'dispatch;
	}
	// 82DBB45C: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB460: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DBB464: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82DBB468: EC20F82A  fadds f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82DBB46C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82DBB470: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DBB474: 4B4DE97D  bl 0x82299df0
	ctx.lr = 0x82DBB478;
	sub_82299DF0(ctx, base);
	// 82DBB478: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DBB47C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DBB480: 4BEEDFD0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82DBB484: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB488: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DBB48C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DBB490: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DBB494: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB49C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBB4A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB4A4: 4E800421  bctrl
	ctx.lr = 0x82DBB4A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB4A8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB4AC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DBB4B0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DBB4B4: 4099006C  ble cr6, 0x82dbb520
	if !ctx.cr[6].gt {
	pc = 0x82DBB520; continue 'dispatch;
	}
	// 82DBB4B8: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82DBB4BC: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82DBB4C0: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB4C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DBB4C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DBB4CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DBB4D0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBB4D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB4D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBB4DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB4E0: 4E800421  bctrl
	ctx.lr = 0x82DBB4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB4E4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB530 size=232
    let mut pc: u32 = 0x82DBB530;
    'dispatch: loop {
        match pc {
            0x82DBB530 => {
    //   block [0x82DBB530..0x82DBB618)
	// 82DBB530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB534: 4BEEDED1  bl 0x82ca9404
	ctx.lr = 0x82DBB538;
	sub_82CA93D0(ctx, base);
	// 82DBB538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB53C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBB540: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBB544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB548: 388B2BF8  addi r4, r11, 0x2bf8
	ctx.r[4].s64 = ctx.r[11].s64 + 11256;
	// 82DBB54C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DBB550: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB554: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBB558: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB55C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB564: 4E800421  bctrl
	ctx.lr = 0x82DBB568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB568: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DBB56C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBB570: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBB574: 409A0034  bne cr6, 0x82dbb5a8
	if !ctx.cr[6].eq {
	pc = 0x82DBB5A8; continue 'dispatch;
	}
	// 82DBB578: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB57C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBB580: 80FF0048  lwz r7, 0x48(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB584: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DBB588: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB58C: 388A2BEC  addi r4, r10, 0x2bec
	ctx.r[4].s64 = ctx.r[10].s64 + 11244;
	// 82DBB590: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DBB594: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBB598: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB59C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB5A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB5A4: 4E800421  bctrl
	ctx.lr = 0x82DBB5A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB5A8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB5AC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DBB5B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBB5B4: 40990048  ble cr6, 0x82dbb5fc
	if !ctx.cr[6].gt {
	pc = 0x82DBB5FC; continue 'dispatch;
	}
	// 82DBB5B8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DBB5BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DBB5C0: 3B6BB3B4  addi r27, r11, -0x4c4c
	ctx.r[27].s64 = ctx.r[11].s64 + -19532;
	// 82DBB5C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB5C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBB5CC: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DBB5D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DBB5D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB5D8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB5DC: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBB5E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB5E4: 4E800421  bctrl
	ctx.lr = 0x82DBB5E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB5E8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DBB5EC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DBB5F0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DBB5F4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB5F8: 4198FFCC  blt cr6, 0x82dbb5c4
	if ctx.cr[6].lt {
	pc = 0x82DBB5C4; continue 'dispatch;
	}
	// 82DBB5FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB604: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBB608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB60C: 4E800421  bctrl
	ctx.lr = 0x82DBB610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBB614: 4BEEDE40  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB618 size=360
    let mut pc: u32 = 0x82DBB618;
    'dispatch: loop {
        match pc {
            0x82DBB618 => {
    //   block [0x82DBB618..0x82DBB780)
	// 82DBB618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB61C: 4BEEDDD9  bl 0x82ca93f4
	ctx.lr = 0x82DBB620;
	sub_82CA93D0(ctx, base);
	// 82DBB620: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB624: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB628: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DBB62C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DBB630: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DBB634: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DBB638: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DBB63C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB640: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB644: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB648: 40980020  bge cr6, 0x82dbb668
	if !ctx.cr[6].lt {
	pc = 0x82DBB668; continue 'dispatch;
	}
	// 82DBB64C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBB650: 39292C08  addi r9, r9, 0x2c08
	ctx.r[9].s64 = ctx.r[9].s64 + 11272;
	// 82DBB654: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB658: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB65C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBB660: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB664: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBB668: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DBB66C: 3BC40014  addi r30, r4, 0x14
	ctx.r[30].s64 = ctx.r[4].s64 + 20;
	// 82DBB670: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82DBB674: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBB678: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82DBB67C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB680: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DBB684: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB688: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB690: 4E800421  bctrl
	ctx.lr = 0x82DBB694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB698: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB69C: 419A0074  beq cr6, 0x82dbb710
	if ctx.cr[6].eq {
	pc = 0x82DBB710; continue 'dispatch;
	}
	// 82DBB6A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB6A4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBB6A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB6AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB6B0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB6B8: 4E800421  bctrl
	ctx.lr = 0x82DBB6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB6BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DBB6C0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DBB6C4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DBB6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBB6CC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB6D0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBB6D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB6D8: 4E800421  bctrl
	ctx.lr = 0x82DBB6DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB6DC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DBB6E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB6E4: 419A0008  beq cr6, 0x82dbb6ec
	if ctx.cr[6].eq {
	pc = 0x82DBB6EC; continue 'dispatch;
	}
	// 82DBB6E8: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DBB6EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB6F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB6F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB6F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB6FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB700: 4E800421  bctrl
	ctx.lr = 0x82DBB704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB708: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB70C: 409AFF94  bne cr6, 0x82dbb6a0
	if !ctx.cr[6].eq {
	pc = 0x82DBB6A0; continue 'dispatch;
	}
	// 82DBB710: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DBB714: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82DBB718: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBB71C: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DBB720: 419A0010  beq cr6, 0x82dbb730
	if ctx.cr[6].eq {
	pc = 0x82DBB730; continue 'dispatch;
	}
	// 82DBB724: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBB728: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB72C: 7F8BE92E  stwx r28, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	// 82DBB730: 7D7CD050  subf r11, r28, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[28].s64;
	// 82DBB734: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DBB738: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DBB73C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBB740: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB744: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DBB748: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DBB74C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB750: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB754: 40980020  bge cr6, 0x82dbb774
	if !ctx.cr[6].lt {
	pc = 0x82DBB774; continue 'dispatch;
	}
	// 82DBB758: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DBB75C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DBB760: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB764: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB768: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DBB76C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB770: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBB774: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DBB778: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82DBB77C: 4BEEDCC8  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB780 size=284
    let mut pc: u32 = 0x82DBB780;
    'dispatch: loop {
        match pc {
            0x82DBB780 => {
    //   block [0x82DBB780..0x82DBB89C)
	// 82DBB780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB784: 4BEEDC79  bl 0x82ca93fc
	ctx.lr = 0x82DBB788;
	sub_82CA93D0(ctx, base);
	// 82DBB788: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB78C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB790: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DBB794: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DBB798: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DBB79C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DBB7A0: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DBB7A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB7A8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB7AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB7B0: 40980020  bge cr6, 0x82dbb7d0
	if !ctx.cr[6].lt {
	pc = 0x82DBB7D0; continue 'dispatch;
	}
	// 82DBB7B4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBB7B8: 39292B5C  addi r9, r9, 0x2b5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11100;
	// 82DBB7BC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB7C0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB7C4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBB7C8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB7CC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBB7D0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB7D4: 3BC30014  addi r30, r3, 0x14
	ctx.r[30].s64 = ctx.r[3].s64 + 20;
	// 82DBB7D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB7DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB7E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB7E4: 4E800421  bctrl
	ctx.lr = 0x82DBB7E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB7E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB7EC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB7F0: 419A0074  beq cr6, 0x82dbb864
	if ctx.cr[6].eq {
	pc = 0x82DBB864; continue 'dispatch;
	}
	// 82DBB7F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB7F8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBB7FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB800: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB804: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBB808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB80C: 4E800421  bctrl
	ctx.lr = 0x82DBB810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB810: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB814: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82DBB818: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DBB81C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DBB820: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DBB824: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DBB828: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DBB82C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DBB830: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB834: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DBB838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB83C: 4E800421  bctrl
	ctx.lr = 0x82DBB840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB840: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBB848: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBB84C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBB854: 4E800421  bctrl
	ctx.lr = 0x82DBB858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBB858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB85C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DBB860: 409AFF94  bne cr6, 0x82dbb7f4
	if !ctx.cr[6].eq {
	pc = 0x82DBB7F4; continue 'dispatch;
	}
	// 82DBB864: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DBB868: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB86C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB870: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBB874: 40980020  bge cr6, 0x82dbb894
	if !ctx.cr[6].lt {
	pc = 0x82DBB894; continue 'dispatch;
	}
	// 82DBB878: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DBB87C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DBB880: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB884: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBB888: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DBB88C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBB890: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBB894: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DBB898: 4BEEDBB4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBB8A0 size=160
    let mut pc: u32 = 0x82DBB8A0;
    'dispatch: loop {
        match pc {
            0x82DBB8A0 => {
    //   block [0x82DBB8A0..0x82DBB940)
	// 82DBB8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBB8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBB8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB8B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBB8B8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DBB8BC: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82DBB8C0: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB8C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBB8C8: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBB8CC: 394B00C4  addi r10, r11, 0xc4
	ctx.r[10].s64 = ctx.r[11].s64 + 196;
	// 82DBB8D0: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DBB8D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8D8: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DBB8DC: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DBB8E0: 392B3734  addi r9, r11, 0x3734
	ctx.r[9].s64 = ctx.r[11].s64 + 14132;
	// 82DBB8E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8E8: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBB8EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBB8F0: 390B24EC  addi r8, r11, 0x24ec
	ctx.r[8].s64 = ctx.r[11].s64 + 9452;
	// 82DBB8F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBB8F8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBB8FC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBB900: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBB904: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DBB908: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82DBB90C: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82DBB910: 90FF004C  stw r7, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[7].u32 ) };
	// 82DBB914: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DBB918: 4BFFFA71  bl 0x82dbb388
	ctx.lr = 0x82DBB91C;
	sub_82DBB388(ctx, base);
	// 82DBB91C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DBB920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB924: 4BFFF73D  bl 0x82dbb060
	ctx.lr = 0x82DBB928;
	sub_82DBB060(ctx, base);
	// 82DBB928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB92C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBB930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBB934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBB938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBB93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBB940 size=176
    let mut pc: u32 = 0x82DBB940;
    'dispatch: loop {
        match pc {
            0x82DBB940 => {
    //   block [0x82DBB940..0x82DBB9F0)
	// 82DBB940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBB944: 4BEEDAC9  bl 0x82ca940c
	ctx.lr = 0x82DBB948;
	sub_82CA93D0(ctx, base);
	// 82DBB948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBB94C: 3BE30008  addi r31, r3, 8
	ctx.r[31].s64 = ctx.r[3].s64 + 8;
	// 82DBB950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBB954: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DBB958: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBB95C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB960: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBB964: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBB968: 409A0010  bne cr6, 0x82dbb978
	if !ctx.cr[6].eq {
	pc = 0x82DBB978; continue 'dispatch;
	}
	// 82DBB96C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82DBB970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBB974: 4BF9B625  bl 0x82d56f98
	ctx.lr = 0x82DBB978;
	sub_82D56F98(ctx, base);
	// 82DBB978: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB97C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82DBB980: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB984: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82DBB988: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBB98C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DBB990: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBB994: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB99C: 419A0014  beq cr6, 0x82dbb9b0
	if ctx.cr[6].eq {
	pc = 0x82DBB9B0; continue 'dispatch;
	}
	// 82DBB9A0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DBB9A4: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBB9AC: 409AFFF4  bne cr6, 0x82dbb9a0
	if !ctx.cr[6].eq {
	pc = 0x82DBB9A0; continue 'dispatch;
	}
	// 82DBB9B0: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBB9B4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DBB9B8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB9BC: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DBB9C0: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB9C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DBB9C8: 419A0014  beq cr6, 0x82dbb9dc
	if ctx.cr[6].eq {
	pc = 0x82DBB9DC; continue 'dispatch;
	}
	// 82DBB9CC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBB9D0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBB9D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DBB9D8: 409AFFF4  bne cr6, 0x82dbb9cc
	if !ctx.cr[6].eq {
	pc = 0x82DBB9CC; continue 'dispatch;
	}
	// 82DBB9DC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBB9E0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBB9E4: 9169000C  stw r11, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBB9E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBB9EC: 4BEEDA70  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBB9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBB9F0 size=20
    let mut pc: u32 = 0x82DBB9F0;
    'dispatch: loop {
        match pc {
            0x82DBB9F0 => {
    //   block [0x82DBB9F0..0x82DBBA04)
	// 82DBB9F0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBB9F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBB9F8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBB9FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBA08 size=144
    let mut pc: u32 = 0x82DBBA08;
    'dispatch: loop {
        match pc {
            0x82DBBA08 => {
    //   block [0x82DBBA08..0x82DBBA98)
	// 82DBBA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBBA10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBBA14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBBA18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBA1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBBA20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBBA24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBBA28: 388B2C30  addi r4, r11, 0x2c30
	ctx.r[4].s64 = ctx.r[11].s64 + 11312;
	// 82DBBA2C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBBA30: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBA34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBBA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBBA3C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBA40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA44: 4E800421  bctrl
	ctx.lr = 0x82DBBA48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBA48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBA4C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DBBA50: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBA54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBBA58: 388BB3B4  addi r4, r11, -0x4c4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19532;
	// 82DBBA5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBBA60: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBBA64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA68: 4E800421  bctrl
	ctx.lr = 0x82DBBA6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBA6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBBA74: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBA78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBA7C: 4E800421  bctrl
	ctx.lr = 0x82DBBA80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBA80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBBA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBBA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBBA8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBBA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBBA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBBA98 size=8
    let mut pc: u32 = 0x82DBBA98;
    'dispatch: loop {
        match pc {
            0x82DBBA98 => {
    //   block [0x82DBBA98..0x82DBBAA0)
	// 82DBBA98: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DBBA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBBAA0 size=52
    let mut pc: u32 = 0x82DBBAA0;
    'dispatch: loop {
        match pc {
            0x82DBBAA0 => {
    //   block [0x82DBBAA0..0x82DBBAD4)
	// 82DBBAA0: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBBAD8 size=136
    let mut pc: u32 = 0x82DBBAD8;
    'dispatch: loop {
        match pc {
            0x82DBBAD8 => {
    //   block [0x82DBBAD8..0x82DBBB60)
	// 82DBBAD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBBADC: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBBAE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBBAE4: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBBAE8: 396B346C  addi r11, r11, 0x346c
	ctx.r[11].s64 = ctx.r[11].s64 + 13420;
	// 82DBBAEC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DBBAF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DBBAF4: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 82DBBAF8: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DBBAFC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBBB00: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DBBB04: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DBBB08: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBBB0C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBBB10: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82DBBB14: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBB18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBBB1C: 419A0010  beq cr6, 0x82dbbb2c
	if ctx.cr[6].eq {
	pc = 0x82DBBB2C; continue 'dispatch;
	}
	// 82DBBB20: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBBB24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBBB28: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DBBB2C: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBB60 size=108
    let mut pc: u32 = 0x82DBBB60;
    'dispatch: loop {
        match pc {
            0x82DBBB60 => {
    //   block [0x82DBBB60..0x82DBBBCC)
	// 82DBBB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBBB68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBBB6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBBB70: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DBBB74: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBB78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBBB7C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DBBB80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBBB84: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82DBBB88: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBBB8C: 4BF9B775  bl 0x82d57300
	ctx.lr = 0x82DBBB90;
	sub_82D57300(ctx, base);
	// 82DBBB90: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBB94: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBBB98: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DBBB9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DBBBA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBBA4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBBBA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBBBAC: 4E800421  bctrl
	ctx.lr = 0x82DBBBB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBBBB0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DBBBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBBBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBBBBC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DBBBC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBBBC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBBBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBBD0 size=216
    let mut pc: u32 = 0x82DBBBD0;
    'dispatch: loop {
        match pc {
            0x82DBBBD0 => {
    //   block [0x82DBBBD0..0x82DBBCA8)
	// 82DBBBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBBD4: 4BEED835  bl 0x82ca9408
	ctx.lr = 0x82DBBBD8;
	sub_82CA93D0(ctx, base);
	// 82DBBBD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBBDC: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 82DBBBE0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBBBE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBBBE8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82DBBBEC: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82DBBBF0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBCA8 size=436
    let mut pc: u32 = 0x82DBBCA8;
    'dispatch: loop {
        match pc {
            0x82DBBCA8 => {
    //   block [0x82DBBCA8..0x82DBBE5C)
	// 82DBBCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBCAC: 4BEED759  bl 0x82ca9404
	ctx.lr = 0x82DBBCB0;
	sub_82CA93D0(ctx, base);
	// 82DBBCB0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBCB4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBCB8: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DBBCBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBBCC0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBBCC4: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DBBCC8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBCCC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBBCD0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBBCD4: 40980020  bge cr6, 0x82dbbcf4
	if !ctx.cr[6].lt {
	pc = 0x82DBBCF4; continue 'dispatch;
	}
	// 82DBBCD8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBBCDC: 39292C40  addi r9, r9, 0x2c40
	ctx.r[9].s64 = ctx.r[9].s64 + 11328;
	// 82DBBCE0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBBCE4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBBCE8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBBCEC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBBCF0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBBCF4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DBBCF8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DBBCFC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DBBD00: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DBBD04: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBBD08: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBBD0C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DBBD10: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DBBD14: 4200FFF0  bdnz 0x82dbbd04
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBBD04; continue 'dispatch;
	}
	// 82DBBD18: 3BE40020  addi r31, r4, 0x20
	ctx.r[31].s64 = ctx.r[4].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBE60 size=384
    let mut pc: u32 = 0x82DBBE60;
    'dispatch: loop {
        match pc {
            0x82DBBE60 => {
    //   block [0x82DBBE60..0x82DBBFE0)
	// 82DBBE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBE64: 4BEED5A1  bl 0x82ca9404
	ctx.lr = 0x82DBBE68;
	sub_82CA93D0(ctx, base);
	// 82DBBE68: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBE6C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBBE70: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DBBE74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBBE78: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DBBE7C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBBE80: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DBBE84: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DBBE88: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBBE8C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBBE90: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBBE94: 40980020  bge cr6, 0x82dbbeb4
	if !ctx.cr[6].lt {
	pc = 0x82DBBEB4; continue 'dispatch;
	}
	// 82DBBE98: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBBE9C: 39082C50  addi r8, r8, 0x2c50
	ctx.r[8].s64 = ctx.r[8].s64 + 11344;
	// 82DBBEA0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBBEA4: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBBEA8: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82DBBEAC: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBBEB0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DBBEB4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DBBEB8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBBEBC: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82DBBEC0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82DBBEC4: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBBEC8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBBECC: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DBBED0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DBBED4: 4200FFF0  bdnz 0x82dbbec4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBBEC4; continue 'dispatch;
	}
	// 82DBBED8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBBFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBBFE0 size=188
    let mut pc: u32 = 0x82DBBFE0;
    'dispatch: loop {
        match pc {
            0x82DBBFE0 => {
    //   block [0x82DBBFE0..0x82DBC09C)
	// 82DBBFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBBFE4: 4BEED425  bl 0x82ca9408
	ctx.lr = 0x82DBBFE8;
	sub_82CA93D0(ctx, base);
	// 82DBBFE8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBBFEC: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC0A0 size=252
    let mut pc: u32 = 0x82DBC0A0;
    'dispatch: loop {
        match pc {
            0x82DBC0A0 => {
    //   block [0x82DBC0A0..0x82DBC19C)
	// 82DBC0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC0A4: 4BEED361  bl 0x82ca9404
	ctx.lr = 0x82DBC0A8;
	sub_82CA93D0(ctx, base);
	// 82DBC0A8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC0AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC0B0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBC0B4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DBC0B8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC0BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC0C0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBC0C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC0C8: 4E800421  bctrl
	ctx.lr = 0x82DBC0CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC0CC: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC0D0: 387EFFFF  addi r3, r30, -1
	ctx.r[3].s64 = ctx.r[30].s64 + -1;
	// 82DBC0D4: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82DBC0D8: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DBC0DC: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DBC0E0: EBCB0000  ld r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBC0E4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBC0E8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DBC0EC: 547F2036  slwi r31, r3, 4
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DBC0F0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DBC0F4: 7FFFEA14  add r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82DBC0F8: EBA60000  ld r29, 0(r6)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DBC0FC: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82DBC100: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82DBC104: FBCA0000  std r30, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82DBC108: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82DBC10C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DBC110: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DBC114: EB850000  ld r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DBC118: FBA90000  std r29, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 82DBC11C: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82DBC120: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC1A0 size=140
    let mut pc: u32 = 0x82DBC1A0;
    'dispatch: loop {
        match pc {
            0x82DBC1A0 => {
    //   block [0x82DBC1A0..0x82DBC22C)
	// 82DBC1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC1A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBC1AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC1B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBC1B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBC1BC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC1C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC1C4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBC1C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC1CC: 4E800421  bctrl
	ctx.lr = 0x82DBC1D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC1D0: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC230 size=148
    let mut pc: u32 = 0x82DBC230;
    'dispatch: loop {
        match pc {
            0x82DBC230 => {
    //   block [0x82DBC230..0x82DBC2C4)
	// 82DBC230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBC23C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC244: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC248: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBC24C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DBC250: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC258: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBC25C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC260: 4E800421  bctrl
	ctx.lr = 0x82DBC264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC264: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBC268: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC26C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DBC270: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC2C8 size=280
    let mut pc: u32 = 0x82DBC2C8;
    'dispatch: loop {
        match pc {
            0x82DBC2C8 => {
    //   block [0x82DBC2C8..0x82DBC3E0)
	// 82DBC2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC2CC: 4BEED12D  bl 0x82ca93f8
	ctx.lr = 0x82DBC2D0;
	sub_82CA93D0(ctx, base);
	// 82DBC2D0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC2D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC2D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DBC2DC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC2E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC2E4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBC2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC2EC: 4E800421  bctrl
	ctx.lr = 0x82DBC2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC2F0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBC2F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBC2F8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DBC2FC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC300: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBC304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC308: 4E800421  bctrl
	ctx.lr = 0x82DBC30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC30C: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC310: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 82DBC314: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DBC318: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82DBC31C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DBC320: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBC324: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DBC328: EB0B0008  ld r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DBC32C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DBC330: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82DBC334: EB650000  ld r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82DBC338: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82DBC33C: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82DBC340: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBC344: F8690000  std r3, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82DBC348: 7F9EE850  subf r28, r30, r29
	ctx.r[28].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82DBC34C: FB090008  std r24, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82DBC350: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DBC354: EB440000  ld r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DBC358: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DBC35C: FB680000  std r27, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DBC360: F8A80008  std r5, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82DBC364: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBC3E0 size=4
    let mut pc: u32 = 0x82DBC3E0;
    'dispatch: loop {
        match pc {
            0x82DBC3E0 => {
    //   block [0x82DBC3E0..0x82DBC3E4)
	// 82DBC3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC3E8 size=196
    let mut pc: u32 = 0x82DBC3E8;
    'dispatch: loop {
        match pc {
            0x82DBC3E8 => {
    //   block [0x82DBC3E8..0x82DBC4AC)
	// 82DBC3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC3F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC3F4: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC3F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC3FC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBC400: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBC404: 409A001C  bne cr6, 0x82dbc420
	if !ctx.cr[6].eq {
	pc = 0x82DBC420; continue 'dispatch;
	}
	// 82DBC408: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DBC40C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DBC410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBC41C: 4E800020  blr
	return;
	// 82DBC420: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC424: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBC428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DBC42C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC430: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBC434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC438: 4E800421  bctrl
	ctx.lr = 0x82DBC43C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC43C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DBC440: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBC444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBC448: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 82DBC44C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82DBC450: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DBC454: C02AB384  lfs f1, -0x4c7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBC458: 4BFFE961  bl 0x82dbadb8
	ctx.lr = 0x82DBC45C;
	sub_82DBADB8(ctx, base);
	// 82DBC45C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBC464: 409A001C  bne cr6, 0x82dbc480
	if !ctx.cr[6].eq {
	pc = 0x82DBC480; continue 'dispatch;
	}
	// 82DBC468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBC46C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DBC470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBC47C: 4E800020  blr
	return;
	// 82DBC480: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC484: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DBC488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC48C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBC490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBC494: 4E800421  bctrl
	ctx.lr = 0x82DBC498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC498: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DBC49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC4A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBC4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC4B0 size=204
    let mut pc: u32 = 0x82DBC4B0;
    'dispatch: loop {
        match pc {
            0x82DBC4B0 => {
    //   block [0x82DBC4B0..0x82DBC57C)
	// 82DBC4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC4B4: 4BEECF49  bl 0x82ca93fc
	ctx.lr = 0x82DBC4B8;
	sub_82CA93D0(ctx, base);
	// 82DBC4B8: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC4BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC4C0: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82DBC4C4: 3F408330  lis r26, -0x7cd0
	ctx.r[26].s64 = -2094006272;
	// 82DBC4C8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBC4CC: 214B0020  subfic r10, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DBC4D0: 7F2B5C30  srw r11, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[25].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBC4D4: 7C9D5430  srw r29, r4, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[4].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBC4D8: 7D7E2038  and r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82DBC4DC: 1F7D0038  mulli r27, r29, 0x38
	ctx.r[27].s64 = ctx.r[29].s64 * 56;
	// 82DBC4E0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBC4E4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DBC4E8: 7D7B5A14  add r11, r27, r11
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82DBC4EC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBC4F0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBC4F4: 4198001C  blt cr6, 0x82dbc510
	if ctx.cr[6].lt {
	pc = 0x82DBC510; continue 'dispatch;
	}
	// 82DBC4F8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBC4FC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DBC500: 3B7B0038  addi r27, r27, 0x38
	ctx.r[27].s64 = ctx.r[27].s64 + 56;
	// 82DBC504: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DBC508: 40980068  bge cr6, 0x82dbc570
	if !ctx.cr[6].lt {
	pc = 0x82DBC570; continue 'dispatch;
	}
	// 82DBC50C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DBC510: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBC514: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBC518: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC520: 216B0020  subfic r11, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DBC524: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBC528: 7FAB5830  slw r11, r29, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[29].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBC52C: 7D7CF378  or r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 | ctx.r[30].u64;
	// 82DBC530: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DBC534: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DBC538: 4E800421  bctrl
	ctx.lr = 0x82DBC53C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBC53C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBC540: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBC544: C03AB384  lfs f1, -0x4c7c(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBC548: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 82DBC54C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82DBC550: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82DBC554: 4BFFE865  bl 0x82dbadb8
	ctx.lr = 0x82DBC558;
	sub_82DBADB8(ctx, base);
	// 82DBC558: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC55C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBC560: 409AFF80  bne cr6, 0x82dbc4e0
	if !ctx.cr[6].eq {
	pc = 0x82DBC4E0; continue 'dispatch;
	}
	// 82DBC564: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DBC568: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DBC56C: 4BEECEE0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82DBC570: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DBC574: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82DBC578: 4BEECED4  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBC580 size=68
    let mut pc: u32 = 0x82DBC580;
    'dispatch: loop {
        match pc {
            0x82DBC580 => {
    //   block [0x82DBC580..0x82DBC5C4)
	// 82DBC580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC58C: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DBC590: 480009D9  bl 0x82dbcf68
	ctx.lr = 0x82DBC594;
	sub_82DBCF68(ctx, base);
	// 82DBC594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DBC598: 419A0018  beq cr6, 0x82dbc5b0
	if ctx.cr[6].eq {
	pc = 0x82DBC5B0; continue 'dispatch;
	}
	// 82DBC59C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBC5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBC5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC5AC: 4E800020  blr
	return;
	// 82DBC5B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBC5B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBC5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBC5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBC5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC5C8 size=168
    let mut pc: u32 = 0x82DBC5C8;
    'dispatch: loop {
        match pc {
            0x82DBC5C8 => {
    //   block [0x82DBC5C8..0x82DBC670)
	// 82DBC5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC5D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC5D4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC5D8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBC5DC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DBC5E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC5E4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DBC5E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DBC5EC: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC5F0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DBC5F4: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBC5F8: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC5FC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBC600: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DBC604: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DBC608: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC60C: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBC610: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DBC614: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBC618: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DBC61C: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC620: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBC624: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DBC628: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC670 size=704
    let mut pc: u32 = 0x82DBC670;
    'dispatch: loop {
        match pc {
            0x82DBC670 => {
    //   block [0x82DBC670..0x82DBC930)
	// 82DBC670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC674: 4BEECD79  bl 0x82ca93ec
	ctx.lr = 0x82DBC678;
	sub_82CA93D0(ctx, base);
	// 82DBC678: DBC1FF90  stfd f30, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[30].u64 ) };
	// 82DBC67C: DBE1FF98  stfd f31, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82DBC680: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC684: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC688: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82DBC68C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DBC690: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DBC694: 3B3A0010  addi r25, r26, 0x10
	ctx.r[25].s64 = ctx.r[26].s64 + 16;
	// 82DBC698: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DBC69C: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC6A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC6A4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DBC6A8: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DBC6AC: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DBC6B0: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DBC6B4: C3EB0C18  lfs f31, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DBC6B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC6BC: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DBC6C0: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DBC6C4: C00B0BE4  lfs f0, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC6C8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DBC6CC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DBC6D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DBC6D4: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBC930 size=16
    let mut pc: u32 = 0x82DBC930;
    'dispatch: loop {
        match pc {
            0x82DBC930 => {
    //   block [0x82DBC930..0x82DBC940)
	// 82DBC930: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBC940 size=176
    let mut pc: u32 = 0x82DBC940;
    'dispatch: loop {
        match pc {
            0x82DBC940 => {
    //   block [0x82DBC940..0x82DBC9F0)
	// 82DBC940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBC944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBC948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBC94C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBC950: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DBC954: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBC958: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DBC95C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DBC960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBC964: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBC968: 4BFFC631  bl 0x82db8f98
	ctx.lr = 0x82DBC96C;
	sub_82DB8F98(ctx, base);
	// 82DBC96C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBC970: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBC974: 392A254C  addi r9, r10, 0x254c
	ctx.r[9].s64 = ctx.r[10].s64 + 9548;
	// 82DBC978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBC97C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DBC980: 390A2C64  addi r8, r10, 0x2c64
	ctx.r[8].s64 = ctx.r[10].s64 + 11364;
	// 82DBC984: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBC988: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBC98C: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82DBC990: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBC994: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DBC998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBC99C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DBC9A0: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82DBC9A4: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DBC9A8: 90FF003C  stw r7, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[7].u32 ) };
	// 82DBC9AC: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82DBC9B0: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82DBC9B4: 90FF0048  stw r7, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[7].u32 ) };
	// 82DBC9B8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBC9F0 size=136
    let mut pc: u32 = 0x82DBC9F0;
    'dispatch: loop {
        match pc {
            0x82DBC9F0 => {
    //   block [0x82DBC9F0..0x82DBCA78)
	// 82DBC9F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBC9F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBC9F8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBC9FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DBCA00: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DBCA04: 394A254C  addi r10, r10, 0x254c
	ctx.r[10].s64 = ctx.r[10].s64 + 9548;
	// 82DBCA08: 39292C64  addi r9, r9, 0x2c64
	ctx.r[9].s64 = ctx.r[9].s64 + 11364;
	// 82DBCA0C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82DBCA10: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DBCA14: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DBCA18: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DBCA1C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBCA20: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82DBCA24: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBCA28: 419A0044  beq cr6, 0x82dbca6c
	if ctx.cr[6].eq {
	pc = 0x82DBCA6C; continue 'dispatch;
	}
	// 82DBCA2C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCA30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBCA34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCA38: 40990034  ble cr6, 0x82dbca6c
	if !ctx.cr[6].gt {
	pc = 0x82DBCA6C; continue 'dispatch;
	}
	// 82DBCA3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBCA40: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCA44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCA48: 88EB0011  lbz r7, 0x11(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DBCA4C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DBCA50: 409A0008  bne cr6, 0x82dbca58
	if !ctx.cr[6].eq {
	pc = 0x82DBCA58; continue 'dispatch;
	}
	// 82DBCA54: 990B0011  stb r8, 0x11(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(17 as u32), ctx.r[8].u8 ) };
	// 82DBCA58: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCA5C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBCA60: 394A0038  addi r10, r10, 0x38
	ctx.r[10].s64 = ctx.r[10].s64 + 56;
	// 82DBCA64: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCA68: 4198FFD8  blt cr6, 0x82dbca40
	if ctx.cr[6].lt {
	pc = 0x82DBCA40; continue 'dispatch;
	}
	// 82DBCA6C: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82DBCA70: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBCA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBCA78 size=364
    let mut pc: u32 = 0x82DBCA78;
    'dispatch: loop {
        match pc {
            0x82DBCA78 => {
    //   block [0x82DBCA78..0x82DBCBE4)
	// 82DBCA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBCA7C: 4BEEC971  bl 0x82ca93ec
	ctx.lr = 0x82DBCA80;
	sub_82CA93D0(ctx, base);
	// 82DBCA80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBCA84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBCA88: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82DBCA8C: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82DBCA90: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBCA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBCA98: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCA9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCAA0: 40990038  ble cr6, 0x82dbcad8
	if !ctx.cr[6].gt {
	pc = 0x82DBCAD8; continue 'dispatch;
	}
	// 82DBCAA4: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBCAAC: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCAB0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBCAB4: 93EA0034  stw r31, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 82DBCAB8: 811D0038  lwz r8, 0x38(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCABC: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCAC0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DBCAC4: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCAC8: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 82DBCACC: 8108001C  lwz r8, 0x1c(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBCAD0: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82DBCAD4: 4198FFD8  blt cr6, 0x82dbcaac
	if ctx.cr[6].lt {
	pc = 0x82DBCAAC; continue 'dispatch;
	}
	// 82DBCAD8: 3AFD0040  addi r23, r29, 0x40
	ctx.r[23].s64 = ctx.r[29].s64 + 64;
	// 82DBCADC: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCAE0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCAE4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBCAE8: 40980014  bge cr6, 0x82dbcafc
	if !ctx.cr[6].lt {
	pc = 0x82DBCAFC; continue 'dispatch;
	}
	// 82DBCAEC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DBCAF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBCAF4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DBCAF8: 4BF9A419  bl 0x82d56f10
	ctx.lr = 0x82DBCAFC;
	sub_82D56F10(ctx, base);
	// 82DBCAFC: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCB00: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCB04: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBCB08: 40980024  bge cr6, 0x82dbcb2c
	if !ctx.cr[6].lt {
	pc = 0x82DBCB2C; continue 'dispatch;
	}
	// 82DBCB0C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBCB10: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCB14: 41980008  blt cr6, 0x82dbcb1c
	if ctx.cr[6].lt {
	pc = 0x82DBCB1C; continue 'dispatch;
	}
	// 82DBCB18: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DBCB1C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DBCB20: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBCB24: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DBCB28: 4BF9A3E9  bl 0x82d56f10
	ctx.lr = 0x82DBCB2C;
	sub_82D56F10(ctx, base);
	// 82DBCB2C: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBCB30: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DBCB34: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCB38: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82DBCB3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCB40: 40990090  ble cr6, 0x82dbcbd0
	if !ctx.cr[6].gt {
	pc = 0x82DBCBD0; continue 'dispatch;
	}
	// 82DBCB44: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DBCB48: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCB4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DBCB50: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82DBCB54: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBCB58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBCB5C: 40990060  ble cr6, 0x82dbcbbc
	if !ctx.cr[6].gt {
	pc = 0x82DBCBBC; continue 'dispatch;
	}
	// 82DBCB60: 3975FFFB  addi r11, r21, -5
	ctx.r[11].s64 = ctx.r[21].s64 + -5;
	// 82DBCB64: 577E083C  slwi r30, r27, 1
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DBCB68: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DBCB6C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBCB70: 69790001  xori r25, r11, 1
	ctx.r[25].u64 = ctx.r[11].u64 ^ 1;
	// 82DBCB74: 815D0030  lwz r10, 0x30(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBCB78: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DBCB7C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCB80: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82DBCB84: 214A0020  subfic r10, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[10].s64;
	// 82DBCB88: 7F8BF214  add r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DBCB8C: 7F0B5030  slw r11, r24, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[24].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCB90: 7D63FB78  or r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 82DBCB94: 48025EDD  bl 0x82de2a70
	ctx.lr = 0x82DBCB98;
	sub_82DE2A70(ctx, base);
	// 82DBCB98: B07C0000  sth r3, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 82DBCB9C: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCBA0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBCBA4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DBCBA8: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 82DBCBAC: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DBCBB0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBCBB4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCBB8: 4198FFBC  blt cr6, 0x82dbcb74
	if ctx.cr[6].lt {
	pc = 0x82DBCB74; continue 'dispatch;
	}
	// 82DBCBBC: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCBC0: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82DBCBC4: 3B5A0038  addi r26, r26, 0x38
	ctx.r[26].s64 = ctx.r[26].s64 + 56;
	// 82DBCBC8: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCBCC: 4198FF7C  blt cr6, 0x82dbcb48
	if ctx.cr[6].lt {
	pc = 0x82DBCB48; continue 'dispatch;
	}
	// 82DBCBD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DBCBD4: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82DBCBD8: 9ABD004C  stb r21, 0x4c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(76 as u32), ctx.r[21].u8 ) };
	// 82DBCBDC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DBCBE0: 4BEEC85C  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBCBE8 size=560
    let mut pc: u32 = 0x82DBCBE8;
    'dispatch: loop {
        match pc {
            0x82DBCBE8 => {
    //   block [0x82DBCBE8..0x82DBCE18)
	// 82DBCBE8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DBCBEC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DBCBF0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBCBF4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82DBCBF8: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBCBFC: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 82DBCC00: 7D2B5C30  srw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCC04: 7D662038  and r6, r11, r4
	ctx.r[6].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 82DBCC08: 7C8B4430  srw r11, r4, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[4].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCC0C: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 82DBCC10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBCC14: 892B0010  lbz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DBCC18: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBCC1C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82DBCC20: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBCC24: 7D4A3038  and r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[6].u64;
	// 82DBCC28: 409A0054  bne cr6, 0x82dbcc7c
	if !ctx.cr[6].eq {
	pc = 0x82DBCC7C; continue 'dispatch;
	}
	// 82DBCC2C: 69440001  xori r4, r10, 1
	ctx.r[4].u64 = ctx.r[10].u64 ^ 1;
	// 82DBCC30: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBCC34: 7CE931D6  mullw r7, r9, r6
	ctx.r[7].s64 = (ctx.r[9].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82DBCC38: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCC3C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCC40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DBCC44: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82DBCC48: 7CE7FA14  add r7, r7, r31
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82DBCC4C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBCC50: 5484083C  slwi r4, r4, 1
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DBCC54: A3E70000  lhz r31, 0(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCC58: 7FCA3A2E  lhzx r30, r10, r7
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DBCC5C: 7C843A2E  lhzx r4, r4, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DBCC60: 7D5F49D6  mullw r10, r31, r9
	ctx.r[10].s64 = (ctx.r[31].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82DBCC64: 7CFE49D6  mullw r7, r30, r9
	ctx.r[7].s64 = (ctx.r[30].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82DBCC68: 7D2449D6  mullw r9, r4, r9
	ctx.r[9].s64 = (ctx.r[4].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82DBCC6C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DBCC70: 7CE74214  add r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82DBCC74: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DBCC78: 48000050  b 0x82dbccc8
	pc = 0x82DBCCC8; continue 'dispatch;
	// 82DBCC7C: 69470001  xori r7, r10, 1
	ctx.r[7].u64 = ctx.r[10].u64 ^ 1;
	// 82DBCC80: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBCC84: 7D0931D6  mullw r8, r9, r6
	ctx.r[8].s64 = (ctx.r[9].s32 as i64) * (ctx.r[6].s32 as i64);
	// 82DBCC88: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCC8C: 3BEA0001  addi r31, r10, 1
	ctx.r[31].s64 = ctx.r[10].s64 + 1;
	// 82DBCC90: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DBCC94: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCC98: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82DBCC9C: 57E4103A  slwi r4, r31, 2
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DBCCA0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DBCCA4: 83E80000  lwz r31, 0(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCCA8: 7FC4402E  lwzx r30, r4, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DBCCAC: 7D07402E  lwzx r8, r7, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DBCCB0: 7C9F51D6  mullw r4, r31, r10
	ctx.r[4].s64 = (ctx.r[31].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82DBCCB4: 7CFE51D6  mullw r7, r30, r10
	ctx.r[7].s64 = (ctx.r[30].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82DBCCB8: 7D0851D6  mullw r8, r8, r10
	ctx.r[8].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82DBCCBC: 7D444A14  add r10, r4, r9
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[9].u64;
	// 82DBCCC0: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82DBCCC4: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82DBCCC8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DBCCCC: C1A30014  lfs f13, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBCCD0: C1630018  lfs f11, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DBCCD4: 80830034  lwz r4, 0x34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCCD8: C12A0008  lfs f9, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DBCCDC: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBCCE0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DBCCE4: C1070000  lfs f8, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DBCCE8: C1880C18  lfs f12, 0xc18(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DBCCEC: D181FFCC  stfs f12, -0x34(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 82DBCCF0: D181FFDC  stfs f12, -0x24(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82DBCCF4: D181FFEC  stfs f12, -0x14(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82DBCCF8: C18A0004  lfs f12, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DBCCFC: ED8C0372  fmuls f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DBCD00: D181FFC4  stfs f12, -0x3c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82DBCD04: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DBCD08: ED8902F2  fmuls f12, f9, f11
	ctx.f[12].f64 = (((ctx.f[9].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DBCD0C: C0A90000  lfs f5, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82DBCD10: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBCD14: D181FFC8  stfs f12, -0x38(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 82DBCD18: ED800232  fmuls f12, f0, f8
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[8].f64) as f32) as f64);
	// 82DBCD1C: C0E70004  lfs f7, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82DBCD20: EC000172  fmuls f0, f0, f5
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[5].f64) as f32) as f64);
	// 82DBCD24: C0890004  lfs f4, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82DBCD28: D181FFD0  stfs f12, -0x30(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82DBCD2C: ED870372  fmuls f12, f7, f13
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DBCD30: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82DBCD34: EC040372  fmuls f0, f4, f13
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DBCD38: C0C70008  lfs f6, 8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82DBCD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DBCD40: C0690008  lfs f3, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82DBCD44: D181FFD4  stfs f12, -0x2c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82DBCD48: ED8602F2  fmuls f12, f6, f11
	ctx.f[12].f64 = (((ctx.f[6].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DBCD4C: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82DBCD50: EC0302F2  fmuls f0, f3, f11
	ctx.f[0].f64 = (((ctx.f[3].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DBCD54: D141FFC0  stfs f10, -0x40(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82DBCD58: D181FFD8  stfs f12, -0x28(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82DBCD5C: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82DBCD60: 409A000C  bne cr6, 0x82dbcd6c
	if !ctx.cr[6].eq {
	pc = 0x82DBCD6C; continue 'dispatch;
	}
	// 82DBCD64: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DBCD68: 48000018  b 0x82dbcd80
	pc = 0x82DBCD80; continue 'dispatch;
	// 82DBCD6C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCD70: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBCD74: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82DBCD78: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBCD7C: 7D0B522E  lhzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBCD80: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DBCD84: 419A0054  beq cr6, 0x82dbcdd8
	if ctx.cr[6].eq {
	pc = 0x82DBCDD8; continue 'dispatch;
	}
	// 82DBCD88: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DBCD8C: 8943003C  lbz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBCD90: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DBCD94: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBCD98: 39294CA4  addi r9, r9, 0x4ca4
	ctx.r[9].s64 = ctx.r[9].s64 + 19620;
	// 82DBCD9C: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBCDA0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82DBCDA4: 90E50008  stw r7, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DBCDA8: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DBCDAC: B1050014  sth r8, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[8].u16 ) };
	// 82DBCDB0: 99450016  stb r10, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[10].u8 ) };
	// 82DBCDB4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DBCDB8: B0C50006  sth r6, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82DBCDBC: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBCDC0: 9085000C  stw r4, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBCE18 size=160
    let mut pc: u32 = 0x82DBCE18;
    'dispatch: loop {
        match pc {
            0x82DBCE18 => {
    //   block [0x82DBCE18..0x82DBCEB8)
	// 82DBCE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBCE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBCE20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBCE24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBCE28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBCE2C: 3BE30034  addi r31, r3, 0x34
	ctx.r[31].s64 = ctx.r[3].s64 + 52;
	// 82DBCE30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBCE34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCE38: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCE3C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCE40: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBCE44: 409A0010  bne cr6, 0x82dbce54
	if !ctx.cr[6].eq {
	pc = 0x82DBCE54; continue 'dispatch;
	}
	// 82DBCE48: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DBCE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCE50: 4BF9A149  bl 0x82d56f98
	ctx.lr = 0x82DBCE54;
	sub_82D56F98(ctx, base);
	// 82DBCE54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCE58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBCE5C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCE60: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 82DBCE64: 1D4B0038  mulli r10, r11, 0x38
	ctx.r[10].s64 = ctx.r[11].s64 * 56;
	// 82DBCE68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBCE6C: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DBCE70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DBCE74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DBCE78: 4BEEC609  bl 0x82ca9480
	ctx.lr = 0x82DBCE7C;
	sub_82CA9480(ctx, base);
	// 82DBCE7C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBCE80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBCE84: 409A001C  bne cr6, 0x82dbcea0
	if !ctx.cr[6].eq {
	pc = 0x82DBCEA0; continue 'dispatch;
	}
	// 82DBCE88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DBCE8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBCE90: 396B4C30  addi r11, r11, 0x4c30
	ctx.r[11].s64 = ctx.r[11].s64 + 19504;
	// 82DBCE94: 915E0030  stw r10, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DBCE98: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DBCE9C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DBCEA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBCEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBCEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBCEAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBCEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBCEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBCEB8 size=176
    let mut pc: u32 = 0x82DBCEB8;
    'dispatch: loop {
        match pc {
            0x82DBCEB8 => {
    //   block [0x82DBCEB8..0x82DBCF68)
	// 82DBCEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBCEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBCEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBCEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBCEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBCECC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBCED0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBCED4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBCED8: 388B29E8  addi r4, r11, 0x29e8
	ctx.r[4].s64 = ctx.r[11].s64 + 10728;
	// 82DBCEDC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBCEE0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCEE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBCEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCEEC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBCEF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBCEF4: 4E800421  bctrl
	ctx.lr = 0x82DBCEF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBCEF8: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBCEFC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBCF00: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBCF04: 409A0038  bne cr6, 0x82dbcf3c
	if !ctx.cr[6].eq {
	pc = 0x82DBCF3C; continue 'dispatch;
	}
	// 82DBCF08: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCF0C: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBCF10: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBCF14: 813E0038  lwz r9, 0x38(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DBCF18: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCF1C: 1D080038  mulli r8, r8, 0x38
	ctx.r[8].s64 = ctx.r[8].s64 * 56;
	// 82DBCF20: 388B2C84  addi r4, r11, 0x2c84
	ctx.r[4].s64 = ctx.r[11].s64 + 11396;
	// 82DBCF24: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBCF28: 1CE90038  mulli r7, r9, 0x38
	ctx.r[7].s64 = ctx.r[9].s64 * 56;
	// 82DBCF2C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBCF30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCF34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBCF38: 4E800421  bctrl
	ctx.lr = 0x82DBCF3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBCF3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBCF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBCF44: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBCF48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBCF4C: 4E800421  bctrl
	ctx.lr = 0x82DBCF50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBCF50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBCF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBCF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBCF5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBCF60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBCF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBCF68 size=76
    let mut pc: u32 = 0x82DBCF68;
    'dispatch: loop {
        match pc {
            0x82DBCF68 => {
    //   block [0x82DBCF68..0x82DBCFB4)
	// 82DBCF68: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBCF6C: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBCF70: 216A0020  subfic r11, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[10].s64;
	// 82DBCF74: 7C8B5C30  srw r11, r4, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[4].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCF78: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 82DBCF7C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DBCF80: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBCF84: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DBCF88: 419A004C  beq cr6, 0x82dbcfd4
	if ctx.cr[6].eq {
		sub_82DBCFD4(ctx, base);
		return;
	}
	// 82DBCF8C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82DBCF90: 80EB0024  lwz r7, 0x24(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBCF94: 88CB0011  lbz r6, 0x11(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DBCF98: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DBCF9C: 7D0A5430  srw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82DBCFA0: 7D4A2038  and r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 82DBCFA4: 7D4A39D6  mullw r10, r10, r7
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82DBCFA8: 409A000C  bne cr6, 0x82dbcfb4
	if !ctx.cr[6].eq {
		sub_82DBCFB4(ctx, base);
		return;
	}
	// 82DBCFAC: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DBCFB0: 48000008  b 0x82dbcfb8
	sub_82DBCFB4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCFB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBCFB4 size=32
    let mut pc: u32 = 0x82DBCFB4;
    'dispatch: loop {
        match pc {
            0x82DBCFB4 => {
    //   block [0x82DBCFB4..0x82DBCFD4)
	// 82DBCFB4: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DBCFB8: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 82DBCFBC: 419A0018  beq cr6, 0x82dbcfd4
	if ctx.cr[6].eq {
		sub_82DBCFD4(ctx, base);
		return;
	}
	// 82DBCFC0: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBCFC4: 812B0028  lwz r9, 0x28(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBCFC8: 7D6851D6  mullw r11, r8, r10
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82DBCFCC: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DBCFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCFD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBCFD4 size=8
    let mut pc: u32 = 0x82DBCFD4;
    'dispatch: loop {
        match pc {
            0x82DBCFD4 => {
    //   block [0x82DBCFD4..0x82DBCFDC)
	// 82DBCFD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DBCFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBCFE0 size=4
    let mut pc: u32 = 0x82DBCFE0;
    'dispatch: loop {
        match pc {
            0x82DBCFE0 => {
    //   block [0x82DBCFE0..0x82DBCFE4)
	// 82DBCFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBCFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBCFE8 size=84
    let mut pc: u32 = 0x82DBCFE8;
    'dispatch: loop {
        match pc {
            0x82DBCFE8 => {
    //   block [0x82DBCFE8..0x82DBD03C)
	// 82DBCFE8: 7C6B1E70  srawi r11, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 82DBCFEC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82DBCFF0: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBCFF4: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82DBCFF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBCFFC: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBD000: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DBD004: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DBD008: C00B0BF8  lfs f0, 0xbf8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3064 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD00C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBD010: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DBD014: C00B2CA0  lfs f0, 0x2ca0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11424 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD018: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DBD01C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBD020: D00BB388  stfs f0, -0x4c78(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19576 as u32), tmp.u32 ) };
	// 82DBD024: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD028: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD02C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DBD030: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DBD034: D00BB38C  stfs f0, -0x4c74(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-19572 as u32), tmp.u32 ) };
	// 82DBD038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD040 size=8
    let mut pc: u32 = 0x82DBD040;
    'dispatch: loop {
        match pc {
            0x82DBD040 => {
    //   block [0x82DBD040..0x82DBD048)
	// 82DBD040: C0230014  lfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBD044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD048 size=8
    let mut pc: u32 = 0x82DBD048;
    'dispatch: loop {
        match pc {
            0x82DBD048 => {
    //   block [0x82DBD048..0x82DBD050)
	// 82DBD048: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 82DBD04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBD050 size=96
    let mut pc: u32 = 0x82DBD050;
    'dispatch: loop {
        match pc {
            0x82DBD050 => {
    //   block [0x82DBD050..0x82DBD0B0)
	// 82DBD050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBD058: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBD05C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBD060: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBD064: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBD068: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DBD06C: 388B2CA4  addi r4, r11, 0x2ca4
	ctx.r[4].s64 = ctx.r[11].s64 + 11428;
	// 82DBD070: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBD074: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBD078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBD07C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBD080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBD084: 4E800421  bctrl
	ctx.lr = 0x82DBD088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBD088: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBD08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBD090: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBD094: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBD098: 4E800421  bctrl
	ctx.lr = 0x82DBD09C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBD09C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBD0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBD0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBD0A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBD0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD0B0 size=48
    let mut pc: u32 = 0x82DBD0B0;
    'dispatch: loop {
        match pc {
            0x82DBD0B0 => {
    //   block [0x82DBD0B0..0x82DBD0E0)
	// 82DBD0B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD0B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DBD0B8: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBD0BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD0C0: C1AA0C80  lfs f13, 0xc80(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3200 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD0C4: C00B0BE8  lfs f0, 0xbe8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD0C8: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DBD0CC: FD80081E  fctiwz f12, f1
	ctx.f[12].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 82DBD0D0: 7D805FAE  stfiwx f12, 0, r11
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82DBD0D4: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82DBD0D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBD0DC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD0E0 size=24
    let mut pc: u32 = 0x82DBD0E0;
    'dispatch: loop {
        match pc {
            0x82DBD0E0 => {
    //   block [0x82DBD0E0..0x82DBD0F8)
	// 82DBD0E0: EC21002A  fadds f1, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DBD0E4: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82DBD0E8: 4198FFE0  blt cr6, 0x82dbd0c8
	if ctx.cr[6].lt {
		sub_82DBD0B0(ctx, base);
		return;
	}
	// 82DBD0EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD0F0: C02B0C14  lfs f1, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DBD0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD0F8 size=292
    let mut pc: u32 = 0x82DBD0F8;
    'dispatch: loop {
        match pc {
            0x82DBD0F8 => {
    //   block [0x82DBD0F8..0x82DBD21C)
	// 82DBD0F8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DBD0FC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82DBD100: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 82DBD104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DBD108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD220 size=32
    let mut pc: u32 = 0x82DBD220;
    'dispatch: loop {
        match pc {
            0x82DBD220 => {
    //   block [0x82DBD220..0x82DBD240)
	// 82DBD220: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD224: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DBD228: D0230014  stfs f1, 0x14(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DBD22C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBD230: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD234: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DBD238: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82DBD23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD240 size=152
    let mut pc: u32 = 0x82DBD240;
    'dispatch: loop {
        match pc {
            0x82DBD240 => {
    //   block [0x82DBD240..0x82DBD2D8)
	// 82DBD240: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD2D8 size=40
    let mut pc: u32 = 0x82DBD2D8;
    'dispatch: loop {
        match pc {
            0x82DBD2D8 => {
    //   block [0x82DBD2D8..0x82DBD300)
	// 82DBD2D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD2DC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DBD2E0: 396B0C18  addi r11, r11, 0xc18
	ctx.r[11].s64 = ctx.r[11].s64 + 3096;
	// 82DBD2E4: C14B0000  lfs f10, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DBD2E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD2EC: FDA05090  fmr f13, f10
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82DBD2F0: 396B0C14  addi r11, r11, 0xc14
	ctx.r[11].s64 = ctx.r[11].s64 + 3092;
	// 82DBD2F4: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DBD2F8: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 82DBD2FC: 48000008  b 0x82dbd304
	sub_82DBD300(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD300 size=244
    let mut pc: u32 = 0x82DBD300;
    'dispatch: loop {
        match pc {
            0x82DBD300 => {
    //   block [0x82DBD300..0x82DBD3F4)
	// 82DBD300: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DBD304: FF005000  fcmpu cr6, f0, f10
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82DBD308: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DBD30C: 40980008  bge cr6, 0x82dbd314
	if !ctx.cr[6].lt {
	pc = 0x82DBD314; continue 'dispatch;
	}
	// 82DBD310: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DBD314: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82DBD318: FD600210  fabs f11, f0
	ctx.f[11].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82DBD31C: FF0D5800  fcmpu cr6, f13, f11
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[11].f64);
	// 82DBD320: 419900D4  bgt cr6, 0x82dbd3f4
	if ctx.cr[6].gt {
		sub_82DBD3F4(ctx, base);
		return;
	}
	// 82DBD324: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DBD328: FD606890  fmr f11, f13
	ctx.f[11].f64 = ctx.f[13].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD3F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD3F4 size=8
    let mut pc: u32 = 0x82DBD3F4;
    'dispatch: loop {
        match pc {
            0x82DBD3F4 => {
    //   block [0x82DBD3F4..0x82DBD3FC)
	// 82DBD3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBD3F8: 4BFFFF34  b 0x82dbd32c
	sub_82DBD300(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD3FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD3FC size=124
    let mut pc: u32 = 0x82DBD3FC;
    'dispatch: loop {
        match pc {
            0x82DBD3FC => {
    //   block [0x82DBD3FC..0x82DBD478)
	// 82DBD3FC: FD800090  fmr f12, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].f64 = ctx.f[0].f64;
	// 82DBD400: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DBD404: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DBD408: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DBD40C: D181FFF8  stfs f12, -8(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DBD410: 409A000C  bne cr6, 0x82dbd41c
	if !ctx.cr[6].eq {
	pc = 0x82DBD41C; continue 'dispatch;
	}
	// 82DBD414: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DBD418: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DBD41C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DBD420: 409A000C  bne cr6, 0x82dbd42c
	if !ctx.cr[6].eq {
	pc = 0x82DBD42C; continue 'dispatch;
	}
	// 82DBD424: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DBD428: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DBD42C: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82DBD430: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 82DBD434: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82DBD438: 21290003  subfic r9, r9, 3
	ctx.xer.ca = ctx.r[9].u32 <= 3 as u32;
	ctx.r[9].s64 = (3 as i64) - ctx.r[9].s64;
	// 82DBD43C: 654A3F00  oris r10, r10, 0x3f00
	ctx.r[10].u64 = ctx.r[10].u64 | 1056964608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD478 size=276
    let mut pc: u32 = 0x82DBD478;
    'dispatch: loop {
        match pc {
            0x82DBD478 => {
    //   block [0x82DBD478..0x82DBD58C)
	// 82DBD478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD47C: 4BEEBF89  bl 0x82ca9404
	ctx.lr = 0x82DBD480;
	sub_82CA93D0(ctx, base);
	// 82DBD480: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 82DBD484: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBD488: 41980100  blt cr6, 0x82dbd588
	if ctx.cr[6].lt {
	pc = 0x82DBD588; continue 'dispatch;
	}
	// 82DBD48C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DBD490: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD494: 38A30014  addi r5, r3, 0x14
	ctx.r[5].s64 = ctx.r[3].s64 + 20;
	// 82DBD498: 3BE30050  addi r31, r3, 0x50
	ctx.r[31].s64 = ctx.r[3].s64 + 80;
	// 82DBD49C: 3BC30040  addi r30, r3, 0x40
	ctx.r[30].s64 = ctx.r[3].s64 + 64;
	// 82DBD4A0: C1490C14  lfs f10, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DBD4A4: 3FA08330  lis r29, -0x7cd0
	ctx.r[29].s64 = -2094006272;
	// 82DBD4A8: C16B0BFC  lfs f11, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DBD4AC: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBD4B0: 557C073E  clrlwi r28, r11, 0x1c
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DBD4B4: 557B06F6  rlwinm r27, r11, 0, 0x1b, 0x1b
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBD4B8: 5569CFFE  rlwinm r9, r11, 0x19, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82DBD4BC: 5568D7FE  rlwinm r8, r11, 0x1a, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82DBD4C0: 5567DFFE  rlwinm r7, r11, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DBD4C4: FB81FFC8  std r28, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[28].u64 ) };
	// 82DBD4C8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DBD4CC: C801FFC8  lfd f0, -0x38(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DBD4D0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DBD4D4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DBD4D8: EDA0582A  fadds f13, f0, f11
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64;
	// 82DBD4DC: C01DB38C  lfs f0, -0x4c74(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD4E0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBD4E4: EDA0503C  fnmsubs f13, f0, f0, f10
	ctx.f[13].f64 = -(((ctx.f[0].f64 * ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82DBD4E8: EDA0682C  fsqrts f13, f13
	ctx.f[13].f64 = ((ctx.f[13].f64).sqrt() as f32) as f64;
	// 82DBD4EC: 419A000C  beq cr6, 0x82dbd4f8
	if ctx.cr[6].eq {
	pc = 0x82DBD4F8; continue 'dispatch;
	}
	// 82DBD4F0: FD806890  fmr f12, f13
	ctx.f[12].f64 = ctx.f[13].f64;
	// 82DBD4F4: 4800000C  b 0x82dbd500
	pc = 0x82DBD500; continue 'dispatch;
	// 82DBD4F8: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 82DBD4FC: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DBD500: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82DBD504: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DBD508: D181FFC4  stfs f12, -0x3c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82DBD50C: 409A000C  bne cr6, 0x82dbd518
	if !ctx.cr[6].eq {
	pc = 0x82DBD518; continue 'dispatch;
	}
	// 82DBD510: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DBD514: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82DBD518: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DBD51C: 409A000C  bne cr6, 0x82dbd528
	if !ctx.cr[6].eq {
	pc = 0x82DBD528; continue 'dispatch;
	}
	// 82DBD520: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DBD524: D001FFC4  stfs f0, -0x3c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82DBD528: 3901FFC0  addi r8, r1, -0x40
	ctx.r[8].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD590 size=56
    let mut pc: u32 = 0x82DBD590;
    'dispatch: loop {
        match pc {
            0x82DBD590 => {
    //   block [0x82DBD590..0x82DBD5C8)
	// 82DBD590: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD594: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DBD598: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82DBD59C: C00B0BFC  lfs f0, 0xbfc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD5A0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD5C8 size=16
    let mut pc: u32 = 0x82DBD5C8;
    'dispatch: loop {
        match pc {
            0x82DBD5C8 => {
    //   block [0x82DBD5C8..0x82DBD5D8)
	// 82DBD5C8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBD5D8 size=592
    let mut pc: u32 = 0x82DBD5D8;
    'dispatch: loop {
        match pc {
            0x82DBD5D8 => {
    //   block [0x82DBD5D8..0x82DBD828)
	// 82DBD5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD5DC: 4BEEBE19  bl 0x82ca93f4
	ctx.lr = 0x82DBD5E0;
	sub_82CA93D0(ctx, base);
	// 82DBD5E0: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82DBD5E4: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD5E8: 39230030  addi r9, r3, 0x30
	ctx.r[9].s64 = ctx.r[3].s64 + 48;
	// 82DBD5EC: ED400032  fmuls f10, f0, f0
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DBD5F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD5F4: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 82DBD5F8: D181FF90  stfs f12, -0x70(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBD828 size=184
    let mut pc: u32 = 0x82DBD828;
    'dispatch: loop {
        match pc {
            0x82DBD828 => {
    //   block [0x82DBD828..0x82DBD8E0)
	// 82DBD828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBD830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBD834: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD838: D0430010  stfs f2, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBD83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBD840: FD600890  fmr f11, f1
	ctx.f[11].f64 = ctx.f[1].f64;
	// 82DBD844: 396B4C14  addi r11, r11, 0x4c14
	ctx.r[11].s64 = ctx.r[11].s64 + 19476;
	// 82DBD848: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBD84C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82DBD850: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DBD854: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 82DBD858: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBD85C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD860: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DBD864: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DBD868: C009B390  lfs f0, -0x4c70(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD86C: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD870: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBD874: 40980018  bge cr6, 0x82dbd88c
	if !ctx.cr[6].lt {
	pc = 0x82DBD88C; continue 'dispatch;
	}
	// 82DBD878: 4BFFF839  bl 0x82dbd0b0
	ctx.lr = 0x82DBD87C;
	sub_82DBD0B0(ctx, base);
	// 82DBD87C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD880: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD884: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DBD888: D009B390  stfs f0, -0x4c70(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-19568 as u32), tmp.u32 ) };
	// 82DBD88C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBD8E0 size=112
    let mut pc: u32 = 0x82DBD8E0;
    'dispatch: loop {
        match pc {
            0x82DBD8E0 => {
    //   block [0x82DBD8E0..0x82DBD950)
	// 82DBD8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBD8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBD8EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD8F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBD8F4: 396B4C14  addi r11, r11, 0x4c14
	ctx.r[11].s64 = ctx.r[11].s64 + 19476;
	// 82DBD8F8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DBD8FC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DBD900: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DBD904: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBD908: 419A0030  beq cr6, 0x82dbd938
	if ctx.cr[6].eq {
	pc = 0x82DBD938; continue 'dispatch;
	}
	// 82DBD90C: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 82DBD910: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD914: C009B390  lfs f0, -0x4c70(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD918: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBD91C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBD920: 40980018  bge cr6, 0x82dbd938
	if !ctx.cr[6].lt {
	pc = 0x82DBD938; continue 'dispatch;
	}
	// 82DBD924: 4BFFF78D  bl 0x82dbd0b0
	ctx.lr = 0x82DBD928;
	sub_82DBD0B0(ctx, base);
	// 82DBD928: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBD92C: C00B0C14  lfs f0, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBD930: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82DBD934: D009B390  stfs f0, -0x4c70(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-19568 as u32), tmp.u32 ) };
	// 82DBD938: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82DBD93C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DBD940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBD944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBD948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBD94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBD950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBD950 size=436
    let mut pc: u32 = 0x82DBD950;
    'dispatch: loop {
        match pc {
            0x82DBD950 => {
    //   block [0x82DBD950..0x82DBDB04)
	// 82DBD950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBD954: 4BEEBAA5  bl 0x82ca93f8
	ctx.lr = 0x82DBD958;
	sub_82CA93D0(ctx, base);
	// 82DBD958: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82DBD95C: EB640000  ld r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DBD960: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82DBD964: 39240030  addi r9, r4, 0x30
	ctx.r[9].s64 = ctx.r[4].s64 + 48;
	// 82DBD968: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DBD96C: 3901FF60  addi r8, r1, -0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + -160;
	// 82DBD970: 38E1FF70  addi r7, r1, -0x90
	ctx.r[7].s64 = ctx.r[1].s64 + -144;
	// 82DBD974: EB4B0000  ld r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBD978: 38A1FF80  addi r5, r1, -0x80
	ctx.r[5].s64 = ctx.r[1].s64 + -128;
	// 82DBD97C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DBD980: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82DBD984: EB2A0000  ld r25, 0(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DBD988: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DBD98C: FB680000  std r27, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DBD990: 3B81FFA0  addi r28, r1, -0x60
	ctx.r[28].s64 = ctx.r[1].s64 + -96;
	// 82DBD994: F8880008  std r4, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 82DBD998: 3BA30030  addi r29, r3, 0x30
	ctx.r[29].s64 = ctx.r[3].s64 + 48;
	// 82DBD99C: FB470000  std r26, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82DBD9A0: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DBD9A4: 3961FF60  addi r11, r1, -0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + -160;
	// 82DBD9A8: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82DBD9AC: FB250000  std r25, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82DBD9B0: EB090000  ld r24, 0(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DBD9B4: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBDB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DBDB08 size=1420
    let mut pc: u32 = 0x82DBDB08;
    'dispatch: loop {
        match pc {
            0x82DBDB08 => {
    //   block [0x82DBDB08..0x82DBE094)
	// 82DBDB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBDB0C: 4BEEB8F9  bl 0x82ca9404
	ctx.lr = 0x82DBDB10;
	sub_82CA93D0(ctx, base);
	// 82DBDB10: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82DBDB14: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DBDB18: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBDB1C: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBDB20: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DBDB24: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DBDB28: 7FAA4A14  add r29, r10, r9
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DBDB2C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DBDB30: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBDB34: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBDB38: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBDB3C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBDB40: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBDB44: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBDB48: 40980020  bge cr6, 0x82dbdb68
	if !ctx.cr[6].lt {
	pc = 0x82DBDB68; continue 'dispatch;
	}
	// 82DBDB4C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBDB50: 39082CB0  addi r8, r8, 0x2cb0
	ctx.r[8].s64 = ctx.r[8].s64 + 11440;
	// 82DBDB54: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBDB58: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBDB5C: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82DBDB60: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBDB64: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DBDB68: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DBDB6C: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBDB70: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82DBDB74: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBDB78: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DBDB7C: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DBDB80: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82DBDB84: 3B800030  li r28, 0x30
	ctx.r[28].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE098 size=8
    let mut pc: u32 = 0x82DBE098;
    'dispatch: loop {
        match pc {
            0x82DBE098 => {
    //   block [0x82DBE098..0x82DBE0A0)
	// 82DBE098: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82DBE09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE0A0 size=96
    let mut pc: u32 = 0x82DBE0A0;
    'dispatch: loop {
        match pc {
            0x82DBE0A0 => {
    //   block [0x82DBE0A0..0x82DBE100)
	// 82DBE0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE0A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE0AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE0B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBE0B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBE0B8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DBE0BC: 388B2D50  addi r4, r11, 0x2d50
	ctx.r[4].s64 = ctx.r[11].s64 + 11600;
	// 82DBE0C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBE0C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE0C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE0CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE0D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE0D4: 4E800421  bctrl
	ctx.lr = 0x82DBE0D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE0D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE0E0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE0E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE0E8: 4E800421  bctrl
	ctx.lr = 0x82DBE0EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE0EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBE0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBE0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBE0F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBE0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE100 size=8
    let mut pc: u32 = 0x82DBE100;
    'dispatch: loop {
        match pc {
            0x82DBE100 => {
    //   block [0x82DBE100..0x82DBE108)
	// 82DBE100: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82DBE104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE108 size=16
    let mut pc: u32 = 0x82DBE108;
    'dispatch: loop {
        match pc {
            0x82DBE108 => {
    //   block [0x82DBE108..0x82DBE118)
	// 82DBE108: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DBE10C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DBE110: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBE114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE118 size=56
    let mut pc: u32 = 0x82DBE118;
    'dispatch: loop {
        match pc {
            0x82DBE118 => {
    //   block [0x82DBE118..0x82DBE150)
	// 82DBE118: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE150 size=8
    let mut pc: u32 = 0x82DBE150;
    'dispatch: loop {
        match pc {
            0x82DBE150 => {
    //   block [0x82DBE150..0x82DBE158)
	// 82DBE150: D003002C  stfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBE154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE158 size=16
    let mut pc: u32 = 0x82DBE158;
    'dispatch: loop {
        match pc {
            0x82DBE158 => {
    //   block [0x82DBE158..0x82DBE168)
	// 82DBE158: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE168 size=156
    let mut pc: u32 = 0x82DBE168;
    'dispatch: loop {
        match pc {
            0x82DBE168 => {
    //   block [0x82DBE168..0x82DBE204)
	// 82DBE168: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE208 size=204
    let mut pc: u32 = 0x82DBE208;
    'dispatch: loop {
        match pc {
            0x82DBE208 => {
    //   block [0x82DBE208..0x82DBE2D4)
	// 82DBE208: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE2D8 size=16
    let mut pc: u32 = 0x82DBE2D8;
    'dispatch: loop {
        match pc {
            0x82DBE2D8 => {
    //   block [0x82DBE2D8..0x82DBE2E8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE2E8 size=12
    let mut pc: u32 = 0x82DBE2E8;
    'dispatch: loop {
        match pc {
            0x82DBE2E8 => {
    //   block [0x82DBE2E8..0x82DBE2F4)
	// 82DBE2E8: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 82DBE2EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBE2F0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE2F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE2F4 size=68
    let mut pc: u32 = 0x82DBE2F4;
    'dispatch: loop {
        match pc {
            0x82DBE2F4 => {
    //   block [0x82DBE2F4..0x82DBE338)
	// 82DBE2F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBE2F8: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 82DBE2FC: 392A2CD0  addi r9, r10, 0x2cd0
	ctx.r[9].s64 = ctx.r[10].s64 + 11472;
	// 82DBE300: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE338 size=220
    let mut pc: u32 = 0x82DBE338;
    'dispatch: loop {
        match pc {
            0x82DBE338 => {
    //   block [0x82DBE338..0x82DBE414)
	// 82DBE338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE33C: 4BEEB0C9  bl 0x82ca9404
	ctx.lr = 0x82DBE340;
	sub_82CA93D0(ctx, base);
	// 82DBE340: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBE344: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DBE348: 396B2CD0  addi r11, r11, 0x2cd0
	ctx.r[11].s64 = ctx.r[11].s64 + 11472;
	// 82DBE34C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DBE350: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 82DBE354: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DBE358: C00A0010  lfs f0, 0x10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBE35C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE418 size=120
    let mut pc: u32 = 0x82DBE418;
    'dispatch: loop {
        match pc {
            0x82DBE418 => {
    //   block [0x82DBE418..0x82DBE490)
	// 82DBE418: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBE41C: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBE420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBE424: 396B4D6C  addi r11, r11, 0x4d6c
	ctx.r[11].s64 = ctx.r[11].s64 + 19820;
	// 82DBE428: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBE42C: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82DBE430: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DBE434: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBE438: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DBE43C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DBE440: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE444: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DBE448: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE44C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82DBE450: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBE454: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DBE458: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE45C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82DBE460: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBE464: C1A30024  lfs f13, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBE468: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DBE46C: 41980008  blt cr6, 0x82dbe474
	if ctx.cr[6].lt {
	pc = 0x82DBE474; continue 'dispatch;
	}
	// 82DBE470: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DBE474: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DBE478: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBE47C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DBE480: 40980008  bge cr6, 0x82dbe488
	if !ctx.cr[6].lt {
	pc = 0x82DBE488; continue 'dispatch;
	}
	// 82DBE484: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DBE488: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DBE48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBE490 size=908
    let mut pc: u32 = 0x82DBE490;
    'dispatch: loop {
        match pc {
            0x82DBE490 => {
    //   block [0x82DBE490..0x82DBE81C)
	// 82DBE490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE494: 4BEEAF71  bl 0x82ca9404
	ctx.lr = 0x82DBE498;
	sub_82CA93D0(ctx, base);
	// 82DBE498: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE49C: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DBE4A0: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DBE4A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE4A8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE4AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBE4B0: 40980020  bge cr6, 0x82dbe4d0
	if !ctx.cr[6].lt {
	pc = 0x82DBE4D0; continue 'dispatch;
	}
	// 82DBE4B4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBE4B8: 39292D5C  addi r9, r9, 0x2d5c
	ctx.r[9].s64 = ctx.r[9].s64 + 11612;
	// 82DBE4BC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBE4C0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBE4C4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBE4C8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBE4CC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBE4D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DBE4D4: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBE4D8: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 82DBE4DC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DBE4E0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE820 size=144
    let mut pc: u32 = 0x82DBE820;
    'dispatch: loop {
        match pc {
            0x82DBE820 => {
    //   block [0x82DBE820..0x82DBE8B0)
	// 82DBE820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBE82C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBE838: 3BC5FFD0  addi r30, r5, -0x30
	ctx.r[30].s64 = ctx.r[5].s64 + -48;
	// 82DBE83C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DBE840: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE848: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE850: 4E800421  bctrl
	ctx.lr = 0x82DBE854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE854: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBE858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBE85C: 41980038  blt cr6, 0x82dbe894
	if ctx.cr[6].lt {
	pc = 0x82DBE894; continue 'dispatch;
	}
	// 82DBE860: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DBE864: 41990030  bgt cr6, 0x82dbe894
	if ctx.cr[6].gt {
	pc = 0x82DBE894; continue 'dispatch;
	}
	// 82DBE868: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE86C: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82DBE870: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBE874: 409A0014  bne cr6, 0x82dbe888
	if !ctx.cr[6].eq {
	pc = 0x82DBE888; continue 'dispatch;
	}
	// 82DBE878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBE87C: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82DBE880: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82DBE884: 48000014  b 0x82dbe898
	pc = 0x82DBE898; continue 'dispatch;
	// 82DBE888: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82DBE88C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DBE890: 48000008  b 0x82dbe898
	pc = 0x82DBE898; continue 'dispatch;
	// 82DBE894: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DBE898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBE89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBE8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBE8A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBE8A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBE8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE8B0 size=20
    let mut pc: u32 = 0x82DBE8B0;
    'dispatch: loop {
        match pc {
            0x82DBE8B0 => {
    //   block [0x82DBE8B0..0x82DBE8C4)
	// 82DBE8B0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE8B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE8B8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBE8BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE8C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE8C8 size=144
    let mut pc: u32 = 0x82DBE8C8;
    'dispatch: loop {
        match pc {
            0x82DBE8C8 => {
    //   block [0x82DBE8C8..0x82DBE958)
	// 82DBE8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE8D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBE8D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE8DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBE8E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBE8E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBE8E8: 388B2D64  addi r4, r11, 0x2d64
	ctx.r[4].s64 = ctx.r[11].s64 + 11620;
	// 82DBE8EC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBE8F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE8F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBE8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE8FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBE900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE904: 4E800421  bctrl
	ctx.lr = 0x82DBE908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE908: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE90C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DBE910: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE914: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBE918: 388BB3B4  addi r4, r11, -0x4c4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19532;
	// 82DBE91C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE920: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE928: 4E800421  bctrl
	ctx.lr = 0x82DBE92C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE92C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBE934: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE93C: 4E800421  bctrl
	ctx.lr = 0x82DBE940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBE944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBE948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBE94C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DBE950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBE954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBE958 size=8
    let mut pc: u32 = 0x82DBE958;
    'dispatch: loop {
        match pc {
            0x82DBE958 => {
    //   block [0x82DBE958..0x82DBE960)
	// 82DBE958: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DBE95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE960 size=104
    let mut pc: u32 = 0x82DBE960;
    'dispatch: loop {
        match pc {
            0x82DBE960 => {
    //   block [0x82DBE960..0x82DBE9C8)
	// 82DBE960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE968: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBE96C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBE970: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBE978: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DBE97C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBE980: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBE984: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBE988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBE98C: 4E800421  bctrl
	ctx.lr = 0x82DBE990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBE990: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBE9C8 size=164
    let mut pc: u32 = 0x82DBE9C8;
    'dispatch: loop {
        match pc {
            0x82DBE9C8 => {
    //   block [0x82DBE9C8..0x82DBEA6C)
	// 82DBE9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBE9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBE9D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBE9D4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DBE9D8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DBE9DC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DBE9E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DBE9E4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBE9E8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBE9EC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DBE9F0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DBE9F4: 4200FFF0  bdnz 0x82dbe9e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBE9E4; continue 'dispatch;
	}
	// 82DBE9F8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEA70 size=92
    let mut pc: u32 = 0x82DBEA70;
    'dispatch: loop {
        match pc {
            0x82DBEA70 => {
    //   block [0x82DBEA70..0x82DBEACC)
	// 82DBEA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEA78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEA7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBEA80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEA84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEA88: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DBEA8C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEA90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEA94: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DBEA98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEA9C: 4E800421  bctrl
	ctx.lr = 0x82DBEAA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEAA0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEAD0 size=104
    let mut pc: u32 = 0x82DBEAD0;
    'dispatch: loop {
        match pc {
            0x82DBEAD0 => {
    //   block [0x82DBEAD0..0x82DBEB38)
	// 82DBEAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEAD4: 4BEEA939  bl 0x82ca940c
	ctx.lr = 0x82DBEAD8;
	sub_82CA93D0(ctx, base);
	// 82DBEAD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEAE0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DBEAE4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DBEAE8: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEAEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEAF0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DBEAF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEAF8: 4E800421  bctrl
	ctx.lr = 0x82DBEAFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEAFC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DBEB00: 40990030  ble cr6, 0x82dbeb30
	if !ctx.cr[6].gt {
	pc = 0x82DBEB30; continue 'dispatch;
	}
	// 82DBEB04: 393E0020  addi r9, r30, 0x20
	ctx.r[9].s64 = ctx.r[30].s64 + 32;
	// 82DBEB08: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DBEB0C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEB38 size=92
    let mut pc: u32 = 0x82DBEB38;
    'dispatch: loop {
        match pc {
            0x82DBEB38 => {
    //   block [0x82DBEB38..0x82DBEB94)
	// 82DBEB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEB40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEB44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBEB48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEB4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEB50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBEB54: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEB58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEB5C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DBEB60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEB64: 4E800421  bctrl
	ctx.lr = 0x82DBEB68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEB68: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEB98 size=92
    let mut pc: u32 = 0x82DBEB98;
    'dispatch: loop {
        match pc {
            0x82DBEB98 => {
    //   block [0x82DBEB98..0x82DBEBF4)
	// 82DBEB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEBA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEBA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBEBA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEBAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBEBB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBEBB4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEBB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEBBC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBEBC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEBC4: 4E800421  bctrl
	ctx.lr = 0x82DBEBC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEBC8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEBF8 size=136
    let mut pc: u32 = 0x82DBEBF8;
    'dispatch: loop {
        match pc {
            0x82DBEBF8 => {
    //   block [0x82DBEBF8..0x82DBEC80)
	// 82DBEBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEBFC: 4BEEA80D  bl 0x82ca9408
	ctx.lr = 0x82DBEC00;
	sub_82CA93D0(ctx, base);
	// 82DBEC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEC04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBEC08: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DBEC0C: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEC10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEC14: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBEC18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEC1C: 4E800421  bctrl
	ctx.lr = 0x82DBEC20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEC20: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBEC24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBEC28: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DBEC2C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82DBEC30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEC34: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DBEC38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBEC3C: 4E800421  bctrl
	ctx.lr = 0x82DBEC40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEC40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DBEC44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBEC48: 4099002C  ble cr6, 0x82dbec74
	if !ctx.cr[6].gt {
	pc = 0x82DBEC74; continue 'dispatch;
	}
	// 82DBEC4C: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DBEC80 size=116
    let mut pc: u32 = 0x82DBEC80;
    'dispatch: loop {
        match pc {
            0x82DBEC80 => {
    //   block [0x82DBEC80..0x82DBECF4)
	// 82DBEC80: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBEC84: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DBEC88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBEC8C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DBEC90: 396B191C  addi r11, r11, 0x191c
	ctx.r[11].s64 = ctx.r[11].s64 + 6428;
	// 82DBEC94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DBEC98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBEC9C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82DBECA0: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DBECA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBECA8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DBECAC: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DBECB0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBECB4: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBECB8: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82DBECBC: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBECC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBECC4: 419A0010  beq cr6, 0x82dbecd4
	if ctx.cr[6].eq {
	pc = 0x82DBECD4; continue 'dispatch;
	}
	// 82DBECC8: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DBECCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBECD0: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DBECD4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBECF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBECF8 size=132
    let mut pc: u32 = 0x82DBECF8;
    'dispatch: loop {
        match pc {
            0x82DBECF8 => {
    //   block [0x82DBECF8..0x82DBED7C)
	// 82DBECF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBECFC: 4BEEA711  bl 0x82ca940c
	ctx.lr = 0x82DBED00;
	sub_82CA93D0(ctx, base);
	// 82DBED00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBED04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBED08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBED0C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBED10: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBED14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBED18: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DBED1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBED20: 4E800421  bctrl
	ctx.lr = 0x82DBED24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBED24: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBED80 size=304
    let mut pc: u32 = 0x82DBED80;
    'dispatch: loop {
        match pc {
            0x82DBED80 => {
    //   block [0x82DBED80..0x82DBEEB0)
	// 82DBED80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBED84: 4BEEA685  bl 0x82ca9408
	ctx.lr = 0x82DBED88;
	sub_82CA93D0(ctx, base);
	// 82DBED88: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBED8C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBED90: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DBED94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DBED98: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82DBED9C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBEDA0: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DBEDA4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEDA8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBEDAC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBEDB0: 40980020  bge cr6, 0x82dbedd0
	if !ctx.cr[6].lt {
	pc = 0x82DBEDD0; continue 'dispatch;
	}
	// 82DBEDB4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBEDB8: 39082C40  addi r8, r8, 0x2c40
	ctx.r[8].s64 = ctx.r[8].s64 + 11328;
	// 82DBEDBC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBEDC0: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBEDC4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DBEDC8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBEDCC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DBEDD0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBEDD4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82DBEDD8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82DBEDDC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82DBEDE0: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DBEDE4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBEDE8: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DBEDEC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DBEDF0: 4200FFF0  bdnz 0x82dbede0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DBEDE0; continue 'dispatch;
	}
	// 82DBEDF4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DBEDF8: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEEB0 size=320
    let mut pc: u32 = 0x82DBEEB0;
    'dispatch: loop {
        match pc {
            0x82DBEEB0 => {
    //   block [0x82DBEEB0..0x82DBEFF0)
	// 82DBEEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEEB4: 4BEEA549  bl 0x82ca93fc
	ctx.lr = 0x82DBEEB8;
	sub_82CA93D0(ctx, base);
	// 82DBEEB8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBEEBC: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DBEEC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBEEC4: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82DBEEC8: 7FC92214  add r30, r9, r4
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 82DBEECC: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DBEED0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBEED4: 7D6829D6  mullw r11, r8, r5
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82DBEED8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEEDC: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBEEE0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DBEEE4: 419A0100  beq cr6, 0x82dbefe4
	if ctx.cr[6].eq {
	pc = 0x82DBEFE4; continue 'dispatch;
	}
	// 82DBEEE8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBEEEC: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBEEF0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEEF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DBEEF8: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82DBEEFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBEF00: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBEF04: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEF08: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBEF0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DBEF10: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEF14: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DBEF18: 4E800421  bctrl
	ctx.lr = 0x82DBEF1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBEF1C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBEF20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBEF24: 419A00C0  beq cr6, 0x82dbefe4
	if ctx.cr[6].eq {
	pc = 0x82DBEFE4; continue 'dispatch;
	}
	// 82DBEF28: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBEF2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBEF30: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBEF34: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82DBEF38: 394A4C30  addi r10, r10, 0x4c30
	ctx.r[10].s64 = ctx.r[10].s64 + 19504;
	// 82DBEF3C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DBEF40: 38EB0020  addi r7, r11, 0x20
	ctx.r[7].s64 = ctx.r[11].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBEFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBEFF0 size=300
    let mut pc: u32 = 0x82DBEFF0;
    'dispatch: loop {
        match pc {
            0x82DBEFF0 => {
    //   block [0x82DBEFF0..0x82DBF11C)
	// 82DBEFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBEFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBEFF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBEFFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBF000: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF004: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF008: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DBF00C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DBF010: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBF014: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBF018: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF01C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF020: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBF024: 40980020  bge cr6, 0x82dbf044
	if !ctx.cr[6].lt {
	pc = 0x82DBF044; continue 'dispatch;
	}
	// 82DBF028: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBF02C: 39082D74  addi r8, r8, 0x2d74
	ctx.r[8].s64 = ctx.r[8].s64 + 11636;
	// 82DBF030: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBF034: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBF038: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82DBF03C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBF040: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DBF044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF048: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBF04C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF050: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBF054: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82DBF058: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF05C: 409A0008  bne cr6, 0x82dbf064
	if !ctx.cr[6].eq {
	pc = 0x82DBF064; continue 'dispatch;
	}
	// 82DBF060: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBF064: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBF068: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF06C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF070: 419A001C  beq cr6, 0x82dbf08c
	if ctx.cr[6].eq {
	pc = 0x82DBF08C; continue 'dispatch;
	}
	// 82DBF074: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF078: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DBF07C: 409A0008  bne cr6, 0x82dbf084
	if !ctx.cr[6].eq {
	pc = 0x82DBF084; continue 'dispatch;
	}
	// 82DBF080: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBF084: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82DBF088: 48000008  b 0x82dbf090
	pc = 0x82DBF090; continue 'dispatch;
	// 82DBF08C: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82DBF090: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF120 size=276
    let mut pc: u32 = 0x82DBF120;
    'dispatch: loop {
        match pc {
            0x82DBF120 => {
    //   block [0x82DBF120..0x82DBF234)
	// 82DBF120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF124: 4BEEA2E9  bl 0x82ca940c
	ctx.lr = 0x82DBF128;
	sub_82CA93D0(ctx, base);
	// 82DBF128: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF12C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF130: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DBF134: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DBF138: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBF13C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBF140: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF144: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF148: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DBF14C: 40980020  bge cr6, 0x82dbf16c
	if !ctx.cr[6].lt {
	pc = 0x82DBF16C; continue 'dispatch;
	}
	// 82DBF150: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DBF154: 38842D80  addi r4, r4, 0x2d80
	ctx.r[4].s64 = ctx.r[4].s64 + 11648;
	// 82DBF158: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DBF15C: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82DBF160: 3BAA000C  addi r29, r10, 0xc
	ctx.r[29].s64 = ctx.r[10].s64 + 12;
	// 82DBF164: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DBF168: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DBF16C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF170: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBF174: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF178: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DBF17C: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82DBF180: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF184: 409A0008  bne cr6, 0x82dbf18c
	if !ctx.cr[6].eq {
	pc = 0x82DBF18C; continue 'dispatch;
	}
	// 82DBF188: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBF18C: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBF190: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF198: 419A001C  beq cr6, 0x82dbf1b4
	if ctx.cr[6].eq {
	pc = 0x82DBF1B4; continue 'dispatch;
	}
	// 82DBF19C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF1A0: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DBF1A4: 409A0008  bne cr6, 0x82dbf1ac
	if !ctx.cr[6].eq {
	pc = 0x82DBF1AC; continue 'dispatch;
	}
	// 82DBF1A8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBF1AC: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82DBF1B0: 48000008  b 0x82dbf1b8
	pc = 0x82DBF1B8; continue 'dispatch;
	// 82DBF1B4: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82DBF1B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF238 size=472
    let mut pc: u32 = 0x82DBF238;
    'dispatch: loop {
        match pc {
            0x82DBF238 => {
    //   block [0x82DBF238..0x82DBF410)
	// 82DBF238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF23C: 4BEEA1B1  bl 0x82ca93ec
	ctx.lr = 0x82DBF240;
	sub_82CA93D0(ctx, base);
	// 82DBF240: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF244: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF248: 3AA00008  li r21, 8
	ctx.r[21].s64 = 8;
	// 82DBF24C: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82DBF250: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DBF254: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DBF258: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DBF25C: 7D7AA82E  lwzx r11, r26, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82DBF260: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DBF264: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DBF268: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82DBF26C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF270: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF274: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBF278: 40980020  bge cr6, 0x82dbf298
	if !ctx.cr[6].lt {
	pc = 0x82DBF298; continue 'dispatch;
	}
	// 82DBF27C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBF280: 39292D90  addi r9, r9, 0x2d90
	ctx.r[9].s64 = ctx.r[9].s64 + 11664;
	// 82DBF284: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBF288: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DBF28C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DBF290: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DBF294: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBF298: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF410 size=260
    let mut pc: u32 = 0x82DBF410;
    'dispatch: loop {
        match pc {
            0x82DBF410 => {
    //   block [0x82DBF410..0x82DBF514)
	// 82DBF410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF414: 4BEE9FF1  bl 0x82ca9404
	ctx.lr = 0x82DBF418;
	sub_82CA93D0(ctx, base);
	// 82DBF418: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF41C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF420: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82DBF424: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DBF428: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBF42C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF430: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBF434: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DBF438: 40980020  bge cr6, 0x82dbf458
	if !ctx.cr[6].lt {
	pc = 0x82DBF458; continue 'dispatch;
	}
	// 82DBF43C: 3FE08203  lis r31, -0x7dfd
	ctx.r[31].s64 = -2113732608;
	// 82DBF440: 3BFF2DA0  addi r31, r31, 0x2da0
	ctx.r[31].s64 = ctx.r[31].s64 + 11680;
	// 82DBF444: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DBF448: 7FEC42E6  mftb r31, 0x10c
	ctx.r[31].u64 = crate::rt::rdtsc_u64();
	// 82DBF44C: 3B64000C  addi r27, r4, 0xc
	ctx.r[27].s64 = ctx.r[4].s64 + 12;
	// 82DBF450: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBF454: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DBF458: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DBF45C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBF460: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 82DBF464: 409A0008  bne cr6, 0x82dbf46c
	if !ctx.cr[6].eq {
	pc = 0x82DBF46C; continue 'dispatch;
	}
	// 82DBF468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBF46C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBF470: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DBF474: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF478: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF47C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF480: 419A0010  beq cr6, 0x82dbf490
	if ctx.cr[6].eq {
	pc = 0x82DBF490; continue 'dispatch;
	}
	// 82DBF484: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DBF488: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 82DBF48C: 409A0008  bne cr6, 0x82dbf494
	if !ctx.cr[6].eq {
	pc = 0x82DBF494; continue 'dispatch;
	}
	// 82DBF490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBF494: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DBF498: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBF518 size=8
    let mut pc: u32 = 0x82DBF518;
    'dispatch: loop {
        match pc {
            0x82DBF518 => {
    //   block [0x82DBF518..0x82DBF520)
	// 82DBF518: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82DBF51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBF520 size=236
    let mut pc: u32 = 0x82DBF520;
    'dispatch: loop {
        match pc {
            0x82DBF520 => {
    //   block [0x82DBF520..0x82DBF60C)
	// 82DBF520: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBF524: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBF528: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBF52C: 39630100  addi r11, r3, 0x100
	ctx.r[11].s64 = ctx.r[3].s64 + 256;
	// 82DBF530: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DBF534: 394A85F4  addi r10, r10, -0x7a0c
	ctx.r[10].s64 = ctx.r[10].s64 + -31244;
	// 82DBF538: 39292DC4  addi r9, r9, 0x2dc4
	ctx.r[9].s64 = ctx.r[9].s64 + 11716;
	// 82DBF53C: 39082DB4  addi r8, r8, 0x2db4
	ctx.r[8].s64 = ctx.r[8].s64 + 11700;
	// 82DBF540: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 82DBF544: B0AB0006  sth r5, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82DBF548: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82DBF54C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DBF550: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBF554: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DBF558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF55C: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF560: 409A0008  bne cr6, 0x82dbf568
	if !ctx.cr[6].eq {
	pc = 0x82DBF568; continue 'dispatch;
	}
	// 82DBF564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF568: 9147FFF8  stw r10, -8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 82DBF56C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF570: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF574: 409A0008  bne cr6, 0x82dbf57c
	if !ctx.cr[6].eq {
	pc = 0x82DBF57C; continue 'dispatch;
	}
	// 82DBF578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF57C: 9147FFFC  stw r10, -4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 82DBF580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF584: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF588: 409A0008  bne cr6, 0x82dbf590
	if !ctx.cr[6].eq {
	pc = 0x82DBF590; continue 'dispatch;
	}
	// 82DBF58C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF590: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBF594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF598: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF59C: 409A0008  bne cr6, 0x82dbf5a4
	if !ctx.cr[6].eq {
	pc = 0x82DBF5A4; continue 'dispatch;
	}
	// 82DBF5A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF5A4: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DBF5A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5AC: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5B0: 409A0008  bne cr6, 0x82dbf5b8
	if !ctx.cr[6].eq {
	pc = 0x82DBF5B8; continue 'dispatch;
	}
	// 82DBF5B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF5B8: 91470008  stw r10, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DBF5BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5C0: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5C4: 409A0008  bne cr6, 0x82dbf5cc
	if !ctx.cr[6].eq {
	pc = 0x82DBF5CC; continue 'dispatch;
	}
	// 82DBF5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF5CC: 9147000C  stw r10, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DBF5D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5D4: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5D8: 409A0008  bne cr6, 0x82dbf5e0
	if !ctx.cr[6].eq {
	pc = 0x82DBF5E0; continue 'dispatch;
	}
	// 82DBF5DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF5E0: 91470010  stw r10, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DBF5E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF5E8: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 82DBF5EC: 409A0008  bne cr6, 0x82dbf5f4
	if !ctx.cr[6].eq {
	pc = 0x82DBF5F4; continue 'dispatch;
	}
	// 82DBF5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBF5F4: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 82DBF5F8: 91470014  stw r10, 0x14(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DBF5FC: 38E70020  addi r7, r7, 0x20
	ctx.r[7].s64 = ctx.r[7].s64 + 32;
	// 82DBF600: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBF604: 409AFF54  bne cr6, 0x82dbf558
	if !ctx.cr[6].eq {
	pc = 0x82DBF558; continue 'dispatch;
	}
	// 82DBF608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBF610 size=48
    let mut pc: u32 = 0x82DBF610;
    'dispatch: loop {
        match pc {
            0x82DBF610 => {
    //   block [0x82DBF610..0x82DBF640)
	// 82DBF610: 39230100  addi r9, r3, 0x100
	ctx.r[9].s64 = ctx.r[3].s64 + 256;
	// 82DBF614: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DBF618: 39090008  addi r8, r9, 8
	ctx.r[8].s64 = ctx.r[9].s64 + 8;
	// 82DBF61C: 409A0008  bne cr6, 0x82dbf624
	if !ctx.cr[6].eq {
	pc = 0x82DBF624; continue 'dispatch;
	}
	// 82DBF620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DBF624: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBF628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBF62C: 396B85F4  addi r11, r11, -0x7a0c
	ctx.r[11].s64 = ctx.r[11].s64 + -31244;
	// 82DBF630: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DBF634: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBF638: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBF63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF640 size=188
    let mut pc: u32 = 0x82DBF640;
    'dispatch: loop {
        match pc {
            0x82DBF640 => {
    //   block [0x82DBF640..0x82DBF6FC)
	// 82DBF640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF644: 4BEE9DC5  bl 0x82ca9408
	ctx.lr = 0x82DBF648;
	sub_82CA93D0(ctx, base);
	// 82DBF648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF64C: 3B85FFFF  addi r28, r5, -1
	ctx.r[28].s64 = ctx.r[5].s64 + -1;
	// 82DBF650: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBF654: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBF658: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DBF65C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DBF660: 41980094  blt cr6, 0x82dbf6f4
	if ctx.cr[6].lt {
	pc = 0x82DBF6F4; continue 'dispatch;
	}
	// 82DBF664: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF668: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBF66C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DBF674: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF678: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF67C: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DBF680: 88CA0005  lbz r6, 5(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DBF684: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 82DBF688: 7CC90774  extsb r9, r6
	ctx.r[9].s64 = ctx.r[6].s8 as i64;
	// 82DBF68C: 7CA85A14  add r5, r8, r11
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DBF690: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DBF694: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82DBF698: 4E800421  bctrl
	ctx.lr = 0x82DBF69C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBF69C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF6A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBF6A4: 419A0040  beq cr6, 0x82dbf6e4
	if ctx.cr[6].eq {
	pc = 0x82DBF6E4; continue 'dispatch;
	}
	// 82DBF6A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF6AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBF6B0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6B4: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6B8: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6BC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DBF6C0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DBF6C4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF6C8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBF6CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF6D0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBF6D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF6D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBF6E0: 4E800421  bctrl
	ctx.lr = 0x82DBF6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBF6E4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DBF6E8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DBF6EC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82DBF6F0: 4098FF74  bge cr6, 0x82dbf664
	if !ctx.cr[6].lt {
	pc = 0x82DBF664; continue 'dispatch;
	}
	// 82DBF6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DBF6F8: 4BEE9D60  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF700 size=116
    let mut pc: u32 = 0x82DBF700;
    'dispatch: loop {
        match pc {
            0x82DBF700 => {
    //   block [0x82DBF700..0x82DBF774)
	// 82DBF700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF704: 4BEE9D09  bl 0x82ca940c
	ctx.lr = 0x82DBF708;
	sub_82CA93D0(ctx, base);
	// 82DBF708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF70C: 3BC5FFFF  addi r30, r5, -1
	ctx.r[30].s64 = ctx.r[5].s64 + -1;
	// 82DBF710: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DBF714: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DBF718: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DBF71C: 41980050  blt cr6, 0x82dbf76c
	if ctx.cr[6].lt {
	pc = 0x82DBF76C; continue 'dispatch;
	}
	// 82DBF720: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF724: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBF728: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF72C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF730: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF734: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DBF738: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DBF73C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF740: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DBF744: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF748: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DBF74C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF750: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBF754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBF758: 4E800421  bctrl
	ctx.lr = 0x82DBF75C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBF75C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DBF760: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DBF764: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DBF768: 4098FFB8  bge cr6, 0x82dbf720
	if !ctx.cr[6].lt {
	pc = 0x82DBF720; continue 'dispatch;
	}
	// 82DBF76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DBF770: 4BEE9CEC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBF778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBF778 size=1172
    let mut pc: u32 = 0x82DBF778;
    'dispatch: loop {
        match pc {
            0x82DBF778 => {
    //   block [0x82DBF778..0x82DBFC0C)
	// 82DBF778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBF77C: 4BEE9C71  bl 0x82ca93ec
	ctx.lr = 0x82DBF780;
	sub_82CA93D0(ctx, base);
	// 82DBF780: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBF784: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DBF788: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DBF78C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF790: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF794: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DBF798: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF79C: 41980008  blt cr6, 0x82dbf7a4
	if ctx.cr[6].lt {
	pc = 0x82DBF7A4; continue 'dispatch;
	}
	// 82DBF7A0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DBF7A4: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82DBF7A8: 40980154  bge cr6, 0x82dbf8fc
	if !ctx.cr[6].lt {
	pc = 0x82DBF8FC; continue 'dispatch;
	}
	// 82DBF7AC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DBF7B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBF7B4: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82DBF7B8: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82DBF7BC: 40990104  ble cr6, 0x82dbf8c0
	if !ctx.cr[6].gt {
	pc = 0x82DBF8C0; continue 'dispatch;
	}
	// 82DBF7C0: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82DBF7C4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DBF7C8: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF7CC: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 82DBF7D0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DBF7D4: 4099005C  ble cr6, 0x82dbf830
	if !ctx.cr[6].gt {
	pc = 0x82DBF830; continue 'dispatch;
	}
	// 82DBF7D8: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF7DC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF7E0: 7D054A14  add r8, r5, r9
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82DBF7E4: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF7E8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF7EC: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DBF7F0: 409A0014  bne cr6, 0x82dbf804
	if !ctx.cr[6].eq {
	pc = 0x82DBF804; continue 'dispatch;
	}
	// 82DBF7F4: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF7F8: 83A80004  lwz r29, 4(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF7FC: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DBF800: 419A0030  beq cr6, 0x82dbf830
	if ctx.cr[6].eq {
	pc = 0x82DBF830; continue 'dispatch;
	}
	// 82DBF804: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF808: 7F1E3840  cmplw cr6, r30, r7
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DBF80C: 409A0010  bne cr6, 0x82dbf81c
	if !ctx.cr[6].eq {
	pc = 0x82DBF81C; continue 'dispatch;
	}
	// 82DBF810: 83C80004  lwz r30, 4(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF814: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DBF818: 419A0018  beq cr6, 0x82dbf830
	if ctx.cr[6].eq {
	pc = 0x82DBF830; continue 'dispatch;
	}
	// 82DBF81C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF820: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DBF824: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DBF828: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DBF82C: 4198FFBC  blt cr6, 0x82dbf7e8
	if ctx.cr[6].lt {
	pc = 0x82DBF7E8; continue 'dispatch;
	}
	// 82DBF830: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DBF834: 409A0034  bne cr6, 0x82dbf868
	if !ctx.cr[6].eq {
	pc = 0x82DBF868; continue 'dispatch;
	}
	// 82DBF838: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82DBF83C: 419A0020  beq cr6, 0x82dbf85c
	if ctx.cr[6].eq {
	pc = 0x82DBF85C; continue 'dispatch;
	}
	// 82DBF840: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF844: 7D455A14  add r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82DBF848: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 82DBF84C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF850: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBF854: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF858: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DBF85C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBF860: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82DBF864: 48000048  b 0x82dbf8ac
	pc = 0x82DBF8AC; continue 'dispatch;
	// 82DBF868: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 82DBF86C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF870: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DBF874: 40980038  bge cr6, 0x82dbf8ac
	if !ctx.cr[6].lt {
	pc = 0x82DBF8AC; continue 'dispatch;
	}
	// 82DBF878: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DBF87C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF880: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DBF884: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DBF888: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DBF88C: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 82DBF890: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF894: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DBF898: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF89C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBF8A0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF8A4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF8A8: 4198FFD4  blt cr6, 0x82dbf87c
	if ctx.cr[6].lt {
	pc = 0x82DBF87C; continue 'dispatch;
	}
	// 82DBF8AC: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF8B0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82DBF8B4: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 82DBF8B8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF8BC: 4198FF0C  blt cr6, 0x82dbf7c8
	if ctx.cr[6].lt {
	pc = 0x82DBF7C8; continue 'dispatch;
	}
	// 82DBF8C0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBF8C4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBF8C8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBF8CC: 40980024  bge cr6, 0x82dbf8f0
	if !ctx.cr[6].lt {
	pc = 0x82DBF8F0; continue 'dispatch;
	}
	// 82DBF8D0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF8D4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF8D8: 41980008  blt cr6, 0x82dbf8e0
	if ctx.cr[6].lt {
	pc = 0x82DBF8E0; continue 'dispatch;
	}
	// 82DBF8DC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DBF8E0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DBF8E4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBF8E8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DBF8EC: 4BF97625  bl 0x82d56f10
	ctx.lr = 0x82DBF8F0;
	sub_82D56F10(ctx, base);
	// 82DBF8F0: 93F80004  stw r31, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBF8F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DBF8F8: 4BEE9B44  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82DBF8FC: 4BFA0D15  bl 0x82d60610
	ctx.lr = 0x82DBF900;
	sub_82D60610(ctx, base);
	// 82DBF900: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF904: 3AE00004  li r23, 4
	ctx.r[23].s64 = 4;
	// 82DBF908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBF90C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82DBF910: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DBF914: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DBF918: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBF91C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DBF920: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DBF924: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DBF928: 41990010  bgt cr6, 0x82dbf938
	if ctx.cr[6].gt {
	pc = 0x82DBF938; continue 'dispatch;
	}
	// 82DBF92C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82DBF930: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DBF934: 48000018  b 0x82dbf94c
	pc = 0x82DBF94C; continue 'dispatch;
	// 82DBF938: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF93C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DBF940: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBF944: 4E800421  bctrl
	ctx.lr = 0x82DBF948;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBF948: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DBF94C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DBF950: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DBF954: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBF958: 4BFA0C71  bl 0x82d605c8
	ctx.lr = 0x82DBF95C;
	sub_82D605C8(ctx, base);
	// 82DBF95C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBF960: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DBF964: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBF968: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 82DBF96C: 409900B0  ble cr6, 0x82dbfa1c
	if !ctx.cr[6].gt {
	pc = 0x82DBFA1C; continue 'dispatch;
	}
	// 82DBF970: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82DBF974: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF978: 7FDF582A  ldx r30, r31, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 82DBF97C: FBC10050  std r30, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u64 ) };
	// 82DBF980: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DBF984: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DBF988: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DBF98C: 40990010  ble cr6, 0x82dbf99c
	if !ctx.cr[6].gt {
	pc = 0x82DBF99C; continue 'dispatch;
	}
	// 82DBF990: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DBF994: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DBF998: EBC10050  ld r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DBF99C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBF9A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBF9A4: 4BFA102D  bl 0x82d609d0
	ctx.lr = 0x82DBF9A8;
	sub_82D609D0(ctx, base);
	// 82DBF9A8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DBF9AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DBF9B0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBF9B4: 40990008  ble cr6, 0x82dbf9bc
	if !ctx.cr[6].gt {
	pc = 0x82DBF9BC; continue 'dispatch;
	}
	// 82DBF9B8: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 82DBF9BC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DBF9C0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DBF9C4: 419A002C  beq cr6, 0x82dbf9f0
	if ctx.cr[6].eq {
	pc = 0x82DBF9F0; continue 'dispatch;
	}
	// 82DBF9C8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DBF9CC: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DBF9D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBF9D4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF9D8: 7D2B502A  ldx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	// 82DBF9DC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DBF9E0: 7D2B512A  stdx r9, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u64) };
	// 82DBF9E4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBF9E8: 7EBF592E  stwx r21, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 82DBF9EC: 4800001C  b 0x82dbfa08
	pc = 0x82DBFA08; continue 'dispatch;
	// 82DBF9F0: 57AB402E  slwi r11, r29, 8
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBF9F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DBF9F8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DBF9FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFA00: 61650001  ori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 | 1;
	// 82DBFA04: 4BFA0EED  bl 0x82d608f0
	ctx.lr = 0x82DBFA08;
	sub_82D608F0(ctx, base);
	// 82DBFA08: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFA0C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DBFA10: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82DBFA14: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFA18: 4198FF5C  blt cr6, 0x82dbf974
	if ctx.cr[6].lt {
	pc = 0x82DBF974; continue 'dispatch;
	}
	// 82DBFA1C: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFA20: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 82DBFA24: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 82DBFA28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBFA2C: 409900E0  ble cr6, 0x82dbfb0c
	if !ctx.cr[6].gt {
	pc = 0x82DBFB0C; continue 'dispatch;
	}
	// 82DBFA30: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 82DBFA34: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 82DBFA38: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFA3C: 7D7E582A  ldx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) };
	// 82DBFA40: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DBFA44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DBFA48: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DBFA4C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DBFA50: 4099000C  ble cr6, 0x82dbfa5c
	if !ctx.cr[6].gt {
	pc = 0x82DBFA5C; continue 'dispatch;
	}
	// 82DBFA54: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DBFA58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DBFA5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFA60: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DBFA64: 4BFA0F6D  bl 0x82d609d0
	ctx.lr = 0x82DBFA68;
	sub_82D609D0(ctx, base);
	// 82DBFA68: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DBFA6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DBFA70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DBFA74: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DBFA78: 40990008  ble cr6, 0x82dbfa80
	if !ctx.cr[6].gt {
	pc = 0x82DBFA80; continue 'dispatch;
	}
	// 82DBFA7C: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82DBFA80: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DBFA84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBFA88: 419A004C  beq cr6, 0x82dbfad4
	if ctx.cr[6].eq {
	pc = 0x82DBFAD4; continue 'dispatch;
	}
	// 82DBFA8C: 7D6A2214  add r11, r10, r4
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82DBFA90: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DBFA94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DBFA98: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBFA9C: 7D6A482A  ldx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82DBFAA0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DBFAA4: 57E8063E  clrlwi r8, r31, 0x18
	ctx.r[8].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82DBFAA8: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82DBFAAC: 40990010  ble cr6, 0x82dbfabc
	if !ctx.cr[6].gt {
	pc = 0x82DBFABC; continue 'dispatch;
	}
	// 82DBFAB0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DBFAB4: 7D6A492A  stdx r11, r10, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u64) };
	// 82DBFAB8: 48000040  b 0x82dbfaf8
	pc = 0x82DBFAF8; continue 'dispatch;
	// 82DBFABC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFAC0: 4BFA0F71  bl 0x82d60a30
	ctx.lr = 0x82DBFAC4;
	sub_82D60A30(ctx, base);
	// 82DBFAC4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFAC8: 57EAD978  rlwinm r10, r31, 0x1b, 5, 0x1c
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 82DBFACC: 7EAA592E  stwx r21, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 82DBFAD0: 48000028  b 0x82dbfaf8
	pc = 0x82DBFAF8; continue 'dispatch;
	// 82DBFAD4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFAD8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DBFADC: 7D5E5A14  add r10, r30, r11
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DBFAE0: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82DBFAE4: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82DBFAE8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFAEC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DBFAF0: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFAF4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DBFAF8: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFAFC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DBFB00: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DBFB04: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFB08: 4198FF30  blt cr6, 0x82dbfa38
	if ctx.cr[6].lt {
	pc = 0x82DBFA38; continue 'dispatch;
	}
	// 82DBFB0C: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBFB10: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBFB14: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DBFB18: 40980024  bge cr6, 0x82dbfb3c
	if !ctx.cr[6].lt {
	pc = 0x82DBFB3C; continue 'dispatch;
	}
	// 82DBFB1C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBFB20: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFB24: 41980008  blt cr6, 0x82dbfb2c
	if ctx.cr[6].lt {
	pc = 0x82DBFB2C; continue 'dispatch;
	}
	// 82DBFB28: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82DBFB2C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DBFB30: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBFB34: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DBFB38: 4BF973D9  bl 0x82d56f10
	ctx.lr = 0x82DBFB3C;
	sub_82D56F10(ctx, base);
	// 82DBFB3C: 93980004  stw r28, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DBFB40: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82DBFB44: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFB48: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82DBFB4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DBFB50: 40990050  ble cr6, 0x82dbfba0
	if !ctx.cr[6].gt {
	pc = 0x82DBFBA0; continue 'dispatch;
	}
	// 82DBFB54: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 82DBFB58: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82DBFB5C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFB60: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DBFB64: 7CC9582E  lwzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DBFB68: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBFB6C: 419A0020  beq cr6, 0x82dbfb8c
	if ctx.cr[6].eq {
	pc = 0x82DBFB8C; continue 'dispatch;
	}
	// 82DBFB70: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFB74: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DBFB78: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DBFB7C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82DBFB80: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82DBFB84: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFB88: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DBFB8C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFB90: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DBFB94: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DBFB98: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFB9C: 4198FFC0  blt cr6, 0x82dbfb5c
	if ctx.cr[6].lt {
	pc = 0x82DBFB5C; continue 'dispatch;
	}
	// 82DBFBA0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBFBA4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DBFBA8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DBFBAC: 40980024  bge cr6, 0x82dbfbd0
	if !ctx.cr[6].lt {
	pc = 0x82DBFBD0; continue 'dispatch;
	}
	// 82DBFBB0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBFBB4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DBFBB8: 41980008  blt cr6, 0x82dbfbc0
	if ctx.cr[6].lt {
	pc = 0x82DBFBC0; continue 'dispatch;
	}
	// 82DBFBBC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DBFBC0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DBFBC4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DBFBC8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DBFBCC: 4BF97345  bl 0x82d56f10
	ctx.lr = 0x82DBFBD0;
	sub_82D56F10(ctx, base);
	// 82DBFBD0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DBFBD4: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DBFBD8: 4BFA0CE1  bl 0x82d608b8
	ctx.lr = 0x82DBFBDC;
	sub_82D608B8(ctx, base);
	// 82DBFBDC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DBFBE0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DBFBE4: 93230020  stw r25, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[25].u32 ) };
	// 82DBFBE8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DBFBEC: 409A0018  bne cr6, 0x82dbfc04
	if !ctx.cr[6].eq {
	pc = 0x82DBFC04; continue 'dispatch;
	}
	// 82DBFBF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFBF4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DBFBF8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DBFBFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DBFC00: 4E800421  bctrl
	ctx.lr = 0x82DBFC04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBFC04: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DBFC08: 4BEE9834  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBFC10 size=8
    let mut pc: u32 = 0x82DBFC10;
    'dispatch: loop {
        match pc {
            0x82DBFC10 => {
    //   block [0x82DBFC10..0x82DBFC18)
	// 82DBFC10: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82DBFC14: 48000004  b 0x82dbfc18
	sub_82DBFC18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBFC18 size=124
    let mut pc: u32 = 0x82DBFC18;
    'dispatch: loop {
        match pc {
            0x82DBFC18 => {
    //   block [0x82DBFC18..0x82DBFC94)
	// 82DBFC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBFC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBFC20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBFC24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBFC28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBFC2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DBFC30: 393F0008  addi r9, r31, 8
	ctx.r[9].s64 = ctx.r[31].s64 + 8;
	// 82DBFC34: 409A0008  bne cr6, 0x82dbfc3c
	if !ctx.cr[6].eq {
	pc = 0x82DBFC3C; continue 'dispatch;
	}
	// 82DBFC38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DBFC3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBFC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBFC44: 396B85F4  addi r11, r11, -0x7a0c
	ctx.r[11].s64 = ctx.r[11].s64 + -31244;
	// 82DBFC48: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DBFC4C: 548807FE  clrlwi r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DBFC50: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DBFC54: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBFC58: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DBFC5C: 419A0020  beq cr6, 0x82dbfc7c
	if ctx.cr[6].eq {
	pc = 0x82DBFC7C; continue 'dispatch;
	}
	// 82DBFC60: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFC64: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DBFC68: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DBFC6C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFC70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DBFC74: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DBFC78: 4BF95651  bl 0x82d552c8
	ctx.lr = 0x82DBFC7C;
	sub_82D552C8(ctx, base);
	// 82DBFC7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DBFC80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DBFC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DBFC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DBFC8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DBFC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBFC98 size=64
    let mut pc: u32 = 0x82DBFC98;
    'dispatch: loop {
        match pc {
            0x82DBFC98 => {
    //   block [0x82DBFC98..0x82DBFCD8)
	// 82DBFC98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DBFC9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DBFCA0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DBFCA4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBFCA8: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DBFCAC: 396B86B8  addi r11, r11, -0x7948
	ctx.r[11].s64 = ctx.r[11].s64 + -31048;
	// 82DBFCB0: 394A86AC  addi r10, r10, -0x7954
	ctx.r[10].s64 = ctx.r[10].s64 + -31060;
	// 82DBFCB4: 39298698  addi r9, r9, -0x7968
	ctx.r[9].s64 = ctx.r[9].s64 + -31080;
	// 82DBFCB8: 3908868C  addi r8, r8, -0x7974
	ctx.r[8].s64 = ctx.r[8].s64 + -31092;
	// 82DBFCBC: 38E78680  addi r7, r7, -0x7980
	ctx.r[7].s64 = ctx.r[7].s64 + -31104;
	// 82DBFCC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DBFCC4: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DBFCC8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DBFCCC: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DBFCD0: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82DBFCD4: 4B89F804  b 0x8265f4d8
	sub_8265F4D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DBFCD8 size=32
    let mut pc: u32 = 0x82DBFCD8;
    'dispatch: loop {
        match pc {
            0x82DBFCD8 => {
    //   block [0x82DBFCD8..0x82DBFCF8)
	// 82DBFCD8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFCDC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFCE0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBFCE4: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 82DBFCE8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DBFCEC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DBFCF0: 55635FFE  rlwinm r3, r11, 0xb, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	// 82DBFCF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBFCF8 size=376
    let mut pc: u32 = 0x82DBFCF8;
    'dispatch: loop {
        match pc {
            0x82DBFCF8 => {
    //   block [0x82DBFCF8..0x82DBFE70)
	// 82DBFCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBFCFC: 4BEE9705  bl 0x82ca9400
	ctx.lr = 0x82DBFD00;
	sub_82CA93D0(ctx, base);
	// 82DBFD00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBFD04: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DBFD08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DBFD0C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DBFD10: 7F6B2214  add r27, r11, r4
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DBFD14: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFD18: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82DBFD1C: 419A0144  beq cr6, 0x82dbfe60
	if ctx.cr[6].eq {
	pc = 0x82DBFE60; continue 'dispatch;
	}
	// 82DBFD20: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBFD24: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DBFD28: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFD2C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DBFD30: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 82DBFD34: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82DBFD38: 555D2036  slwi r29, r10, 4
	ctx.r[29].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82DBFD3C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFD40: 7CABEA14  add r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DBFD44: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFD48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DBFD4C: 4E800421  bctrl
	ctx.lr = 0x82DBFD50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DBFD50: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFD54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBFD58: 419A0108  beq cr6, 0x82dbfe60
	if ctx.cr[6].eq {
	pc = 0x82DBFE60; continue 'dispatch;
	}
	// 82DBFD5C: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DBFD60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DBFD64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFD68: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 82DBFD6C: 393E0030  addi r9, r30, 0x30
	ctx.r[9].s64 = ctx.r[30].s64 + 48;
	// 82DBFD70: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBFD74: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DBFD78: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 82DBFD7C: 3B9E0020  addi r28, r30, 0x20
	ctx.r[28].s64 = ctx.r[30].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBFE70 size=296
    let mut pc: u32 = 0x82DBFE70;
    'dispatch: loop {
        match pc {
            0x82DBFE70 => {
    //   block [0x82DBFE70..0x82DBFF98)
	// 82DBFE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBFE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DBFE78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DBFE7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DBFE80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBFE84: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFE88: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DBFE8C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DBFE90: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBFE94: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBFE98: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFE9C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBFEA0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DBFEA4: 40980020  bge cr6, 0x82dbfec4
	if !ctx.cr[6].lt {
	pc = 0x82DBFEC4; continue 'dispatch;
	}
	// 82DBFEA8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DBFEAC: 39082DD0  addi r8, r8, 0x2dd0
	ctx.r[8].s64 = ctx.r[8].s64 + 11728;
	// 82DBFEB0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DBFEB4: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DBFEB8: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82DBFEBC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DBFEC0: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DBFEC4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBFEC8: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBFECC: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DBFED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DBFED4: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82DBFED8: 409A0008  bne cr6, 0x82dbfee0
	if !ctx.cr[6].eq {
	pc = 0x82DBFEE0; continue 'dispatch;
	}
	// 82DBFEDC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBFEE0: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DBFEE4: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DBFEE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DBFEEC: 419A001C  beq cr6, 0x82dbff08
	if ctx.cr[6].eq {
	pc = 0x82DBFF08; continue 'dispatch;
	}
	// 82DBFEF0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBFEF4: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DBFEF8: 409A0008  bne cr6, 0x82dbff00
	if !ctx.cr[6].eq {
	pc = 0x82DBFF00; continue 'dispatch;
	}
	// 82DBFEFC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DBFF00: 91690034  stw r11, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DBFF04: 48000008  b 0x82dbff0c
	pc = 0x82DBFF0C; continue 'dispatch;
	// 82DBFF08: 91490034  stw r10, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82DBFF0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DBFF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DBFF98 size=260
    let mut pc: u32 = 0x82DBFF98;
    'dispatch: loop {
        match pc {
            0x82DBFF98 => {
    //   block [0x82DBFF98..0x82DC009C)
	// 82DBFF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DBFF9C: 4BEE9471  bl 0x82ca940c
	ctx.lr = 0x82DBFFA0;
	sub_82CA93D0(ctx, base);
	// 82DBFFA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DBFFA4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DBFFA8: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DBFFAC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DBFFB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DBFFB4: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DBFFB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DBFFBC: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DBFFC0: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DBFFC4: 40980020  bge cr6, 0x82dbffe4
	if !ctx.cr[6].lt {
	pc = 0x82DBFFE4; continue 'dispatch;
	}
	// 82DBFFC8: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DBFFCC: 38842DE0  addi r4, r4, 0x2de0
	ctx.r[4].s64 = ctx.r[4].s64 + 11744;
	// 82DBFFD0: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DBFFD4: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82DBFFD8: 3BAA000C  addi r29, r10, 0xc
	ctx.r[29].s64 = ctx.r[10].s64 + 12;
	// 82DBFFDC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DBFFE0: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DBFFE4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DBFFE8: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DBFFEC: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DBFFF0: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82DBFFF4: 409A0008  bne cr6, 0x82dbfffc
	if !ctx.cr[6].eq {
	pc = 0x82DBFFFC; continue 'dispatch;
	}
	// 82DBFFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DBFFFC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC0000: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC0004: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0008: 419A0010  beq cr6, 0x82dc0018
	if ctx.cr[6].eq {
	pc = 0x82DC0018; continue 'dispatch;
	}
	// 82DC000C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DC0010: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DC0014: 409A0008  bne cr6, 0x82dc001c
	if !ctx.cr[6].eq {
	pc = 0x82DC001C; continue 'dispatch;
	}
	// 82DC0018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC001C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DC0020: 91690034  stw r11, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC00A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC00A0 size=468
    let mut pc: u32 = 0x82DC00A0;
    'dispatch: loop {
        match pc {
            0x82DC00A0 => {
    //   block [0x82DC00A0..0x82DC0274)
	// 82DC00A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC00A4: 4BEE934D  bl 0x82ca93f0
	ctx.lr = 0x82DC00A8;
	sub_82CA93D0(ctx, base);
	// 82DC00A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC00AC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC00B0: 3AC00008  li r22, 8
	ctx.r[22].s64 = 8;
	// 82DC00B4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DC00B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DC00BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DC00C0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DC00C4: 7D79B02E  lwzx r11, r25, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DC00C8: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82DC00CC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82DC00D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC00D4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC00D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC00DC: 40980020  bge cr6, 0x82dc00fc
	if !ctx.cr[6].lt {
	pc = 0x82DC00FC; continue 'dispatch;
	}
	// 82DC00E0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC00E4: 39292DF0  addi r9, r9, 0x2df0
	ctx.r[9].s64 = ctx.r[9].s64 + 11760;
	// 82DC00E8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC00EC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC00F0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC00F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC00F8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC00FC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0278 size=256
    let mut pc: u32 = 0x82DC0278;
    'dispatch: loop {
        match pc {
            0x82DC0278 => {
    //   block [0x82DC0278..0x82DC0378)
	// 82DC0278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC027C: 4BEE918D  bl 0x82ca9408
	ctx.lr = 0x82DC0280;
	sub_82CA93D0(ctx, base);
	// 82DC0280: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC0284: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0288: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DC028C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DC0290: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC0294: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0298: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC029C: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DC02A0: 40980020  bge cr6, 0x82dc02c0
	if !ctx.cr[6].lt {
	pc = 0x82DC02C0; continue 'dispatch;
	}
	// 82DC02A4: 3C808203  lis r4, -0x7dfd
	ctx.r[4].s64 = -2113732608;
	// 82DC02A8: 38842E00  addi r4, r4, 0x2e00
	ctx.r[4].s64 = ctx.r[4].s64 + 11776;
	// 82DC02AC: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DC02B0: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82DC02B4: 3B8A000C  addi r28, r10, 0xc
	ctx.r[28].s64 = ctx.r[10].s64 + 12;
	// 82DC02B8: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DC02BC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DC02C0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DC02C4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DC02C8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82DC02CC: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 82DC02D0: 409A0008  bne cr6, 0x82dc02d8
	if !ctx.cr[6].eq {
	pc = 0x82DC02D8; continue 'dispatch;
	}
	// 82DC02D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC02D8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC02DC: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC02E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC02E4: 419A0010  beq cr6, 0x82dc02f4
	if ctx.cr[6].eq {
	pc = 0x82DC02F4; continue 'dispatch;
	}
	// 82DC02E8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DC02EC: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 82DC02F0: 409A0008  bne cr6, 0x82dc02f8
	if !ctx.cr[6].eq {
	pc = 0x82DC02F8; continue 'dispatch;
	}
	// 82DC02F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC02F8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DC02FC: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC0378 size=16
    let mut pc: u32 = 0x82DC0378;
    'dispatch: loop {
        match pc {
            0x82DC0378 => {
    //   block [0x82DC0378..0x82DC0388)
	// 82DC0378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC037C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DC0380: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC0384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC0388 size=4
    let mut pc: u32 = 0x82DC0388;
    'dispatch: loop {
        match pc {
            0x82DC0388 => {
    //   block [0x82DC0388..0x82DC038C)
	// 82DC0388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC0390 size=8
    let mut pc: u32 = 0x82DC0390;
    'dispatch: loop {
        match pc {
            0x82DC0390 => {
    //   block [0x82DC0390..0x82DC0398)
	// 82DC0390: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DC0394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0398 size=368
    let mut pc: u32 = 0x82DC0398;
    'dispatch: loop {
        match pc {
            0x82DC0398 => {
    //   block [0x82DC0398..0x82DC0508)
	// 82DC0398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC039C: 4BEE9071  bl 0x82ca940c
	ctx.lr = 0x82DC03A0;
	sub_82CA93D0(ctx, base);
	// 82DC03A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC03A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC03A8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DC03AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC03B0: 395F0EB0  addi r10, r31, 0xeb0
	ctx.r[10].s64 = ctx.r[31].s64 + 3760;
	// 82DC03B4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DC03B8: 3BA00064  li r29, 0x64
	ctx.r[29].s64 = 100;
	// 82DC03BC: 90FF0190  stw r7, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[7].u32 ) };
	// 82DC03C0: 90FF0EA0  stw r7, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[7].u32 ) };
	// 82DC03C4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DC03C8: 813F1E30  lwz r9, 0x1e30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC03CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DC03D0: 419A0030  beq cr6, 0x82dc0400
	if ctx.cr[6].eq {
	pc = 0x82DC0400; continue 'dispatch;
	}
	// 82DC03D4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DC03D8: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82DC03DC: 813F1E34  lwz r9, 0x1e34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC03E0: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC03E4: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82DC03E8: 813F1E38  lwz r9, 0x1e38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC03EC: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC03F0: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82DC03F4: 813F1E3C  lwz r9, 0x1e3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC03F8: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC03FC: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82DC0400: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DC0404: 9BCAF2F0  stb r30, -0xd10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-3344 as u32), ctx.r[30].u8 ) };
	// 82DC0408: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82DC040C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82DC0410: 9BCAF6F0  stb r30, -0x910(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-2320 as u32), ctx.r[30].u8 ) };
	// 82DC0414: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DC0418: 9BCA0400  stb r30, 0x400(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1024 as u32), ctx.r[30].u8 ) };
	// 82DC041C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC0420: 409AFFA8  bne cr6, 0x82dc03c8
	if !ctx.cr[6].eq {
	pc = 0x82DC03C8; continue 'dispatch;
	}
	// 82DC0424: 2F0B0C00  cmpwi cr6, r11, 0xc00
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3072, &mut ctx.xer);
	// 82DC0428: 4198FF9C  blt cr6, 0x82dc03c4
	if ctx.cr[6].lt {
	pc = 0x82DC03C4; continue 'dispatch;
	}
	// 82DC042C: 3D2082DC  lis r9, -0x7d24
	ctx.r[9].s64 = -2099511296;
	// 82DC0430: 3D4082DC  lis r10, -0x7d24
	ctx.r[10].s64 = -2099511296;
	// 82DC0434: 3D6082DC  lis r11, -0x7d24
	ctx.r[11].s64 = -2099511296;
	// 82DC0438: 3D0082DC  lis r8, -0x7d24
	ctx.r[8].s64 = -2099511296;
	// 82DC043C: 38C91F08  addi r6, r9, 0x1f08
	ctx.r[6].s64 = ctx.r[9].s64 + 7944;
	// 82DC0440: 38AA1F00  addi r5, r10, 0x1f00
	ctx.r[5].s64 = ctx.r[10].s64 + 7936;
	// 82DC0444: 388B1F10  addi r4, r11, 0x1f10
	ctx.r[4].s64 = ctx.r[11].s64 + 7952;
	// 82DC0448: 39081F18  addi r8, r8, 0x1f18
	ctx.r[8].s64 = ctx.r[8].s64 + 7960;
	// 82DC044C: 3D2082DC  lis r9, -0x7d24
	ctx.r[9].s64 = -2099511296;
	// 82DC0450: 3D4082DC  lis r10, -0x7d24
	ctx.r[10].s64 = -2099511296;
	// 82DC0454: 90DF09A4  stw r6, 0x9a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2468 as u32), ctx.r[6].u32 ) };
	// 82DC0458: 3D6082DC  lis r11, -0x7d24
	ctx.r[11].s64 = -2099511296;
	// 82DC045C: 90BF09A8  stw r5, 0x9a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2472 as u32), ctx.r[5].u32 ) };
	// 82DC0460: 39290378  addi r9, r9, 0x378
	ctx.r[9].s64 = ctx.r[9].s64 + 888;
	// 82DC0464: 909F09AC  stw r4, 0x9ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2476 as u32), ctx.r[4].u32 ) };
	// 82DC0468: 394A0388  addi r10, r10, 0x388
	ctx.r[10].s64 = ctx.r[10].s64 + 904;
	// 82DC046C: 911F09A0  stw r8, 0x9a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2464 as u32), ctx.r[8].u32 ) };
	// 82DC0470: 396B0390  addi r11, r11, 0x390
	ctx.r[11].s64 = ctx.r[11].s64 + 912;
	// 82DC0474: 9BDF09B0  stb r30, 0x9b0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2480 as u32), ctx.r[30].u8 ) };
	// 82DC0478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC047C: 98FF09B1  stb r7, 0x9b1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2481 as u32), ctx.r[7].u8 ) };
	// 82DC0480: 93DF16B8  stw r30, 0x16b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5816 as u32), ctx.r[30].u32 ) };
	// 82DC0484: 913F16B0  stw r9, 0x16b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5808 as u32), ctx.r[9].u32 ) };
	// 82DC0488: 915F16B4  stw r10, 0x16b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5812 as u32), ctx.r[10].u32 ) };
	// 82DC048C: 93DF16BC  stw r30, 0x16bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5820 as u32), ctx.r[30].u32 ) };
	// 82DC0490: 93DF16C0  stw r30, 0x16c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5824 as u32), ctx.r[30].u32 ) };
	// 82DC0494: 93DF16C4  stw r30, 0x16c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5828 as u32), ctx.r[30].u32 ) };
	// 82DC0498: 93DF16C8  stw r30, 0x16c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5832 as u32), ctx.r[30].u32 ) };
	// 82DC049C: 93DF16CC  stw r30, 0x16cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5836 as u32), ctx.r[30].u32 ) };
	// 82DC04A0: 93DF16D8  stw r30, 0x16d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5848 as u32), ctx.r[30].u32 ) };
	// 82DC04A4: 93DF16D0  stw r30, 0x16d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5840 as u32), ctx.r[30].u32 ) };
	// 82DC04A8: 93DF16D4  stw r30, 0x16d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5844 as u32), ctx.r[30].u32 ) };
	// 82DC04AC: 917F16DC  stw r11, 0x16dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5852 as u32), ctx.r[11].u32 ) };
	// 82DC04B0: 93DF16F0  stw r30, 0x16f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5872 as u32), ctx.r[30].u32 ) };
	// 82DC04B4: 48022895  bl 0x82de2d48
	ctx.lr = 0x82DC04B8;
	sub_82DE2D48(ctx, base);
	// 82DC04B8: 9BDF1E20  stb r30, 0x1e20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7712 as u32), ctx.r[30].u8 ) };
	// 82DC04BC: 817F1E38  lwz r11, 0x1e38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC04C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC04C4: 419A003C  beq cr6, 0x82dc0500
	if ctx.cr[6].eq {
	pc = 0x82DC0500; continue 'dispatch;
	}
	// 82DC04C8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DC04CC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DC04D0: 813F1E38  lwz r9, 0x1e38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC04D4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DC04D8: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DC04DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC04E0: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82DC04E4: 813F1E3C  lwz r9, 0x1e3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC04E8: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DC04EC: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82DC04F0: 9BA90002  stb r29, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 82DC04F4: 409AFFDC  bne cr6, 0x82dc04d0
	if !ctx.cr[6].eq {
	pc = 0x82DC04D0; continue 'dispatch;
	}
	// 82DC04F8: 2F0B0C00  cmpwi cr6, r11, 0xc00
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3072, &mut ctx.xer);
	// 82DC04FC: 4198FFD0  blt cr6, 0x82dc04cc
	if ctx.cr[6].lt {
	pc = 0x82DC04CC; continue 'dispatch;
	}
	// 82DC0500: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC0504: 4BEE8F58  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC0508 size=8
    let mut pc: u32 = 0x82DC0508;
    'dispatch: loop {
        match pc {
            0x82DC0508 => {
    //   block [0x82DC0508..0x82DC0510)
	// 82DC0508: 98831E23  stb r4, 0x1e23(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7715 as u32), ctx.r[4].u8 ) };
	// 82DC050C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0510 size=244
    let mut pc: u32 = 0x82DC0510;
    'dispatch: loop {
        match pc {
            0x82DC0510 => {
    //   block [0x82DC0510..0x82DC0604)
	// 82DC0510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC0514: 4BEE8EF1  bl 0x82ca9404
	ctx.lr = 0x82DC0518;
	sub_82CA93D0(ctx, base);
	// 82DC0518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC051C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DC0520: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC0524: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DC0528: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DC052C: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0534: 419A0010  beq cr6, 0x82dc0544
	if ctx.cr[6].eq {
	pc = 0x82DC0544; continue 'dispatch;
	}
	// 82DC0538: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC053C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC0540: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC0544: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC0548: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DC054C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82DC0550: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DC0554: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC0558: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC055C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0560: 419A0030  beq cr6, 0x82dc0590
	if ctx.cr[6].eq {
	pc = 0x82DC0590; continue 'dispatch;
	}
	// 82DC0564: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC0568: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC056C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC0570: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC0574: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC0578: 409A0018  bne cr6, 0x82dc0590
	if !ctx.cr[6].eq {
	pc = 0x82DC0590; continue 'dispatch;
	}
	// 82DC057C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0580: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC0584: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC058C: 4E800421  bctrl
	ctx.lr = 0x82DC0590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC0590: 7FBFF12E  stwx r29, r31, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 82DC0594: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC059C: 419A0010  beq cr6, 0x82dc05ac
	if ctx.cr[6].eq {
	pc = 0x82DC05AC; continue 'dispatch;
	}
	// 82DC05A0: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC05A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC05A8: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC05AC: 578B1838  slwi r11, r28, 3
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC05B0: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82DC05B4: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82DC05B8: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DC05BC: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC05C0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC05C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC05C8: 419A0030  beq cr6, 0x82dc05f8
	if ctx.cr[6].eq {
	pc = 0x82DC05F8; continue 'dispatch;
	}
	// 82DC05CC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC05D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC05D4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC05D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC05DC: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC05E0: 409A0018  bne cr6, 0x82dc05f8
	if !ctx.cr[6].eq {
	pc = 0x82DC05F8; continue 'dispatch;
	}
	// 82DC05E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC05E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC05EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC05F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC05F4: 4E800421  bctrl
	ctx.lr = 0x82DC05F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC05F8: 7FBFF12E  stwx r29, r31, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 82DC05FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC0600: 4BEE8E54  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0608 size=248
    let mut pc: u32 = 0x82DC0608;
    'dispatch: loop {
        match pc {
            0x82DC0608 => {
    //   block [0x82DC0608..0x82DC0700)
	// 82DC0608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC060C: 4BEE8DFD  bl 0x82ca9408
	ctx.lr = 0x82DC0610;
	sub_82CA93D0(ctx, base);
	// 82DC0610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC0614: 54AB2834  slwi r11, r5, 5
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC0618: 39450003  addi r10, r5, 3
	ctx.r[10].s64 = ctx.r[5].s64 + 3;
	// 82DC061C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DC0620: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC0624: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC0628: 3BAB000C  addi r29, r11, 0xc
	ctx.r[29].s64 = ctx.r[11].s64 + 12;
	// 82DC062C: 7FCA1A14  add r30, r10, r3
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82DC0630: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DC0634: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC063C: 419A0010  beq cr6, 0x82dc064c
	if ctx.cr[6].eq {
	pc = 0x82DC064C; continue 'dispatch;
	}
	// 82DC0640: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC0644: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC0648: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC064C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0650: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0654: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0658: 419A0030  beq cr6, 0x82dc0688
	if ctx.cr[6].eq {
	pc = 0x82DC0688; continue 'dispatch;
	}
	// 82DC065C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC0660: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC0664: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC0668: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC066C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC0670: 409A0018  bne cr6, 0x82dc0688
	if !ctx.cr[6].eq {
	pc = 0x82DC0688; continue 'dispatch;
	}
	// 82DC0674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0678: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC067C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC0684: 4E800421  bctrl
	ctx.lr = 0x82DC0688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC0688: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DC068C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0694: 419A0010  beq cr6, 0x82dc06a4
	if ctx.cr[6].eq {
	pc = 0x82DC06A4; continue 'dispatch;
	}
	// 82DC0698: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC069C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC06A0: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC06A4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC06A8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC06AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC06B0: 419A0030  beq cr6, 0x82dc06e0
	if ctx.cr[6].eq {
	pc = 0x82DC06E0; continue 'dispatch;
	}
	// 82DC06B4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC06B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC06BC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC06C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC06C4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC06C8: 409A0018  bne cr6, 0x82dc06e0
	if !ctx.cr[6].eq {
	pc = 0x82DC06E0; continue 'dispatch;
	}
	// 82DC06CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC06D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC06D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC06D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC06DC: 4E800421  bctrl
	ctx.lr = 0x82DC06E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC06E0: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DC06E4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82DC06E8: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 82DC06EC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DC06F0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DC06F4: 409AFF40  bne cr6, 0x82dc0634
	if !ctx.cr[6].eq {
	pc = 0x82DC0634; continue 'dispatch;
	}
	// 82DC06F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC06FC: 4BEE8D5C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0700 size=108
    let mut pc: u32 = 0x82DC0700;
    'dispatch: loop {
        match pc {
            0x82DC0700 => {
    //   block [0x82DC0700..0x82DC076C)
	// 82DC0700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC0704: 4BEE8D09  bl 0x82ca940c
	ctx.lr = 0x82DC0708;
	sub_82CA93D0(ctx, base);
	// 82DC0708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC070C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC0710: 809F1E30  lwz r4, 0x1e30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC0714: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DC0718: 419A004C  beq cr6, 0x82dc0764
	if ctx.cr[6].eq {
	pc = 0x82DC0764; continue 'dispatch;
	}
	// 82DC071C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0720: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82DC0724: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DC0728: 4BF94DA9  bl 0x82d554d0
	ctx.lr = 0x82DC072C;
	sub_82D554D0(ctx, base);
	// 82DC072C: 809F1E34  lwz r4, 0x1e34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC0730: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DC0734: 4BF94D9D  bl 0x82d554d0
	ctx.lr = 0x82DC0738;
	sub_82D554D0(ctx, base);
	// 82DC0738: 809F1E38  lwz r4, 0x1e38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC073C: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DC0740: 4BF94D91  bl 0x82d554d0
	ctx.lr = 0x82DC0744;
	sub_82D554D0(ctx, base);
	// 82DC0744: 809F1E3C  lwz r4, 0x1e3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC0748: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DC074C: 4BF94D85  bl 0x82d554d0
	ctx.lr = 0x82DC0750;
	sub_82D554D0(ctx, base);
	// 82DC0750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC0754: 917F1E30  stw r11, 0x1e30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7728 as u32), ctx.r[11].u32 ) };
	// 82DC0758: 917F1E34  stw r11, 0x1e34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7732 as u32), ctx.r[11].u32 ) };
	// 82DC075C: 917F1E38  stw r11, 0x1e38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7736 as u32), ctx.r[11].u32 ) };
	// 82DC0760: 917F1E3C  stw r11, 0x1e3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7740 as u32), ctx.r[11].u32 ) };
	// 82DC0764: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC0768: 4BEE8CF4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0770 size=540
    let mut pc: u32 = 0x82DC0770;
    'dispatch: loop {
        match pc {
            0x82DC0770 => {
    //   block [0x82DC0770..0x82DC098C)
	// 82DC0770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC0774: 4BEE8C5D  bl 0x82ca93d0
	ctx.lr = 0x82DC0778;
	sub_82CA93D0(ctx, base);
	// 82DC0778: 9421FB00  stwu r1, -0x500(r1)
	ea = ctx.r[1].u32.wrapping_add(-1280 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC077C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82DC0780: 83C10554  lwz r30, 0x554(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1364 as u32) ) } as u64;
	// 82DC0784: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC0788: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82DC078C: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DC0790: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DC0794: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DC0798: 99741E20  stb r11, 0x1e20(r20)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[20].u32.wrapping_add(7712 as u32), ctx.r[11].u8 ) };
	// 82DC079C: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 82DC07A0: 81741E28  lwz r11, 0x1e28(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82DC07A4: 7D0E4378  mr r14, r8
	ctx.r[14].u64 = ctx.r[8].u64;
	// 82DC07A8: 92E10524  stw r23, 0x524(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1316 as u32), ctx.r[23].u32 ) };
	// 82DC07AC: 7D535378  mr r19, r10
	ctx.r[19].u64 = ctx.r[10].u64;
	// 82DC07B0: 93210544  stw r25, 0x544(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1348 as u32), ctx.r[25].u32 ) };
	// 82DC07B4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DC07B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC07BC: 40990094  ble cr6, 0x82dc0850
	if !ctx.cr[6].gt {
	pc = 0x82DC0850; continue 'dispatch;
	}
	// 82DC07C0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DC07C4: 81741E24  lwz r11, 0x1e24(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7716 as u32) ) } as u64;
	// 82DC07C8: 7FEBE214  add r31, r11, r28
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DC07CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC07D0: 7F0BD000  cmpw cr6, r11, r26
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82DC07D4: 409A0030  bne cr6, 0x82dc0804
	if !ctx.cr[6].eq {
	pc = 0x82DC0804; continue 'dispatch;
	}
	// 82DC07D8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82DC07DC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC07E0: 7E6A9B78  mr r10, r19
	ctx.r[10].u64 = ctx.r[19].u64;
	// 82DC07E4: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82DC07E8: 7DC87378  mr r8, r14
	ctx.r[8].u64 = ctx.r[14].u64;
	// 82DC07EC: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82DC07F0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DC07F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DC07F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DC07FC: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82DC0800: 4BFFFF71  bl 0x82dc0770
	ctx.lr = 0x82DC0804;
	sub_82DC0770(ctx, base);
	// 82DC0804: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0808: 7F0BA800  cmpw cr6, r11, r21
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[21].s32, &mut ctx.xer);
	// 82DC080C: 409A0030  bne cr6, 0x82dc083c
	if !ctx.cr[6].eq {
	pc = 0x82DC083C; continue 'dispatch;
	}
	// 82DC0810: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82DC0814: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0818: 7E6A9B78  mr r10, r19
	ctx.r[10].u64 = ctx.r[19].u64;
	// 82DC081C: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82DC0820: 7DC87378  mr r8, r14
	ctx.r[8].u64 = ctx.r[14].u64;
	// 82DC0824: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DC0828: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DC082C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DC0830: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DC0834: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82DC0838: 4BFFFF39  bl 0x82dc0770
	ctx.lr = 0x82DC083C;
	sub_82DC0770(ctx, base);
	// 82DC083C: 81741E28  lwz r11, 0x1e28(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82DC0840: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DC0844: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 82DC0848: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC084C: 4198FF78  blt cr6, 0x82dc07c4
	if ctx.cr[6].lt {
	pc = 0x82DC07C4; continue 'dispatch;
	}
	// 82DC0850: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82DC0854: 7EB0AB78  mr r16, r21
	ctx.r[16].u64 = ctx.r[21].u64;
	// 82DC0858: 393A0001  addi r9, r26, 1
	ctx.r[9].s64 = ctx.r[26].s64 + 1;
	// 82DC085C: 39F50001  addi r15, r21, 1
	ctx.r[15].s64 = ctx.r[21].s64 + 1;
	// 82DC0860: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
	// 82DC0864: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 82DC0868: 409A0010  bne cr6, 0x82dc0878
	if !ctx.cr[6].eq {
	pc = 0x82DC0878; continue 'dispatch;
	}
	// 82DC086C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DC0870: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DC0874: 3ADE0001  addi r22, r30, 1
	ctx.r[22].s64 = ctx.r[30].s64 + 1;
	// 82DC0878: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 82DC087C: 409A0010  bne cr6, 0x82dc088c
	if !ctx.cr[6].eq {
	pc = 0x82DC088C; continue 'dispatch;
	}
	// 82DC0880: 3A000001  li r16, 1
	ctx.r[16].s64 = 1;
	// 82DC0884: 39E00020  li r15, 0x20
	ctx.r[15].s64 = 32;
	// 82DC0888: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82DC088C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DC0890: 409800F4  bge cr6, 0x82dc0984
	if !ctx.cr[6].lt {
	pc = 0x82DC0984; continue 'dispatch;
	}
	// 82DC0894: 554B2834  slwi r11, r10, 5
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC0898: 7E2A4850  subf r17, r10, r9
	ctx.r[17].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82DC089C: 7D4B8214  add r10, r11, r16
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[16].u64;
	// 82DC08A0: 7F0BEA14  add r24, r11, r29
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DC08A4: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC08A8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DC08AC: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82DC08B0: 3A4B0001  addi r18, r11, 1
	ctx.r[18].s64 = ctx.r[11].s64 + 1;
	// 82DC08B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC08B8: 3B2B2E20  addi r25, r11, 0x2e20
	ctx.r[25].s64 = ctx.r[11].s64 + 11808;
	// 82DC08BC: 7E1B8378  mr r27, r16
	ctx.r[27].u64 = ctx.r[16].u64;
	// 82DC08C0: 7F107800  cmpw cr6, r16, r15
	ctx.cr[6].compare_i32(ctx.r[16].s32, ctx.r[15].s32, &mut ctx.xer);
	// 82DC08C4: 409800AC  bge cr6, 0x82dc0970
	if !ctx.cr[6].lt {
	pc = 0x82DC0970; continue 'dispatch;
	}
	// 82DC08C8: 56F7063E  clrlwi r23, r23, 0x18
	ctx.r[23].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	// 82DC08CC: 7E5F9378  mr r31, r18
	ctx.r[31].u64 = ctx.r[18].u64;
	// 82DC08D0: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 82DC08D4: 7EF8D9AE  stbx r23, r24, r27
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[27].u32), ctx.r[23].u8) };
	// 82DC08D8: 419A0084  beq cr6, 0x82dc095c
	if ctx.cr[6].eq {
	pc = 0x82DC095C; continue 'dispatch;
	}
	// 82DC08DC: 89741E23  lbz r11, 0x1e23(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[20].u32.wrapping_add(7715 as u32) ) } as u64;
	// 82DC08E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC08E4: 419A0068  beq cr6, 0x82dc094c
	if ctx.cr[6].eq {
	pc = 0x82DC094C; continue 'dispatch;
	}
	// 82DC08E8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC08EC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DC08F0: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC08F4: 40990058  ble cr6, 0x82dc094c
	if !ctx.cr[6].gt {
	pc = 0x82DC094C; continue 'dispatch;
	}
	// 82DC08F8: 897FFFFF  lbz r11, -1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82DC08FC: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82DC0900: 4801F909  bl 0x82de0208
	ctx.lr = 0x82DC0904;
	sub_82DE0208(ctx, base);
	// 82DC0904: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC0908: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC090C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82DC0910: 4801F8F9  bl 0x82de0208
	ctx.lr = 0x82DC0914;
	sub_82DE0208(ctx, base);
	// 82DC0914: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC0918: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DC091C: 4801F8ED  bl 0x82de0208
	ctx.lr = 0x82DC0920;
	sub_82DE0208(ctx, base);
	// 82DC0920: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DC0924: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DC0928: 4801F8E1  bl 0x82de0208
	ctx.lr = 0x82DC092C;
	sub_82DE0208(ctx, base);
	// 82DC092C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82DC0930: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC0934: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC0938: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC093C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DC0940: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 82DC0944: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DC0948: 4BF98001  bl 0x82d58948
	ctx.lr = 0x82DC094C;
	sub_82D58948(ctx, base);
	// 82DC094C: 81210544  lwz r9, 0x544(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1348 as u32) ) } as u64;
	// 82DC0950: 9ADF0001  stb r22, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[22].u8 ) };
	// 82DC0954: 99DFFFFF  stb r14, -1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-1 as u32), ctx.r[14].u8 ) };
	// 82DC0958: 993F0000  stb r9, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DC095C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DC0960: 3BFF0003  addi r31, r31, 3
	ctx.r[31].s64 = ctx.r[31].s64 + 3;
	// 82DC0964: 7F1B7800  cmpw cr6, r27, r15
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[15].s32, &mut ctx.xer);
	// 82DC0968: 4198FF68  blt cr6, 0x82dc08d0
	if ctx.cr[6].lt {
	pc = 0x82DC08D0; continue 'dispatch;
	}
	// 82DC096C: 82E10524  lwz r23, 0x524(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1316 as u32) ) } as u64;
	// 82DC0970: 3A31FFFF  addi r17, r17, -1
	ctx.r[17].s64 = ctx.r[17].s64 + -1;
	// 82DC0974: 3A520060  addi r18, r18, 0x60
	ctx.r[18].s64 = ctx.r[18].s64 + 96;
	// 82DC0978: 3B180020  addi r24, r24, 0x20
	ctx.r[24].s64 = ctx.r[24].s64 + 32;
	// 82DC097C: 2B110000  cmplwi cr6, r17, 0
	ctx.cr[6].compare_u32(ctx.r[17].u32, 0 as u32, &mut ctx.xer);
	// 82DC0980: 409AFF3C  bne cr6, 0x82dc08bc
	if !ctx.cr[6].eq {
	pc = 0x82DC08BC; continue 'dispatch;
	}
	// 82DC0984: 38210500  addi r1, r1, 0x500
	ctx.r[1].s64 = ctx.r[1].s64 + 1280;
	// 82DC0988: 4BEE8A98  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0990 size=140
    let mut pc: u32 = 0x82DC0990;
    'dispatch: loop {
        match pc {
            0x82DC0990 => {
    //   block [0x82DC0990..0x82DC0A1C)
	// 82DC0990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC0994: 4BEE8A71  bl 0x82ca9404
	ctx.lr = 0x82DC0998;
	sub_82CA93D0(ctx, base);
	// 82DC0998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC099C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DC09A0: 39450044  addi r10, r5, 0x44
	ctx.r[10].s64 = ctx.r[5].s64 + 68;
	// 82DC09A4: 397D0044  addi r11, r29, 0x44
	ctx.r[11].s64 = ctx.r[29].s64 + 68;
	// 82DC09A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC09AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC09B0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC09B4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DC09B8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DC09BC: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC09C0: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC09C4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82DC09C8: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82DC09CC: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82DC09D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC09D4: 40990040  ble cr6, 0x82dc0a14
	if !ctx.cr[6].gt {
	pc = 0x82DC0A14; continue 'dispatch;
	}
	// 82DC09D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC09DC: 817F1E24  lwz r11, 0x1e24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7716 as u32) ) } as u64;
	// 82DC09E0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DC09E4: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC09E8: 7F05E800  cmpw cr6, r5, r29
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC09EC: 409A0014  bne cr6, 0x82dc0a00
	if !ctx.cr[6].eq {
	pc = 0x82DC0A00; continue 'dispatch;
	}
	// 82DC09F0: 38DB0001  addi r6, r27, 1
	ctx.r[6].s64 = ctx.r[27].s64 + 1;
	// 82DC09F4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC09F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC09FC: 4BFFFF95  bl 0x82dc0990
	ctx.lr = 0x82DC0A00;
	sub_82DC0990(ctx, base);
	// 82DC0A00: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82DC0A04: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DC0A08: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DC0A0C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC0A10: 4198FFCC  blt cr6, 0x82dc09dc
	if ctx.cr[6].lt {
	pc = 0x82DC09DC; continue 'dispatch;
	}
	// 82DC0A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC0A18: 4BEE8A3C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DC0A20 size=1220
    let mut pc: u32 = 0x82DC0A20;
    'dispatch: loop {
        match pc {
            0x82DC0A20 => {
    //   block [0x82DC0A20..0x82DC0EE4)
	// 82DC0A20: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DC0A24: C0040008  lfs f0, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0A28: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC0A2C: D0031E44  stfs f0, 0x1e44(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(7748 as u32), tmp.u32 ) };
	// 82DC0A30: 39631C60  addi r11, r3, 0x1c60
	ctx.r[11].s64 = ctx.r[3].s64 + 7264;
	// 82DC0A34: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0A38: D0031E40  stfs f0, 0x1e40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(7744 as u32), tmp.u32 ) };
	// 82DC0A3C: 8924001B  lbz r9, 0x1b(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(27 as u32) ) } as u64;
	// 82DC0A40: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0A44: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DC0A48: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC0A4C: 992B003A  stb r9, 0x3a(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(58 as u32), ctx.r[9].u8 ) };
	// 82DC0A50: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DC0A54: C00A0BFC  lfs f0, 0xbfc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0A58: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DC0A5C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC0A60: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DC0A64: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC0A68: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DC0A6C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC0A70: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DC0A74: 89440019  lbz r10, 0x19(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DC0A78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC0A7C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC0A80: C10A0C68  lfs f8, 0xc68(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3176 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DC0A84: 419A000C  beq cr6, 0x82dc0a90
	if ctx.cr[6].eq {
	pc = 0x82DC0A90; continue 'dispatch;
	}
	// 82DC0A88: EDA00232  fmuls f13, f0, f8
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[8].f64) as f32) as f64);
	// 82DC0A8C: D1AB000C  stfs f13, 0xc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DC0A90: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC0A94: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DC0A98: 8944001A  lbz r10, 0x1a(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(26 as u32) ) } as u64;
	// 82DC0A9C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC0AA0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC0AA4: C12A0EE0  lfs f9, 0xee0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3808 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DC0AA8: 419A000C  beq cr6, 0x82dc0ab4
	if ctx.cr[6].eq {
	pc = 0x82DC0AB4; continue 'dispatch;
	}
	// 82DC0AAC: EC000272  fmuls f0, f0, f9
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 82DC0AB0: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DC0AB4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC0AB8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC0ABC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DC0AC0: 38A31DA0  addi r5, r3, 0x1da0
	ctx.r[5].s64 = ctx.r[3].s64 + 7584;
	// 82DC0AC4: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82DC0AC8: C1892F28  lfs f12, 0x2f28(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12072 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DC0ACC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC0AD0: C00A0C64  lfs f0, 0xc64(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0AD4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC0AD8: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DC0ADC: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 82DC0AE0: A0E40010  lhz r7, 0x10(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC0AE4: D18B0024  stfs f12, 0x24(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82DC0AE8: D18B0028  stfs f12, 0x28(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82DC0AEC: C8092F20  lfd f0, 0x2f20(r9)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(12064 as u32) ) };
	// 82DC0AF0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DC0AF4: FD600210  fabs f11, f0
	ctx.f[11].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82DC0AF8: C1AA2F1C  lfs f13, 0x2f1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12060 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC0AFC: D1AB0014  stfs f13, 0x14(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DC0B00: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82DC0B04: D1AB0018  stfs f13, 0x18(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DC0B08: B0EB0038  sth r7, 0x38(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[7].u16 ) };
	// 82DC0B0C: D16B002C  stfs f11, 0x2c(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DC0B10: C0090C14  lfs f0, 0xc14(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0B14: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82DC0B18: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DC0B1C: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DC0B20: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82DC0B24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DC0B28: E9480000  ld r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82DC0B2C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82DC0B30: F9490000  std r10, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82DC0B34: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DC0B38: 4200FFF0  bdnz 0x82dc0b28
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC0B28; continue 'dispatch;
	}
	// 82DC0B3C: 38E31CA0  addi r7, r3, 0x1ca0
	ctx.r[7].s64 = ctx.r[3].s64 + 7328;
	// 82DC0B40: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DC0B44: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82DC0B48: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82DC0B4C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82DC0B50: E90A0000  ld r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DC0B54: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC0B58: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DC0B5C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DC0B60: 4200FFF0  bdnz 0x82dc0b50
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC0B50; continue 'dispatch;
	}
	// 82DC0B64: 39031CE0  addi r8, r3, 0x1ce0
	ctx.r[8].s64 = ctx.r[3].s64 + 7392;
	// 82DC0B68: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DC0B6C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82DC0B70: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82DC0B74: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82DC0B78: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DC0B7C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC0B80: F8C90000  std r6, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82DC0B84: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DC0B88: 4200FFF0  bdnz 0x82dc0b78
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC0B78; continue 'dispatch;
	}
	// 82DC0B8C: 39231D20  addi r9, r3, 0x1d20
	ctx.r[9].s64 = ctx.r[3].s64 + 7456;
	// 82DC0B90: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82DC0B94: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82DC0B98: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82DC0B9C: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DC0BA0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC0BA4: F8CA0000  std r6, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82DC0BA8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC0BAC: 4200FFF0  bdnz 0x82dc0b9c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC0B9C; continue 'dispatch;
	}
	// 82DC0BB0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0BB4: 3CC08203  lis r6, -0x7dfd
	ctx.r[6].s64 = -2113732608;
	// 82DC0BB8: D005000C  stfs f0, 0xc(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DC0BBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DC0BC0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0BC4: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DC0BC8: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0BCC: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DC0BD0: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC0BD4: C1A6FD40  lfs f13, -0x2c0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC0BD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0BDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82DC0BE0: C00BBE10  lfs f0, -0x41f0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16880 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0BE4: 419A0080  beq cr6, 0x82dc0c64
	if ctx.cr[6].eq {
	pc = 0x82DC0C64; continue 'dispatch;
	}
	// 82DC0BE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0BEC: C16B2F18  lfs f11, 0x2f18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12056 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0BF0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0BF4: D1670014  stfs f11, 0x14(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DC0BF8: C18B2F14  lfs f12, 0x2f14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12052 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DC0BFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0C00: D1870018  stfs f12, 0x18(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DC0C04: C16B2F10  lfs f11, 0x2f10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12048 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0C08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0C0C: D1670024  stfs f11, 0x24(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82DC0C10: C14B2F0C  lfs f10, 0x2f0c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12044 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DC0C14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0C18: D1470028  stfs f10, 0x28(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82DC0C1C: C16B2F08  lfs f11, 0x2f08(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12040 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0C20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0C24: D167002C  stfs f11, 0x2c(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DC0C28: C14B2F04  lfs f10, 0x2f04(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12036 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DC0C2C: D1470030  stfs f10, 0x30(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DC0C30: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0C34: ED605824  fdivs f11, f0, f11
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 82DC0C38: D167001C  stfs f11, 0x1c(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DC0C3C: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0C40: ED6D5824  fdivs f11, f13, f11
	ctx.f[11].f64 = ((ctx.f[13].f64 / ctx.f[11].f64) as f32) as f64;
	// 82DC0C44: D1670020  stfs f11, 0x20(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DC0C48: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0C4C: ED8C5824  fdivs f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 82DC0C50: ED8C0232  fmuls f12, f12, f8
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[8].f64) as f32) as f64);
	// 82DC0C54: D1870034  stfs f12, 0x34(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82DC0C58: A1640012  lhz r11, 0x12(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DC0C5C: 91470010  stw r10, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC0C60: B1670038  sth r11, 0x38(r7)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[7].u32.wrapping_add(56 as u32), ctx.r[11].u16 ) };
	// 82DC0C64: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC0C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0C6C: 419A0080  beq cr6, 0x82dc0cec
	if ctx.cr[6].eq {
	pc = 0x82DC0CEC; continue 'dispatch;
	}
	// 82DC0C70: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC0C74: C16B0C8C  lfs f11, 0xc8c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0C78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0C7C: D1680014  stfs f11, 0x14(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DC0C80: C18B2F00  lfs f12, 0x2f00(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12032 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DC0C84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0C88: D1880018  stfs f12, 0x18(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DC0C8C: C16B2EFC  lfs f11, 0x2efc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12028 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0C90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0C94: D1680024  stfs f11, 0x24(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82DC0C98: C14B2EF8  lfs f10, 0x2ef8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12024 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DC0C9C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82DC0CA0: D1480028  stfs f10, 0x28(r8)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82DC0CA4: C16B0EA4  lfs f11, 0xea4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3748 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0CA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DC0CAC: D168002C  stfs f11, 0x2c(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DC0CB0: C14BC40C  lfs f10, -0x3bf4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15348 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DC0CB4: D1480030  stfs f10, 0x30(r8)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DC0CB8: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0CBC: ED605824  fdivs f11, f0, f11
	ctx.f[11].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 82DC0CC0: D168001C  stfs f11, 0x1c(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DC0CC4: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0CC8: ED6D5824  fdivs f11, f13, f11
	ctx.f[11].f64 = ((ctx.f[13].f64 / ctx.f[11].f64) as f32) as f64;
	// 82DC0CCC: D1680020  stfs f11, 0x20(r8)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DC0CD0: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0CD4: ED8C5824  fdivs f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 82DC0CD8: ED8C0272  fmuls f12, f12, f9
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[9].f64) as f32) as f64);
	// 82DC0CDC: D1880034  stfs f12, 0x34(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82DC0CE0: A1640014  lhz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC0CE4: 91480010  stw r10, 0x10(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC0CE8: B1680038  sth r11, 0x38(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), ctx.r[11].u16 ) };
	// 82DC0CEC: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DC0CF0: D1890000  stfs f12, 0(r9)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DC0CF4: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC0CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC0CFC: 419A0080  beq cr6, 0x82dc0d7c
	if ctx.cr[6].eq {
	pc = 0x82DC0D7C; continue 'dispatch;
	}
	// 82DC0D00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0D04: C16B2EF4  lfs f11, 0x2ef4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12020 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0D08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0D0C: D1690014  stfs f11, 0x14(r9)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DC0D10: C18B2EF0  lfs f12, 0x2ef0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12016 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DC0D14: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82DC0D18: D1890018  stfs f12, 0x18(r9)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DC0D1C: C16B1024  lfs f11, 0x1024(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4132 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0D20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0D24: D1690024  stfs f11, 0x24(r9)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82DC0D28: C14B2EEC  lfs f10, 0x2eec(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12012 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DC0D2C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0D30: D1490028  stfs f10, 0x28(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82DC0D34: C16B2EE8  lfs f11, 0x2ee8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12008 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0D38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC0D3C: D169002C  stfs f11, 0x2c(r9)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DC0D40: C14B2EE4  lfs f10, 0x2ee4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12004 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DC0D44: D1490030  stfs f10, 0x30(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DC0D48: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DC0D4C: EC005824  fdivs f0, f0, f11
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 82DC0D50: D009001C  stfs f0, 0x1c(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DC0D54: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0D58: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DC0D5C: D0090020  stfs f0, 0x20(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DC0D60: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0D64: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DC0D68: EC000272  fmuls f0, f0, f9
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 82DC0D6C: D0090034  stfs f0, 0x34(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82DC0D70: A1640016  lhz r11, 0x16(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(22 as u32) ) } as u64;
	// 82DC0D74: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC0D78: B1690038  sth r11, 0x38(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(56 as u32), ctx.r[11].u16 ) };
	// 82DC0D7C: 39631D60  addi r11, r3, 0x1d60
	ctx.r[11].s64 = ctx.r[3].s64 + 7520;
	// 82DC0D80: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82DC0D84: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82DC0D88: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82DC0D8C: E8E90000  ld r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DC0D90: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DC0D94: F8E80000  std r7, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 82DC0D98: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82DC0D9C: 4200FFF0  bdnz 0x82dc0d8c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC0D8C; continue 'dispatch;
	}
	// 82DC0DA0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0DA4: 39231BB0  addi r9, r3, 0x1bb0
	ctx.r[9].s64 = ctx.r[3].s64 + 7088;
	// 82DC0DA8: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DC0DAC: 39000064  li r8, 0x64
	ctx.r[8].s64 = 100;
	// 82DC0DB0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC0DB4: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DC0DB8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82DC0DBC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82DC0DC0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82DC0DC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC0DC8: 4200FFF8  bdnz 0x82dc0dc0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC0DC0; continue 'dispatch;
	}
	// 82DC0DCC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82DC0DD0: 9BE31BBB  stb r31, 0x1bbb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7099 as u32), ctx.r[31].u8 ) };
	// 82DC0DD4: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82DC0DD8: 9BE31BBC  stb r31, 0x1bbc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7100 as u32), ctx.r[31].u8 ) };
	// 82DC0DDC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82DC0DE0: 99431BBD  stb r10, 0x1bbd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7101 as u32), ctx.r[10].u8 ) };
	// 82DC0DE4: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82DC0DE8: 9BE31BC5  stb r31, 0x1bc5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7109 as u32), ctx.r[31].u8 ) };
	// 82DC0DEC: 9BE31BC6  stb r31, 0x1bc6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7110 as u32), ctx.r[31].u8 ) };
	// 82DC0DF0: 99631BC3  stb r11, 0x1bc3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7107 as u32), ctx.r[11].u8 ) };
	// 82DC0DF4: 99231BBE  stb r9, 0x1bbe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7102 as u32), ctx.r[9].u8 ) };
	// 82DC0DF8: 98E31BBF  stb r7, 0x1bbf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7103 as u32), ctx.r[7].u8 ) };
	// 82DC0DFC: 99231BC0  stb r9, 0x1bc0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7104 as u32), ctx.r[9].u8 ) };
	// 82DC0E00: 99031BC2  stb r8, 0x1bc2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7106 as u32), ctx.r[8].u8 ) };
	// 82DC0E04: 99631BCD  stb r11, 0x1bcd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7117 as u32), ctx.r[11].u8 ) };
	// 82DC0E08: 99431BC7  stb r10, 0x1bc7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7111 as u32), ctx.r[10].u8 ) };
	// 82DC0E0C: 99631BC8  stb r11, 0x1bc8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7112 as u32), ctx.r[11].u8 ) };
	// 82DC0E10: 99631BC9  stb r11, 0x1bc9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7113 as u32), ctx.r[11].u8 ) };
	// 82DC0E14: 99631BCA  stb r11, 0x1bca(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7114 as u32), ctx.r[11].u8 ) };
	// 82DC0E18: 99631BCC  stb r11, 0x1bcc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7116 as u32), ctx.r[11].u8 ) };
	// 82DC0E1C: 99631C0B  stb r11, 0x1c0b(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7179 as u32), ctx.r[11].u8 ) };
	// 82DC0E20: 99631C0C  stb r11, 0x1c0c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7180 as u32), ctx.r[11].u8 ) };
	// 82DC0E24: 99631C13  stb r11, 0x1c13(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7187 as u32), ctx.r[11].u8 ) };
	// 82DC0E28: 99431C0D  stb r10, 0x1c0d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7181 as u32), ctx.r[10].u8 ) };
	// 82DC0E2C: 99631C0E  stb r11, 0x1c0e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7182 as u32), ctx.r[11].u8 ) };
	// 82DC0E30: 99631C0F  stb r11, 0x1c0f(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7183 as u32), ctx.r[11].u8 ) };
	// 82DC0E34: 99631C10  stb r11, 0x1c10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7184 as u32), ctx.r[11].u8 ) };
	// 82DC0E38: 99631C12  stb r11, 0x1c12(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7186 as u32), ctx.r[11].u8 ) };
	// 82DC0E3C: 99431BCF  stb r10, 0x1bcf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7119 as u32), ctx.r[10].u8 ) };
	// 82DC0E40: 99431BD0  stb r10, 0x1bd0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7120 as u32), ctx.r[10].u8 ) };
	// 82DC0E44: 99431BD7  stb r10, 0x1bd7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7127 as u32), ctx.r[10].u8 ) };
	// 82DC0E48: 99431BD1  stb r10, 0x1bd1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7121 as u32), ctx.r[10].u8 ) };
	// 82DC0E4C: 99431BD2  stb r10, 0x1bd2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7122 as u32), ctx.r[10].u8 ) };
	// 82DC0E50: 99431BD3  stb r10, 0x1bd3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7123 as u32), ctx.r[10].u8 ) };
	// 82DC0E54: 99631BD4  stb r11, 0x1bd4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7124 as u32), ctx.r[11].u8 ) };
	// 82DC0E58: 99431BD6  stb r10, 0x1bd6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7126 as u32), ctx.r[10].u8 ) };
	// 82DC0E5C: 99231BD9  stb r9, 0x1bd9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7129 as u32), ctx.r[9].u8 ) };
	// 82DC0E60: 99631BDA  stb r11, 0x1bda(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7130 as u32), ctx.r[11].u8 ) };
	// 82DC0E64: 99631BE1  stb r11, 0x1be1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7137 as u32), ctx.r[11].u8 ) };
	// 82DC0E68: 99431BDB  stb r10, 0x1bdb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7131 as u32), ctx.r[10].u8 ) };
	// 82DC0E6C: 99431BDC  stb r10, 0x1bdc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7132 as u32), ctx.r[10].u8 ) };
	// 82DC0E70: 99631BDD  stb r11, 0x1bdd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7133 as u32), ctx.r[11].u8 ) };
	// 82DC0E74: 99631BDE  stb r11, 0x1bde(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7134 as u32), ctx.r[11].u8 ) };
	// 82DC0E78: 99631BE0  stb r11, 0x1be0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7136 as u32), ctx.r[11].u8 ) };
	// 82DC0E7C: 98E31BE3  stb r7, 0x1be3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7139 as u32), ctx.r[7].u8 ) };
	// 82DC0E80: 99631BE4  stb r11, 0x1be4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7140 as u32), ctx.r[11].u8 ) };
	// 82DC0E84: 99631BEB  stb r11, 0x1beb(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7147 as u32), ctx.r[11].u8 ) };
	// 82DC0E88: 99431BE5  stb r10, 0x1be5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7141 as u32), ctx.r[10].u8 ) };
	// 82DC0E8C: 99631BE6  stb r11, 0x1be6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7142 as u32), ctx.r[11].u8 ) };
	// 82DC0E90: 99631BE7  stb r11, 0x1be7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7143 as u32), ctx.r[11].u8 ) };
	// 82DC0E94: 99631BE8  stb r11, 0x1be8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7144 as u32), ctx.r[11].u8 ) };
	// 82DC0E98: 99631BEA  stb r11, 0x1bea(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7146 as u32), ctx.r[11].u8 ) };
	// 82DC0E9C: 99231BED  stb r9, 0x1bed(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7149 as u32), ctx.r[9].u8 ) };
	// 82DC0EA0: 99631BEE  stb r11, 0x1bee(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7150 as u32), ctx.r[11].u8 ) };
	// 82DC0EA4: 99631BF5  stb r11, 0x1bf5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7157 as u32), ctx.r[11].u8 ) };
	// 82DC0EA8: 99631BEF  stb r11, 0x1bef(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7151 as u32), ctx.r[11].u8 ) };
	// 82DC0EAC: 99631BF0  stb r11, 0x1bf0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7152 as u32), ctx.r[11].u8 ) };
	// 82DC0EB0: 99631BF1  stb r11, 0x1bf1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7153 as u32), ctx.r[11].u8 ) };
	// 82DC0EB4: 99631BF2  stb r11, 0x1bf2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7154 as u32), ctx.r[11].u8 ) };
	// 82DC0EB8: 99631BF4  stb r11, 0x1bf4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7156 as u32), ctx.r[11].u8 ) };
	// 82DC0EBC: 99031C01  stb r8, 0x1c01(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7169 as u32), ctx.r[8].u8 ) };
	// 82DC0EC0: 99631C02  stb r11, 0x1c02(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7170 as u32), ctx.r[11].u8 ) };
	// 82DC0EC4: 99631C09  stb r11, 0x1c09(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7177 as u32), ctx.r[11].u8 ) };
	// 82DC0EC8: 99431C03  stb r10, 0x1c03(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7171 as u32), ctx.r[10].u8 ) };
	// 82DC0ECC: 99631C04  stb r11, 0x1c04(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7172 as u32), ctx.r[11].u8 ) };
	// 82DC0ED0: 99631C05  stb r11, 0x1c05(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7173 as u32), ctx.r[11].u8 ) };
	// 82DC0ED4: 99631C06  stb r11, 0x1c06(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7174 as u32), ctx.r[11].u8 ) };
	// 82DC0ED8: 99631C08  stb r11, 0x1c08(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7176 as u32), ctx.r[11].u8 ) };
	// 82DC0EDC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82DC0EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC0EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC0EE8 size=344
    let mut pc: u32 = 0x82DC0EE8;
    'dispatch: loop {
        match pc {
            0x82DC0EE8 => {
    //   block [0x82DC0EE8..0x82DC1040)
	// 82DC0EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC0EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC0EF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC0EF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC0EF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC0EFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC0F00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC0F04: 392A2F30  addi r9, r10, 0x2f30
	ctx.r[9].s64 = ctx.r[10].s64 + 12080;
	// 82DC0F08: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DC0F0C: 397F09A0  addi r11, r31, 0x9a0
	ctx.r[11].s64 = ctx.r[31].s64 + 2464;
	// 82DC0F10: 3940003F  li r10, 0x3f
	ctx.r[10].s64 = 63;
	// 82DC0F14: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DC0F18: 396B0011  addi r11, r11, 0x11
	ctx.r[11].s64 = ctx.r[11].s64 + 17;
	// 82DC0F1C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC0F20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC0F24: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DC0F28: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DC0F2C: 9BCBFFFF  stb r30, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[30].u8 ) };
	// 82DC0F30: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82DC0F34: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82DC0F38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC0F3C: 4098FFEC  bge cr6, 0x82dc0f28
	if !ctx.cr[6].lt {
	pc = 0x82DC0F28; continue 'dispatch;
	}
	// 82DC0F40: 397F16B0  addi r11, r31, 0x16b0
	ctx.r[11].s64 = ctx.r[31].s64 + 5808;
	// 82DC0F44: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 82DC0F48: 396B0031  addi r11, r11, 0x31
	ctx.r[11].s64 = ctx.r[11].s64 + 49;
	// 82DC0F4C: 9BCBFFFF  stb r30, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[30].u8 ) };
	// 82DC0F50: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DC0F54: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82DC0F58: 9BCB0001  stb r30, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 82DC0F5C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC0F60: 93CBFFE7  stw r30, -0x19(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25 as u32), ctx.r[30].u32 ) };
	// 82DC0F64: 93CBFFEB  stw r30, -0x15(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-21 as u32), ctx.r[30].u32 ) };
	// 82DC0F68: 93CBFFEF  stw r30, -0x11(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-17 as u32), ctx.r[30].u32 ) };
	// 82DC0F6C: 93CBFFF3  stw r30, -0xd(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-13 as u32), ctx.r[30].u32 ) };
	// 82DC0F70: 93CBFFF7  stw r30, -9(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-9 as u32), ctx.r[30].u32 ) };
	// 82DC0F74: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 82DC0F78: 4098FFD4  bge cr6, 0x82dc0f4c
	if !ctx.cr[6].lt {
	pc = 0x82DC0F4C; continue 'dispatch;
	}
	// 82DC0F7C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82DC0F80: 93DF1E24  stw r30, 0x1e24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7716 as u32), ctx.r[30].u32 ) };
	// 82DC0F84: 93DF1E28  stw r30, 0x1e28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7720 as u32), ctx.r[30].u32 ) };
	// 82DC0F88: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82DC0F8C: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82DC0F90: 915F1E2C  stw r10, 0x1e2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7724 as u32), ctx.r[10].u32 ) };
	// 82DC0F94: 93DF1E30  stw r30, 0x1e30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7728 as u32), ctx.r[30].u32 ) };
	// 82DC0F98: 93DF1E34  stw r30, 0x1e34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7732 as u32), ctx.r[30].u32 ) };
	// 82DC0F9C: 93DF1E38  stw r30, 0x1e38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7736 as u32), ctx.r[30].u32 ) };
	// 82DC0FA0: 93DF1E3C  stw r30, 0x1e3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7740 as u32), ctx.r[30].u32 ) };
	// 82DC0FA4: 9BDF1E20  stb r30, 0x1e20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7712 as u32), ctx.r[30].u8 ) };
	// 82DC0FA8: 98FF1E23  stb r7, 0x1e23(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7715 as u32), ctx.r[7].u8 ) };
	// 82DC0FAC: 93DF0EA0  stw r30, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[30].u32 ) };
	// 82DC0FB0: 9BDF1E22  stb r30, 0x1e22(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7714 as u32), ctx.r[30].u8 ) };
	// 82DC0FB4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DC0FB8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DC0FBC: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DC0FC0: 419A001C  beq cr6, 0x82dc0fdc
	if ctx.cr[6].eq {
	pc = 0x82DC0FDC; continue 'dispatch;
	}
	// 82DC0FC4: A1450004  lhz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC0FC8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC0FCC: 419A0010  beq cr6, 0x82dc0fdc
	if ctx.cr[6].eq {
	pc = 0x82DC0FDC; continue 'dispatch;
	}
	// 82DC0FD0: A1450006  lhz r10, 6(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC0FD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC0FD8: B1450006  sth r10, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DC0FDC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DC0FE0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DC0FE4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DC0FE8: 409AFFD0  bne cr6, 0x82dc0fb8
	if !ctx.cr[6].eq {
	pc = 0x82DC0FB8; continue 'dispatch;
	}
	// 82DC0FEC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DC0FF0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DC0FF4: 409AFFC0  bne cr6, 0x82dc0fb4
	if !ctx.cr[6].eq {
	pc = 0x82DC0FB4; continue 'dispatch;
	}
	// 82DC0FF8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DC0FFC: 395F0110  addi r10, r31, 0x110
	ctx.r[10].s64 = ctx.r[31].s64 + 272;
	// 82DC1000: 7CE95830  slw r9, r7, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[7].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DC1004: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC1008: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 82DC100C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC1010: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DC1014: 4198FFEC  blt cr6, 0x82dc1000
	if ctx.cr[6].lt {
	pc = 0x82DC1000; continue 'dispatch;
	}
	// 82DC1018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC101C: 4BFFF37D  bl 0x82dc0398
	ctx.lr = 0x82DC1020;
	sub_82DC0398(ctx, base);
	// 82DC1020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC1024: 9BDF1E21  stb r30, 0x1e21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7713 as u32), ctx.r[30].u8 ) };
	// 82DC1028: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC102C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC1030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC1034: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC1038: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC103C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1040 size=292
    let mut pc: u32 = 0x82DC1040;
    'dispatch: loop {
        match pc {
            0x82DC1040 => {
    //   block [0x82DC1040..0x82DC1164)
	// 82DC1040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC1044: 4BEE83C5  bl 0x82ca9408
	ctx.lr = 0x82DC1048;
	sub_82CA93D0(ctx, base);
	// 82DC1048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC104C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC1050: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC1054: 396B2F30  addi r11, r11, 0x2f30
	ctx.r[11].s64 = ctx.r[11].s64 + 12080;
	// 82DC1058: 809D1E30  lwz r4, 0x1e30(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC105C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DC1060: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC1064: 419A004C  beq cr6, 0x82dc10b0
	if ctx.cr[6].eq {
	pc = 0x82DC10B0; continue 'dispatch;
	}
	// 82DC1068: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC106C: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 82DC1070: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC1074: 4BF9445D  bl 0x82d554d0
	ctx.lr = 0x82DC1078;
	sub_82D554D0(ctx, base);
	// 82DC1078: 809D1E34  lwz r4, 0x1e34(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC107C: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC1080: 4BF94451  bl 0x82d554d0
	ctx.lr = 0x82DC1084;
	sub_82D554D0(ctx, base);
	// 82DC1084: 809D1E38  lwz r4, 0x1e38(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC1088: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC108C: 4BF94445  bl 0x82d554d0
	ctx.lr = 0x82DC1090;
	sub_82D554D0(ctx, base);
	// 82DC1090: 809D1E3C  lwz r4, 0x1e3c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC1094: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC1098: 4BF94439  bl 0x82d554d0
	ctx.lr = 0x82DC109C;
	sub_82D554D0(ctx, base);
	// 82DC109C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC10A0: 917D1E30  stw r11, 0x1e30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7728 as u32), ctx.r[11].u32 ) };
	// 82DC10A4: 917D1E34  stw r11, 0x1e34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7732 as u32), ctx.r[11].u32 ) };
	// 82DC10A8: 917D1E38  stw r11, 0x1e38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7736 as u32), ctx.r[11].u32 ) };
	// 82DC10AC: 917D1E3C  stw r11, 0x1e3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(7740 as u32), ctx.r[11].u32 ) };
	// 82DC10B0: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 82DC10B4: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DC10B8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DC10BC: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82DC10C0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC10C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DC10C8: 419A003C  beq cr6, 0x82dc1104
	if ctx.cr[6].eq {
	pc = 0x82DC1104; continue 'dispatch;
	}
	// 82DC10CC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC10D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC10D4: 419A0030  beq cr6, 0x82dc1104
	if ctx.cr[6].eq {
	pc = 0x82DC1104; continue 'dispatch;
	}
	// 82DC10D8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC10DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC10E0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC10E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC10E8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC10EC: 409A0018  bne cr6, 0x82dc1104
	if !ctx.cr[6].eq {
	pc = 0x82DC1104; continue 'dispatch;
	}
	// 82DC10F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC10F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC10F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC10FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1100: 4E800421  bctrl
	ctx.lr = 0x82DC1104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1104: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DC1108: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DC110C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DC1110: 409AFFB0  bne cr6, 0x82dc10c0
	if !ctx.cr[6].eq {
	pc = 0x82DC10C0; continue 'dispatch;
	}
	// 82DC1114: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DC1118: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DC111C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DC1120: 409AFF98  bne cr6, 0x82dc10b8
	if !ctx.cr[6].eq {
	pc = 0x82DC10B8; continue 'dispatch;
	}
	// 82DC1124: 817D1E2C  lwz r11, 0x1e2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7724 as u32) ) } as u64;
	// 82DC1128: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC112C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC1130: 409A0020  bne cr6, 0x82dc1150
	if !ctx.cr[6].eq {
	pc = 0x82DC1150; continue 'dispatch;
	}
	// 82DC1134: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1138: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC113C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC1140: 809D1E24  lwz r4, 0x1e24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(7716 as u32) ) } as u64;
	// 82DC1144: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC1148: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC114C: 4BF9417D  bl 0x82d552c8
	ctx.lr = 0x82DC1150;
	sub_82D552C8(ctx, base);
	// 82DC1150: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC1154: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DC1158: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC115C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC1160: 4BEE82F8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1168 size=280
    let mut pc: u32 = 0x82DC1168;
    'dispatch: loop {
        match pc {
            0x82DC1168 => {
    //   block [0x82DC1168..0x82DC1280)
	// 82DC1168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC116C: 4BEE8299  bl 0x82ca9404
	ctx.lr = 0x82DC1170;
	sub_82CA93D0(ctx, base);
	// 82DC1170: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC1174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC1178: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DC117C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DC1180: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DC1184: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DC1188: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC118C: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82DC1190: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC1194: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DC1198: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC119C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DC11A0: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82DC11A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC11A8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC11AC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DC11B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC11B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DC11B8: 4200FFF0  bdnz 0x82dc11a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC11A8; continue 'dispatch;
	}
	// 82DC11BC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DC11C0: 815F1E38  lwz r10, 0x1e38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC11C4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC11C8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC11CC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC11D0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC11D4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC11D8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC11DC: 389F0EB0  addi r4, r31, 0xeb0
	ctx.r[4].s64 = ctx.r[31].s64 + 3760;
	// 82DC11E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC11E4: 4BFFF58D  bl 0x82dc0770
	ctx.lr = 0x82DC11E8;
	sub_82DC0770(ctx, base);
	// 82DC11E8: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC11EC: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC11F0: 815F1E30  lwz r10, 0x1e30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC11F4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC11F8: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC11FC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC1200: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC1204: 389F01A0  addi r4, r31, 0x1a0
	ctx.r[4].s64 = ctx.r[31].s64 + 416;
	// 82DC1208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC120C: 4BFFF565  bl 0x82dc0770
	ctx.lr = 0x82DC1210;
	sub_82DC0770(ctx, base);
	// 82DC1210: 897C0011  lbz r11, 0x11(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DC1214: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC1218: 419A0054  beq cr6, 0x82dc126c
	if ctx.cr[6].eq {
	pc = 0x82DC126C; continue 'dispatch;
	}
	// 82DC121C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC1220: 815F1E3C  lwz r10, 0x1e3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC1224: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC1228: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC122C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC1230: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC1234: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC1238: 389F12B0  addi r4, r31, 0x12b0
	ctx.r[4].s64 = ctx.r[31].s64 + 4784;
	// 82DC123C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC1240: 4BFFF531  bl 0x82dc0770
	ctx.lr = 0x82DC1244;
	sub_82DC0770(ctx, base);
	// 82DC1244: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC1248: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC124C: 815F1E34  lwz r10, 0x1e34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC1250: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC1254: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC1258: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC125C: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC1260: 389F05A0  addi r4, r31, 0x5a0
	ctx.r[4].s64 = ctx.r[31].s64 + 1440;
	// 82DC1264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC1268: 4BFFF509  bl 0x82dc0770
	ctx.lr = 0x82DC126C;
	sub_82DC0770(ctx, base);
	// 82DC126C: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC1270: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC1274: 917F0190  stw r11, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 82DC1278: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC127C: 4BEE81D8  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1280 size=200
    let mut pc: u32 = 0x82DC1280;
    'dispatch: loop {
        match pc {
            0x82DC1280 => {
    //   block [0x82DC1280..0x82DC1348)
	// 82DC1280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC1284: 4BEE8181  bl 0x82ca9404
	ctx.lr = 0x82DC1288;
	sub_82CA93D0(ctx, base);
	// 82DC1288: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC128C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC1290: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DC1294: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DC1298: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DC129C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DC12A0: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC12A4: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82DC12A8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC12AC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DC12B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC12B4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DC12B8: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82DC12BC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC12C0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC12C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DC12C8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC12CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DC12D0: 4200FFF0  bdnz 0x82dc12c0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC12C0; continue 'dispatch;
	}
	// 82DC12D4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DC12D8: 815F1E30  lwz r10, 0x1e30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC12DC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC12E0: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC12E4: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC12E8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC12EC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC12F0: 389F01A0  addi r4, r31, 0x1a0
	ctx.r[4].s64 = ctx.r[31].s64 + 416;
	// 82DC12F4: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC12F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC12FC: 4BFFF475  bl 0x82dc0770
	ctx.lr = 0x82DC1300;
	sub_82DC0770(ctx, base);
	// 82DC1300: 897C0011  lbz r11, 0x11(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DC1304: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC1308: 419A002C  beq cr6, 0x82dc1334
	if ctx.cr[6].eq {
	pc = 0x82DC1334; continue 'dispatch;
	}
	// 82DC130C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC1310: 815F1E34  lwz r10, 0x1e34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC1314: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82DC1318: 80BF0190  lwz r5, 0x190(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC131C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC1320: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC1324: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC1328: 389F05A0  addi r4, r31, 0x5a0
	ctx.r[4].s64 = ctx.r[31].s64 + 1440;
	// 82DC132C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC1330: 4BFFF441  bl 0x82dc0770
	ctx.lr = 0x82DC1334;
	sub_82DC0770(ctx, base);
	// 82DC1334: 817F0190  lwz r11, 0x190(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(400 as u32) ) } as u64;
	// 82DC1338: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC133C: 917F0190  stw r11, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[11].u32 ) };
	// 82DC1340: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC1344: 4BEE8110  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1348 size=468
    let mut pc: u32 = 0x82DC1348;
    'dispatch: loop {
        match pc {
            0x82DC1348 => {
    //   block [0x82DC1348..0x82DC151C)
	// 82DC1348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC134C: 4BEE80B1  bl 0x82ca93fc
	ctx.lr = 0x82DC1350;
	sub_82CA93D0(ctx, base);
	// 82DC1350: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC1354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC1358: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DC135C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC1360: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC1364: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC1368: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DC136C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DC1370: 9B5F1E21  stb r26, 0x1e21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7713 as u32), ctx.r[26].u8 ) };
	// 82DC1374: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DC1378: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC137C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DC1380: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC1384: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DC1388: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC138C: 4200FFF0  bdnz 0x82dc137c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC137C; continue 'dispatch;
	}
	// 82DC1390: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DC1394: 8B210090  lbz r25, 0x90(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DC1398: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82DC139C: 936100A0  stw r27, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u32 ) };
	// 82DC13A0: 419A00BC  beq cr6, 0x82dc145c
	if ctx.cr[6].eq {
	pc = 0x82DC145C; continue 'dispatch;
	}
	// 82DC13A4: 897E0031  lbz r11, 0x31(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(49 as u32) ) } as u64;
	// 82DC13A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC13AC: 409A00B0  bne cr6, 0x82dc145c
	if !ctx.cr[6].eq {
	pc = 0x82DC145C; continue 'dispatch;
	}
	// 82DC13B0: 817F0EA0  lwz r11, 0xea0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82DC13B4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82DC13B8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82DC13BC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC13C0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DC13C4: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82DC13C8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DC13CC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC13D0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DC13D4: 396B16B0  addi r11, r11, 0x16b0
	ctx.r[11].s64 = ctx.r[11].s64 + 5808;
	// 82DC13D8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC13DC: E92A0000  ld r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DC13E0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC13E4: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DC13E8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC13EC: 4200FFF0  bdnz 0x82dc13dc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC13DC; continue 'dispatch;
	}
	// 82DC13F0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC13F4: 815F1E38  lwz r10, 0x1e38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC13F8: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82DC13FC: 80BF0EA0  lwz r5, 0xea0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82DC1400: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC1404: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC1408: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC140C: 389F0EB0  addi r4, r31, 0xeb0
	ctx.r[4].s64 = ctx.r[31].s64 + 3760;
	// 82DC1410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC1414: 4BFFF35D  bl 0x82dc0770
	ctx.lr = 0x82DC1418;
	sub_82DC0770(ctx, base);
	// 82DC1418: 7F2B0774  extsb r11, r25
	ctx.r[11].s64 = ctx.r[25].s8 as i64;
	// 82DC141C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC1420: 419A002C  beq cr6, 0x82dc144c
	if ctx.cr[6].eq {
	pc = 0x82DC144C; continue 'dispatch;
	}
	// 82DC1424: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82DC1428: 815F1E3C  lwz r10, 0x1e3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC142C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82DC1430: 80BF0EA0  lwz r5, 0xea0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82DC1434: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DC1438: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC143C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC1440: 389F12B0  addi r4, r31, 0x12b0
	ctx.r[4].s64 = ctx.r[31].s64 + 4784;
	// 82DC1444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC1448: 4BFFF329  bl 0x82dc0770
	ctx.lr = 0x82DC144C;
	sub_82DC0770(ctx, base);
	// 82DC144C: 817F0EA0  lwz r11, 0xea0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82DC1450: 934100A0  stw r26, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[26].u32 ) };
	// 82DC1454: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC1458: 917F0EA0  stw r11, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[11].u32 ) };
	// 82DC145C: 897E0032  lbz r11, 0x32(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(50 as u32) ) } as u64;
	// 82DC1460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC1464: 409A0048  bne cr6, 0x82dc14ac
	if !ctx.cr[6].eq {
	pc = 0x82DC14AC; continue 'dispatch;
	}
	// 82DC1468: 83DF0EA0  lwz r30, 0xea0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82DC146C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DC1470: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82DC1474: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 82DC1478: 915F0EA0  stw r10, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[10].u32 ) };
	// 82DC147C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC1480: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 82DC1484: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC1488: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82DC148C: 394A16B0  addi r10, r10, 0x16b0
	ctx.r[10].s64 = ctx.r[10].s64 + 5808;
	// 82DC1490: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC1494: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DC1498: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC149C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DC14A0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC14A4: 4200FFF0  bdnz 0x82dc1494
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC1494; continue 'dispatch;
	}
	// 82DC14A8: 4800000C  b 0x82dc14b4
	pc = 0x82DC14B4; continue 'dispatch;
	// 82DC14AC: 817F0EA0  lwz r11, 0xea0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3744 as u32) ) } as u64;
	// 82DC14B0: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82DC14B4: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82DC14B8: 815F1E38  lwz r10, 0x1e38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC14BC: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DC14C0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC14C4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DC14C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DC14CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DC14D0: 389F0EB0  addi r4, r31, 0xeb0
	ctx.r[4].s64 = ctx.r[31].s64 + 3760;
	// 82DC14D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC14D8: 4BFFF299  bl 0x82dc0770
	ctx.lr = 0x82DC14DC;
	sub_82DC0770(ctx, base);
	// 82DC14DC: 7F2B0774  extsb r11, r25
	ctx.r[11].s64 = ctx.r[25].s8 as i64;
	// 82DC14E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC14E4: 419A002C  beq cr6, 0x82dc1510
	if ctx.cr[6].eq {
	pc = 0x82DC1510; continue 'dispatch;
	}
	// 82DC14E8: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82DC14EC: 815F1E3C  lwz r10, 0x1e3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC14F0: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DC14F4: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DC14F8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DC14FC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DC1500: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DC1504: 389F12B0  addi r4, r31, 0x12b0
	ctx.r[4].s64 = ctx.r[31].s64 + 4784;
	// 82DC1508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC150C: 4BFFF265  bl 0x82dc0770
	ctx.lr = 0x82DC1510;
	sub_82DC0770(ctx, base);
	// 82DC1510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC1514: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DC1518: 4BEE7F34  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1520 size=280
    let mut pc: u32 = 0x82DC1520;
    'dispatch: loop {
        match pc {
            0x82DC1520 => {
    //   block [0x82DC1520..0x82DC1638)
	// 82DC1520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC1524: 4BEE7EE9  bl 0x82ca940c
	ctx.lr = 0x82DC1528;
	sub_82CA93D0(ctx, base);
	// 82DC1528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC152C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC1530: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC1534: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC1538: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DC153C: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82DC1540: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC1544: 40990094  ble cr6, 0x82dc15d8
	if !ctx.cr[6].gt {
	pc = 0x82DC15D8; continue 'dispatch;
	}
	// 82DC1548: 391F1E24  addi r8, r31, 0x1e24
	ctx.r[8].s64 = ctx.r[31].s64 + 7716;
	// 82DC154C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DC1550: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1554: 7D665A14  add r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82DC1558: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC155C: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82DC1560: 409A0064  bne cr6, 0x82dc15c4
	if !ctx.cr[6].eq {
	pc = 0x82DC15C4; continue 'dispatch;
	}
	// 82DC1564: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC1568: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC156C: 409A0058  bne cr6, 0x82dc15c4
	if !ctx.cr[6].eq {
	pc = 0x82DC15C4; continue 'dispatch;
	}
	// 82DC1570: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC1574: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DC1578: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC157C: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC1580: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC1584: 40980038  bge cr6, 0x82dc15bc
	if !ctx.cr[6].lt {
	pc = 0x82DC15BC; continue 'dispatch;
	}
	// 82DC1588: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82DC158C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1590: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC1594: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC1598: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DC159C: 38AB0008  addi r5, r11, 8
	ctx.r[5].s64 = ctx.r[11].s64 + 8;
	// 82DC15A0: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC15A4: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DC15A8: 80A50004  lwz r5, 4(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC15AC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DC15B0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC15B4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC15B8: 4198FFD4  blt cr6, 0x82dc158c
	if ctx.cr[6].lt {
	pc = 0x82DC158C; continue 'dispatch;
	}
	// 82DC15BC: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82DC15C0: 38C6FFF8  addi r6, r6, -8
	ctx.r[6].s64 = ctx.r[6].s64 + -8;
	// 82DC15C4: 817F1E28  lwz r11, 0x1e28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82DC15C8: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DC15CC: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 82DC15D0: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC15D4: 4198FF7C  blt cr6, 0x82dc1550
	if ctx.cr[6].lt {
	pc = 0x82DC1550; continue 'dispatch;
	}
	// 82DC15D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DC15DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC15E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DC15E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC15E8: 4BFFF3A9  bl 0x82dc0990
	ctx.lr = 0x82DC15EC;
	sub_82DC0990(ctx, base);
	// 82DC15EC: 3BFF1E24  addi r31, r31, 0x1e24
	ctx.r[31].s64 = ctx.r[31].s64 + 7716;
	// 82DC15F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC15F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC15F8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC15FC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC1600: 409A0010  bne cr6, 0x82dc1610
	if !ctx.cr[6].eq {
	pc = 0x82DC1610; continue 'dispatch;
	}
	// 82DC1604: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82DC1608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC160C: 4BF9598D  bl 0x82d56f98
	ctx.lr = 0x82DC1610;
	sub_82D56F98(ctx, base);
	// 82DC1610: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC1614: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1618: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC161C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82DC1620: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DC1624: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC1628: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DC162C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC1630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC1634: 4BEE7E28  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1638 size=320
    let mut pc: u32 = 0x82DC1638;
    'dispatch: loop {
        match pc {
            0x82DC1638 => {
    //   block [0x82DC1638..0x82DC1778)
	// 82DC1638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC163C: 4BEE7DC9  bl 0x82ca9404
	ctx.lr = 0x82DC1640;
	sub_82CA93D0(ctx, base);
	// 82DC1640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC1644: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DC1648: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC164C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DC1650: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DC1654: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DC1658: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC165C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC1660: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC1664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1668: 4E800421  bctrl
	ctx.lr = 0x82DC166C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC166C: 80DF1E30  lwz r6, 0x1e30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC1670: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DC1674: 419A0058  beq cr6, 0x82dc16cc
	if ctx.cr[6].eq {
	pc = 0x82DC16CC; continue 'dispatch;
	}
	// 82DC1678: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DC167C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC1680: 3BCB677C  addi r30, r11, 0x677c
	ctx.r[30].s64 = ctx.r[11].s64 + 26492;
	// 82DC1684: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC1688: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DC168C: 4BF935A5  bl 0x82d54c30
	ctx.lr = 0x82DC1690;
	sub_82D54C30(ctx, base);
	// 82DC1690: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC1694: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DC1698: 80DF1E34  lwz r6, 0x1e34(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC169C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC16A0: 4BF93591  bl 0x82d54c30
	ctx.lr = 0x82DC16A4;
	sub_82D54C30(ctx, base);
	// 82DC16A4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC16A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DC16AC: 80DF1E38  lwz r6, 0x1e38(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82DC16B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC16B4: 4BF9357D  bl 0x82d54c30
	ctx.lr = 0x82DC16B8;
	sub_82D54C30(ctx, base);
	// 82DC16B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC16BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DC16C0: 80DF1E3C  lwz r6, 0x1e3c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82DC16C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC16C8: 4BF93569  bl 0x82d54c30
	ctx.lr = 0x82DC16CC;
	sub_82D54C30(ctx, base);
	// 82DC16CC: 815F1E2C  lwz r10, 0x1e2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7724 as u32) ) } as u64;
	// 82DC16D0: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC16D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC16D8: 409A0034  bne cr6, 0x82dc170c
	if !ctx.cr[6].eq {
	pc = 0x82DC170C; continue 'dispatch;
	}
	// 82DC16DC: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC16E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC16E4: 80FF1E28  lwz r7, 0x1e28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7720 as u32) ) } as u64;
	// 82DC16E8: 55481838  slwi r8, r10, 3
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC16EC: 388B2F50  addi r4, r11, 0x2f50
	ctx.r[4].s64 = ctx.r[11].s64 + 12112;
	// 82DC16F0: 80DF1E24  lwz r6, 0x1e24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7716 as u32) ) } as u64;
	// 82DC16F4: 54E71838  slwi r7, r7, 3
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DC16F8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC16FC: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC1700: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC1704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1708: 4E800421  bctrl
	ctx.lr = 0x82DC170C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC170C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC1710: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82DC1714: 3B600008  li r27, 8
	ctx.r[27].s64 = 8;
	// 82DC1718: 3B8B2F3C  addi r28, r11, 0x2f3c
	ctx.r[28].s64 = ctx.r[11].s64 + 12092;
	// 82DC171C: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82DC1720: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1724: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC1728: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC172C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DC1730: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC1734: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC1738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC173C: 4E800421  bctrl
	ctx.lr = 0x82DC1740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1740: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82DC1744: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DC1748: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82DC174C: 409AFFD4  bne cr6, 0x82dc1720
	if !ctx.cr[6].eq {
	pc = 0x82DC1720; continue 'dispatch;
	}
	// 82DC1750: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82DC1754: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DC1758: 409AFFC4  bne cr6, 0x82dc171c
	if !ctx.cr[6].eq {
	pc = 0x82DC171C; continue 'dispatch;
	}
	// 82DC175C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1760: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC1764: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC1768: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC176C: 4E800421  bctrl
	ctx.lr = 0x82DC1770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC1774: 4BEE7CE0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1778 size=1156
    let mut pc: u32 = 0x82DC1778;
    'dispatch: loop {
        match pc {
            0x82DC1778 => {
    //   block [0x82DC1778..0x82DC1BFC)
	// 82DC1778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC177C: 4BEE7C65  bl 0x82ca93e0
	ctx.lr = 0x82DC1780;
	sub_82CA93D0(ctx, base);
	// 82DC1780: 9421FBB0  stwu r1, -0x450(r1)
	ea = ctx.r[1].u32.wrapping_add(-1104 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC1784: 3F208333  lis r25, -0x7ccd
	ctx.r[25].s64 = -2093809664;
	// 82DC1788: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82DC178C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC1790: 3C805E43  lis r4, 0x5e43
	ctx.r[4].s64 = 1581449216;
	// 82DC1794: 38AB2FD8  addi r5, r11, 0x2fd8
	ctx.r[5].s64 = ctx.r[11].s64 + 12248;
	// 82DC1798: 80797630  lwz r3, 0x7630(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82DC179C: 608445E4  ori r4, r4, 0x45e4
	ctx.r[4].u64 = ctx.r[4].u64 | 17892;
	// 82DC17A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC17A4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC17A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC17AC: 4E800421  bctrl
	ctx.lr = 0x82DC17B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC17B0: 81731E30  lwz r11, 0x1e30(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC17B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC17B8: 419A043C  beq cr6, 0x82dc1bf4
	if ctx.cr[6].eq {
	pc = 0x82DC1BF4; continue 'dispatch;
	}
	// 82DC17BC: 81731E34  lwz r11, 0x1e34(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC17C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC17C4: 419A0430  beq cr6, 0x82dc1bf4
	if ctx.cr[6].eq {
	pc = 0x82DC1BF4; continue 'dispatch;
	}
	// 82DC17C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC17CC: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82DC17D0: 3AAB2FC4  addi r21, r11, 0x2fc4
	ctx.r[21].s64 = ctx.r[11].s64 + 12228;
	// 82DC17D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC17D8: 7E96A378  mr r22, r20
	ctx.r[22].u64 = ctx.r[20].u64;
	// 82DC17DC: 3B4B2F9C  addi r26, r11, 0x2f9c
	ctx.r[26].s64 = ctx.r[11].s64 + 12188;
	// 82DC17E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC17E4: 3AE00003  li r23, 3
	ctx.r[23].s64 = 3;
	// 82DC17E8: 3B0B2F80  addi r24, r11, 0x2f80
	ctx.r[24].s64 = ctx.r[11].s64 + 12160;
	// 82DC17EC: 3E408000  lis r18, -0x8000
	ctx.r[18].s64 = -2147483648;
	// 82DC17F0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DC17F4: 92810060  stw r20, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[20].u32 ) };
	// 82DC17F8: 92810064  stw r20, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[20].u32 ) };
	// 82DC17FC: 92410068  stw r18, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[18].u32 ) };
	// 82DC1800: 4BF97429  bl 0x82d58c28
	ctx.lr = 0x82DC1804;
	sub_82D58C28(ctx, base);
	// 82DC1804: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DC1808: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82DC180C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC1810: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DC1814: 40980024  bge cr6, 0x82dc1838
	if !ctx.cr[6].lt {
	pc = 0x82DC1838; continue 'dispatch;
	}
	// 82DC1818: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC181C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC1820: 41980008  blt cr6, 0x82dc1828
	if ctx.cr[6].lt {
	pc = 0x82DC1828; continue 'dispatch;
	}
	// 82DC1824: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DC1828: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC182C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC1830: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DC1834: 4BF956DD  bl 0x82d56f10
	ctx.lr = 0x82DC1838;
	sub_82D56F10(ctx, base);
	// 82DC1838: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DC183C: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DC1840: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82DC1844: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82DC1848: 4BF974E9  bl 0x82d58d30
	ctx.lr = 0x82DC184C;
	sub_82D58D30(ctx, base);
	// 82DC184C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DC1850: 4801E9B9  bl 0x82de0208
	ctx.lr = 0x82DC1854;
	sub_82DE0208(ctx, base);
	// 82DC1854: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DC1858: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DC185C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DC1860: 4BF97B31  bl 0x82d59390
	ctx.lr = 0x82DC1864;
	sub_82D59390(ctx, base);
	// 82DC1864: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DC1868: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC186C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC1870: 409A0020  bne cr6, 0x82dc1890
	if !ctx.cr[6].eq {
	pc = 0x82DC1890; continue 'dispatch;
	}
	// 82DC1874: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1878: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC187C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC1880: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DC1884: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC1888: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC188C: 4BF93A3D  bl 0x82d552c8
	ctx.lr = 0x82DC1890;
	sub_82D552C8(ctx, base);
	// 82DC1890: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82DC1894: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82DC1898: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82DC189C: 4BF9614D  bl 0x82d579e8
	ctx.lr = 0x82DC18A0;
	sub_82D579E8(ctx, base);
	// 82DC18A0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DC18A4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82DC18A8: 4BF96CA1  bl 0x82d58548
	ctx.lr = 0x82DC18AC;
	sub_82D58548(ctx, base);
	// 82DC18AC: 80797630  lwz r3, 0x7630(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82DC18B0: 3900021F  li r8, 0x21f
	ctx.r[8].s64 = 543;
	// 82DC18B4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DC18B8: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 82DC18BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DC18C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC18C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DC18C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC18CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC18D0: 4E800421  bctrl
	ctx.lr = 0x82DC18D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC18D4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82DC18D8: 4BF96B59  bl 0x82d58430
	ctx.lr = 0x82DC18DC;
	sub_82D58430(ctx, base);
	// 82DC18DC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82DC18E0: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 82DC18E4: 81731E34  lwz r11, 0x1e34(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82DC18E8: 7FFB5A14  add r31, r27, r11
	ctx.r[31].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82DC18EC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DC18F0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DC18F4: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 82DC18F8: 4098009C  bge cr6, 0x82dc1994
	if !ctx.cr[6].lt {
	pc = 0x82DC1994; continue 'dispatch;
	}
	// 82DC18FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DC1900: 4801E909  bl 0x82de0208
	ctx.lr = 0x82DC1904;
	sub_82DE0208(ctx, base);
	// 82DC1904: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1908: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC190C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82DC1910: 4801E8F9  bl 0x82de0208
	ctx.lr = 0x82DC1914;
	sub_82DE0208(ctx, base);
	// 82DC1914: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC1918: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC191C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82DC1920: 4801E8E9  bl 0x82de0208
	ctx.lr = 0x82DC1924;
	sub_82DE0208(ctx, base);
	// 82DC1924: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DC1928: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DC192C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DC1930: 7D670774  extsb r7, r11
	ctx.r[7].s64 = ctx.r[11].s8 as i64;
	// 82DC1934: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC1938: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82DC193C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82DC1940: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82DC1944: 4BF97005  bl 0x82d58948
	ctx.lr = 0x82DC1948;
	sub_82D58948(ctx, base);
	// 82DC1948: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82DC194C: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82DC1950: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DC1954: 4BF96095  bl 0x82d579e8
	ctx.lr = 0x82DC1958;
	sub_82D579E8(ctx, base);
	// 82DC1958: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82DC195C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DC1960: 4BF96491  bl 0x82d57df0
	ctx.lr = 0x82DC1964;
	sub_82D57DF0(ctx, base);
	// 82DC1964: 80797630  lwz r3, 0x7630(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82DC1968: 3900022E  li r8, 0x22e
	ctx.r[8].s64 = 558;
	// 82DC196C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DC1970: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 82DC1974: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DC1978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC197C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DC1980: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC1984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1988: 4E800421  bctrl
	ctx.lr = 0x82DC198C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC198C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DC1990: 4BF96AA1  bl 0x82d58430
	ctx.lr = 0x82DC1994;
	sub_82D58430(ctx, base);
	// 82DC1994: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DC1998: 3B7B0003  addi r27, r27, 3
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	// 82DC199C: 2F1C0020  cmpwi cr6, r28, 0x20
	ctx.cr[6].compare_i32(ctx.r[28].s32, 32, &mut ctx.xer);
	// 82DC19A0: 4198FF44  blt cr6, 0x82dc18e4
	if ctx.cr[6].lt {
	pc = 0x82DC18E4; continue 'dispatch;
	}
	// 82DC19A4: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82DC19A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC19AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC19B0: 409A0020  bne cr6, 0x82dc19d0
	if !ctx.cr[6].eq {
	pc = 0x82DC19D0; continue 'dispatch;
	}
	// 82DC19B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC19B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC19BC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC19C0: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82DC19C4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC19C8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC19CC: 4BF938FD  bl 0x82d552c8
	ctx.lr = 0x82DC19D0;
	sub_82D552C8(ctx, base);
	// 82DC19D0: 3AF70060  addi r23, r23, 0x60
	ctx.r[23].s64 = ctx.r[23].s64 + 96;
	// 82DC19D4: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82DC19D8: 2F170C03  cmpwi cr6, r23, 0xc03
	ctx.cr[6].compare_i32(ctx.r[23].s32, 3075, &mut ctx.xer);
	// 82DC19DC: 4198FE14  blt cr6, 0x82dc17f0
	if ctx.cr[6].lt {
	pc = 0x82DC17F0; continue 'dispatch;
	}
	// 82DC19E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC19E4: 7E96A378  mr r22, r20
	ctx.r[22].u64 = ctx.r[20].u64;
	// 82DC19E8: 3B000003  li r24, 3
	ctx.r[24].s64 = 3;
	// 82DC19EC: 3AEB2F68  addi r23, r11, 0x2f68
	ctx.r[23].s64 = ctx.r[11].s64 + 12136;
	// 82DC19F0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DC19F4: 92810050  stw r20, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[20].u32 ) };
	// 82DC19F8: 92810054  stw r20, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[20].u32 ) };
	// 82DC19FC: 92410058  stw r18, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[18].u32 ) };
	// 82DC1A00: 4BF97229  bl 0x82d58c28
	ctx.lr = 0x82DC1A04;
	sub_82D58C28(ctx, base);
	// 82DC1A04: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DC1A08: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82DC1A0C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC1A10: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DC1A14: 40980024  bge cr6, 0x82dc1a38
	if !ctx.cr[6].lt {
	pc = 0x82DC1A38; continue 'dispatch;
	}
	// 82DC1A18: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC1A1C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC1A20: 41980008  blt cr6, 0x82dc1a28
	if ctx.cr[6].lt {
	pc = 0x82DC1A28; continue 'dispatch;
	}
	// 82DC1A24: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DC1A28: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC1A2C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC1A30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC1A34: 4BF954DD  bl 0x82d56f10
	ctx.lr = 0x82DC1A38;
	sub_82D56F10(ctx, base);
	// 82DC1A38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DC1A3C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DC1A40: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82DC1A44: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DC1A48: 4BF972E9  bl 0x82d58d30
	ctx.lr = 0x82DC1A4C;
	sub_82D58D30(ctx, base);
	// 82DC1A4C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DC1A50: 4801E7B9  bl 0x82de0208
	ctx.lr = 0x82DC1A54;
	sub_82DE0208(ctx, base);
	// 82DC1A54: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82DC1A58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DC1A5C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DC1A60: 4BF97931  bl 0x82d59390
	ctx.lr = 0x82DC1A64;
	sub_82D59390(ctx, base);
	// 82DC1A64: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DC1A68: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC1A6C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC1A70: 409A0020  bne cr6, 0x82dc1a90
	if !ctx.cr[6].eq {
	pc = 0x82DC1A90; continue 'dispatch;
	}
	// 82DC1A74: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC1A7C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC1A80: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DC1A84: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC1A88: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC1A8C: 4BF9383D  bl 0x82d552c8
	ctx.lr = 0x82DC1A90;
	sub_82D552C8(ctx, base);
	// 82DC1A90: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82DC1A94: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82DC1A98: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82DC1A9C: 4BF95F4D  bl 0x82d579e8
	ctx.lr = 0x82DC1AA0;
	sub_82D579E8(ctx, base);
	// 82DC1AA0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82DC1AA4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82DC1AA8: 4BF96AA1  bl 0x82d58548
	ctx.lr = 0x82DC1AAC;
	sub_82D58548(ctx, base);
	// 82DC1AAC: 80797630  lwz r3, 0x7630(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82DC1AB0: 39000236  li r8, 0x236
	ctx.r[8].s64 = 566;
	// 82DC1AB4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DC1AB8: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 82DC1ABC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DC1AC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1AC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DC1AC8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC1ACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1AD0: 4E800421  bctrl
	ctx.lr = 0x82DC1AD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1AD4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82DC1AD8: 4BF96959  bl 0x82d58430
	ctx.lr = 0x82DC1ADC;
	sub_82D58430(ctx, base);
	// 82DC1ADC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82DC1AE0: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82DC1AE4: 81731E30  lwz r11, 0x1e30(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82DC1AE8: 7FFB5A14  add r31, r27, r11
	ctx.r[31].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82DC1AEC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DC1AF0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DC1AF4: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 82DC1AF8: 4098009C  bge cr6, 0x82dc1b94
	if !ctx.cr[6].lt {
	pc = 0x82DC1B94; continue 'dispatch;
	}
	// 82DC1AFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DC1B00: 4801E709  bl 0x82de0208
	ctx.lr = 0x82DC1B04;
	sub_82DE0208(ctx, base);
	// 82DC1B04: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1B08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC1B0C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82DC1B10: 4801E6F9  bl 0x82de0208
	ctx.lr = 0x82DC1B14;
	sub_82DE0208(ctx, base);
	// 82DC1B14: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC1B18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC1B1C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82DC1B20: 4801E6E9  bl 0x82de0208
	ctx.lr = 0x82DC1B24;
	sub_82DE0208(ctx, base);
	// 82DC1B24: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DC1B28: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DC1B2C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DC1B30: 7D670774  extsb r7, r11
	ctx.r[7].s64 = ctx.r[11].s8 as i64;
	// 82DC1B34: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC1B38: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82DC1B3C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82DC1B40: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82DC1B44: 4BF96E05  bl 0x82d58948
	ctx.lr = 0x82DC1B48;
	sub_82D58948(ctx, base);
	// 82DC1B48: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82DC1B4C: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82DC1B50: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82DC1B54: 4BF95E95  bl 0x82d579e8
	ctx.lr = 0x82DC1B58;
	sub_82D579E8(ctx, base);
	// 82DC1B58: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 82DC1B5C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82DC1B60: 4BF96291  bl 0x82d57df0
	ctx.lr = 0x82DC1B64;
	sub_82D57DF0(ctx, base);
	// 82DC1B64: 80797630  lwz r3, 0x7630(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82DC1B68: 39000245  li r8, 0x245
	ctx.r[8].s64 = 581;
	// 82DC1B6C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82DC1B70: 38C101D0  addi r6, r1, 0x1d0
	ctx.r[6].s64 = ctx.r[1].s64 + 464;
	// 82DC1B74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DC1B78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1B7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DC1B80: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC1B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1B88: 4E800421  bctrl
	ctx.lr = 0x82DC1B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1B8C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82DC1B90: 4BF968A1  bl 0x82d58430
	ctx.lr = 0x82DC1B94;
	sub_82D58430(ctx, base);
	// 82DC1B94: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DC1B98: 3B7B0003  addi r27, r27, 3
	ctx.r[27].s64 = ctx.r[27].s64 + 3;
	// 82DC1B9C: 2F1C0020  cmpwi cr6, r28, 0x20
	ctx.cr[6].compare_i32(ctx.r[28].s32, 32, &mut ctx.xer);
	// 82DC1BA0: 4198FF44  blt cr6, 0x82dc1ae4
	if ctx.cr[6].lt {
	pc = 0x82DC1AE4; continue 'dispatch;
	}
	// 82DC1BA4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DC1BA8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC1BAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC1BB0: 409A0020  bne cr6, 0x82dc1bd0
	if !ctx.cr[6].eq {
	pc = 0x82DC1BD0; continue 'dispatch;
	}
	// 82DC1BB4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1BB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC1BBC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC1BC0: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DC1BC4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC1BC8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC1BCC: 4BF936FD  bl 0x82d552c8
	ctx.lr = 0x82DC1BD0;
	sub_82D552C8(ctx, base);
	// 82DC1BD0: 3B180060  addi r24, r24, 0x60
	ctx.r[24].s64 = ctx.r[24].s64 + 96;
	// 82DC1BD4: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82DC1BD8: 2F180C03  cmpwi cr6, r24, 0xc03
	ctx.cr[6].compare_i32(ctx.r[24].s32, 3075, &mut ctx.xer);
	// 82DC1BDC: 4198FE14  blt cr6, 0x82dc19f0
	if ctx.cr[6].lt {
	pc = 0x82DC19F0; continue 'dispatch;
	}
	// 82DC1BE0: 80797630  lwz r3, 0x7630(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82DC1BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1BE8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC1BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1BF0: 4E800421  bctrl
	ctx.lr = 0x82DC1BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1BF4: 38210450  addi r1, r1, 0x450
	ctx.r[1].s64 = ctx.r[1].s64 + 1104;
	// 82DC1BF8: 4BEE7838  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC1C00 size=100
    let mut pc: u32 = 0x82DC1C00;
    'dispatch: loop {
        match pc {
            0x82DC1C00 => {
    //   block [0x82DC1C00..0x82DC1C64)
	// 82DC1C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC1C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC1C08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC1C0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC1C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC1C14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC1C18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC1C1C: 4BFFF425  bl 0x82dc1040
	ctx.lr = 0x82DC1C20;
	sub_82DC1040(ctx, base);
	// 82DC1C20: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DC1C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC1C28: 419A0020  beq cr6, 0x82dc1c48
	if ctx.cr[6].eq {
	pc = 0x82DC1C48; continue 'dispatch;
	}
	// 82DC1C2C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1C30: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC1C34: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DC1C38: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC1C3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC1C40: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC1C44: 4BF93685  bl 0x82d552c8
	ctx.lr = 0x82DC1C48;
	sub_82D552C8(ctx, base);
	// 82DC1C48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC1C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC1C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC1C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC1C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC1C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC1C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC1C68 size=184
    let mut pc: u32 = 0x82DC1C68;
    'dispatch: loop {
        match pc {
            0x82DC1C68 => {
    //   block [0x82DC1C68..0x82DC1D20)
	// 82DC1C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC1C6C: 4BEE77A1  bl 0x82ca940c
	ctx.lr = 0x82DC1C70;
	sub_82CA93D0(ctx, base);
	// 82DC1C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC1C74: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82DC1C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC1C7C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DC1C80: 7FCB2214  add r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DC1C84: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1C88: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DC1C8C: 419A0084  beq cr6, 0x82dc1d10
	if ctx.cr[6].eq {
	pc = 0x82DC1D10; continue 'dispatch;
	}
	// 82DC1C90: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC1C94: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DC1C98: 419A0078  beq cr6, 0x82dc1d10
	if ctx.cr[6].eq {
	pc = 0x82DC1D10; continue 'dispatch;
	}
	// 82DC1C9C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC1CA0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC1CA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC1CA8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1CAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC1CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1CB4: 4E800421  bctrl
	ctx.lr = 0x82DC1CB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1CB8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC1CBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC1CC0: 419A0050  beq cr6, 0x82dc1d10
	if ctx.cr[6].eq {
	pc = 0x82DC1D10; continue 'dispatch;
	}
	// 82DC1CC4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC1CC8: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82DC1CCC: 813D000C  lwz r9, 0xc(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC1CD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DC1CD4: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DC1CD8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC1CDC: 80FF0010  lwz r7, 0x10(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC1CE0: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC1CE4: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC1CE8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC1CEC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DC1CF0: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC1CF4: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DC1CF8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DC1CFC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC1D00: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DC1D04: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82DC1D08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC1D0C: 4E800421  bctrl
	ctx.lr = 0x82DC1D10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC1D10: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC1D14: C02B0004  lfs f1, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DC1D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC1D1C: 4BEE7740  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC1D20 size=476
    let mut pc: u32 = 0x82DC1D20;
    'dispatch: loop {
        match pc {
            0x82DC1D20 => {
    //   block [0x82DC1D20..0x82DC1EFC)
	// 82DC1D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC1D24: 4BEE76C9  bl 0x82ca93ec
	ctx.lr = 0x82DC1D28;
	sub_82CA93D0(ctx, base);
	// 82DC1D28: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC1F00 size=4
    let mut pc: u32 = 0x82DC1F00;
    'dispatch: loop {
        match pc {
            0x82DC1F00 => {
    //   block [0x82DC1F00..0x82DC1F04)
	// 82DC1F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC1F08 size=4
    let mut pc: u32 = 0x82DC1F08;
    'dispatch: loop {
        match pc {
            0x82DC1F08 => {
    //   block [0x82DC1F08..0x82DC1F0C)
	// 82DC1F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC1F10 size=4
    let mut pc: u32 = 0x82DC1F10;
    'dispatch: loop {
        match pc {
            0x82DC1F10 => {
    //   block [0x82DC1F10..0x82DC1F14)
	// 82DC1F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC1F18 size=12
    let mut pc: u32 = 0x82DC1F18;
    'dispatch: loop {
        match pc {
            0x82DC1F18 => {
    //   block [0x82DC1F18..0x82DC1F24)
	// 82DC1F18: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DC1F1C: 386BB440  addi r3, r11, -0x4bc0
	ctx.r[3].s64 = ctx.r[11].s64 + -19392;
	// 82DC1F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC1F28 size=12
    let mut pc: u32 = 0x82DC1F28;
    'dispatch: loop {
        match pc {
            0x82DC1F28 => {
    //   block [0x82DC1F28..0x82DC1F34)
	// 82DC1F28: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DC1F2C: 386BB440  addi r3, r11, -0x4bc0
	ctx.r[3].s64 = ctx.r[11].s64 + -19392;
	// 82DC1F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC1F38 size=32
    let mut pc: u32 = 0x82DC1F38;
    'dispatch: loop {
        match pc {
            0x82DC1F38 => {
    //   block [0x82DC1F38..0x82DC1F58)
	// 82DC1F38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC1F3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DC1F40: 396B3004  addi r11, r11, 0x3004
	ctx.r[11].s64 = ctx.r[11].s64 + 12292;
	// 82DC1F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DC1F48: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DC1F4C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC1F50: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DC1F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC1F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC1F58 size=368
    let mut pc: u32 = 0x82DC1F58;
    'dispatch: loop {
        match pc {
            0x82DC1F58 => {
    //   block [0x82DC1F58..0x82DC20C8)
	// 82DC1F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC1F5C: 4BEE74B1  bl 0x82ca940c
	ctx.lr = 0x82DC1F60;
	sub_82CA93D0(ctx, base);
	// 82DC1F60: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82DC1F64: 4BEEBD61  bl 0x82cadcc4
	ctx.lr = 0x82DC1F68;
	sub_82CADCA0(ctx, base);
	// 82DC1F68: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC1F6C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82DC1F70: FF400890  fmr f26, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[26].f64 = ctx.f[1].f64;
	// 82DC1F74: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DC1F78: FF601090  fmr f27, f2
	ctx.f[27].f64 = ctx.f[2].f64;
	// 82DC1F7C: 3BABA950  addi r29, r11, -0x56b0
	ctx.r[29].s64 = ctx.r[11].s64 + -22192;
	// 82DC1F80: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DC1F84: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DC1F88: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC1F8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC1F90: C2E70EE0  lfs f23, 0xee0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3808 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82DC1F94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC1F98: C3280C14  lfs f25, 0xc14(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3092 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82DC1F9C: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 82DC1FA0: C3890C18  lfs f28, 0xc18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82DC1FA4: C3CA0B40  lfs f30, 0xb40(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2880 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82DC1FA8: C30B303C  lfs f24, 0x303c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12348 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 82DC1FAC: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 82DC1FB0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DC1FB4: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DC1FB8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DC1FBC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DC1FC0: EFE0F638  fmsubs f31, f0, f24, f30
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[24].f64 - ctx.f[30].f64) as f32) as f64);
	// 82DC1FC4: FF1FE000  fcmpu cr6, f31, f28
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[28].f64);
	// 82DC1FC8: 40980070  bge cr6, 0x82dc2038
	if !ctx.cr[6].lt {
	pc = 0x82DC2038; continue 'dispatch;
	}
	// 82DC1FCC: FC00FA10  fabs f0, f31
	ctx.f[0].u64 = ctx.f[31].u64 & !0x8000_0000_0000_0000u64;
	// 82DC1FD0: D33FFFFC  stfs f25, -4(r31)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DC1FD4: D39F0000  stfs f28, 0(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DC1FD8: EFBE0028  fsubs f29, f30, f0
	ctx.f[29].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DC1FDC: EFFDD028  fsubs f31, f29, f26
	ctx.f[31].f64 = (((ctx.f[29].f64 - ctx.f[26].f64) as f32) as f64);
	// 82DC1FE0: FF1FD800  fcmpu cr6, f31, f27
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[27].f64);
	// 82DC1FE4: 4099000C  ble cr6, 0x82dc1ff0
	if !ctx.cr[6].gt {
	pc = 0x82DC1FF0; continue 'dispatch;
	}
	// 82DC1FE8: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 82DC1FEC: 48000008  b 0x82dc1ff4
	pc = 0x82DC1FF4; continue 'dispatch;
	// 82DC1FF0: FC3FE7EE  fsel f1, f31, f31, f28
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[28].f64 };
	// 82DC1FF4: 4B477EBD  bl 0x82239eb0
	ctx.lr = 0x82DC1FF8;
	sub_82239EB0(ctx, base);
	// 82DC1FF8: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DC1FFC: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DC2000: FC3FE7EE  fsel f1, f31, f31, f28
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[28].f64 };
	// 82DC2004: 4B477EAD  bl 0x82239eb0
	ctx.lr = 0x82DC2008;
	sub_82239EB0(ctx, base);
	// 82DC2008: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DC200C: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DC2010: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82DC2014: 4B477E9D  bl 0x82239eb0
	ctx.lr = 0x82DC2018;
	sub_82239EB0(ctx, base);
	// 82DC2018: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82DC201C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82DC2020: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DC2024: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DC2028: 4B477F69  bl 0x82239f90
	ctx.lr = 0x82DC202C;
	sub_82239F90(ctx, base);
	// 82DC202C: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DC2030: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DC2034: 48000070  b 0x82dc20a4
	pc = 0x82DC20A4; continue 'dispatch;
	// 82DC2038: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DC203C: 4B477E75  bl 0x82239eb0
	ctx.lr = 0x82DC2040;
	sub_82239EB0(ctx, base);
	// 82DC2040: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82DC2044: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DC2048: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DC204C: D01FFFFC  stfs f0, -4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DC2050: 4B477F41  bl 0x82239f90
	ctx.lr = 0x82DC2054;
	sub_82239F90(ctx, base);
	// 82DC2054: EFFFD02A  fadds f31, f31, f26
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[26].f64) as f32) as f64;
	// 82DC2058: FC000818  frsp f0, f1
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DC205C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DC2060: EC1FF028  fsubs f0, f31, f30
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 82DC2064: FC20FFAE  fsel f1, f0, f30, f31
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[31].f64 };
	// 82DC2068: 4B477E49  bl 0x82239eb0
	ctx.lr = 0x82DC206C;
	sub_82239EB0(ctx, base);
	// 82DC206C: EC1ED828  fsubs f0, f30, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[27].f64) as f32) as f64);
	// 82DC2070: FDA00818  frsp f13, f1
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DC2074: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DC2078: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DC207C: 4098000C  bge cr6, 0x82dc2088
	if !ctx.cr[6].lt {
	pc = 0x82DC2088; continue 'dispatch;
	}
	// 82DC2080: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82DC2084: 4800000C  b 0x82dc2090
	pc = 0x82DC2090; continue 'dispatch;
	// 82DC2088: EC1FF028  fsubs f0, f31, f30
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 82DC208C: FC20FFAE  fsel f1, f0, f30, f31
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[31].f64 };
	// 82DC2090: 4B477E21  bl 0x82239eb0
	ctx.lr = 0x82DC2094;
	sub_82239EB0(ctx, base);
	// 82DC2094: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82DC2098: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DC209C: D2FF000C  stfs f23, 0xc(r31)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DC20A0: D39F0010  stfs f28, 0x10(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DC20A4: 3BFF0018  addi r31, r31, 0x18
	ctx.r[31].s64 = ctx.r[31].s64 + 24;
	// 82DC20A8: 397D02EC  addi r11, r29, 0x2ec
	ctx.r[11].s64 = ctx.r[29].s64 + 748;
	// 82DC20AC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82DC20B0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC20B4: 4198FEF8  blt cr6, 0x82dc1fac
	if ctx.cr[6].lt {
	pc = 0x82DC1FAC; continue 'dispatch;
	}
	// 82DC20B8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DC20BC: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82DC20C0: 4BEEBC51  bl 0x82cadd10
	ctx.lr = 0x82DC20C4;
	sub_82CADCEC(ctx, base);
	// 82DC20C4: 4BEE7398  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC20C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DC20C8 size=448
    let mut pc: u32 = 0x82DC20C8;
    'dispatch: loop {
        match pc {
            0x82DC20C8 => {
    //   block [0x82DC20C8..0x82DC2288)
	// 82DC20C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC20CC: 4BEE7335  bl 0x82ca9400
	ctx.lr = 0x82DC20D0;
	sub_82CA93D0(ctx, base);
	// 82DC20D0: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC20D4: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82DC20D8: D001FFB0  stfs f0, -0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), tmp.u32 ) };
	// 82DC20DC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82DC20E0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC20E4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82DC20E8: D001FFB4  stfs f0, -0x4c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-76 as u32), tmp.u32 ) };
	// 82DC20EC: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC20F0: D001FFB8  stfs f0, -0x48(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), tmp.u32 ) };
	// 82DC20F4: 3961FFB0  addi r11, r1, -0x50
	ctx.r[11].s64 = ctx.r[1].s64 + -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC2288 size=1084
    let mut pc: u32 = 0x82DC2288;
    'dispatch: loop {
        match pc {
            0x82DC2288 => {
    //   block [0x82DC2288..0x82DC26C4)
	// 82DC2288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC228C: 4BEE7165  bl 0x82ca93f0
	ctx.lr = 0x82DC2290;
	sub_82CA93D0(ctx, base);
	// 82DC2290: 3981FFA8  addi r12, r1, -0x58
	ctx.r[12].s64 = ctx.r[1].s64 + -88;
	// 82DC2294: 4BEEBA39  bl 0x82cadccc
	ctx.lr = 0x82DC2298;
	sub_82CADCA0(ctx, base);
	// 82DC2298: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC26C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC26C8 size=160
    let mut pc: u32 = 0x82DC26C8;
    'dispatch: loop {
        match pc {
            0x82DC26C8 => {
    //   block [0x82DC26C8..0x82DC2768)
	// 82DC26C8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DC26CC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82DC26D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC26D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC26D8: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC26DC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC26E0: 419A0024  beq cr6, 0x82dc2704
	if ctx.cr[6].eq {
	pc = 0x82DC2704; continue 'dispatch;
	}
	// 82DC26E4: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 82DC26E8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DC26EC: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82DC26F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC26F4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DC26F8: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC26FC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC2700: 409AFFE8  bne cr6, 0x82dc26e8
	if !ctx.cr[6].eq {
	pc = 0x82DC26E8; continue 'dispatch;
	}
	// 82DC2704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DC2708: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC270C: 40990044  ble cr6, 0x82dc2750
	if !ctx.cr[6].gt {
	pc = 0x82DC2750; continue 'dispatch;
	}
	// 82DC2710: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC2714: 38E1FFD0  addi r7, r1, -0x30
	ctx.r[7].s64 = ctx.r[1].s64 + -48;
	// 82DC2718: 38C4FFFF  addi r6, r4, -1
	ctx.r[6].s64 = ctx.r[4].s64 + -1;
	// 82DC271C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82DC2720: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DC2724: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DC2728: 40980028  bge cr6, 0x82dc2750
	if !ctx.cr[6].lt {
	pc = 0x82DC2750; continue 'dispatch;
	}
	// 82DC272C: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82DC2730: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC2734: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82DC2738: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC273C: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2740: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC2744: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DC2748: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DC274C: 4199FFD8  bgt cr6, 0x82dc2724
	if ctx.cr[6].gt {
	pc = 0x82DC2724; continue 'dispatch;
	}
	// 82DC2750: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC2754: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DC2758: 38680001  addi r3, r8, 1
	ctx.r[3].s64 = ctx.r[8].s64 + 1;
	// 82DC275C: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82DC2760: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82DC2764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC2768 size=8
    let mut pc: u32 = 0x82DC2768;
    'dispatch: loop {
        match pc {
            0x82DC2768 => {
    //   block [0x82DC2768..0x82DC2770)
	// 82DC2768: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DC276C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC2770 size=124
    let mut pc: u32 = 0x82DC2770;
    'dispatch: loop {
        match pc {
            0x82DC2770 => {
    //   block [0x82DC2770..0x82DC27EC)
	// 82DC2770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC2774: 4BEE6C8D  bl 0x82ca9400
	ctx.lr = 0x82DC2778;
	sub_82CA93D0(ctx, base);
	// 82DC2778: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC277C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC2780: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DC2784: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DC2788: 815D0028  lwz r10, 0x28(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC278C: 98BD0040  stb r5, 0x40(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[5].u8 ) };
	// 82DC2790: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC2794: 40990048  ble cr6, 0x82dc27dc
	if !ctx.cr[6].gt {
	pc = 0x82DC27DC; continue 'dispatch;
	}
	// 82DC2798: 3965FFFB  addi r11, r5, -5
	ctx.r[11].s64 = ctx.r[5].s64 + -5;
	// 82DC279C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC27A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DC27A4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DC27A8: 697B0001  xori r27, r11, 1
	ctx.r[27].u64 = ctx.r[11].u64 ^ 1;
	// 82DC27AC: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC27B0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DC27B4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DC27B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC27BC: 7F9E5A14  add r28, r30, r11
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DC27C0: 480202B1  bl 0x82de2a70
	ctx.lr = 0x82DC27C4;
	sub_82DE2A70(ctx, base);
	// 82DC27C4: B07C000C  sth r3, 0xc(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[3].u16 ) };
	// 82DC27C8: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC27CC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DC27D0: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82DC27D4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC27D8: 4198FFD4  blt cr6, 0x82dc27ac
	if ctx.cr[6].lt {
	pc = 0x82DC27AC; continue 'dispatch;
	}
	// 82DC27DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC27E0: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82DC27E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC27E8: 4BEE6C68  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC27F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC27F0 size=268
    let mut pc: u32 = 0x82DC27F0;
    'dispatch: loop {
        match pc {
            0x82DC27F0 => {
    //   block [0x82DC27F0..0x82DC28FC)
	// 82DC27F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC27F4: 4BEE6C0D  bl 0x82ca9400
	ctx.lr = 0x82DC27F8;
	sub_82CA93D0(ctx, base);
	// 82DC27F8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82DC27FC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC2800: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC2804: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DC2808: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DC280C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DC2810: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82DC2814: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DC2818: C00B0C64  lfs f0, 0xc64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC281C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC2820: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DC2824: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC2828: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DC282C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DC2830: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DC2834: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC2838: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DC283C: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DC2840: C00B0BE4  lfs f0, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC2844: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DC2848: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DC284C: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DC2850: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC2900 size=156
    let mut pc: u32 = 0x82DC2900;
    'dispatch: loop {
        match pc {
            0x82DC2900 => {
    //   block [0x82DC2900..0x82DC299C)
	// 82DC2900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC2904: 4BEE6B05  bl 0x82ca9408
	ctx.lr = 0x82DC2908;
	sub_82CA93D0(ctx, base);
	// 82DC2908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC290C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC2910: 3B840001  addi r28, r4, 1
	ctx.r[28].s64 = ctx.r[4].s64 + 1;
	// 82DC2914: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC2918: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC291C: 40980068  bge cr6, 0x82dc2984
	if !ctx.cr[6].lt {
	pc = 0x82DC2984; continue 'dispatch;
	}
	// 82DC2920: 579E2036  slwi r30, r28, 4
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DC2924: 3FA08330  lis r29, -0x7cd0
	ctx.r[29].s64 = -2094006272;
	// 82DC2928: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC292C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC2930: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC2934: C03DB384  lfs f1, -0x4c7c(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DC2938: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DC293C: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC2940: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC2944: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2948: 55282036  slwi r8, r9, 4
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC294C: 54E92036  slwi r9, r7, 4
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DC2950: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC2954: 7CC85A14  add r6, r8, r11
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DC2958: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC295C: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DC2960: 4BFF8459  bl 0x82dbadb8
	ctx.lr = 0x82DC2964;
	sub_82DBADB8(ctx, base);
	// 82DC2964: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC296C: 419A0024  beq cr6, 0x82dc2990
	if ctx.cr[6].eq {
	pc = 0x82DC2990; continue 'dispatch;
	}
	// 82DC2970: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC2974: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DC2978: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82DC297C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC2980: 4198FFA8  blt cr6, 0x82dc2928
	if ctx.cr[6].lt {
	pc = 0x82DC2928; continue 'dispatch;
	}
	// 82DC2984: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DC2988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC298C: 4BEE6ACC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82DC2990: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DC2994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC2998: 4BEE6AC0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC29A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC29A0 size=144
    let mut pc: u32 = 0x82DC29A0;
    'dispatch: loop {
        match pc {
            0x82DC29A0 => {
    //   block [0x82DC29A0..0x82DC2A30)
	// 82DC29A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC29A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC29A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC29AC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82DC29B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC29B4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DC29B8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DC29BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC29C0: 4BFF65D9  bl 0x82db8f98
	ctx.lr = 0x82DC29C4;
	sub_82DB8F98(ctx, base);
	// 82DC29C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC29C8: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82DC29CC: 392B236C  addi r9, r11, 0x236c
	ctx.r[9].s64 = ctx.r[11].s64 + 9068;
	// 82DC29D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC29D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC29D8: 390B1EF4  addi r8, r11, 0x1ef4
	ctx.r[8].s64 = ctx.r[11].s64 + 7924;
	// 82DC29DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC29E0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC29E4: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DC29E8: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DC29EC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DC29F0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DC29F4: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DC29F8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82DC29FC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DC2A00: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82DC2A04: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DC2A08: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DC2A0C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82DC2A10: D3FF003C  stfs f31, 0x3c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82DC2A14: 993F0040  stb r9, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u8 ) };
	// 82DC2A18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC2A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC2A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC2A24: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC2A28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC2A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DC2A30 size=224
    let mut pc: u32 = 0x82DC2A30;
    'dispatch: loop {
        match pc {
            0x82DC2A30 => {
    //   block [0x82DC2A30..0x82DC2B10)
	// 82DC2A30: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DC2A34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DC2A38: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DC2A3C: 419A0068  beq cr6, 0x82dc2aa4
	if ctx.cr[6].eq {
	pc = 0x82DC2AA4; continue 'dispatch;
	}
	// 82DC2A40: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC2A44: 54882036  slwi r8, r4, 4
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC2A48: 892B0030  lbz r9, 0x30(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC2A4C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DC2A50: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82DC2A54: C00B002C  lfs f0, 0x2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC2A58: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DC2A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DC2A60: 39084CA4  addi r8, r8, 0x4ca4
	ctx.r[8].s64 = ctx.r[8].s64 + 19620;
	// 82DC2A64: 3BE00005  li r31, 5
	ctx.r[31].s64 = 5;
	// 82DC2A68: A0E7000C  lhz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC2A6C: 39450050  addi r10, r5, 0x50
	ctx.r[10].s64 = ctx.r[5].s64 + 80;
	// 82DC2A70: B0650006  sth r3, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[3].u16 ) };
	// 82DC2A74: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DC2A78: 90C50008  stw r6, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DC2A7C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DC2A80: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DC2A84: 93E5000C  stw r31, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82DC2A88: 99250016  stb r9, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[9].u8 ) };
	// 82DC2A8C: B0E50014  sth r7, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC2B10 size=296
    let mut pc: u32 = 0x82DC2B10;
    'dispatch: loop {
        match pc {
            0x82DC2B10 => {
    //   block [0x82DC2B10..0x82DC2C38)
	// 82DC2B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC2B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC2B18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC2B1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC2B20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC2B24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC2B28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC2B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC2B30: 388B306C  addi r4, r11, 0x306c
	ctx.r[4].s64 = ctx.r[11].s64 + 12396;
	// 82DC2B34: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DC2B38: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2B3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC2B40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC2B44: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC2B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2B4C: 4E800421  bctrl
	ctx.lr = 0x82DC2B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2B50: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC2B54: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC2B58: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC2B5C: 409A0034  bne cr6, 0x82dc2b90
	if !ctx.cr[6].eq {
	pc = 0x82DC2B90; continue 'dispatch;
	}
	// 82DC2B60: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2B64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC2B68: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC2B6C: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC2B70: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC2B74: 388A3060  addi r4, r10, 0x3060
	ctx.r[4].s64 = ctx.r[10].s64 + 12384;
	// 82DC2B78: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DC2B7C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC2B80: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC2B84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC2B88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2B8C: 4E800421  bctrl
	ctx.lr = 0x82DC2B90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2B90: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC2B94: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC2B98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2B9C: 409A0034  bne cr6, 0x82dc2bd0
	if !ctx.cr[6].eq {
	pc = 0x82DC2BD0; continue 'dispatch;
	}
	// 82DC2BA0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2BA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC2BA8: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC2BAC: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DC2BB0: 388B3054  addi r4, r11, 0x3054
	ctx.r[4].s64 = ctx.r[11].s64 + 12372;
	// 82DC2BB4: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC2BB8: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DC2BBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC2BC0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC2BC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC2BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2BCC: 4E800421  bctrl
	ctx.lr = 0x82DC2BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2BD0: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC2BD4: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC2BD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2BDC: 409A0030  bne cr6, 0x82dc2c0c
	if !ctx.cr[6].eq {
	pc = 0x82DC2C0C; continue 'dispatch;
	}
	// 82DC2BE0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2BE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC2BE8: 80FF0034  lwz r7, 0x34(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC2BEC: 554800BE  clrlwi r8, r10, 2
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC2BF0: 388B3048  addi r4, r11, 0x3048
	ctx.r[4].s64 = ctx.r[11].s64 + 12360;
	// 82DC2BF4: 80DF0030  lwz r6, 0x30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC2BF8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC2BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC2C00: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC2C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2C08: 4E800421  bctrl
	ctx.lr = 0x82DC2C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2C0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC2C14: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC2C18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2C1C: 4E800421  bctrl
	ctx.lr = 0x82DC2C20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2C20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC2C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC2C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC2C2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC2C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC2C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC2C38 size=24
    let mut pc: u32 = 0x82DC2C38;
    'dispatch: loop {
        match pc {
            0x82DC2C38 => {
    //   block [0x82DC2C38..0x82DC2C50)
	// 82DC2C38: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DC2C3C: 419A0020  beq cr6, 0x82dc2c5c
	if ctx.cr[6].eq {
		sub_82DC2C5C(ctx, base);
		return;
	}
	// 82DC2C40: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82DC2C44: 419A000C  beq cr6, 0x82dc2c50
	if ctx.cr[6].eq {
		sub_82DC2C50(ctx, base);
		return;
	}
	// 82DC2C48: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DC2C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC2C50 size=12
    let mut pc: u32 = 0x82DC2C50;
    'dispatch: loop {
        match pc {
            0x82DC2C50 => {
    //   block [0x82DC2C50..0x82DC2C5C)
	// 82DC2C50: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC2C54: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82DC2C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC2C5C size=8
    let mut pc: u32 = 0x82DC2C5C;
    'dispatch: loop {
        match pc {
            0x82DC2C5C => {
    //   block [0x82DC2C5C..0x82DC2C64)
	// 82DC2C5C: 38640002  addi r3, r4, 2
	ctx.r[3].s64 = ctx.r[4].s64 + 2;
	// 82DC2C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC2C68 size=24
    let mut pc: u32 = 0x82DC2C68;
    'dispatch: loop {
        match pc {
            0x82DC2C68 => {
    //   block [0x82DC2C68..0x82DC2C80)
	// 82DC2C68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC2C6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DC2C70: 396B307C  addi r11, r11, 0x307c
	ctx.r[11].s64 = ctx.r[11].s64 + 12412;
	// 82DC2C74: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DC2C78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC2C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC2C80 size=340
    let mut pc: u32 = 0x82DC2C80;
    'dispatch: loop {
        match pc {
            0x82DC2C80 => {
    //   block [0x82DC2C80..0x82DC2DD4)
	// 82DC2C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC2C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC2C88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC2C8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC2C90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC2C94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC2C98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC2C9C: 4BFF223D  bl 0x82db4ed8
	ctx.lr = 0x82DC2CA0;
	sub_82DB4ED8(ctx, base);
	// 82DC2CA0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC2CA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC2CA8: 396B2788  addi r11, r11, 0x2788
	ctx.r[11].s64 = ctx.r[11].s64 + 10120;
	// 82DC2CAC: 394A2324  addi r10, r10, 0x2324
	ctx.r[10].s64 = ctx.r[10].s64 + 8996;
	// 82DC2CB0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DC2CB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC2CB8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC2CBC: 419A00FC  beq cr6, 0x82dc2db8
	if ctx.cr[6].eq {
	pc = 0x82DC2DB8; continue 'dispatch;
	}
	// 82DC2CC0: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DC2CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DC2CC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2CCC: 40990088  ble cr6, 0x82dc2d54
	if !ctx.cr[6].gt {
	pc = 0x82DC2D54; continue 'dispatch;
	}
	// 82DC2CD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DC2CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DC2CD8: 815F0118  lwz r10, 0x118(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 82DC2CDC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DC2CE0: 80DF0100  lwz r6, 0x100(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82DC2CE4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC2CE8: 7D48522E  lhzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC2CEC: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DC2CF0: 7D4A302E  lwzx r10, r10, r6
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82DC2CF4: 88CB0030  lbz r6, 0x30(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC2CF8: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DC2CFC: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC2D00: 90CB001C  stw r6, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82DC2D04: 409A000C  bne cr6, 0x82dc2d10
	if !ctx.cr[6].eq {
	pc = 0x82DC2D10; continue 'dispatch;
	}
	// 82DC2D08: 80CA0014  lwz r6, 0x14(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC2D0C: 48000008  b 0x82dc2d14
	pc = 0x82DC2D14; continue 'dispatch;
	// 82DC2D10: 80CA0020  lwz r6, 0x20(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC2D14: 90CB0028  stw r6, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82DC2D18: 88CB0001  lbz r6, 1(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC2D1C: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DC2D20: 409A000C  bne cr6, 0x82dc2d2c
	if !ctx.cr[6].eq {
	pc = 0x82DC2D2C; continue 'dispatch;
	}
	// 82DC2D24: 80CA002C  lwz r6, 0x2c(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC2D28: 48000008  b 0x82dc2d30
	pc = 0x82DC2D30; continue 'dispatch;
	// 82DC2D2C: 80CA0044  lwz r6, 0x44(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DC2D30: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82DC2D34: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DC2D38: 814A0038  lwz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC2D3C: 39290080  addi r9, r9, 0x80
	ctx.r[9].s64 = ctx.r[9].s64 + 128;
	// 82DC2D40: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82DC2D44: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DC2D48: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DC2D4C: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC2D50: 4198FF88  blt cr6, 0x82dc2cd8
	if ctx.cr[6].lt {
	pc = 0x82DC2CD8; continue 'dispatch;
	}
	// 82DC2D54: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DC2D58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DC2D5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2D60: 40990058  ble cr6, 0x82dc2db8
	if !ctx.cr[6].gt {
	pc = 0x82DC2DB8; continue 'dispatch;
	}
	// 82DC2D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DC2D68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DC2D6C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DC2D70: 815F010C  lwz r10, 0x10c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 82DC2D74: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC2D78: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC2D7C: 88CB0001  lbz r6, 1(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC2D80: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DC2D84: 409A000C  bne cr6, 0x82dc2d90
	if !ctx.cr[6].eq {
	pc = 0x82DC2D90; continue 'dispatch;
	}
	// 82DC2D88: 80CA0014  lwz r6, 0x14(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC2D8C: 48000008  b 0x82dc2d94
	pc = 0x82DC2D94; continue 'dispatch;
	// 82DC2D90: 80CA002C  lwz r6, 0x2c(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC2D94: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82DC2D98: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DC2D9C: 814A0020  lwz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC2DA0: 39290090  addi r9, r9, 0x90
	ctx.r[9].s64 = ctx.r[9].s64 + 144;
	// 82DC2DA4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82DC2DA8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DC2DAC: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DC2DB0: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC2DB4: 4198FFB8  blt cr6, 0x82dc2d6c
	if ctx.cr[6].lt {
	pc = 0x82DC2D6C; continue 'dispatch;
	}
	// 82DC2DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC2DBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC2DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC2DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC2DC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC2DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC2DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC2DD8 size=116
    let mut pc: u32 = 0x82DC2DD8;
    'dispatch: loop {
        match pc {
            0x82DC2DD8 => {
    //   block [0x82DC2DD8..0x82DC2E4C)
	// 82DC2DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC2DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC2DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC2DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC2DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC2DEC: 4BFF1B45  bl 0x82db4930
	ctx.lr = 0x82DC2DF0;
	sub_82DB4930(ctx, base);
	// 82DC2DF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC2DF4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82DC2DF8: 392B2788  addi r9, r11, 0x2788
	ctx.r[9].s64 = ctx.r[11].s64 + 10120;
	// 82DC2DFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC2E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC2E04: 390B2324  addi r8, r11, 0x2324
	ctx.r[8].s64 = ctx.r[11].s64 + 8996;
	// 82DC2E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC2E0C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC2E10: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82DC2E14: 917F0100  stw r11, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 82DC2E18: 917F0104  stw r11, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 82DC2E1C: 915F0108  stw r10, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[10].u32 ) };
	// 82DC2E20: 917F010C  stw r11, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[11].u32 ) };
	// 82DC2E24: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 82DC2E28: 915F0114  stw r10, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[10].u32 ) };
	// 82DC2E2C: 917F0118  stw r11, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[11].u32 ) };
	// 82DC2E30: 917F011C  stw r11, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u32 ) };
	// 82DC2E34: 915F0120  stw r10, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[10].u32 ) };
	// 82DC2E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC2E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC2E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC2E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC2E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC2E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC2E50 size=556
    let mut pc: u32 = 0x82DC2E50;
    'dispatch: loop {
        match pc {
            0x82DC2E50 => {
    //   block [0x82DC2E50..0x82DC307C)
	// 82DC2E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC2E54: 4BEE65AD  bl 0x82ca9400
	ctx.lr = 0x82DC2E58;
	sub_82CA93D0(ctx, base);
	// 82DC2E58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC2E5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC2E60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DC2E64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DC2E68: 396B2788  addi r11, r11, 0x2788
	ctx.r[11].s64 = ctx.r[11].s64 + 10120;
	// 82DC2E6C: 394A2324  addi r10, r10, 0x2324
	ctx.r[10].s64 = ctx.r[10].s64 + 8996;
	// 82DC2E70: 813E0104  lwz r9, 0x104(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DC2E74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC2E78: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DC2E7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC2E80: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC2E84: 4099005C  ble cr6, 0x82dc2ee0
	if !ctx.cr[6].gt {
	pc = 0x82DC2EE0; continue 'dispatch;
	}
	// 82DC2E88: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DC2E8C: 817E0100  lwz r11, 0x100(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) } as u64;
	// 82DC2E90: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC2E94: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC2E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC2E9C: 419A0030  beq cr6, 0x82dc2ecc
	if ctx.cr[6].eq {
	pc = 0x82DC2ECC; continue 'dispatch;
	}
	// 82DC2EA0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC2EA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC2EA8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC2EAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2EB0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC2EB4: 409A0018  bne cr6, 0x82dc2ecc
	if !ctx.cr[6].eq {
	pc = 0x82DC2ECC; continue 'dispatch;
	}
	// 82DC2EB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2EBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC2EC0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2EC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2EC8: 4E800421  bctrl
	ctx.lr = 0x82DC2ECC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2ECC: 817E0104  lwz r11, 0x104(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DC2ED0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DC2ED4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DC2ED8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC2EDC: 4198FFB0  blt cr6, 0x82dc2e8c
	if ctx.cr[6].lt {
	pc = 0x82DC2E8C; continue 'dispatch;
	}
	// 82DC2EE0: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DC2EE4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DC2EE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2EEC: 4099008C  ble cr6, 0x82dc2f78
	if !ctx.cr[6].gt {
	pc = 0x82DC2F78; continue 'dispatch;
	}
	// 82DC2EF0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DC2EF4: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DC2EF8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DC2EFC: 7FFB5A14  add r31, r27, r11
	ctx.r[31].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82DC2F00: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC2F04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2F08: 4099005C  ble cr6, 0x82dc2f64
	if !ctx.cr[6].gt {
	pc = 0x82DC2F64; continue 'dispatch;
	}
	// 82DC2F0C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC2F10: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC2F14: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC2F18: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC2F1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC2F20: 419A0030  beq cr6, 0x82dc2f50
	if ctx.cr[6].eq {
	pc = 0x82DC2F50; continue 'dispatch;
	}
	// 82DC2F24: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC2F28: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC2F2C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC2F30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2F34: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC2F38: 409A0018  bne cr6, 0x82dc2f50
	if !ctx.cr[6].eq {
	pc = 0x82DC2F50; continue 'dispatch;
	}
	// 82DC2F3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2F40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC2F44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2F48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2F4C: 4E800421  bctrl
	ctx.lr = 0x82DC2F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2F50: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC2F54: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DC2F58: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DC2F5C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC2F60: 4198FFB0  blt cr6, 0x82dc2f10
	if ctx.cr[6].lt {
	pc = 0x82DC2F10; continue 'dispatch;
	}
	// 82DC2F64: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DC2F68: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DC2F6C: 3B7B0090  addi r27, r27, 0x90
	ctx.r[27].s64 = ctx.r[27].s64 + 144;
	// 82DC2F70: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC2F74: 4198FF80  blt cr6, 0x82dc2ef4
	if ctx.cr[6].lt {
	pc = 0x82DC2EF4; continue 'dispatch;
	}
	// 82DC2F78: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82DC2F7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC2F80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2F84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC2F88: 917E005C  stw r11, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DC2F8C: 4099005C  ble cr6, 0x82dc2fe8
	if !ctx.cr[6].gt {
	pc = 0x82DC2FE8; continue 'dispatch;
	}
	// 82DC2F90: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DC2F94: 817E010C  lwz r11, 0x10c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(268 as u32) ) } as u64;
	// 82DC2F98: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC2F9C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC2FA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC2FA4: 419A0030  beq cr6, 0x82dc2fd4
	if ctx.cr[6].eq {
	pc = 0x82DC2FD4; continue 'dispatch;
	}
	// 82DC2FA8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC2FAC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC2FB0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC2FB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC2FB8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC2FBC: 409A0018  bne cr6, 0x82dc2fd4
	if !ctx.cr[6].eq {
	pc = 0x82DC2FD4; continue 'dispatch;
	}
	// 82DC2FC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2FC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC2FC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2FCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC2FD0: 4E800421  bctrl
	ctx.lr = 0x82DC2FD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC2FD4: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82DC2FD8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DC2FDC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DC2FE0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC2FE4: 4198FFB0  blt cr6, 0x82dc2f94
	if ctx.cr[6].lt {
	pc = 0x82DC2F94; continue 'dispatch;
	}
	// 82DC2FE8: 817E0120  lwz r11, 0x120(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(288 as u32) ) } as u64;
	// 82DC2FEC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC2FF0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC2FF4: 409A0020  bne cr6, 0x82dc3014
	if !ctx.cr[6].eq {
	pc = 0x82DC3014; continue 'dispatch;
	}
	// 82DC2FF8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC2FFC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC3000: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC3004: 809E0118  lwz r4, 0x118(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(280 as u32) ) } as u64;
	// 82DC3008: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DC300C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC3010: 4BF922B9  bl 0x82d552c8
	ctx.lr = 0x82DC3014;
	sub_82D552C8(ctx, base);
	// 82DC3014: 817E0114  lwz r11, 0x114(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(276 as u32) ) } as u64;
	// 82DC3018: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC301C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC3020: 409A0020  bne cr6, 0x82dc3040
	if !ctx.cr[6].eq {
	pc = 0x82DC3040; continue 'dispatch;
	}
	// 82DC3024: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3028: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC302C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC3030: 809E010C  lwz r4, 0x10c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(268 as u32) ) } as u64;
	// 82DC3034: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC3038: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC303C: 4BF9228D  bl 0x82d552c8
	ctx.lr = 0x82DC3040;
	sub_82D552C8(ctx, base);
	// 82DC3040: 817E0108  lwz r11, 0x108(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) } as u64;
	// 82DC3044: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC3048: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC304C: 409A0020  bne cr6, 0x82dc306c
	if !ctx.cr[6].eq {
	pc = 0x82DC306C; continue 'dispatch;
	}
	// 82DC3050: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3054: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC3058: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC305C: 809E0100  lwz r4, 0x100(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) } as u64;
	// 82DC3060: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC3064: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC3068: 4BF92261  bl 0x82d552c8
	ctx.lr = 0x82DC306C;
	sub_82D552C8(ctx, base);
	// 82DC306C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC3070: 4BFF1DE1  bl 0x82db4e50
	ctx.lr = 0x82DC3074;
	sub_82DB4E50(ctx, base);
	// 82DC3074: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC3078: 4BEE63D8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC3080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC3080 size=24
    let mut pc: u32 = 0x82DC3080;
    'dispatch: loop {
        match pc {
            0x82DC3080 => {
    //   block [0x82DC3080..0x82DC3098)
	// 82DC3080: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC3084: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DC3088: 396B308C  addi r11, r11, 0x308c
	ctx.r[11].s64 = ctx.r[11].s64 + 12428;
	// 82DC308C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DC3090: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC3094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC3098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC3098 size=1912
    let mut pc: u32 = 0x82DC3098;
    'dispatch: loop {
        match pc {
            0x82DC3098 => {
    //   block [0x82DC3098..0x82DC3810)
	// 82DC3098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC309C: 4BEE6359  bl 0x82ca93f4
	ctx.lr = 0x82DC30A0;
	sub_82CA93D0(ctx, base);
	// 82DC30A0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC30A4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DC30A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC30AC: 4BFF276D  bl 0x82db5818
	ctx.lr = 0x82DC30B0;
	sub_82DB5818(ctx, base);
	// 82DC30B0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DC30B4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DC30B8: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 82DC30BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DC30C0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC3810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC3810 size=1208
    let mut pc: u32 = 0x82DC3810;
    'dispatch: loop {
        match pc {
            0x82DC3810 => {
    //   block [0x82DC3810..0x82DC3CC8)
	// 82DC3810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC3814: 4BEE5BDD  bl 0x82ca93f0
	ctx.lr = 0x82DC3818;
	sub_82CA93D0(ctx, base);
	// 82DC3818: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC381C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DC3820: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC3824: 4BFF20B5  bl 0x82db58d8
	ctx.lr = 0x82DC3828;
	sub_82DB58D8(ctx, base);
	// 82DC3828: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC382C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC3830: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 82DC3834: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DC3838: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DC383C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC3840: 4BF91A09  bl 0x82d55248
	ctx.lr = 0x82DC3844;
	sub_82D55248(ctx, base);
	// 82DC3844: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC3848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC384C: 394B307C  addi r10, r11, 0x307c
	ctx.r[10].s64 = ctx.r[11].s64 + 12412;
	// 82DC3850: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82DC3854: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82DC3858: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DC385C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DC3860: 3BB6010C  addi r29, r22, 0x10c
	ctx.r[29].s64 = ctx.r[22].s64 + 268;
	// 82DC3864: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DC3868: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DC386C: B2FF0006  sth r23, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[23].u16 ) };
	// 82DC3870: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82DC3874: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DC3878: 935F000C  stw r26, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 82DC387C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DC3880: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82DC3884: 935F0018  stw r26, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 82DC3888: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DC388C: 935F0020  stw r26, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 82DC3890: 935F0024  stw r26, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 82DC3894: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DC3898: 935F002C  stw r26, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82DC389C: 935F0030  stw r26, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[26].u32 ) };
	// 82DC38A0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC38A4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC38A8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC38AC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC38B0: 409A0010  bne cr6, 0x82dc38c0
	if !ctx.cr[6].eq {
	pc = 0x82DC38C0; continue 'dispatch;
	}
	// 82DC38B4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DC38B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC38BC: 4BF936DD  bl 0x82d56f98
	ctx.lr = 0x82DC38C0;
	sub_82D56F98(ctx, base);
	// 82DC38C0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC38C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DC38C8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC38CC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DC38D0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC38D4: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82DC38D8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC38DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC38E0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC38E4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC38E8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC38EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC38F0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC38F4: 830BFFFC  lwz r24, -4(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DC38F8: 4BFF2399  bl 0x82db5c90
	ctx.lr = 0x82DC38FC;
	sub_82DB5C90(ctx, base);
	// 82DC38FC: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3900: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82DC3904: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC3908: 4099007C  ble cr6, 0x82dc3984
	if !ctx.cr[6].gt {
	pc = 0x82DC3984; continue 'dispatch;
	}
	// 82DC390C: 3BF80008  addi r31, r24, 8
	ctx.r[31].s64 = ctx.r[24].s64 + 8;
	// 82DC3910: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82DC3914: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC3918: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC391C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3920: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC3924: 7FBC582E  lwzx r29, r28, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC3928: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DC392C: 409A0010  bne cr6, 0x82dc393c
	if !ctx.cr[6].eq {
	pc = 0x82DC393C; continue 'dispatch;
	}
	// 82DC3930: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DC3934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC3938: 4BF93661  bl 0x82d56f98
	ctx.lr = 0x82DC393C;
	sub_82D56F98(ctx, base);
	// 82DC393C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3940: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3944: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3948: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82DC394C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3950: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC3954: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC3958: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC395C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC3960: 419A0010  beq cr6, 0x82dc3970
	if ctx.cr[6].eq {
	pc = 0x82DC3970; continue 'dispatch;
	}
	// 82DC3964: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC3968: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC396C: B17D0006  sth r11, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC3970: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3974: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DC3978: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82DC397C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC3980: 4198FF94  blt cr6, 0x82dc3914
	if ctx.cr[6].lt {
	pc = 0x82DC3914; continue 'dispatch;
	}
	// 82DC3984: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC3988: 91790018  stw r11, 0x18(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DC398C: 897E0001  lbz r11, 1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC3990: 99790001  stb r11, 1(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82DC3994: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3998: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DC399C: 419A0210  beq cr6, 0x82dc3bac
	if ctx.cr[6].eq {
	pc = 0x82DC3BAC; continue 'dispatch;
	}
	// 82DC39A0: 897E0001  lbz r11, 1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC39A4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DC39A8: A17E0008  lhz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC39AC: 409A00E0  bne cr6, 0x82dc3a8c
	if !ctx.cr[6].eq {
	pc = 0x82DC3A8C; continue 'dispatch;
	}
	// 82DC39B0: 3BF80014  addi r31, r24, 0x14
	ctx.r[31].s64 = ctx.r[24].s64 + 20;
	// 82DC39B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC39B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC39BC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC39C0: 409A0050  bne cr6, 0x82dc3a10
	if !ctx.cr[6].eq {
	pc = 0x82DC3A10; continue 'dispatch;
	}
	// 82DC39C4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC39C8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC39CC: 409A0010  bne cr6, 0x82dc39dc
	if !ctx.cr[6].eq {
	pc = 0x82DC39DC; continue 'dispatch;
	}
	// 82DC39D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC39D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC39D8: 4BF935C1  bl 0x82d56f98
	ctx.lr = 0x82DC39DC;
	sub_82D56F98(ctx, base);
	// 82DC39DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC39E0: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC39E4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC39E8: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82DC39EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC39F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC39F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC39F8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC39FC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3A00: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC3A04: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC3A08: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC3A0C: 48000168  b 0x82dc3b74
	pc = 0x82DC3B74; continue 'dispatch;
	// 82DC3A10: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3A14: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3A18: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82DC3A1C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC3A20: 40980024  bge cr6, 0x82dc3a44
	if !ctx.cr[6].lt {
	pc = 0x82DC3A44; continue 'dispatch;
	}
	// 82DC3A24: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3A28: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC3A2C: 41980008  blt cr6, 0x82dc3a34
	if ctx.cr[6].lt {
	pc = 0x82DC3A34; continue 'dispatch;
	}
	// 82DC3A30: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC3A34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC3A38: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC3A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC3A40: 4BF934D1  bl 0x82d56f10
	ctx.lr = 0x82DC3A44;
	sub_82D56F10(ctx, base);
	// 82DC3A44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3A48: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC3A4C: 7D3C5A14  add r9, r28, r11
	ctx.r[9].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82DC3A50: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DC3A54: 91390004  stw r9, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC3A58: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3A5C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3A60: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC3A64: 40990110  ble cr6, 0x82dc3b74
	if !ctx.cr[6].gt {
	pc = 0x82DC3B74; continue 'dispatch;
	}
	// 82DC3A68: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3A6C: 7D0B49AE  stbx r8, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82DC3A70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC3A74: A11E0008  lhz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC3A78: 80FE001C  lwz r7, 0x1c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3A7C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DC3A80: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DC3A84: 4198FFE4  blt cr6, 0x82dc3a68
	if ctx.cr[6].lt {
	pc = 0x82DC3A68; continue 'dispatch;
	}
	// 82DC3A88: 480000EC  b 0x82dc3b74
	pc = 0x82DC3B74; continue 'dispatch;
	// 82DC3A8C: 3BF8002C  addi r31, r24, 0x2c
	ctx.r[31].s64 = ctx.r[24].s64 + 44;
	// 82DC3A90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC3A94: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC3A98: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC3A9C: 409A0058  bne cr6, 0x82dc3af4
	if !ctx.cr[6].eq {
	pc = 0x82DC3AF4; continue 'dispatch;
	}
	// 82DC3AA0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3AA4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC3AA8: 409A0010  bne cr6, 0x82dc3ab8
	if !ctx.cr[6].eq {
	pc = 0x82DC3AB8; continue 'dispatch;
	}
	// 82DC3AAC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DC3AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC3AB4: 4BF934E5  bl 0x82d56f98
	ctx.lr = 0x82DC3AB8;
	sub_82D56F98(ctx, base);
	// 82DC3AB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3ABC: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3AC0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3AC4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3AC8: 7D4B4B2E  sthx r10, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u16) };
	// 82DC3ACC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3AD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC3AD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC3AD8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3ADC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3AE0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3AE4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC3AE8: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82DC3AEC: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC3AF0: 48000084  b 0x82dc3b74
	pc = 0x82DC3B74; continue 'dispatch;
	// 82DC3AF4: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3AF8: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3AFC: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82DC3B00: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC3B04: 40980024  bge cr6, 0x82dc3b28
	if !ctx.cr[6].lt {
	pc = 0x82DC3B28; continue 'dispatch;
	}
	// 82DC3B08: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3B0C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC3B10: 41980008  blt cr6, 0x82dc3b18
	if ctx.cr[6].lt {
	pc = 0x82DC3B18; continue 'dispatch;
	}
	// 82DC3B14: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC3B18: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DC3B1C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC3B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC3B24: 4BF933ED  bl 0x82d56f10
	ctx.lr = 0x82DC3B28;
	sub_82D56F10(ctx, base);
	// 82DC3B28: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3B2C: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3B30: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC3B34: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC3B38: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DC3B3C: 91390004  stw r9, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC3B40: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3B44: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3B48: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC3B4C: 40990028  ble cr6, 0x82dc3b74
	if !ctx.cr[6].gt {
	pc = 0x82DC3B74; continue 'dispatch;
	}
	// 82DC3B50: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3B54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC3B58: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82DC3B5C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82DC3B60: A11E0008  lhz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC3B64: 80FE001C  lwz r7, 0x1c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC3B68: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DC3B6C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DC3B70: 4198FFE0  blt cr6, 0x82dc3b50
	if ctx.cr[6].lt {
	pc = 0x82DC3B50; continue 'dispatch;
	}
	// 82DC3B74: A17E0008  lhz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC3B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC3B7C: B1790008  sth r11, 8(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 82DC3B80: 419A0034  beq cr6, 0x82dc3bb4
	if ctx.cr[6].eq {
	pc = 0x82DC3BB4; continue 'dispatch;
	}
	// 82DC3B84: 89790001  lbz r11, 1(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DC3B88: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DC3B8C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DC3B90: 419A0014  beq cr6, 0x82dc3ba4
	if ctx.cr[6].eq {
	pc = 0x82DC3BA4; continue 'dispatch;
	}
	// 82DC3B94: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82DC3B98: 409A001C  bne cr6, 0x82dc3bb4
	if !ctx.cr[6].eq {
	pc = 0x82DC3BB4; continue 'dispatch;
	}
	// 82DC3B9C: B1790008  sth r11, 8(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 82DC3BA0: 48000014  b 0x82dc3bb4
	pc = 0x82DC3BB4; continue 'dispatch;
	// 82DC3BA4: B2F90008  sth r23, 8(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[23].u16 ) };
	// 82DC3BA8: 4800000C  b 0x82dc3bb4
	pc = 0x82DC3BB4; continue 'dispatch;
	// 82DC3BAC: 93590004  stw r26, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82DC3BB0: B3590008  sth r26, 8(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[26].u16 ) };
	// 82DC3BB4: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3BB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC3BBC: 419A00FC  beq cr6, 0x82dc3cb8
	if ctx.cr[6].eq {
	pc = 0x82DC3CB8; continue 'dispatch;
	}
	// 82DC3BC0: 3BF80020  addi r31, r24, 0x20
	ctx.r[31].s64 = ctx.r[24].s64 + 32;
	// 82DC3BC4: A17E0002  lhz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DC3BC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC3BCC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC3BD0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC3BD4: 409A004C  bne cr6, 0x82dc3c20
	if !ctx.cr[6].eq {
	pc = 0x82DC3C20; continue 'dispatch;
	}
	// 82DC3BD8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3BDC: 83DE000C  lwz r30, 0xc(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC3BE0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC3BE4: 409A0010  bne cr6, 0x82dc3bf4
	if !ctx.cr[6].eq {
	pc = 0x82DC3BF4; continue 'dispatch;
	}
	// 82DC3BE8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DC3BEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC3BF0: 4BF933A9  bl 0x82d56f98
	ctx.lr = 0x82DC3BF4;
	sub_82D56F98(ctx, base);
	// 82DC3BF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3BF8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3BFC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3C00: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3C04: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82DC3C08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3C0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC3C10: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC3C14: B3590002  sth r26, 2(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 82DC3C18: B2F9000A  sth r23, 0xa(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(10 as u32), ctx.r[23].u16 ) };
	// 82DC3C1C: 48000094  b 0x82dc3cb0
	pc = 0x82DC3CB0; continue 'dispatch;
	// 82DC3C20: A15E000A  lhz r10, 0xa(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DC3C24: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC3C28: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82DC3C2C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC3C30: 40980024  bge cr6, 0x82dc3c54
	if !ctx.cr[6].lt {
	pc = 0x82DC3C54; continue 'dispatch;
	}
	// 82DC3C34: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC3C38: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC3C3C: 41980008  blt cr6, 0x82dc3c44
	if ctx.cr[6].lt {
	pc = 0x82DC3C44; continue 'dispatch;
	}
	// 82DC3C40: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC3C44: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC3C48: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC3C4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC3C50: 4BF932C1  bl 0x82d56f10
	ctx.lr = 0x82DC3C54;
	sub_82D56F10(ctx, base);
	// 82DC3C54: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3C58: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC3C5C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC3C60: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DC3C64: A11E000A  lhz r8, 0xa(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DC3C68: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DC3C6C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC3C70: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DC3C74: 419A002C  beq cr6, 0x82dc3ca0
	if ctx.cr[6].eq {
	pc = 0x82DC3CA0; continue 'dispatch;
	}
	// 82DC3C78: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3C7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC3C80: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DC3C84: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DC3C88: A11E0002  lhz r8, 2(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DC3C8C: A0FE000A  lhz r7, 0xa(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DC3C90: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 82DC3C94: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DC3C98: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DC3C9C: 4198FFDC  blt cr6, 0x82dc3c78
	if ctx.cr[6].lt {
	pc = 0x82DC3C78; continue 'dispatch;
	}
	// 82DC3CA0: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82DC3CA4: B1790002  sth r11, 2(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82DC3CA8: A17E000A  lhz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DC3CAC: B179000A  sth r11, 0xa(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82DC3CB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC3CB4: 9179000C  stw r11, 0xc(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DC3CB8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82DC3CBC: 4BFF072D  bl 0x82db43e8
	ctx.lr = 0x82DC3CC0;
	sub_82DB43E8(ctx, base);
	// 82DC3CC0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DC3CC4: 4BEE577C  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC3CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC3CC8 size=2872
    let mut pc: u32 = 0x82DC3CC8;
    'dispatch: loop {
        match pc {
            0x82DC3CC8 => {
    //   block [0x82DC3CC8..0x82DC4800)
	// 82DC3CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC3CCC: 4BEE5705  bl 0x82ca93d0
	ctx.lr = 0x82DC3CD0;
	sub_82CA93D0(ctx, base);
	// 82DC3CD0: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DC3CD4: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DC3CD8: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC4800 size=240
    let mut pc: u32 = 0x82DC4800;
    'dispatch: loop {
        match pc {
            0x82DC4800 => {
    //   block [0x82DC4800..0x82DC48F0)
	// 82DC4800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC4808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC480C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4810: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC4814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4818: 396B307C  addi r11, r11, 0x307c
	ctx.r[11].s64 = ctx.r[11].s64 + 12412;
	// 82DC481C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4820: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC4824: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4828: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC482C: 409A0020  bne cr6, 0x82dc484c
	if !ctx.cr[6].eq {
	pc = 0x82DC484C; continue 'dispatch;
	}
	// 82DC4830: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4834: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4838: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC483C: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC4840: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DC4844: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4848: 4BF90A81  bl 0x82d552c8
	ctx.lr = 0x82DC484C;
	sub_82D552C8(ctx, base);
	// 82DC484C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC4850: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4854: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC4858: 409A0020  bne cr6, 0x82dc4878
	if !ctx.cr[6].eq {
	pc = 0x82DC4878; continue 'dispatch;
	}
	// 82DC485C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4860: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4864: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC4868: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC486C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC4870: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4874: 4BF90A55  bl 0x82d552c8
	ctx.lr = 0x82DC4878;
	sub_82D552C8(ctx, base);
	// 82DC4878: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC487C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4880: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC4884: 409A0020  bne cr6, 0x82dc48a4
	if !ctx.cr[6].eq {
	pc = 0x82DC48A4; continue 'dispatch;
	}
	// 82DC4888: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC488C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4890: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC4894: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC4898: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC489C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC48A0: 4BF90A29  bl 0x82d552c8
	ctx.lr = 0x82DC48A4;
	sub_82D552C8(ctx, base);
	// 82DC48A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC48A8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC48AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC48B0: 409A0020  bne cr6, 0x82dc48d0
	if !ctx.cr[6].eq {
	pc = 0x82DC48D0; continue 'dispatch;
	}
	// 82DC48B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC48B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC48BC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC48C0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC48C4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC48C8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC48CC: 4BF909FD  bl 0x82d552c8
	ctx.lr = 0x82DC48D0;
	sub_82D552C8(ctx, base);
	// 82DC48D0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC48D4: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DC48D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC48DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC48E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC48E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC48E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC48EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC48F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC48F0 size=100
    let mut pc: u32 = 0x82DC48F0;
    'dispatch: loop {
        match pc {
            0x82DC48F0 => {
    //   block [0x82DC48F0..0x82DC4954)
	// 82DC48F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC48F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC48F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC48FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC4900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4908: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC490C: 4BFFFEF5  bl 0x82dc4800
	ctx.lr = 0x82DC4910;
	sub_82DC4800(ctx, base);
	// 82DC4910: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DC4914: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC4918: 419A0020  beq cr6, 0x82dc4938
	if ctx.cr[6].eq {
	pc = 0x82DC4938; continue 'dispatch;
	}
	// 82DC491C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4920: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC4924: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82DC4928: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC492C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC4930: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC4934: 4BF90995  bl 0x82d552c8
	ctx.lr = 0x82DC4938;
	sub_82D552C8(ctx, base);
	// 82DC4938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC493C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC4940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC4944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC4948: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC494C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC4950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC4958 size=8
    let mut pc: u32 = 0x82DC4958;
    'dispatch: loop {
        match pc {
            0x82DC4958 => {
    //   block [0x82DC4958..0x82DC4960)
	// 82DC4958: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DC495C: 480001B4  b 0x82dc4b10
	sub_82DC4B10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC4960 size=328
    let mut pc: u32 = 0x82DC4960;
    'dispatch: loop {
        match pc {
            0x82DC4960 => {
    //   block [0x82DC4960..0x82DC4AA8)
	// 82DC4960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC4968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC496C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4970: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC4974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4978: 396B308C  addi r11, r11, 0x308c
	ctx.r[11].s64 = ctx.r[11].s64 + 12428;
	// 82DC497C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4980: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DC4984: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4988: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC498C: 409A0020  bne cr6, 0x82dc49ac
	if !ctx.cr[6].eq {
	pc = 0x82DC49AC; continue 'dispatch;
	}
	// 82DC4990: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4994: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4998: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC499C: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DC49A0: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DC49A4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC49A8: 4BF90921  bl 0x82d552c8
	ctx.lr = 0x82DC49AC;
	sub_82D552C8(ctx, base);
	// 82DC49AC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC49B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC49B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC49B8: 409A0020  bne cr6, 0x82dc49d8
	if !ctx.cr[6].eq {
	pc = 0x82DC49D8; continue 'dispatch;
	}
	// 82DC49BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC49C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC49C4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC49C8: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC49CC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC49D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC49D4: 4BF908F5  bl 0x82d552c8
	ctx.lr = 0x82DC49D8;
	sub_82D552C8(ctx, base);
	// 82DC49D8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC49DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC49E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC49E4: 409A0020  bne cr6, 0x82dc4a04
	if !ctx.cr[6].eq {
	pc = 0x82DC4A04; continue 'dispatch;
	}
	// 82DC49E8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC49EC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC49F0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC49F4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC49F8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC49FC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4A00: 4BF908C9  bl 0x82d552c8
	ctx.lr = 0x82DC4A04;
	sub_82D552C8(ctx, base);
	// 82DC4A04: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC4A08: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4A0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC4A10: 409A0020  bne cr6, 0x82dc4a30
	if !ctx.cr[6].eq {
	pc = 0x82DC4A30; continue 'dispatch;
	}
	// 82DC4A14: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4A18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4A1C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC4A20: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC4A24: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC4A28: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4A2C: 4BF9089D  bl 0x82d552c8
	ctx.lr = 0x82DC4A30;
	sub_82D552C8(ctx, base);
	// 82DC4A30: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC4A34: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4A38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC4A3C: 409A0020  bne cr6, 0x82dc4a5c
	if !ctx.cr[6].eq {
	pc = 0x82DC4A5C; continue 'dispatch;
	}
	// 82DC4A40: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4A44: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4A48: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC4A4C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC4A50: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DC4A54: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4A58: 4BF90871  bl 0x82d552c8
	ctx.lr = 0x82DC4A5C;
	sub_82D552C8(ctx, base);
	// 82DC4A5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC4A60: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4A64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC4A68: 409A0020  bne cr6, 0x82dc4a88
	if !ctx.cr[6].eq {
	pc = 0x82DC4A88; continue 'dispatch;
	}
	// 82DC4A6C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4A70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4A74: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC4A78: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4A7C: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC4A80: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4A84: 4BF90845  bl 0x82d552c8
	ctx.lr = 0x82DC4A88;
	sub_82D552C8(ctx, base);
	// 82DC4A88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC4A8C: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DC4A90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4A94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC4A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC4A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC4AA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC4AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC4AA8 size=100
    let mut pc: u32 = 0x82DC4AA8;
    'dispatch: loop {
        match pc {
            0x82DC4AA8 => {
    //   block [0x82DC4AA8..0x82DC4B0C)
	// 82DC4AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC4AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC4AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC4AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4AC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC4AC4: 4BFFFE9D  bl 0x82dc4960
	ctx.lr = 0x82DC4AC8;
	sub_82DC4960(ctx, base);
	// 82DC4AC8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DC4ACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC4AD0: 419A0020  beq cr6, 0x82dc4af0
	if ctx.cr[6].eq {
	pc = 0x82DC4AF0; continue 'dispatch;
	}
	// 82DC4AD4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4AD8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC4ADC: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82DC4AE0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4AE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC4AE8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC4AEC: 4BF907DD  bl 0x82d552c8
	ctx.lr = 0x82DC4AF0;
	sub_82D552C8(ctx, base);
	// 82DC4AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC4AF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC4AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC4AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC4B00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC4B04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC4B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC4B10 size=100
    let mut pc: u32 = 0x82DC4B10;
    'dispatch: loop {
        match pc {
            0x82DC4B10 => {
    //   block [0x82DC4B10..0x82DC4B74)
	// 82DC4B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC4B18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC4B1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC4B20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4B24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4B28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC4B2C: 4BFFE325  bl 0x82dc2e50
	ctx.lr = 0x82DC4B30;
	sub_82DC2E50(ctx, base);
	// 82DC4B30: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DC4B34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC4B38: 419A0020  beq cr6, 0x82dc4b58
	if ctx.cr[6].eq {
	pc = 0x82DC4B58; continue 'dispatch;
	}
	// 82DC4B3C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4B40: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC4B44: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DC4B48: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4B4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC4B50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC4B54: 4BF90775  bl 0x82d552c8
	ctx.lr = 0x82DC4B58;
	sub_82D552C8(ctx, base);
	// 82DC4B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC4B5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC4B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC4B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC4B68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC4B6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC4B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC4B78 size=28
    let mut pc: u32 = 0x82DC4B78;
    'dispatch: loop {
        match pc {
            0x82DC4B78 => {
    //   block [0x82DC4B78..0x82DC4B94)
	// 82DC4B78: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82DC4B7C: 419A0020  beq cr6, 0x82dc4b9c
	if ctx.cr[6].eq {
		sub_82DC4B9C(ctx, base);
		return;
	}
	// 82DC4B80: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82DC4B84: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC4B88: 419A000C  beq cr6, 0x82dc4b94
	if ctx.cr[6].eq {
		sub_82DC4B94(ctx, base);
		return;
	}
	// 82DC4B8C: 7C645A14  add r3, r4, r11
	ctx.r[3].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82DC4B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4B94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC4B94 size=8
    let mut pc: u32 = 0x82DC4B94;
    'dispatch: loop {
        match pc {
            0x82DC4B94 => {
    //   block [0x82DC4B94..0x82DC4B9C)
	// 82DC4B94: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82DC4B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4B9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC4B9C size=8
    let mut pc: u32 = 0x82DC4B9C;
    'dispatch: loop {
        match pc {
            0x82DC4B9C => {
    //   block [0x82DC4B9C..0x82DC4BA4)
	// 82DC4B9C: 38640002  addi r3, r4, 2
	ctx.r[3].s64 = ctx.r[4].s64 + 2;
	// 82DC4BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC4BA8 size=92
    let mut pc: u32 = 0x82DC4BA8;
    'dispatch: loop {
        match pc {
            0x82DC4BA8 => {
    //   block [0x82DC4BA8..0x82DC4C04)
	// 82DC4BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC4BB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC4BB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4BB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4BBC: 4BFF7D85  bl 0x82dbc940
	ctx.lr = 0x82DC4BC0;
	sub_82DBC940(ctx, base);
	// 82DC4BC0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC4BC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC4BC8: 396B30C0  addi r11, r11, 0x30c0
	ctx.r[11].s64 = ctx.r[11].s64 + 12480;
	// 82DC4BCC: 394A309C  addi r10, r10, 0x309c
	ctx.r[10].s64 = ctx.r[10].s64 + 12444;
	// 82DC4BD0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DC4BD4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DC4BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC4BDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4BE0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC4BE4: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82DC4BE8: 913F0064  stw r9, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82DC4BEC: 911F0068  stw r8, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82DC4BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC4BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC4BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC4BFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC4C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC4C08 size=204
    let mut pc: u32 = 0x82DC4C08;
    'dispatch: loop {
        match pc {
            0x82DC4C08 => {
    //   block [0x82DC4C08..0x82DC4CD4)
	// 82DC4C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4C0C: 4BEE4801  bl 0x82ca940c
	ctx.lr = 0x82DC4C10;
	sub_82CA93D0(ctx, base);
	// 82DC4C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4C14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4C18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC4C1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC4C20: 396B30C0  addi r11, r11, 0x30c0
	ctx.r[11].s64 = ctx.r[11].s64 + 12480;
	// 82DC4C24: 394A309C  addi r10, r10, 0x309c
	ctx.r[10].s64 = ctx.r[10].s64 + 12444;
	// 82DC4C28: 813F0064  lwz r9, 0x64(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DC4C2C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC4C30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DC4C34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4C38: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC4C3C: 4099005C  ble cr6, 0x82dc4c98
	if !ctx.cr[6].gt {
	pc = 0x82DC4C98; continue 'dispatch;
	}
	// 82DC4C40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DC4C44: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DC4C48: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC4C4C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4C50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC4C54: 419A0030  beq cr6, 0x82dc4c84
	if ctx.cr[6].eq {
	pc = 0x82DC4C84; continue 'dispatch;
	}
	// 82DC4C58: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC4C5C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC4C60: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC4C64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC4C68: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC4C6C: 409A0018  bne cr6, 0x82dc4c84
	if !ctx.cr[6].eq {
	pc = 0x82DC4C84; continue 'dispatch;
	}
	// 82DC4C70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4C74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC4C78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4C7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC4C80: 4E800421  bctrl
	ctx.lr = 0x82DC4C84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC4C84: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DC4C88: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DC4C8C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DC4C90: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC4C94: 4198FFB0  blt cr6, 0x82dc4c44
	if ctx.cr[6].lt {
	pc = 0x82DC4C44; continue 'dispatch;
	}
	// 82DC4C98: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DC4C9C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC4CA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC4CA4: 409A0020  bne cr6, 0x82dc4cc4
	if !ctx.cr[6].eq {
	pc = 0x82DC4CC4; continue 'dispatch;
	}
	// 82DC4CA8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4CAC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4CB0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC4CB4: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DC4CB8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC4CBC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4CC0: 4BF90609  bl 0x82d552c8
	ctx.lr = 0x82DC4CC4;
	sub_82D552C8(ctx, base);
	// 82DC4CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC4CC8: 4BFEE8E1  bl 0x82db35a8
	ctx.lr = 0x82DC4CCC;
	sub_82DB35A8(ctx, base);
	// 82DC4CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC4CD0: 4BEE478C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC4CD8 size=44
    let mut pc: u32 = 0x82DC4CD8;
    'dispatch: loop {
        match pc {
            0x82DC4CD8 => {
    //   block [0x82DC4CD8..0x82DC4D04)
	// 82DC4CD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC4CDC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DC4CE0: 396B30EC  addi r11, r11, 0x30ec
	ctx.r[11].s64 = ctx.r[11].s64 + 12524;
	// 82DC4CE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DC4CE8: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DC4CEC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DC4CF0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4CF4: 91430044  stw r10, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82DC4CF8: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82DC4CFC: 9103004C  stw r8, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[8].u32 ) };
	// 82DC4D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC4D08 size=228
    let mut pc: u32 = 0x82DC4D08;
    'dispatch: loop {
        match pc {
            0x82DC4D08 => {
    //   block [0x82DC4D08..0x82DC4DEC)
	// 82DC4D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC4D10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC4D14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC4D18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4D20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC4D24: 4BFF7CCD  bl 0x82dbc9f0
	ctx.lr = 0x82DC4D28;
	sub_82DBC9F0(ctx, base);
	// 82DC4D28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC4D2C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC4D30: 396B30C0  addi r11, r11, 0x30c0
	ctx.r[11].s64 = ctx.r[11].s64 + 12480;
	// 82DC4D34: 394A309C  addi r10, r10, 0x309c
	ctx.r[10].s64 = ctx.r[10].s64 + 12444;
	// 82DC4D38: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DC4D3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4D40: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC4D44: 419A008C  beq cr6, 0x82dc4dd0
	if ctx.cr[6].eq {
	pc = 0x82DC4DD0; continue 'dispatch;
	}
	// 82DC4D48: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC4D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DC4D50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC4D54: 4099007C  ble cr6, 0x82dc4dd0
	if !ctx.cr[6].gt {
	pc = 0x82DC4DD0; continue 'dispatch;
	}
	// 82DC4D58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DC4D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DC4D60: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DC4D64: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC4D68: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DC4D6C: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC4D70: 88CB0010  lbz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC4D74: 80AA0008  lwz r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4D78: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DC4D7C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DC4D80: 409A000C  bne cr6, 0x82dc4d8c
	if !ctx.cr[6].eq {
	pc = 0x82DC4D8C; continue 'dispatch;
	}
	// 82DC4D84: 80CA0014  lwz r6, 0x14(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC4D88: 48000008  b 0x82dc4d90
	pc = 0x82DC4D90; continue 'dispatch;
	// 82DC4D8C: 80CA0020  lwz r6, 0x20(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC4D90: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DC4D94: 88CB0011  lbz r6, 0x11(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DC4D98: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 82DC4D9C: 409A000C  bne cr6, 0x82dc4da8
	if !ctx.cr[6].eq {
	pc = 0x82DC4DA8; continue 'dispatch;
	}
	// 82DC4DA0: 80CA002C  lwz r6, 0x2c(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC4DA4: 48000008  b 0x82dc4dac
	pc = 0x82DC4DAC; continue 'dispatch;
	// 82DC4DA8: 80CA0044  lwz r6, 0x44(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DC4DAC: 90CB0020  stw r6, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82DC4DB0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DC4DB4: 814A0038  lwz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC4DB8: 39290038  addi r9, r9, 0x38
	ctx.r[9].s64 = ctx.r[9].s64 + 56;
	// 82DC4DBC: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82DC4DC0: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82DC4DC4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC4DC8: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC4DCC: 4198FF94  blt cr6, 0x82dc4d60
	if ctx.cr[6].lt {
	pc = 0x82DC4D60; continue 'dispatch;
	}
	// 82DC4DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC4DD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC4DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC4DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC4DE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC4DE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC4DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC4DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC4DF0 size=1880
    let mut pc: u32 = 0x82DC4DF0;
    'dispatch: loop {
        match pc {
            0x82DC4DF0 => {
    //   block [0x82DC4DF0..0x82DC5548)
	// 82DC4DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC4DF4: 4BEE4605  bl 0x82ca93f8
	ctx.lr = 0x82DC4DF8;
	sub_82CA93D0(ctx, base);
	// 82DC4DF8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC4DFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC4E00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC4E04: 3BFD0034  addi r31, r29, 0x34
	ctx.r[31].s64 = ctx.r[29].s64 + 52;
	// 82DC4E08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4E0C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4E10: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC4E14: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC4E18: 409A0010  bne cr6, 0x82dc4e28
	if !ctx.cr[6].eq {
	pc = 0x82DC4E28; continue 'dispatch;
	}
	// 82DC4E1C: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82DC4E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC4E24: 4BF92175  bl 0x82d56f98
	ctx.lr = 0x82DC4E28;
	sub_82D56F98(ctx, base);
	// 82DC4E28: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4E2C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC4E30: 810D0000  lwz r8, 0(r13)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4E34: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 82DC4E38: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82DC4E3C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4E40: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 82DC4E44: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DC4E48: 7C69402E  lwzx r3, r9, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DC4E4C: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82DC4E50: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC4E54: 4BF903F5  bl 0x82d55248
	ctx.lr = 0x82DC4E58;
	sub_82D55248(ctx, base);
	// 82DC4E58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC4E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC4E60: 394B30EC  addi r10, r11, 0x30ec
	ctx.r[10].s64 = ctx.r[11].s64 + 12524;
	// 82DC4E64: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82DC4E68: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 82DC4E6C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DC4E70: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82DC4E74: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DC4E78: 3BBD0060  addi r29, r29, 0x60
	ctx.r[29].s64 = ctx.r[29].s64 + 96;
	// 82DC4E7C: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DC4E80: B31F0006  sth r24, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[24].u16 ) };
	// 82DC4E84: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82DC4E88: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DC4E8C: 935F000C  stw r26, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 82DC4E90: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DC4E94: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82DC4E98: 935F0018  stw r26, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 82DC4E9C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DC4EA0: 935F0020  stw r26, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 82DC4EA4: 935F0024  stw r26, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 82DC4EA8: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DC4EAC: 935F002C  stw r26, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82DC4EB0: 935F0030  stw r26, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[26].u32 ) };
	// 82DC4EB4: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DC4EB8: 935F0038  stw r26, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 82DC4EBC: 935F003C  stw r26, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[26].u32 ) };
	// 82DC4EC0: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82DC4EC4: 935F0044  stw r26, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[26].u32 ) };
	// 82DC4EC8: 935F0048  stw r26, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 82DC4ECC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4ED0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4ED4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC4ED8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC4EDC: 409A0010  bne cr6, 0x82dc4eec
	if !ctx.cr[6].eq {
	pc = 0x82DC4EEC; continue 'dispatch;
	}
	// 82DC4EE0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DC4EE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DC4EE8: 4BF920B1  bl 0x82d56f98
	ctx.lr = 0x82DC4EEC;
	sub_82D56F98(ctx, base);
	// 82DC4EEC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4EF0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4EF4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC4EF8: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82DC4EFC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4F00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC4F04: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC4F08: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4F0C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4F10: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC4F14: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4F18: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DC4F1C: 832AFFFC  lwz r25, -4(r10)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82DC4F20: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC4F24: 3BF90008  addi r31, r25, 8
	ctx.r[31].s64 = ctx.r[25].s64 + 8;
	// 82DC4F28: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC4F2C: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4F30: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4F34: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82DC4F38: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC4F3C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC4F40: 40980024  bge cr6, 0x82dc4f64
	if !ctx.cr[6].lt {
	pc = 0x82DC4F64; continue 'dispatch;
	}
	// 82DC4F44: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC4F48: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC4F4C: 41980008  blt cr6, 0x82dc4f54
	if ctx.cr[6].lt {
	pc = 0x82DC4F54; continue 'dispatch;
	}
	// 82DC4F50: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC4F54: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC4F58: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC4F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC4F60: 4BF91FB1  bl 0x82d56f10
	ctx.lr = 0x82DC4F64;
	sub_82D56F10(ctx, base);
	// 82DC4F64: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4F68: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC4F6C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC4F70: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82DC4F74: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4F78: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC4F7C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4F80: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC4F84: 40990038  ble cr6, 0x82dc4fbc
	if !ctx.cr[6].gt {
	pc = 0x82DC4FBC; continue 'dispatch;
	}
	// 82DC4F88: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC4F8C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DC4F90: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DC4F94: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC4F98: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DC4F9C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DC4FA0: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DC4FA4: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC4FA8: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4FAC: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82DC4FB0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DC4FB4: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DC4FB8: 4198FFD0  blt cr6, 0x82dc4f88
	if ctx.cr[6].lt {
	pc = 0x82DC4F88; continue 'dispatch;
	}
	// 82DC4FBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC4FC0: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82DC4FC4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC4FC8: 915B0004  stw r10, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DC4FCC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC4FD0: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DC4FD4: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC4FD8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DC4FDC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC4FE0: 409A00F0  bne cr6, 0x82dc50d0
	if !ctx.cr[6].eq {
	pc = 0x82DC50D0; continue 'dispatch;
	}
	// 82DC4FE4: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC4FE8: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82DC4FEC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DC4FF0: 40990008  ble cr6, 0x82dc4ff8
	if !ctx.cr[6].gt {
	pc = 0x82DC4FF8; continue 'dispatch;
	}
	// 82DC4FF4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82DC4FF8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC4FFC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82DC5000: 917B0014  stw r11, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DC5004: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC5008: 419A0024  beq cr6, 0x82dc502c
	if ctx.cr[6].eq {
	pc = 0x82DC502C; continue 'dispatch;
	}
	// 82DC500C: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82DC5010: 419A0010  beq cr6, 0x82dc5020
	if ctx.cr[6].eq {
	pc = 0x82DC5020; continue 'dispatch;
	}
	// 82DC5014: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC5018: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC501C: 48000014  b 0x82dc5030
	pc = 0x82DC5030; continue 'dispatch;
	// 82DC5020: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5024: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82DC5028: 48000008  b 0x82dc5030
	pc = 0x82DC5030; continue 'dispatch;
	// 82DC502C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82DC5030: 3BF90014  addi r31, r25, 0x14
	ctx.r[31].s64 = ctx.r[25].s64 + 20;
	// 82DC5034: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5038: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC503C: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82DC5040: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC5044: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC5048: 40980024  bge cr6, 0x82dc506c
	if !ctx.cr[6].lt {
	pc = 0x82DC506C; continue 'dispatch;
	}
	// 82DC504C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5050: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC5054: 41980008  blt cr6, 0x82dc505c
	if ctx.cr[6].lt {
	pc = 0x82DC505C; continue 'dispatch;
	}
	// 82DC5058: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC505C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DC5060: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC5064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5068: 4BF91EA9  bl 0x82d56f10
	ctx.lr = 0x82DC506C;
	sub_82D56F10(ctx, base);
	// 82DC506C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5070: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5074: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC5078: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82DC507C: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC5080: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC5084: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5088: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC508C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82DC5090: 40990128  ble cr6, 0x82dc51b8
	if !ctx.cr[6].gt {
	pc = 0x82DC51B8; continue 'dispatch;
	}
	// 82DC5094: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5098: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DC509C: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82DC50A0: A10A0002  lhz r8, 2(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DC50A4: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82DC50A8: A10A0004  lhz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC50AC: B10B0004  sth r8, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82DC50B0: 80FB0014  lwz r7, 0x14(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC50B4: 811E0014  lwz r8, 0x14(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC50B8: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC50BC: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82DC50C0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DC50C4: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82DC50C8: 4198FFCC  blt cr6, 0x82dc5094
	if ctx.cr[6].lt {
	pc = 0x82DC5094; continue 'dispatch;
	}
	// 82DC50CC: 480000EC  b 0x82dc51b8
	pc = 0x82DC51B8; continue 'dispatch;
	// 82DC50D0: 556AF0BE  srwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC50D4: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82DC50D8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DC50DC: 40990008  ble cr6, 0x82dc50e4
	if !ctx.cr[6].gt {
	pc = 0x82DC50E4; continue 'dispatch;
	}
	// 82DC50E0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82DC50E4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC50E8: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82DC50EC: 917B0014  stw r11, 0x14(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DC50F0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC50F4: 419A0024  beq cr6, 0x82dc5118
	if ctx.cr[6].eq {
	pc = 0x82DC5118; continue 'dispatch;
	}
	// 82DC50F8: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 82DC50FC: 419A0010  beq cr6, 0x82dc510c
	if ctx.cr[6].eq {
	pc = 0x82DC510C; continue 'dispatch;
	}
	// 82DC5100: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC5104: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC5108: 48000014  b 0x82dc511c
	pc = 0x82DC511C; continue 'dispatch;
	// 82DC510C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5110: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82DC5114: 48000008  b 0x82dc511c
	pc = 0x82DC511C; continue 'dispatch;
	// 82DC5118: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82DC511C: 3BF90020  addi r31, r25, 0x20
	ctx.r[31].s64 = ctx.r[25].s64 + 32;
	// 82DC5120: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5124: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC5128: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82DC512C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC5130: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC5134: 40980024  bge cr6, 0x82dc5158
	if !ctx.cr[6].lt {
	pc = 0x82DC5158; continue 'dispatch;
	}
	// 82DC5138: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC513C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC5140: 41980008  blt cr6, 0x82dc5148
	if ctx.cr[6].lt {
	pc = 0x82DC5148; continue 'dispatch;
	}
	// 82DC5144: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC5148: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC514C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC5150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5154: 4BF91DBD  bl 0x82d56f10
	ctx.lr = 0x82DC5158;
	sub_82D56F10(ctx, base);
	// 82DC5158: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC515C: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5160: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC5164: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82DC5168: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC516C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC5170: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5174: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC5178: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82DC517C: 4099003C  ble cr6, 0x82dc51b8
	if !ctx.cr[6].gt {
	pc = 0x82DC51B8; continue 'dispatch;
	}
	// 82DC5180: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5184: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DC5188: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DC518C: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5190: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC5194: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC5198: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DC519C: 80FB0014  lwz r7, 0x14(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC51A0: 811E0014  lwz r8, 0x14(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC51A4: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC51A8: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82DC51AC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DC51B0: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82DC51B4: 4198FFCC  blt cr6, 0x82dc5180
	if ctx.cr[6].lt {
	pc = 0x82DC5180; continue 'dispatch;
	}
	// 82DC51B8: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC51BC: 997B0010  stb r11, 0x10(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82DC51C0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC51C4: 917B001C  stw r11, 0x1c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DC51C8: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC51CC: 90DB000C  stw r6, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DC51D0: 917B0018  stw r11, 0x18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DC51D4: 897E0011  lbz r11, 0x11(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DC51D8: 997B0011  stb r11, 0x11(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82DC51DC: 83BE0020  lwz r29, 0x20(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC51E0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DC51E4: 419A0210  beq cr6, 0x82dc53f4
	if ctx.cr[6].eq {
	pc = 0x82DC53F4; continue 'dispatch;
	}
	// 82DC51E8: 897E0011  lbz r11, 0x11(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DC51EC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DC51F0: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC51F4: 409A00E0  bne cr6, 0x82dc52d4
	if !ctx.cr[6].eq {
	pc = 0x82DC52D4; continue 'dispatch;
	}
	// 82DC51F8: 3BF9002C  addi r31, r25, 0x2c
	ctx.r[31].s64 = ctx.r[25].s64 + 44;
	// 82DC51FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC5200: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC5204: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC5208: 409A0050  bne cr6, 0x82dc5258
	if !ctx.cr[6].eq {
	pc = 0x82DC5258; continue 'dispatch;
	}
	// 82DC520C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5210: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC5214: 409A0010  bne cr6, 0x82dc5224
	if !ctx.cr[6].eq {
	pc = 0x82DC5224; continue 'dispatch;
	}
	// 82DC5218: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC521C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5220: 4BF91D79  bl 0x82d56f98
	ctx.lr = 0x82DC5224;
	sub_82D56F98(ctx, base);
	// 82DC5224: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5228: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC522C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5230: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82DC5234: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5238: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC523C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC5240: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5244: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5248: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC524C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC5250: 917B0020  stw r11, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DC5254: 48000168  b 0x82dc53bc
	pc = 0x82DC53BC; continue 'dispatch;
	// 82DC5258: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC525C: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5260: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82DC5264: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC5268: 40980024  bge cr6, 0x82dc528c
	if !ctx.cr[6].lt {
	pc = 0x82DC528C; continue 'dispatch;
	}
	// 82DC526C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5270: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC5274: 41980008  blt cr6, 0x82dc527c
	if ctx.cr[6].lt {
	pc = 0x82DC527C; continue 'dispatch;
	}
	// 82DC5278: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC527C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC5280: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC5284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5288: 4BF91C89  bl 0x82d56f10
	ctx.lr = 0x82DC528C;
	sub_82D56F10(ctx, base);
	// 82DC528C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5290: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC5294: 7D3C5A14  add r9, r28, r11
	ctx.r[9].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 82DC5298: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DC529C: 913B0020  stw r9, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82DC52A0: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC52A4: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC52A8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC52AC: 40990110  ble cr6, 0x82dc53bc
	if !ctx.cr[6].gt {
	pc = 0x82DC53BC; continue 'dispatch;
	}
	// 82DC52B0: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC52B4: 7D0B49AE  stbx r8, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 82DC52B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC52BC: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC52C0: 80FE001C  lwz r7, 0x1c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC52C4: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DC52C8: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DC52CC: 4198FFE4  blt cr6, 0x82dc52b0
	if ctx.cr[6].lt {
	pc = 0x82DC52B0; continue 'dispatch;
	}
	// 82DC52D0: 480000EC  b 0x82dc53bc
	pc = 0x82DC53BC; continue 'dispatch;
	// 82DC52D4: 3BF90044  addi r31, r25, 0x44
	ctx.r[31].s64 = ctx.r[25].s64 + 68;
	// 82DC52D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC52DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC52E0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC52E4: 409A0058  bne cr6, 0x82dc533c
	if !ctx.cr[6].eq {
	pc = 0x82DC533C; continue 'dispatch;
	}
	// 82DC52E8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC52EC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC52F0: 409A0010  bne cr6, 0x82dc5300
	if !ctx.cr[6].eq {
	pc = 0x82DC5300; continue 'dispatch;
	}
	// 82DC52F4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82DC52F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC52FC: 4BF91C9D  bl 0x82d56f98
	ctx.lr = 0x82DC5300;
	sub_82D56F98(ctx, base);
	// 82DC5300: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5304: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5308: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC530C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5310: 7D4B4B2E  sthx r10, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u16) };
	// 82DC5314: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5318: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC531C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC5320: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5324: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5328: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC532C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC5330: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82DC5334: 917B0020  stw r11, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DC5338: 48000084  b 0x82dc53bc
	pc = 0x82DC53BC; continue 'dispatch;
	// 82DC533C: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC5340: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5344: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82DC5348: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC534C: 40980024  bge cr6, 0x82dc5370
	if !ctx.cr[6].lt {
	pc = 0x82DC5370; continue 'dispatch;
	}
	// 82DC5350: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5354: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC5358: 41980008  blt cr6, 0x82dc5360
	if ctx.cr[6].lt {
	pc = 0x82DC5360; continue 'dispatch;
	}
	// 82DC535C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC5360: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DC5364: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC5368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC536C: 4BF91BA5  bl 0x82d56f10
	ctx.lr = 0x82DC5370;
	sub_82D56F10(ctx, base);
	// 82DC5370: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5374: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5378: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC537C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DC5380: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DC5384: 913B0020  stw r9, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82DC5388: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC538C: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC5390: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC5394: 40990028  ble cr6, 0x82dc53bc
	if !ctx.cr[6].gt {
	pc = 0x82DC53BC; continue 'dispatch;
	}
	// 82DC5398: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC539C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC53A0: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82DC53A4: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82DC53A8: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC53AC: 80FE001C  lwz r7, 0x1c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC53B0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DC53B4: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DC53B8: 4198FFE0  blt cr6, 0x82dc5398
	if ctx.cr[6].lt {
	pc = 0x82DC5398; continue 'dispatch;
	}
	// 82DC53BC: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC53C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC53C4: 917B0024  stw r11, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82DC53C8: 419A0034  beq cr6, 0x82dc53fc
	if ctx.cr[6].eq {
	pc = 0x82DC53FC; continue 'dispatch;
	}
	// 82DC53CC: 897B0011  lbz r11, 0x11(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DC53D0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DC53D4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DC53D8: 419A0014  beq cr6, 0x82dc53ec
	if ctx.cr[6].eq {
	pc = 0x82DC53EC; continue 'dispatch;
	}
	// 82DC53DC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82DC53E0: 409A001C  bne cr6, 0x82dc53fc
	if !ctx.cr[6].eq {
	pc = 0x82DC53FC; continue 'dispatch;
	}
	// 82DC53E4: 917B0024  stw r11, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82DC53E8: 48000014  b 0x82dc53fc
	pc = 0x82DC53FC; continue 'dispatch;
	// 82DC53EC: 931B0024  stw r24, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[24].u32 ) };
	// 82DC53F0: 4800000C  b 0x82dc53fc
	pc = 0x82DC53FC; continue 'dispatch;
	// 82DC53F4: 935B0020  stw r26, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 82DC53F8: 935B0024  stw r26, 0x24(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 82DC53FC: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC5400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC5404: 419A011C  beq cr6, 0x82dc5520
	if ctx.cr[6].eq {
	pc = 0x82DC5520; continue 'dispatch;
	}
	// 82DC5408: 3BF90038  addi r31, r25, 0x38
	ctx.r[31].s64 = ctx.r[25].s64 + 56;
	// 82DC540C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC5410: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC5414: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC5418: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC541C: 409A0060  bne cr6, 0x82dc547c
	if !ctx.cr[6].eq {
	pc = 0x82DC547C; continue 'dispatch;
	}
	// 82DC5420: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5424: 83BE0028  lwz r29, 0x28(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC5428: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC542C: 409A0010  bne cr6, 0x82dc543c
	if !ctx.cr[6].eq {
	pc = 0x82DC543C; continue 'dispatch;
	}
	// 82DC5430: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DC5434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5438: 4BF91B61  bl 0x82d56f98
	ctx.lr = 0x82DC543C;
	sub_82D56F98(ctx, base);
	// 82DC543C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5440: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5444: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5448: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC544C: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82DC5450: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5454: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC5458: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DC545C: 935B002C  stw r26, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82DC5460: 931B0030  stw r24, 0x30(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 82DC5464: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5468: 917B0028  stw r11, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DC546C: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC5470: 917B0034  stw r11, 0x34(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DC5474: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DC5478: 4BEE3FD0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82DC547C: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC5480: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5484: 7FBC5214  add r29, r28, r10
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82DC5488: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DC548C: 40980024  bge cr6, 0x82dc54b0
	if !ctx.cr[6].lt {
	pc = 0x82DC54B0; continue 'dispatch;
	}
	// 82DC5490: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC5494: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DC5498: 41980008  blt cr6, 0x82dc54a0
	if ctx.cr[6].lt {
	pc = 0x82DC54A0; continue 'dispatch;
	}
	// 82DC549C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DC54A0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DC54A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DC54A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC54AC: 4BF91A65  bl 0x82d56f10
	ctx.lr = 0x82DC54B0;
	sub_82D56F10(ctx, base);
	// 82DC54B0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC54B4: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DC54B8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DC54BC: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DC54C0: 811E0030  lwz r8, 0x30(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC54C4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DC54C8: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC54CC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DC54D0: 40990028  ble cr6, 0x82dc54f8
	if !ctx.cr[6].gt {
	pc = 0x82DC54F8; continue 'dispatch;
	}
	// 82DC54D4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC54D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC54DC: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DC54E0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DC54E4: 811E002C  lwz r8, 0x2c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC54E8: 80FE0030  lwz r7, 0x30(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC54EC: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DC54F0: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DC54F4: 4198FFE0  blt cr6, 0x82dc54d4
	if ctx.cr[6].lt {
	pc = 0x82DC54D4; continue 'dispatch;
	}
	// 82DC54F8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82DC54FC: 917B002C  stw r11, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82DC5500: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC5504: 917B0030  stw r11, 0x30(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DC5508: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC550C: 917B0028  stw r11, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DC5510: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC5514: 917B0034  stw r11, 0x34(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DC5518: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DC551C: 4BEE3F2C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82DC5520: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC5524: 931B0030  stw r24, 0x30(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(48 as u32), ctx.r[24].u32 ) };
	// 82DC5528: 935B002C  stw r26, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 82DC552C: 396B4C30  addi r11, r11, 0x4c30
	ctx.r[11].s64 = ctx.r[11].s64 + 19504;
	// 82DC5530: 917B0020  stw r11, 0x20(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DC5534: 917B0028  stw r11, 0x28(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DC5538: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC553C: 917B0034  stw r11, 0x34(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82DC5540: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DC5544: 4BEE3F04  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DC5548 size=348
    let mut pc: u32 = 0x82DC5548;
    'dispatch: loop {
        match pc {
            0x82DC5548 => {
    //   block [0x82DC5548..0x82DC56A4)
	// 82DC5548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC554C: 4BEE3EBD  bl 0x82ca9408
	ctx.lr = 0x82DC5550;
	sub_82CA93D0(ctx, base);
	// 82DC5550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5554: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC5558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC555C: 80BE0030  lwz r5, 0x30(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DC5560: C03E0050  lfs f1, 0x50(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DC5564: 4BFF73DD  bl 0x82dbc940
	ctx.lr = 0x82DC5568;
	sub_82DBC940(ctx, base);
	// 82DC5568: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC556C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC5570: 396B30C0  addi r11, r11, 0x30c0
	ctx.r[11].s64 = ctx.r[11].s64 + 12480;
	// 82DC5574: 394A309C  addi r10, r10, 0x309c
	ctx.r[10].s64 = ctx.r[10].s64 + 12444;
	// 82DC5578: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DC557C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DC5580: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DC5584: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC5588: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC558C: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82DC5590: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82DC5594: 911F0068  stw r8, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC56A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC56A8 size=8
    let mut pc: u32 = 0x82DC56A8;
    'dispatch: loop {
        match pc {
            0x82DC56A8 => {
    //   block [0x82DC56A8..0x82DC56B0)
	// 82DC56A8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82DC56AC: 480001B4  b 0x82dc5860
	sub_82DC5860(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC56B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC56B0 size=328
    let mut pc: u32 = 0x82DC56B0;
    'dispatch: loop {
        match pc {
            0x82DC56B0 => {
    //   block [0x82DC56B0..0x82DC57F8)
	// 82DC56B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC56B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC56B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC56BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC56C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC56C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC56C8: 396B30EC  addi r11, r11, 0x30ec
	ctx.r[11].s64 = ctx.r[11].s64 + 12524;
	// 82DC56CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC56D0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DC56D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC56D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC56DC: 409A0020  bne cr6, 0x82dc56fc
	if !ctx.cr[6].eq {
	pc = 0x82DC56FC; continue 'dispatch;
	}
	// 82DC56E0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC56E4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC56E8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC56EC: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DC56F0: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DC56F4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC56F8: 4BF8FBD1  bl 0x82d552c8
	ctx.lr = 0x82DC56FC;
	sub_82D552C8(ctx, base);
	// 82DC56FC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC5700: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC5704: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC5708: 409A0020  bne cr6, 0x82dc5728
	if !ctx.cr[6].eq {
	pc = 0x82DC5728; continue 'dispatch;
	}
	// 82DC570C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5710: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC5714: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC5718: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DC571C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC5720: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC5724: 4BF8FBA5  bl 0x82d552c8
	ctx.lr = 0x82DC5728;
	sub_82D552C8(ctx, base);
	// 82DC5728: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DC572C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC5730: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC5734: 409A0020  bne cr6, 0x82dc5754
	if !ctx.cr[6].eq {
	pc = 0x82DC5754; continue 'dispatch;
	}
	// 82DC5738: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC573C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC5740: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC5744: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC5748: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DC574C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC5750: 4BF8FB79  bl 0x82d552c8
	ctx.lr = 0x82DC5754;
	sub_82D552C8(ctx, base);
	// 82DC5754: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC5758: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC575C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC5760: 409A0020  bne cr6, 0x82dc5780
	if !ctx.cr[6].eq {
	pc = 0x82DC5780; continue 'dispatch;
	}
	// 82DC5764: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5768: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC576C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC5770: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC5774: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC5778: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC577C: 4BF8FB4D  bl 0x82d552c8
	ctx.lr = 0x82DC5780;
	sub_82D552C8(ctx, base);
	// 82DC5780: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC5784: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC5788: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC578C: 409A0020  bne cr6, 0x82dc57ac
	if !ctx.cr[6].eq {
	pc = 0x82DC57AC; continue 'dispatch;
	}
	// 82DC5790: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5794: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC5798: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC579C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC57A0: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DC57A4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC57A8: 4BF8FB21  bl 0x82d552c8
	ctx.lr = 0x82DC57AC;
	sub_82D552C8(ctx, base);
	// 82DC57AC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC57B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DC57B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DC57B8: 409A0020  bne cr6, 0x82dc57d8
	if !ctx.cr[6].eq {
	pc = 0x82DC57D8; continue 'dispatch;
	}
	// 82DC57BC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC57C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DC57C4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DC57C8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC57CC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DC57D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DC57D4: 4BF8FAF5  bl 0x82d552c8
	ctx.lr = 0x82DC57D8;
	sub_82D552C8(ctx, base);
	// 82DC57D8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC57DC: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DC57E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC57E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC57E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC57EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC57F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC57F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC57F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC57F8 size=100
    let mut pc: u32 = 0x82DC57F8;
    'dispatch: loop {
        match pc {
            0x82DC57F8 => {
    //   block [0x82DC57F8..0x82DC585C)
	// 82DC57F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC57FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC5800: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC5804: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC5808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC580C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC5810: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC5814: 4BFFFE9D  bl 0x82dc56b0
	ctx.lr = 0x82DC5818;
	sub_82DC56B0(ctx, base);
	// 82DC5818: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DC581C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC5820: 419A0020  beq cr6, 0x82dc5840
	if ctx.cr[6].eq {
	pc = 0x82DC5840; continue 'dispatch;
	}
	// 82DC5824: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5828: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC582C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82DC5830: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5834: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC5838: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC583C: 4BF8FA8D  bl 0x82d552c8
	ctx.lr = 0x82DC5840;
	sub_82D552C8(ctx, base);
	// 82DC5840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5844: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC5848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC584C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC5850: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC5854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC5858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5860 size=100
    let mut pc: u32 = 0x82DC5860;
    'dispatch: loop {
        match pc {
            0x82DC5860 => {
    //   block [0x82DC5860..0x82DC58C4)
	// 82DC5860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC5868: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC586C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC5870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC5878: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC587C: 4BFFF38D  bl 0x82dc4c08
	ctx.lr = 0x82DC5880;
	sub_82DC4C08(ctx, base);
	// 82DC5880: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DC5884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC5888: 419A0020  beq cr6, 0x82dc58a8
	if ctx.cr[6].eq {
	pc = 0x82DC58A8; continue 'dispatch;
	}
	// 82DC588C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5890: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DC5894: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DC5898: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC589C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DC58A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DC58A4: 4BF8FA25  bl 0x82d552c8
	ctx.lr = 0x82DC58A8;
	sub_82D552C8(ctx, base);
	// 82DC58A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC58AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC58B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC58B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC58B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC58BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC58C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC58C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC58C8 size=144
    let mut pc: u32 = 0x82DC58C8;
    'dispatch: loop {
        match pc {
            0x82DC58C8 => {
    //   block [0x82DC58C8..0x82DC5958)
	// 82DC58C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC58CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC58D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC58D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC58D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC58DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC58E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC58E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC58E8: 388B3104  addi r4, r11, 0x3104
	ctx.r[4].s64 = ctx.r[11].s64 + 12548;
	// 82DC58EC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC58F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC58F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC58F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC58FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5904: 4E800421  bctrl
	ctx.lr = 0x82DC5908;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5908: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC590C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC5910: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC5914: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC5918: 388B30F8  addi r4, r11, 0x30f8
	ctx.r[4].s64 = ctx.r[11].s64 + 12536;
	// 82DC591C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5920: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5928: 4E800421  bctrl
	ctx.lr = 0x82DC592C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC592C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5934: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC5938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC593C: 4E800421  bctrl
	ctx.lr = 0x82DC5940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC5944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC5948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC594C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC5950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC5954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC5958 size=8
    let mut pc: u32 = 0x82DC5958;
    'dispatch: loop {
        match pc {
            0x82DC5958 => {
    //   block [0x82DC5958..0x82DC5960)
	// 82DC5958: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DC595C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC5960 size=60
    let mut pc: u32 = 0x82DC5960;
    'dispatch: loop {
        match pc {
            0x82DC5960 => {
    //   block [0x82DC5960..0x82DC599C)
	// 82DC5960: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DC5964: 38830030  addi r4, r3, 0x30
	ctx.r[4].s64 = ctx.r[3].s64 + 48;
	// 82DC5968: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82DC596C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DC5970: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC59A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC59A0 size=176
    let mut pc: u32 = 0x82DC59A0;
    'dispatch: loop {
        match pc {
            0x82DC59A0 => {
    //   block [0x82DC59A0..0x82DC5A50)
	// 82DC59A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC59A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC59A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC59AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC59B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC59B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC59B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC59BC: 396B08DC  addi r11, r11, 0x8dc
	ctx.r[11].s64 = ctx.r[11].s64 + 2268;
	// 82DC59C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DC59C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DC59C8: 38E0001C  li r7, 0x1c
	ctx.r[7].s64 = 28;
	// 82DC59CC: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DC59D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC59D4: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DC59D8: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DC59DC: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DC59E0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DC59E4: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82DC59E8: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC59EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC59F0: 419A0010  beq cr6, 0x82dc5a00
	if ctx.cr[6].eq {
	pc = 0x82DC5A00; continue 'dispatch;
	}
	// 82DC59F4: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC59F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC59FC: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC5A00: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5A50 size=108
    let mut pc: u32 = 0x82DC5A50;
    'dispatch: loop {
        match pc {
            0x82DC5A50 => {
    //   block [0x82DC5A50..0x82DC5ABC)
	// 82DC5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC5A58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC5A5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC5A60: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DC5A64: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5A68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC5A6C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DC5A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC5A74: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82DC5A78: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DC5A7C: 4BF91885  bl 0x82d57300
	ctx.lr = 0x82DC5A80;
	sub_82D57300(ctx, base);
	// 82DC5A80: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC5A84: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC5A88: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DC5A8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DC5A90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5A94: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC5A98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5A9C: 4E800421  bctrl
	ctx.lr = 0x82DC5AA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5AA0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DC5AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC5AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC5AAC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DC5AB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC5AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC5AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5AC0 size=216
    let mut pc: u32 = 0x82DC5AC0;
    'dispatch: loop {
        match pc {
            0x82DC5AC0 => {
    //   block [0x82DC5AC0..0x82DC5B98)
	// 82DC5AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5AC4: 4BEE3945  bl 0x82ca9408
	ctx.lr = 0x82DC5AC8;
	sub_82CA93D0(ctx, base);
	// 82DC5AC8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5ACC: 3BE30030  addi r31, r3, 0x30
	ctx.r[31].s64 = ctx.r[3].s64 + 48;
	// 82DC5AD0: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DC5AD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC5AD8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82DC5ADC: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82DC5AE0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5B98 size=436
    let mut pc: u32 = 0x82DC5B98;
    'dispatch: loop {
        match pc {
            0x82DC5B98 => {
    //   block [0x82DC5B98..0x82DC5D4C)
	// 82DC5B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5B9C: 4BEE3869  bl 0x82ca9404
	ctx.lr = 0x82DC5BA0;
	sub_82CA93D0(ctx, base);
	// 82DC5BA0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5BA4: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5BA8: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DC5BAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DC5BB0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DC5BB4: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DC5BB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5BBC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5BC0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC5BC4: 40980020  bge cr6, 0x82dc5be4
	if !ctx.cr[6].lt {
	pc = 0x82DC5BE4; continue 'dispatch;
	}
	// 82DC5BC8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC5BCC: 39293114  addi r9, r9, 0x3114
	ctx.r[9].s64 = ctx.r[9].s64 + 12564;
	// 82DC5BD0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC5BD4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC5BD8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC5BDC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC5BE0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC5BE4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DC5BE8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DC5BEC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DC5BF0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DC5BF4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DC5BF8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC5BFC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DC5C00: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DC5C04: 4200FFF0  bdnz 0x82dc5bf4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC5BF4; continue 'dispatch;
	}
	// 82DC5C08: 3BE40030  addi r31, r4, 0x30
	ctx.r[31].s64 = ctx.r[4].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5D50 size=384
    let mut pc: u32 = 0x82DC5D50;
    'dispatch: loop {
        match pc {
            0x82DC5D50 => {
    //   block [0x82DC5D50..0x82DC5ED0)
	// 82DC5D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5D54: 4BEE36B1  bl 0x82ca9404
	ctx.lr = 0x82DC5D58;
	sub_82CA93D0(ctx, base);
	// 82DC5D58: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5D5C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5D60: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DC5D64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC5D68: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DC5D6C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DC5D70: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DC5D74: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DC5D78: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5D7C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5D80: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DC5D84: 40980020  bge cr6, 0x82dc5da4
	if !ctx.cr[6].lt {
	pc = 0x82DC5DA4; continue 'dispatch;
	}
	// 82DC5D88: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DC5D8C: 39083114  addi r8, r8, 0x3114
	ctx.r[8].s64 = ctx.r[8].s64 + 12564;
	// 82DC5D90: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DC5D94: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DC5D98: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82DC5D9C: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC5DA0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DC5DA4: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DC5DA8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82DC5DAC: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82DC5DB0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82DC5DB4: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DC5DB8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC5DBC: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DC5DC0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DC5DC4: 4200FFF0  bdnz 0x82dc5db4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DC5DB4; continue 'dispatch;
	}
	// 82DC5DC8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5ED0 size=144
    let mut pc: u32 = 0x82DC5ED0;
    'dispatch: loop {
        match pc {
            0x82DC5ED0 => {
    //   block [0x82DC5ED0..0x82DC5F60)
	// 82DC5ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC5ED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC5EDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC5EE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5EE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC5EE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC5EEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC5EF0: 388B3124  addi r4, r11, 0x3124
	ctx.r[4].s64 = ctx.r[11].s64 + 12580;
	// 82DC5EF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DC5EF8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5EFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC5F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5F04: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5F08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5F0C: 4E800421  bctrl
	ctx.lr = 0x82DC5F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5F10: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5F14: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82DC5F18: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC5F1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DC5F20: 388BB3B4  addi r4, r11, -0x4c4c
	ctx.r[4].s64 = ctx.r[11].s64 + -19532;
	// 82DC5F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5F28: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC5F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5F30: 4E800421  bctrl
	ctx.lr = 0x82DC5F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5F34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DC5F3C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC5F40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5F44: 4E800421  bctrl
	ctx.lr = 0x82DC5F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5F48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DC5F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC5F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC5F54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC5F58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC5F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC5F60 size=8
    let mut pc: u32 = 0x82DC5F60;
    'dispatch: loop {
        match pc {
            0x82DC5F60 => {
    //   block [0x82DC5F60..0x82DC5F68)
	// 82DC5F60: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DC5F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC5F68 size=132
    let mut pc: u32 = 0x82DC5F68;
    'dispatch: loop {
        match pc {
            0x82DC5F68 => {
    //   block [0x82DC5F68..0x82DC5FEC)
	// 82DC5F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC5F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC5F70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC5F74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC5F78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DC5F7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC5F80: 396B0E48  addi r11, r11, 0xe48
	ctx.r[11].s64 = ctx.r[11].s64 + 3656;
	// 82DC5F84: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC5F88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC5F8C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC5F90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC5F94: 419A0030  beq cr6, 0x82dc5fc4
	if ctx.cr[6].eq {
	pc = 0x82DC5FC4; continue 'dispatch;
	}
	// 82DC5F98: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC5F9C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC5FA0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82DC5FA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DC5FA8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC5FAC: 409A0018  bne cr6, 0x82dc5fc4
	if !ctx.cr[6].eq {
	pc = 0x82DC5FC4; continue 'dispatch;
	}
	// 82DC5FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5FB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DC5FB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC5FC0: 4E800421  bctrl
	ctx.lr = 0x82DC5FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC5FC4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82DC5FC8: 4BFEBB79  bl 0x82db1b40
	ctx.lr = 0x82DC5FCC;
	sub_82DB1B40(ctx, base);
	// 82DC5FCC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DC5FD0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82DC5FD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC5FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DC5FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC5FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC5FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC5FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC5FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC5FF0 size=20
    let mut pc: u32 = 0x82DC5FF0;
    'dispatch: loop {
        match pc {
            0x82DC5FF0 => {
    //   block [0x82DC5FF0..0x82DC6004)
	// 82DC5FF0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC5FF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC5FF8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DC5FFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC6000: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC6008 size=96
    let mut pc: u32 = 0x82DC6008;
    'dispatch: loop {
        match pc {
            0x82DC6008 => {
    //   block [0x82DC6008..0x82DC6068)
	// 82DC6008: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DC600C: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82DC6010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DC6014: 396B0E48  addi r11, r11, 0xe48
	ctx.r[11].s64 = ctx.r[11].s64 + 3656;
	// 82DC6018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DC601C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DC6020: 38E0001A  li r7, 0x1a
	ctx.r[7].s64 = 26;
	// 82DC6024: 394A00E8  addi r10, r10, 0xe8
	ctx.r[10].s64 = ctx.r[10].s64 + 232;
	// 82DC6028: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DC602C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DC6030: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DC6034: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DC6038: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DC603C: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82DC6040: A1650004  lhz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6044: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6048: 419A0010  beq cr6, 0x82dc6058
	if ctx.cr[6].eq {
	pc = 0x82DC6058; continue 'dispatch;
	}
	// 82DC604C: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC6050: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC6054: B1650006  sth r11, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DC6058: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC605C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6060: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC6064: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC6068 size=16
    let mut pc: u32 = 0x82DC6068;
    'dispatch: loop {
        match pc {
            0x82DC6068 => {
    //   block [0x82DC6068..0x82DC6078)
	// 82DC6068: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DC606C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DC6070: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DC6074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6078 size=216
    let mut pc: u32 = 0x82DC6078;
    'dispatch: loop {
        match pc {
            0x82DC6078 => {
    //   block [0x82DC6078..0x82DC6150)
	// 82DC6078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC607C: 4BEE338D  bl 0x82ca9408
	ctx.lr = 0x82DC6080;
	sub_82CA93D0(ctx, base);
	// 82DC6080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6084: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6088: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DC608C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DC6090: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DC6094: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DC6098: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC609C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC60A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC60A4: 40980020  bge cr6, 0x82dc60c4
	if !ctx.cr[6].lt {
	pc = 0x82DC60C4; continue 'dispatch;
	}
	// 82DC60A8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC60AC: 3929312C  addi r9, r9, 0x312c
	ctx.r[9].s64 = ctx.r[9].s64 + 12588;
	// 82DC60B0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC60B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC60B8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC60BC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC60C0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC60C4: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC60C8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DC60CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC60D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DC60D4: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DC60D8: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC60DC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC60E0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC60E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC60E8: 4E800421  bctrl
	ctx.lr = 0x82DC60EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC60EC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DC60F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DC60F4: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DC60F8: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC60FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DC6100: 419A0014  beq cr6, 0x82dc6114
	if ctx.cr[6].eq {
	pc = 0x82DC6114; continue 'dispatch;
	}
	// 82DC6104: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DC6108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DC610C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DC6110: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82DC6114: 7D5DE02E  lwzx r10, r29, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DC6118: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC611C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC6120: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC6124: 40980020  bge cr6, 0x82dc6144
	if !ctx.cr[6].lt {
	pc = 0x82DC6144; continue 'dispatch;
	}
	// 82DC6128: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DC612C: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DC6130: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC6134: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC6138: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DC613C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC6140: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC6144: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DC6148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC614C: 4BEE330C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6150 size=196
    let mut pc: u32 = 0x82DC6150;
    'dispatch: loop {
        match pc {
            0x82DC6150 => {
    //   block [0x82DC6150..0x82DC6214)
	// 82DC6150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DC6158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DC615C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DC6160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6164: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6168: 3BE00008  li r31, 8
	ctx.r[31].s64 = 8;
	// 82DC616C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC6170: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6174: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC6178: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC617C: 40980020  bge cr6, 0x82dc619c
	if !ctx.cr[6].lt {
	pc = 0x82DC619C; continue 'dispatch;
	}
	// 82DC6180: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DC6184: 3929312C  addi r9, r9, 0x312c
	ctx.r[9].s64 = ctx.r[9].s64 + 12588;
	// 82DC6188: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC618C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC6190: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC6194: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC6198: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC619C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DC61A0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DC61A4: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82DC61A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DC61AC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DC61B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC61B4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DC61B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DC61BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC61C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DC61C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC61C8: 4E800421  bctrl
	ctx.lr = 0x82DC61CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC61CC: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC61D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC61D4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DC61D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DC61DC: 40980020  bge cr6, 0x82dc61fc
	if !ctx.cr[6].lt {
	pc = 0x82DC61FC; continue 'dispatch;
	}
	// 82DC61E0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DC61E4: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DC61E8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DC61EC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DC61F0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DC61F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DC61F8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DC61FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DC6200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DC6204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DC6208: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DC620C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DC6210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DC6218 size=8
    let mut pc: u32 = 0x82DC6218;
    'dispatch: loop {
        match pc {
            0x82DC6218 => {
    //   block [0x82DC6218..0x82DC6220)
	// 82DC6218: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82DC621C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6220 size=156
    let mut pc: u32 = 0x82DC6220;
    'dispatch: loop {
        match pc {
            0x82DC6220 => {
    //   block [0x82DC6220..0x82DC62BC)
	// 82DC6220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC6224: 4BEE31DD  bl 0x82ca9400
	ctx.lr = 0x82DC6228;
	sub_82CA93D0(ctx, base);
	// 82DC6228: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC622C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC6230: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DC6234: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6238: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC623C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DC6240: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82DC6244: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DC6248: 4198004C  blt cr6, 0x82dc6294
	if ctx.cr[6].lt {
	pc = 0x82DC6294; continue 'dispatch;
	}
	// 82DC624C: 577F103A  slwi r31, r27, 2
	ctx.r[31].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DC6250: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC6254: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC6258: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC625C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC6260: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC6264: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82DC6268: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC626C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC6274: 4E800421  bctrl
	ctx.lr = 0x82DC6278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6278: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC627C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6280: 419A0028  beq cr6, 0x82dc62a8
	if ctx.cr[6].eq {
	pc = 0x82DC62A8; continue 'dispatch;
	}
	// 82DC6284: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82DC6288: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82DC628C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82DC6290: 4098FFC0  bge cr6, 0x82dc6250
	if !ctx.cr[6].lt {
	pc = 0x82DC6250; continue 'dispatch;
	}
	// 82DC6294: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC6298: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DC629C: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC62A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC62A4: 4BEE31AC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82DC62A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC62AC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DC62B0: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC62B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DC62B8: 4BEE3198  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC62C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC62C0 size=180
    let mut pc: u32 = 0x82DC62C0;
    'dispatch: loop {
        match pc {
            0x82DC62C0 => {
    //   block [0x82DC62C0..0x82DC6374)
	// 82DC62C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC62C4: 4BEE3131  bl 0x82ca93f4
	ctx.lr = 0x82DC62C8;
	sub_82CA93D0(ctx, base);
	// 82DC62C8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC62CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC62D0: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DC62D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC62D8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC62DC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DC62E0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC62E4: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DC62E8: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DC62EC: 3B0BFFFF  addi r24, r11, -1
	ctx.r[24].s64 = ctx.r[11].s64 + -1;
	// 82DC62F0: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82DC62F4: 41980058  blt cr6, 0x82dc634c
	if ctx.cr[6].lt {
	pc = 0x82DC634C; continue 'dispatch;
	}
	// 82DC62F8: 571E103A  slwi r30, r24, 2
	ctx.r[30].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DC62FC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC6300: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82DC6304: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DC6308: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DC630C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC6310: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC6314: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC6318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC631C: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82DC6320: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6324: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DC6328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC632C: 4E800421  bctrl
	ctx.lr = 0x82DC6330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC6330: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC6334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC6338: 419A0028  beq cr6, 0x82dc6360
	if ctx.cr[6].eq {
	pc = 0x82DC6360; continue 'dispatch;
	}
	// 82DC633C: 3B18FFFF  addi r24, r24, -1
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	// 82DC6340: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82DC6344: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82DC6348: 4098FFB4  bge cr6, 0x82dc62fc
	if !ctx.cr[6].lt {
	pc = 0x82DC62FC; continue 'dispatch;
	}
	// 82DC634C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC6350: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DC6354: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC6358: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DC635C: 4BEE30E8  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
	// 82DC6360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC6364: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DC6368: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC636C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DC6370: 4BEE30D4  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6378 size=172
    let mut pc: u32 = 0x82DC6378;
    'dispatch: loop {
        match pc {
            0x82DC6378 => {
    //   block [0x82DC6378..0x82DC6424)
	// 82DC6378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC637C: 4BEE307D  bl 0x82ca93f8
	ctx.lr = 0x82DC6380;
	sub_82CA93D0(ctx, base);
	// 82DC6380: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6384: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DC6388: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DC638C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6390: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC6394: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DC6398: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC639C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DC63A0: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 82DC63A4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82DC63A8: 41980054  blt cr6, 0x82dc63fc
	if ctx.cr[6].lt {
	pc = 0x82DC63FC; continue 'dispatch;
	}
	// 82DC63AC: 573F103A  slwi r31, r25, 2
	ctx.r[31].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DC63B0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DC63B4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DC63B8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DC63BC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC63C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC63C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DC63C8: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82DC63CC: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82DC63D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC63D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC63D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC63DC: 4E800421  bctrl
	ctx.lr = 0x82DC63E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC63E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC63E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC63E8: 419A0028  beq cr6, 0x82dc6410
	if ctx.cr[6].eq {
	pc = 0x82DC6410; continue 'dispatch;
	}
	// 82DC63EC: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 82DC63F0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82DC63F4: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82DC63F8: 4098FFB8  bge cr6, 0x82dc63b0
	if !ctx.cr[6].lt {
	pc = 0x82DC63B0; continue 'dispatch;
	}
	// 82DC63FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC6400: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DC6404: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC6408: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DC640C: 4BEE303C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
	// 82DC6410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC6414: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DC6418: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC641C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DC6420: 4BEE3028  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DC6428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DC6428 size=196
    let mut pc: u32 = 0x82DC6428;
    'dispatch: loop {
        match pc {
            0x82DC6428 => {
    //   block [0x82DC6428..0x82DC64EC)
	// 82DC6428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DC642C: 4BEE2FC1  bl 0x82ca93ec
	ctx.lr = 0x82DC6430;
	sub_82CA93D0(ctx, base);
	// 82DC6430: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DC6434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DC6438: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82DC643C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DC6440: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DC6444: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DC6448: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DC644C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DC6450: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DC6454: 3ACBFFFF  addi r22, r11, -1
	ctx.r[22].s64 = ctx.r[11].s64 + -1;
	// 82DC6458: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 82DC645C: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82DC6460: 41980064  blt cr6, 0x82dc64c4
	if ctx.cr[6].lt {
	pc = 0x82DC64C4; continue 'dispatch;
	}
	// 82DC6464: 82E10124  lwz r23, 0x124(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 82DC6468: 56DE103A  slwi r30, r22, 2
	ctx.r[30].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DC646C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DC6470: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82DC6474: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82DC6478: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82DC647C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DC6480: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DC6484: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DC6488: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DC648C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DC6490: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DC6494: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82DC6498: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC649C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC64A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DC64A4: 4E800421  bctrl
	ctx.lr = 0x82DC64A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DC64A8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DC64AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DC64B0: 419A0028  beq cr6, 0x82dc64d8
	if ctx.cr[6].eq {
	pc = 0x82DC64D8; continue 'dispatch;
	}
	// 82DC64B4: 3AD6FFFF  addi r22, r22, -1
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	// 82DC64B8: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 82DC64BC: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82DC64C0: 4098FFAC  bge cr6, 0x82dc646c
	if !ctx.cr[6].lt {
	pc = 0x82DC646C; continue 'dispatch;
	}
	// 82DC64C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DC64C8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DC64CC: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC64D0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DC64D4: 4BEE2F68  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
	// 82DC64D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DC64DC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82DC64E0: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DC64E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DC64E8: 4BEE2F54  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


