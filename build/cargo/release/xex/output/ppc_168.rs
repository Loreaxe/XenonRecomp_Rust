pub fn sub_82DE1968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE1968 size=320
    let mut pc: u32 = 0x82DE1968;
    'dispatch: loop {
        match pc {
            0x82DE1968 => {
    //   block [0x82DE1968..0x82DE1AA8)
	// 82DE1968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE196C: 4BEC7A79  bl 0x82ca93e4
	ctx.lr = 0x82DE1970;
	sub_82CA93D0(ctx, base);
	// 82DE1970: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82DE1974: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1978: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DE197C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DE1980: 3AF90020  addi r23, r25, 0x20
	ctx.r[23].s64 = ctx.r[25].s64 + 32;
	// 82DE1984: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DE1988: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82DE198C: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 82DE1990: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DE1994: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82DE1998: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DE199C: 80B90010  lwz r5, 0x10(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE19A0: 4BFFFDA1  bl 0x82de1740
	ctx.lr = 0x82DE19A4;
	sub_82DE1740(ctx, base);
	// 82DE19A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE19A8: 3AA0FFFF  li r21, -1
	ctx.r[21].s64 = -1;
	// 82DE19AC: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82DE19B0: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82DE19B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE19B8: C3EB0C14  lfs f31, 0xc14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DE19BC: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DE19C0: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 82DE19C4: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 82DE19C8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DE19CC: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82DE19D0: 419A00CC  beq cr6, 0x82de1a9c
	if ctx.cr[6].eq {
	pc = 0x82DE1A9C; continue 'dispatch;
	}
	// 82DE19D4: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE19D8: 3A600010  li r19, 0x10
	ctx.r[19].s64 = 16;
	// 82DE19DC: 3A800020  li r20, 0x20
	ctx.r[20].s64 = 32;
	// 82DE19E0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DE19E4: 80990010  lwz r4, 0x10(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE19E8: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82DE19EC: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DE19F0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82DE19F4: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 82DE19F8: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82DE19FC: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 82DE1A00: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82DE1A04: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82DE1A08: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DE1A0C: 4BFFF98D  bl 0x82de1398
	ctx.lr = 0x82DE1A10;
	sub_82DE1398(ctx, base);
	// 82DE1A10: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DE1A14: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1A18: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DE1A1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE1A20: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DE1A24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1AA8 size=96
    let mut pc: u32 = 0x82DE1AA8;
    'dispatch: loop {
        match pc {
            0x82DE1AA8 => {
    //   block [0x82DE1AA8..0x82DE1B08)
	// 82DE1AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1AB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1AB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1AB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE1ABC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE1AC0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DE1AC4: 388B423C  addi r4, r11, 0x423c
	ctx.r[4].s64 = ctx.r[11].s64 + 16956;
	// 82DE1AC8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE1ACC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1AD4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1AD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE1ADC: 4E800421  bctrl
	ctx.lr = 0x82DE1AE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE1AE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1AE8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE1AEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE1AF0: 4E800421  bctrl
	ctx.lr = 0x82DE1AF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE1AF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE1AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1B00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE1B08 size=172
    let mut pc: u32 = 0x82DE1B08;
    'dispatch: loop {
        match pc {
            0x82DE1B08 => {
    //   block [0x82DE1B08..0x82DE1BB4)
	// 82DE1B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE1B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1B18: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1B1C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DE1B20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1B24: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DE1B28: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE1B2C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE1B30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DE1B34: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82DE1B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE1B3C: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE1B40: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE1B44: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82DE1B48: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DE1B4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE1B50: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 82DE1B54: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DE1B58: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82DE1B5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE1B60: 4E800421  bctrl
	ctx.lr = 0x82DE1B64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE1B64: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE1B68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE1B6C: 419A0030  beq cr6, 0x82de1b9c
	if ctx.cr[6].eq {
	pc = 0x82DE1B9C; continue 'dispatch;
	}
	// 82DE1B70: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DE1B74: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE1B78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DE1B7C: 4BF74945  bl 0x82d564c0
	ctx.lr = 0x82DE1B80;
	sub_82D564C0(ctx, base);
	// 82DE1B80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1B84: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DE1B88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE1B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1B90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1B94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE1B98: 4E800421  bctrl
	ctx.lr = 0x82DE1B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE1B9C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DE1BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE1BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1BA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE1BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1BB8 size=360
    let mut pc: u32 = 0x82DE1BB8;
    'dispatch: loop {
        match pc {
            0x82DE1BB8 => {
    //   block [0x82DE1BB8..0x82DE1D20)
	// 82DE1BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1BC0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1BC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1BC8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DE1BCC: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82DE1BD0: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82DE1BD4: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DE1BD8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1D20 size=24
    let mut pc: u32 = 0x82DE1D20;
    'dispatch: loop {
        match pc {
            0x82DE1D20 => {
    //   block [0x82DE1D20..0x82DE1D38)
	// 82DE1D20: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1D24: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82DE1D28: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1D2C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DE1D30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE1D34: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1D38 size=80
    let mut pc: u32 = 0x82DE1D38;
    'dispatch: loop {
        match pc {
            0x82DE1D38 => {
    //   block [0x82DE1D38..0x82DE1D88)
	// 82DE1D38: 39030010  addi r8, r3, 0x10
	ctx.r[8].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE1D88 size=176
    let mut pc: u32 = 0x82DE1D88;
    'dispatch: loop {
        match pc {
            0x82DE1D88 => {
    //   block [0x82DE1D88..0x82DE1E38)
	// 82DE1D88: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE1D8C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE1D90: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 82DE1D94: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE1D98: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DE1D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DE1DA0: C00A0BFC  lfs f0, 0xbfc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE1DA4: 392923C4  addi r9, r9, 0x23c4
	ctx.r[9].s64 = ctx.r[9].s64 + 9156;
	// 82DE1DA8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DE1DAC: 3BE0001B  li r31, 0x1b
	ctx.r[31].s64 = 27;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1E38 size=108
    let mut pc: u32 = 0x82DE1E38;
    'dispatch: loop {
        match pc {
            0x82DE1E38 => {
    //   block [0x82DE1E38..0x82DE1EA4)
	// 82DE1E38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE1E3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE1E40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE1E44: 3900001B  li r8, 0x1b
	ctx.r[8].s64 = 27;
	// 82DE1E48: 396B23C4  addi r11, r11, 0x23c4
	ctx.r[11].s64 = ctx.r[11].s64 + 9156;
	// 82DE1E4C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82DE1E50: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DE1E54: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82DE1E58: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DE1E5C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DE1E60: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DE1E64: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82DE1E68: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1EA8 size=24
    let mut pc: u32 = 0x82DE1EA8;
    'dispatch: loop {
        match pc {
            0x82DE1EA8 => {
    //   block [0x82DE1EA8..0x82DE1EC0)
	// 82DE1EA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1EAC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DE1EB0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DE1EB4: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82DE1EB8: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82DE1EBC: 4B4B7F34  b 0x82299df0
	sub_82299DF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1EC0 size=408
    let mut pc: u32 = 0x82DE1EC0;
    'dispatch: loop {
        match pc {
            0x82DE1EC0 => {
    //   block [0x82DE1EC0..0x82DE2058)
	// 82DE1EC0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE1EC4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1EC8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DE1ECC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE1ED0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1ED4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1ED8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE1EDC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DE1EE0: 40980020  bge cr6, 0x82de1f00
	if !ctx.cr[6].lt {
	pc = 0x82DE1F00; continue 'dispatch;
	}
	// 82DE1EE4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DE1EE8: 39084248  addi r8, r8, 0x4248
	ctx.r[8].s64 = ctx.r[8].s64 + 16968;
	// 82DE1EEC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE1EF0: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DE1EF4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 82DE1EF8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE1EFC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DE1F00: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2058 size=8
    let mut pc: u32 = 0x82DE2058;
    'dispatch: loop {
        match pc {
            0x82DE2058 => {
    //   block [0x82DE2058..0x82DE2060)
	// 82DE2058: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82DE205C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2060 size=8
    let mut pc: u32 = 0x82DE2060;
    'dispatch: loop {
        match pc {
            0x82DE2060 => {
    //   block [0x82DE2060..0x82DE2068)
	// 82DE2060: 5463C23E  srwi r3, r3, 8
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(8);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DE2064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2068 size=28
    let mut pc: u32 = 0x82DE2068;
    'dispatch: loop {
        match pc {
            0x82DE2068 => {
    //   block [0x82DE2068..0x82DE2084)
	// 82DE2068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE206C: 548A063E  clrlwi r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82DE2070: 99630003  stb r11, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82DE2074: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2078: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82DE207C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE2080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2088 size=20
    let mut pc: u32 = 0x82DE2088;
    'dispatch: loop {
        match pc {
            0x82DE2088 => {
    //   block [0x82DE2088..0x82DE209C)
	// 82DE2088: 89630003  lbz r11, 3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE208C: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE2090: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DE2094: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE2098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE20A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE20A0 size=100
    let mut pc: u32 = 0x82DE20A0;
    'dispatch: loop {
        match pc {
            0x82DE20A0 => {
    //   block [0x82DE20A0..0x82DE2104)
	// 82DE20A0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE20A4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE20A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DE20AC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82DE20B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE20B4: 40990048  ble cr6, 0x82de20fc
	if !ctx.cr[6].gt {
	pc = 0x82DE20FC; continue 'dispatch;
	}
	// 82DE20B8: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82DE20BC: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE20C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DE20C4: 80E40020  lwz r7, 0x20(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE20C8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DE20CC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DE20D0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE20D4: 5508C23E  srwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DE20D8: 7CC838AE  lbzx r6, r8, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DE20DC: 98AB0003  stb r5, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 82DE20E0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE20E4: 7CC6FB78  or r6, r6, r31
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[31].u64;
	// 82DE20E8: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82DE20EC: 7CA839AE  stbx r5, r8, r7
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), ctx.r[5].u8) };
	// 82DE20F0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE20F4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DE20F8: 4198FFC4  blt cr6, 0x82de20bc
	if ctx.cr[6].lt {
	pc = 0x82DE20BC; continue 'dispatch;
	}
	// 82DE20FC: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82DE2100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2108 size=16
    let mut pc: u32 = 0x82DE2108;
    'dispatch: loop {
        match pc {
            0x82DE2108 => {
    //   block [0x82DE2108..0x82DE2118)
	// 82DE2108: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE210C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE2110: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE2114: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2118 size=52
    let mut pc: u32 = 0x82DE2118;
    'dispatch: loop {
        match pc {
            0x82DE2118 => {
    //   block [0x82DE2118..0x82DE214C)
	// 82DE2118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE211C: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE2120: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE2124: 81040020  lwz r8, 0x20(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE2128: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE212C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DE2130: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82DE2134: 5529C23E  srwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DE2138: 7CE941AE  stbx r7, r9, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u8) };
	// 82DE213C: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE2140: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE2144: 4198FFD8  blt cr6, 0x82de211c
	if ctx.cr[6].lt {
	pc = 0x82DE211C; continue 'dispatch;
	}
	// 82DE2148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2150 size=80
    let mut pc: u32 = 0x82DE2150;
    'dispatch: loop {
        match pc {
            0x82DE2150 => {
    //   block [0x82DE2150..0x82DE21A0)
	// 82DE2150: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE2154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE2158: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE215C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE2160: 40990024  ble cr6, 0x82de2184
	if !ctx.cr[6].gt {
	pc = 0x82DE2184; continue 'dispatch;
	}
	// 82DE2164: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2168: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE216C: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82DE2170: 419A0018  beq cr6, 0x82de2188
	if ctx.cr[6].eq {
	pc = 0x82DE2188; continue 'dispatch;
	}
	// 82DE2174: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE2178: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DE217C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE2180: 4198FFE8  blt cr6, 0x82de2168
	if ctx.cr[6].lt {
	pc = 0x82DE2168; continue 'dispatch;
	}
	// 82DE2184: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DE2188: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DE218C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DE2190: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DE2194: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DE2198: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DE219C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE21A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE21A0 size=380
    let mut pc: u32 = 0x82DE21A0;
    'dispatch: loop {
        match pc {
            0x82DE21A0 => {
    //   block [0x82DE21A0..0x82DE231C)
	// 82DE21A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE21A4: 4BEC7251  bl 0x82ca93f4
	ctx.lr = 0x82DE21A8;
	sub_82CA93D0(ctx, base);
	// 82DE21A8: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82DE21AC: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE21B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE21B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE21B8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE21BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE21C0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82DE21C4: 396B2454  addi r11, r11, 0x2454
	ctx.r[11].s64 = ctx.r[11].s64 + 9300;
	// 82DE21C8: 394A2474  addi r10, r10, 0x2474
	ctx.r[10].s64 = ctx.r[10].s64 + 9332;
	// 82DE21CC: 39292464  addi r9, r9, 0x2464
	ctx.r[9].s64 = ctx.r[9].s64 + 9316;
	// 82DE21D0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DE21D4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DE21D8: B39F0006  sth r28, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82DE21DC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DE21E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DE21E4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE21E8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DE21EC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DE21F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DE21F4: 933F000C  stw r25, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 82DE21F8: 3AFF0008  addi r23, r31, 8
	ctx.r[23].s64 = ctx.r[31].s64 + 8;
	// 82DE21FC: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 82DE2200: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DE2204: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE2208: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE220C: 419A0100  beq cr6, 0x82de230c
	if ctx.cr[6].eq {
	pc = 0x82DE230C; continue 'dispatch;
	}
	// 82DE2210: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2214: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82DE2218: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE221C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE2220: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2224: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE2228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE222C: 4E800421  bctrl
	ctx.lr = 0x82DE2230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE2230: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82DE2234: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DE2238: 3B4B6E60  addi r26, r11, 0x6e60
	ctx.r[26].s64 = ctx.r[11].s64 + 28256;
	// 82DE223C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE2240: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DE2244: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DE2248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE224C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DE2250: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE2254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2258: 4E800421  bctrl
	ctx.lr = 0x82DE225C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE225C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE2260: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82DE2264: 40990090  ble cr6, 0x82de22f4
	if !ctx.cr[6].gt {
	pc = 0x82DE22F4; continue 'dispatch;
	}
	// 82DE2268: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82DE226C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2270: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82DE2274: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE227C: 7C9B582E  lwzx r4, r27, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE2280: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE2284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2288: 4E800421  bctrl
	ctx.lr = 0x82DE228C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE228C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2290: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DE2294: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DE2298: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DE229C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE22A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE22A4: 4E800421  bctrl
	ctx.lr = 0x82DE22A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE22A8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DE22AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DE22B0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DE22B4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE2320 size=140
    let mut pc: u32 = 0x82DE2320;
    'dispatch: loop {
        match pc {
            0x82DE2320 => {
    //   block [0x82DE2320..0x82DE23AC)
	// 82DE2320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE2324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE2328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE232C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE2330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE2334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE2338: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DE233C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE2340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE2344: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE2348: 396B2454  addi r11, r11, 0x2454
	ctx.r[11].s64 = ctx.r[11].s64 + 9300;
	// 82DE234C: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DE2350: 394A2474  addi r10, r10, 0x2474
	ctx.r[10].s64 = ctx.r[10].s64 + 9332;
	// 82DE2354: 39292464  addi r9, r9, 0x2464
	ctx.r[9].s64 = ctx.r[9].s64 + 9316;
	// 82DE2358: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE235C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DE2360: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DE2364: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DE2368: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DE236C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE2370: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DE2374: 38BF0008  addi r5, r31, 8
	ctx.r[5].s64 = ctx.r[31].s64 + 8;
	// 82DE2378: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82DE237C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82DE2380: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82DE2384: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82DE2388: 48042499  bl 0x82e24820
	ctx.lr = 0x82DE238C;
	sub_82E24820(ctx, base);
	// 82DE238C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE2390: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82DE2394: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE2398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE239C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE23A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE23A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE23A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE23B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE23B0 size=132
    let mut pc: u32 = 0x82DE23B0;
    'dispatch: loop {
        match pc {
            0x82DE23B0 => {
    //   block [0x82DE23B0..0x82DE2434)
	// 82DE23B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE23B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE23B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE23BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE23C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE23C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE23C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE23CC: 396B2474  addi r11, r11, 0x2474
	ctx.r[11].s64 = ctx.r[11].s64 + 9332;
	// 82DE23D0: 394A2464  addi r10, r10, 0x2464
	ctx.r[10].s64 = ctx.r[10].s64 + 9316;
	// 82DE23D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE23D8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DE23DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE23E0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DE23E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE23E8: 409A0020  bne cr6, 0x82de2408
	if !ctx.cr[6].eq {
	pc = 0x82DE2408; continue 'dispatch;
	}
	// 82DE23EC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE23F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DE23F4: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DE23F8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE23FC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DE2400: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE2404: 4BF72EC5  bl 0x82d552c8
	ctx.lr = 0x82DE2408;
	sub_82D552C8(ctx, base);
	// 82DE2408: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE240C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DE2410: 396B2454  addi r11, r11, 0x2454
	ctx.r[11].s64 = ctx.r[11].s64 + 9300;
	// 82DE2414: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DE2418: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DE241C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE2420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE2424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE2428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE242C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE2430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE2438 size=132
    let mut pc: u32 = 0x82DE2438;
    'dispatch: loop {
        match pc {
            0x82DE2438 => {
    //   block [0x82DE2438..0x82DE24BC)
	// 82DE2438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE2440: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE2444: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE2448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE244C: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82DE2450: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE2454: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE2458: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE245C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DE2460: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DE2464: 409A0010  bne cr6, 0x82de2474
	if !ctx.cr[6].eq {
	pc = 0x82DE2474; continue 'dispatch;
	}
	// 82DE2468: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DE246C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE2470: 4BF74B29  bl 0x82d56f98
	ctx.lr = 0x82DE2474;
	sub_82D56F98(ctx, base);
	// 82DE2474: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE2478: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DE247C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2480: 57C7402E  slwi r7, r30, 8
	ctx.r[7].u32 = ctx.r[30].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DE2484: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE2488: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82DE248C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DE2490: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82DE2494: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82DE2498: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82DE249C: 990B0003  stb r8, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[8].u8 ) };
	// 82DE24A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE24A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE24A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE24AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE24B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE24B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE24B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE24C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE24C0 size=40
    let mut pc: u32 = 0x82DE24C0;
    'dispatch: loop {
        match pc {
            0x82DE24C0 => {
    //   block [0x82DE24C0..0x82DE24E8)
	// 82DE24C0: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE24C4: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 82DE24C8: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82DE24CC: 5469043E  clrlwi r9, r3, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DE24D0: 7D4A5830  slw r10, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DE24D4: 7CAB5830  slw r11, r5, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[5].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DE24D8: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82DE24DC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82DE24E0: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DE24E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE24E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE24E8 size=308
    let mut pc: u32 = 0x82DE24E8;
    'dispatch: loop {
        match pc {
            0x82DE24E8 => {
    //   block [0x82DE24E8..0x82DE261C)
	// 82DE24E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE24EC: 4BEC6F11  bl 0x82ca93fc
	ctx.lr = 0x82DE24F0;
	sub_82CA93D0(ctx, base);
	// 82DE24F0: 3D605555  lis r11, 0x5555
	ctx.r[11].s64 = 1431633920;
	// 82DE24F4: 39250001  addi r9, r5, 1
	ctx.r[9].s64 = ctx.r[5].s64 + 1;
	// 82DE24F8: 616B5556  ori r11, r11, 0x5556
	ctx.r[11].u64 = ctx.r[11].u64 | 21846;
	// 82DE24FC: 39050002  addi r8, r5, 2
	ctx.r[8].s64 = ctx.r[5].s64 + 2;
	// 82DE2500: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DE2504: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DE2508: 7D655896  mulhw r11, r5, r11
	ctx.r[11].s64 = ((ctx.r[5].s32 as i64 * ctx.r[11].s32 as i64) >> 32);
	// 82DE250C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DE2510: 7C89F896  mulhw r4, r9, r31
	ctx.r[4].s64 = ((ctx.r[9].s32 as i64 * ctx.r[31].s32 as i64) >> 32);
	// 82DE2514: 7FE8F096  mulhw r31, r8, r30
	ctx.r[31].s64 = ((ctx.r[8].s32 as i64 * ctx.r[30].s32 as i64) >> 32);
	// 82DE2518: 557E0FFE  srwi r30, r11, 0x1f
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE251C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DE2520: 3B400010  li r26, 0x10
	ctx.r[26].s64 = 16;
	// 82DE2524: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DE2528: 549E0FFE  srwi r30, r4, 0x1f
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shr(31);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE252C: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 82DE2530: 7C84F214  add r4, r4, r30
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[30].u64;
	// 82DE2534: 57FE0FFE  srwi r30, r31, 0x1f
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shr(31);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE2538: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82DE253C: 7FFFF214  add r31, r31, r30
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82DE2540: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DE2544: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DE2548: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DE254C: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82DE2550: 7D6B2850  subf r11, r11, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82DE2554: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2620 size=108
    let mut pc: u32 = 0x82DE2620;
    'dispatch: loop {
        match pc {
            0x82DE2620 => {
    //   block [0x82DE2620..0x82DE268C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE2690 size=352
    let mut pc: u32 = 0x82DE2690;
    'dispatch: loop {
        match pc {
            0x82DE2690 => {
    //   block [0x82DE2690..0x82DE27F0)
	// 82DE2690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE2694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE2698: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE269C: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE27F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE27F0 size=148
    let mut pc: u32 = 0x82DE27F0;
    'dispatch: loop {
        match pc {
            0x82DE27F0 => {
    //   block [0x82DE27F0..0x82DE2884)
	// 82DE27F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE27F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE27F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE27FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE2800: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82DE2804: 4BFFFE8D  bl 0x82de2690
	ctx.lr = 0x82DE2808;
	sub_82DE2690(ctx, base);
	// 82DE2808: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE280C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE2810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE2814: C18A0B40  lfs f12, 0xb40(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2880 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE2818: C1AB303C  lfs f13, 0x303c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12348 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE281C: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82DE2820: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DE2824: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DE2828: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DE282C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DE2830: EC006378  fmsubs f0, f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DE2834: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82DE2838: 40990020  ble cr6, 0x82de2858
	if !ctx.cr[6].gt {
	pc = 0x82DE2858; continue 'dispatch;
	}
	// 82DE283C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DE2840: 2F03001F  cmpwi cr6, r3, 0x1f
	ctx.cr[6].compare_i32(ctx.r[3].s32, 31, &mut ctx.xer);
	// 82DE2844: 4198FFD8  blt cr6, 0x82de281c
	if ctx.cr[6].lt {
	pc = 0x82DE281C; continue 'dispatch;
	}
	// 82DE2848: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE284C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE2850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE2854: 4E800020  blr
	return;
	// 82DE2858: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE285C: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE2860: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82DE2864: 40990010  ble cr6, 0x82de2874
	if !ctx.cr[6].gt {
	pc = 0x82DE2874; continue 'dispatch;
	}
	// 82DE2868: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82DE286C: 419A0008  beq cr6, 0x82de2874
	if ctx.cr[6].eq {
	pc = 0x82DE2874; continue 'dispatch;
	}
	// 82DE2870: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82DE2874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE2878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE287C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE2880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE2888 size=484
    let mut pc: u32 = 0x82DE2888;
    'dispatch: loop {
        match pc {
            0x82DE2888 => {
    //   block [0x82DE2888..0x82DE2A6C)
	// 82DE2888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE288C: 4BEC6B69  bl 0x82ca93f4
	ctx.lr = 0x82DE2890;
	sub_82CA93D0(ctx, base);
	// 82DE2890: 9421FAB0  stwu r1, -0x550(r1)
	ea = ctx.r[1].u32.wrapping_add(-1360 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE2894: 390100FC  addi r8, r1, 0xfc
	ctx.r[8].s64 = ctx.r[1].s64 + 252;
	// 82DE2898: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE289C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE28A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE28A4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DE28A8: 910100F0  stw r8, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[8].u32 ) };
	// 82DE28AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DE28B0: C1AA0C18  lfs f13, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE28B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE28B8: C00B0A64  lfs f0, 0xa64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2660 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE28BC: 3D605555  lis r11, 0x5555
	ctx.r[11].s64 = 1431633920;
	// 82DE28C0: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DE28C4: 3B440020  addi r26, r4, 0x20
	ctx.r[26].s64 = ctx.r[4].s64 + 32;
	// 82DE28C8: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DE28CC: 61695556  ori r9, r11, 0x5556
	ctx.r[9].u64 = ctx.r[11].u64 | 21846;
	// 82DE28D0: 910100F4  stw r8, 0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[8].u32 ) };
	// 82DE28D4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82DE28D8: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DE28DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DE28E0: C00A0AD4  lfs f0, 0xad4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2772 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE28E4: 61080080  ori r8, r8, 0x80
	ctx.r[8].u64 = ctx.r[8].u64 | 128;
	// 82DE28E8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE28EC: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DE28F0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DE28F4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82DE28F8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DE28FC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82DE2900: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DE2904: 38A100F0  addi r5, r1, 0xf0
	ctx.r[5].s64 = ctx.r[1].s64 + 240;
	// 82DE2908: 910100F8  stw r8, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[8].u32 ) };
	// 82DE290C: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DE2910: 810A002C  lwz r8, 0x2c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DE2914: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DE2918: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE2A70 size=184
    let mut pc: u32 = 0x82DE2A70;
    'dispatch: loop {
        match pc {
            0x82DE2A70 => {
    //   block [0x82DE2A70..0x82DE2B28)
	// 82DE2A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE2A74: 4BEC6991  bl 0x82ca9404
	ctx.lr = 0x82DE2A78;
	sub_82CA93D0(ctx, base);
	// 82DE2A78: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE2A7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DE2A80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DE2A84: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DE2A88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE2A8C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2A90: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE2A94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2A98: 4E800421  bctrl
	ctx.lr = 0x82DE2A9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE2A9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2AA0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DE2AA4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DE2AA8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE2AAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2AB0: 4E800421  bctrl
	ctx.lr = 0x82DE2AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE2AB4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DE2AB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE2ABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE2AC0: 38DE0040  addi r6, r30, 0x40
	ctx.r[6].s64 = ctx.r[30].s64 + 64;
	// 82DE2AC4: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82DE2AC8: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82DE2ACC: C02BB384  lfs f1, -0x4c7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19580 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DE2AD0: 4BFD82E9  bl 0x82dbadb8
	ctx.lr = 0x82DE2AD4;
	sub_82DBADB8(ctx, base);
	// 82DE2AD4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE2ADC: 419A0010  beq cr6, 0x82de2aec
	if ctx.cr[6].eq {
	pc = 0x82DE2AEC; continue 'dispatch;
	}
	// 82DE2AE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE2AE4: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82DE2AE8: 4BEC696C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82DE2AEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE2AF0: B3E10050  sth r31, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u16 ) };
	// 82DE2AF4: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82DE2AF8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DE2AFC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DE2B00: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DE2B04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE2B08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE2B0C: 4BFFFD7D  bl 0x82de2888
	ctx.lr = 0x82DE2B10;
	sub_82DE2888(ctx, base);
	// 82DE2B10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DE2B14: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 82DE2B18: 4198FFDC  blt cr6, 0x82de2af4
	if ctx.cr[6].lt {
	pc = 0x82DE2AF4; continue 'dispatch;
	}
	// 82DE2B1C: A0610050  lhz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE2B20: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82DE2B24: 4BEC6930  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2B28 size=4
    let mut pc: u32 = 0x82DE2B28;
    'dispatch: loop {
        match pc {
            0x82DE2B28 => {
    //   block [0x82DE2B28..0x82DE2B2C)
	// 82DE2B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2B30 size=4
    let mut pc: u32 = 0x82DE2B30;
    'dispatch: loop {
        match pc {
            0x82DE2B30 => {
    //   block [0x82DE2B30..0x82DE2B34)
	// 82DE2B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2B38 size=28
    let mut pc: u32 = 0x82DE2B38;
    'dispatch: loop {
        match pc {
            0x82DE2B38 => {
    //   block [0x82DE2B38..0x82DE2B54)
	// 82DE2B38: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE2B3C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82DE2B40: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2B44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2B48: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE2B4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2B50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2B58 size=40
    let mut pc: u32 = 0x82DE2B58;
    'dispatch: loop {
        match pc {
            0x82DE2B58 => {
    //   block [0x82DE2B58..0x82DE2B80)
	// 82DE2B58: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE2B5C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DE2B60: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DE2B64: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DE2B68: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82DE2B6C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2B70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2B74: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE2B78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2B7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2B80 size=40
    let mut pc: u32 = 0x82DE2B80;
    'dispatch: loop {
        match pc {
            0x82DE2B80 => {
    //   block [0x82DE2B80..0x82DE2BA8)
	// 82DE2B80: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82DE2B84: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE2B88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE2B8C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82DE2B90: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82DE2B94: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2B98: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2B9C: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE2BA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2BA4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2BA8 size=28
    let mut pc: u32 = 0x82DE2BA8;
    'dispatch: loop {
        match pc {
            0x82DE2BA8 => {
    //   block [0x82DE2BA8..0x82DE2BC4)
	// 82DE2BA8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE2BAC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DE2BB0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2BB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2BB8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE2BBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2BC0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2BC8 size=24
    let mut pc: u32 = 0x82DE2BC8;
    'dispatch: loop {
        match pc {
            0x82DE2BC8 => {
    //   block [0x82DE2BC8..0x82DE2BE0)
	// 82DE2BC8: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2BCC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DE2BD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2BD4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DE2BD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2BDC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2BE0 size=28
    let mut pc: u32 = 0x82DE2BE0;
    'dispatch: loop {
        match pc {
            0x82DE2BE0 => {
    //   block [0x82DE2BE0..0x82DE2BFC)
	// 82DE2BE0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE2BE4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DE2BE8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2BEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2BF0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DE2BF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2BF8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2C00 size=28
    let mut pc: u32 = 0x82DE2C00;
    'dispatch: loop {
        match pc {
            0x82DE2C00 => {
    //   block [0x82DE2C00..0x82DE2C1C)
	// 82DE2C00: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE2C04: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DE2C08: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C10: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE2C14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2C18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2C20 size=28
    let mut pc: u32 = 0x82DE2C20;
    'dispatch: loop {
        match pc {
            0x82DE2C20 => {
    //   block [0x82DE2C20..0x82DE2C3C)
	// 82DE2C20: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE2C24: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DE2C28: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C30: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE2C34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2C38: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE2C40 size=180
    let mut pc: u32 = 0x82DE2C40;
    'dispatch: loop {
        match pc {
            0x82DE2C40 => {
    //   block [0x82DE2C40..0x82DE2CF4)
	// 82DE2C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE2C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE2C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE2C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE2C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE2C54: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE2C58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE2C5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE2C60: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE2C64: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C68: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE2C6C: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE2C70: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C74: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C78: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2C7C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE2C80: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE2C84: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE2C88: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DE2C8C: 390905A0  addi r8, r9, 0x5a0
	ctx.r[8].s64 = ctx.r[9].s64 + 1440;
	// 82DE2C90: 409A0008  bne cr6, 0x82de2c98
	if !ctx.cr[6].eq {
	pc = 0x82DE2C98; continue 'dispatch;
	}
	// 82DE2C94: 390901A0  addi r8, r9, 0x1a0
	ctx.r[8].s64 = ctx.r[9].s64 + 416;
	// 82DE2C98: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE2C9C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DE2CA0: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE2CA4: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DE2CA8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE2CAC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE2CB0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DE2CB4: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DE2CB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2CBC: 4E800421  bctrl
	ctx.lr = 0x82DE2CC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE2CC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE2CC4: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 82DE2CC8: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 82DE2CCC: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82DE2CD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE2CD4: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82DE2CD8: 993F0002  stb r9, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[9].u8 ) };
	// 82DE2CDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE2CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE2CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE2CE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE2CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE2CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE2CF8 size=80
    let mut pc: u32 = 0x82DE2CF8;
    'dispatch: loop {
        match pc {
            0x82DE2CF8 => {
    //   block [0x82DE2CF8..0x82DE2D48)
	// 82DE2CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE2CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE2D00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE2D04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE2D08: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DE2D0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE2D10: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2D14: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE2D18: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE2D1C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2D20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2D24: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE2D28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2D2C: 4E800421  bctrl
	ctx.lr = 0x82DE2D30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE2D30: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DE2D34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE2D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE2D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE2D40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE2D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE2D48 size=184
    let mut pc: u32 = 0x82DE2D48;
    'dispatch: loop {
        match pc {
            0x82DE2D48 => {
    //   block [0x82DE2D48..0x82DE2E00)
	// 82DE2D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE2D4C: 4BEC66B5  bl 0x82ca9400
	ctx.lr = 0x82DE2D50;
	sub_82CA93D0(ctx, base);
	// 82DE2D50: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE2D54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE2D58: 3FE082DE  lis r31, -0x7d22
	ctx.r[31].s64 = -2099380224;
	// 82DE2D5C: 3CE082DE  lis r7, -0x7d22
	ctx.r[7].s64 = -2099380224;
	// 82DE2D60: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DE2D64: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DE2D68: 99610081  stb r11, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 82DE2D6C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DE2D70: 99610082  stb r11, 0x82(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[11].u8 ) };
	// 82DE2D74: 3F4082DE  lis r26, -0x7d22
	ctx.r[26].s64 = -2099380224;
	// 82DE2D78: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82DE2D7C: 3F6082DE  lis r27, -0x7d22
	ctx.r[27].s64 = -2099380224;
	// 82DE2D80: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DE2D84: 397F2B38  addi r11, r31, 0x2b38
	ctx.r[11].s64 = ctx.r[31].s64 + 11064;
	// 82DE2D88: 3F8082DE  lis r28, -0x7d22
	ctx.r[28].s64 = -2099380224;
	// 82DE2D8C: 3FA082DE  lis r29, -0x7d22
	ctx.r[29].s64 = -2099380224;
	// 82DE2D90: 3FC082DE  lis r30, -0x7d22
	ctx.r[30].s64 = -2099380224;
	// 82DE2D94: 3B5A2C00  addi r26, r26, 0x2c00
	ctx.r[26].s64 = ctx.r[26].s64 + 11264;
	// 82DE2D98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DE2D9C: 39672B58  addi r11, r7, 0x2b58
	ctx.r[11].s64 = ctx.r[7].s64 + 11096;
	// 82DE2DA0: 3B7B2C20  addi r27, r27, 0x2c20
	ctx.r[27].s64 = ctx.r[27].s64 + 11296;
	// 82DE2DA4: 3B9C2BE0  addi r28, r28, 0x2be0
	ctx.r[28].s64 = ctx.r[28].s64 + 11232;
	// 82DE2DA8: 3BBD2C40  addi r29, r29, 0x2c40
	ctx.r[29].s64 = ctx.r[29].s64 + 11328;
	// 82DE2DAC: 3BDE2CF8  addi r30, r30, 0x2cf8
	ctx.r[30].s64 = ctx.r[30].s64 + 11512;
	// 82DE2DB0: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 82DE2DB4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DE2DB8: 39682B80  addi r11, r8, 0x2b80
	ctx.r[11].s64 = ctx.r[8].s64 + 11136;
	// 82DE2DBC: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DE2DC0: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82DE2DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DE2DC8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82DE2DCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE2DD0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82DE2DD4: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82DE2DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82DE2DDC: 39692BA8  addi r11, r9, 0x2ba8
	ctx.r[11].s64 = ctx.r[9].s64 + 11176;
	// 82DE2DE0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DE2DE4: 396A2BC8  addi r11, r10, 0x2bc8
	ctx.r[11].s64 = ctx.r[10].s64 + 11208;
	// 82DE2DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82DE2DEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE2DF0: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DE2DF4: 4BFDE555  bl 0x82dc1348
	ctx.lr = 0x82DE2DF8;
	sub_82DC1348(ctx, base);
	// 82DE2DF8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DE2DFC: 4BEC6654  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E00 size=4
    let mut pc: u32 = 0x82DE2E00;
    'dispatch: loop {
        match pc {
            0x82DE2E00 => {
    //   block [0x82DE2E00..0x82DE2E04)
	// 82DE2E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E08 size=4
    let mut pc: u32 = 0x82DE2E08;
    'dispatch: loop {
        match pc {
            0x82DE2E08 => {
    //   block [0x82DE2E08..0x82DE2E0C)
	// 82DE2E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E10 size=4
    let mut pc: u32 = 0x82DE2E10;
    'dispatch: loop {
        match pc {
            0x82DE2E10 => {
    //   block [0x82DE2E10..0x82DE2E14)
	// 82DE2E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E18 size=4
    let mut pc: u32 = 0x82DE2E18;
    'dispatch: loop {
        match pc {
            0x82DE2E18 => {
    //   block [0x82DE2E18..0x82DE2E1C)
	// 82DE2E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E20 size=4
    let mut pc: u32 = 0x82DE2E20;
    'dispatch: loop {
        match pc {
            0x82DE2E20 => {
    //   block [0x82DE2E20..0x82DE2E24)
	// 82DE2E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E28 size=8
    let mut pc: u32 = 0x82DE2E28;
    'dispatch: loop {
        match pc {
            0x82DE2E28 => {
    //   block [0x82DE2E28..0x82DE2E30)
	// 82DE2E28: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE2E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E30 size=12
    let mut pc: u32 = 0x82DE2E30;
    'dispatch: loop {
        match pc {
            0x82DE2E30 => {
    //   block [0x82DE2E30..0x82DE2E3C)
	// 82DE2E30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DE2E34: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DE2E38: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E3C size=8
    let mut pc: u32 = 0x82DE2E3C;
    'dispatch: loop {
        match pc {
            0x82DE2E3C => {
    //   block [0x82DE2E3C..0x82DE2E44)
	// 82DE2E3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE2E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E48 size=8
    let mut pc: u32 = 0x82DE2E48;
    'dispatch: loop {
        match pc {
            0x82DE2E48 => {
    //   block [0x82DE2E48..0x82DE2E50)
	// 82DE2E48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE2E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E50 size=20
    let mut pc: u32 = 0x82DE2E50;
    'dispatch: loop {
        match pc {
            0x82DE2E50 => {
    //   block [0x82DE2E50..0x82DE2E64)
	// 82DE2E50: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE2E54: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82DE2E58: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DE2E5C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DE2E60: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E64 size=8
    let mut pc: u32 = 0x82DE2E64;
    'dispatch: loop {
        match pc {
            0x82DE2E64 => {
    //   block [0x82DE2E64..0x82DE2E6C)
	// 82DE2E64: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE2E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E70 size=36
    let mut pc: u32 = 0x82DE2E70;
    'dispatch: loop {
        match pc {
            0x82DE2E70 => {
    //   block [0x82DE2E70..0x82DE2E94)
	// 82DE2E70: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE2E74: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE2E78: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE2E7C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE2E80: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE2E84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2E88: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE2E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2E90: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2E98 size=16
    let mut pc: u32 = 0x82DE2E98;
    'dispatch: loop {
        match pc {
            0x82DE2E98 => {
    //   block [0x82DE2E98..0x82DE2EA8)
	// 82DE2E98: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2EA8 size=28
    let mut pc: u32 = 0x82DE2EA8;
    'dispatch: loop {
        match pc {
            0x82DE2EA8 => {
    //   block [0x82DE2EA8..0x82DE2EC4)
	// 82DE2EA8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DE2EAC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DE2EB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE2EB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE2EB8: 810A001C  lwz r8, 0x1c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DE2EBC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DE2EC0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2EC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE2EC4 size=52
    let mut pc: u32 = 0x82DE2EC4;
    'dispatch: loop {
        match pc {
            0x82DE2EC4 => {
    //   block [0x82DE2EC4..0x82DE2EF8)
	// 82DE2EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DE2EC8: 80EA0018  lwz r7, 0x18(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE2ECC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE2EF8 size=172
    let mut pc: u32 = 0x82DE2EF8;
    'dispatch: loop {
        match pc {
            0x82DE2EF8 => {
    //   block [0x82DE2EF8..0x82DE2FA4)
	// 82DE2EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE2EFC: 4BEC6501  bl 0x82ca93fc
	ctx.lr = 0x82DE2F00;
	sub_82CA93D0(ctx, base);
	// 82DE2F00: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE2F04: 3BA5FFFF  addi r29, r5, -1
	ctx.r[29].s64 = ctx.r[5].s64 + -1;
	// 82DE2F08: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DE2F0C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DE2F10: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DE2F14: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DE2F18: 41980084  blt cr6, 0x82de2f9c
	if ctx.cr[6].lt {
	pc = 0x82DE2F9C; continue 'dispatch;
	}
	// 82DE2F1C: 3D605555  lis r11, 0x5555
	ctx.r[11].s64 = 1431633920;
	// 82DE2F20: 3B200003  li r25, 3
	ctx.r[25].s64 = 3;
	// 82DE2F24: 617A5556  ori r26, r11, 0x5556
	ctx.r[26].u64 = ctx.r[11].u64 | 21846;
	// 82DE2F28: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE2F2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE2F30: A3FC0000  lhz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2F34: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE2F38: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE2F3C: 7D7FCBD6  divw r11, r31, r25
	ctx.r[11].s32 = ctx.r[31].s32 / ctx.r[25].s32;
	// 82DE2F40: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE2F44: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE2F48: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE2F4C: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE2F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE2F54: 4E800421  bctrl
	ctx.lr = 0x82DE2F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE2F58: 7D7FD096  mulhw r11, r31, r26
	ctx.r[11].s64 = ((ctx.r[31].s32 as i64 * ctx.r[26].s32 as i64) >> 32);
	// 82DE2F5C: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE2F60: 67E93F00  oris r9, r31, 0x3f00
	ctx.r[9].u64 = ctx.r[31].u64 | 1056964608;
	// 82DE2F64: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE2F68: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82DE2F6C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE2F70: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 82DE2F74: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE2F78: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DE2F7C: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82DE2F80: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82DE2F84: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE2FA8 size=68
    let mut pc: u32 = 0x82DE2FA8;
    'dispatch: loop {
        match pc {
            0x82DE2FA8 => {
    //   block [0x82DE2FA8..0x82DE2FEC)
	// 82DE2FA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE2FAC: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DE2FB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE2FB4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE2FB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DE2FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DE2FC0: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 82DE2FC4: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DE2FC8: 394A428C  addi r10, r10, 0x428c
	ctx.r[10].s64 = ctx.r[10].s64 + 17036;
	// 82DE2FCC: 39294264  addi r9, r9, 0x4264
	ctx.r[9].s64 = ctx.r[9].s64 + 16996;
	// 82DE2FD0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DE2FD4: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DE2FD8: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DE2FDC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DE2FE0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE2FE4: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82DE2FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE2FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE2FF0 size=244
    let mut pc: u32 = 0x82DE2FF0;
    'dispatch: loop {
        match pc {
            0x82DE2FF0 => {
    //   block [0x82DE2FF0..0x82DE30E4)
	// 82DE2FF0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE2FF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE2FF8: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DE2FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE3000: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE3004: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE3008: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE30E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE30E8 size=240
    let mut pc: u32 = 0x82DE30E8;
    'dispatch: loop {
        match pc {
            0x82DE30E8 => {
    //   block [0x82DE30E8..0x82DE31D8)
	// 82DE30E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE30EC: 4BEC6309  bl 0x82ca93f4
	ctx.lr = 0x82DE30F0;
	sub_82CA93D0(ctx, base);
	// 82DE30F0: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82DE30F4: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE30F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE30FC: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DE3100: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DE3104: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82DE3108: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DE310C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DE3110: C3EB0BE4  lfs f31, 0xbe4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3044 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DE3114: 3AE30028  addi r23, r3, 0x28
	ctx.r[23].s64 = ctx.r[3].s64 + 40;
	// 82DE3118: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE311C: 409900A8  ble cr6, 0x82de31c4
	if !ctx.cr[6].gt {
	pc = 0x82DE31C4; continue 'dispatch;
	}
	// 82DE3120: 3B630024  addi r27, r3, 0x24
	ctx.r[27].s64 = ctx.r[3].s64 + 36;
	// 82DE3124: 3B430020  addi r26, r3, 0x20
	ctx.r[26].s64 = ctx.r[3].s64 + 32;
	// 82DE3128: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE312C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE3130: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3134: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DE3138: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE313C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE3140: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE3144: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3148: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE314C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3150: 4E800421  bctrl
	ctx.lr = 0x82DE3154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3154: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3158: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DE315C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DE3160: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE3164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3168: 4E800421  bctrl
	ctx.lr = 0x82DE316C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE316C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE31D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE31D8 size=416
    let mut pc: u32 = 0x82DE31D8;
    'dispatch: loop {
        match pc {
            0x82DE31D8 => {
    //   block [0x82DE31D8..0x82DE3378)
	// 82DE31D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE31DC: 4BEC6219  bl 0x82ca93f4
	ctx.lr = 0x82DE31E0;
	sub_82CA93D0(ctx, base);
	// 82DE31E0: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE31E4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE31E8: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DE31EC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DE31F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE31F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DE31F8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DE31FC: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DE3200: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3204: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3208: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE320C: 40980020  bge cr6, 0x82de322c
	if !ctx.cr[6].lt {
	pc = 0x82DE322C; continue 'dispatch;
	}
	// 82DE3210: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE3214: 392942CC  addi r9, r9, 0x42cc
	ctx.r[9].s64 = ctx.r[9].s64 + 17100;
	// 82DE3218: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE321C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE3220: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DE3224: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE3228: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE322C: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DE3230: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82DE3234: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DE3238: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE323C: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DE3240: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DE3244: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE3248: 409900C0  ble cr6, 0x82de3308
	if !ctx.cr[6].gt {
	pc = 0x82DE3308; continue 'dispatch;
	}
	// 82DE324C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE3250: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE3254: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE3258: 419A0048  beq cr6, 0x82de32a0
	if ctx.cr[6].eq {
	pc = 0x82DE32A0; continue 'dispatch;
	}
	// 82DE325C: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE3260: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DE3264: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 82DE3268: 409A0008  bne cr6, 0x82de3270
	if !ctx.cr[6].eq {
	pc = 0x82DE3270; continue 'dispatch;
	}
	// 82DE326C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DE3270: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE3274: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE3278: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE327C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82DE3280: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE3284: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3288: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE328C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3290: 4E800421  bctrl
	ctx.lr = 0x82DE3294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3294: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE329C: 419A0058  beq cr6, 0x82de32f4
	if ctx.cr[6].eq {
	pc = 0x82DE32F4; continue 'dispatch;
	}
	// 82DE32A0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE32A4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DE32A8: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE32AC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE32B0: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE32B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE32B8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE32BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE32C0: 4E800421  bctrl
	ctx.lr = 0x82DE32C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE32C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DE32C8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DE32CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE32D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE32D4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE32D8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE32DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE32E0: 4E800421  bctrl
	ctx.lr = 0x82DE32E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE32E4: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE32E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE32EC: 419A0008  beq cr6, 0x82de32f4
	if ctx.cr[6].eq {
	pc = 0x82DE32F4; continue 'dispatch;
	}
	// 82DE32F0: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 82DE32F4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DE32F8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82DE32FC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DE3300: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DE3304: 4198FF4C  blt cr6, 0x82de3250
	if ctx.cr[6].lt {
	pc = 0x82DE3250; continue 'dispatch;
	}
	// 82DE3308: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DE330C: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 82DE3310: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DE3314: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82DE3318: 419A0010  beq cr6, 0x82de3328
	if ctx.cr[6].eq {
	pc = 0x82DE3328; continue 'dispatch;
	}
	// 82DE331C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE3320: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE3324: 7F4BE12E  stwx r26, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u32) };
	// 82DE3328: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DE332C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3330: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3334: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE3338: 40980020  bge cr6, 0x82de3358
	if !ctx.cr[6].lt {
	pc = 0x82DE3358; continue 'dispatch;
	}
	// 82DE333C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE3340: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DE3344: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE3348: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE334C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE3350: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE3354: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE3358: 397A0001  addi r11, r26, 1
	ctx.r[11].s64 = ctx.r[26].s64 + 1;
	// 82DE335C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DE3360: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DE3364: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DE3368: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DE336C: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DE3370: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82DE3374: 4BEC60D0  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE3378 size=344
    let mut pc: u32 = 0x82DE3378;
    'dispatch: loop {
        match pc {
            0x82DE3378 => {
    //   block [0x82DE3378..0x82DE34D0)
	// 82DE3378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE337C: 4BEC607D  bl 0x82ca93f8
	ctx.lr = 0x82DE3380;
	sub_82CA93D0(ctx, base);
	// 82DE3380: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3384: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3388: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DE338C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE3390: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DE3394: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DE3398: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DE339C: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DE33A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE33A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE33A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE33AC: 40980020  bge cr6, 0x82de33cc
	if !ctx.cr[6].lt {
	pc = 0x82DE33CC; continue 'dispatch;
	}
	// 82DE33B0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE33B4: 392942CC  addi r9, r9, 0x42cc
	ctx.r[9].s64 = ctx.r[9].s64 + 17100;
	// 82DE33B8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE33BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE33C0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DE33C4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE33C8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE33CC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DE33D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DE33D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE33D8: 409900C0  ble cr6, 0x82de3498
	if !ctx.cr[6].gt {
	pc = 0x82DE3498; continue 'dispatch;
	}
	// 82DE33DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE33E0: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE33E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE33E8: 419A0048  beq cr6, 0x82de3430
	if ctx.cr[6].eq {
	pc = 0x82DE3430; continue 'dispatch;
	}
	// 82DE33EC: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE33F0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DE33F4: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 82DE33F8: 409A0008  bne cr6, 0x82de3400
	if !ctx.cr[6].eq {
	pc = 0x82DE3400; continue 'dispatch;
	}
	// 82DE33FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DE3400: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE3404: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE3408: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE340C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE3410: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE3414: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3418: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE341C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3420: 4E800421  bctrl
	ctx.lr = 0x82DE3424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3424: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3428: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE342C: 419A0058  beq cr6, 0x82de3484
	if ctx.cr[6].eq {
	pc = 0x82DE3484; continue 'dispatch;
	}
	// 82DE3430: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE3434: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DE3438: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE343C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE3440: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82DE3444: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3448: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE344C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3450: 4E800421  bctrl
	ctx.lr = 0x82DE3454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3454: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3458: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82DE345C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DE3460: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82DE3464: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DE3468: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DE346C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DE3470: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DE3474: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3478: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE347C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3480: 4E800421  bctrl
	ctx.lr = 0x82DE3484;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3484: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DE3488: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82DE348C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82DE3490: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DE3494: 4198FF4C  blt cr6, 0x82de33e0
	if ctx.cr[6].lt {
	pc = 0x82DE33E0; continue 'dispatch;
	}
	// 82DE3498: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DE349C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE34A0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE34A4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE34A8: 40980020  bge cr6, 0x82de34c8
	if !ctx.cr[6].lt {
	pc = 0x82DE34C8; continue 'dispatch;
	}
	// 82DE34AC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE34B0: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DE34B4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE34B8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE34BC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE34C0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE34C4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE34C8: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82DE34CC: 4BEC5F7C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE34D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE34D0 size=8
    let mut pc: u32 = 0x82DE34D0;
    'dispatch: loop {
        match pc {
            0x82DE34D0 => {
    //   block [0x82DE34D0..0x82DE34D8)
	// 82DE34D0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82DE34D4: 48000004  b 0x82de34d8
	sub_82DE34D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE34D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE34D8 size=124
    let mut pc: u32 = 0x82DE34D8;
    'dispatch: loop {
        match pc {
            0x82DE34D8 => {
    //   block [0x82DE34D8..0x82DE3554)
	// 82DE34D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE34DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE34E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE34E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE34E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE34EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DE34F0: 393F0014  addi r9, r31, 0x14
	ctx.r[9].s64 = ctx.r[31].s64 + 20;
	// 82DE34F4: 409A0008  bne cr6, 0x82de34fc
	if !ctx.cr[6].eq {
	pc = 0x82DE34FC; continue 'dispatch;
	}
	// 82DE34F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE34FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE3500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DE3504: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DE3508: 394A39E0  addi r10, r10, 0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + 14816;
	// 82DE350C: 548807FE  clrlwi r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82DE3510: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DE3514: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE3518: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE351C: 419A0020  beq cr6, 0x82de353c
	if ctx.cr[6].eq {
	pc = 0x82DE353C; continue 'dispatch;
	}
	// 82DE3520: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3524: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DE3528: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DE352C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3530: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE3534: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE3538: 4BF71D91  bl 0x82d552c8
	ctx.lr = 0x82DE353C;
	sub_82D552C8(ctx, base);
	// 82DE353C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE3540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE3544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE3548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE354C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE3550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE3558 size=148
    let mut pc: u32 = 0x82DE3558;
    'dispatch: loop {
        match pc {
            0x82DE3558 => {
    //   block [0x82DE3558..0x82DE35EC)
	// 82DE3558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE355C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE3560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE3564: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE356C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE3570: 394AFC80  addi r10, r10, -0x380
	ctx.r[10].s64 = ctx.r[10].s64 + -896;
	// 82DE3574: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DE3578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE357C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE35F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE35F0 size=152
    let mut pc: u32 = 0x82DE35F0;
    'dispatch: loop {
        match pc {
            0x82DE35F0 => {
    //   block [0x82DE35F0..0x82DE3688)
	// 82DE35F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE35F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE35F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE35FC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3600: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE3604: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE3608: 394AFC80  addi r10, r10, -0x380
	ctx.r[10].s64 = ctx.r[10].s64 + -896;
	// 82DE360C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DE3610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE3614: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE3688 size=84
    let mut pc: u32 = 0x82DE3688;
    'dispatch: loop {
        match pc {
            0x82DE3688 => {
    //   block [0x82DE3688..0x82DE36DC)
	// 82DE3688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE3690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE3694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE369C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE36A0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DE36A4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DE36A8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE36AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE36B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE36B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE36B8: 4E800421  bctrl
	ctx.lr = 0x82DE36BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE36BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE36C0: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE36C4: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DE36C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE36CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE36D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE36D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE36D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE36E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE36E0 size=4
    let mut pc: u32 = 0x82DE36E0;
    'dispatch: loop {
        match pc {
            0x82DE36E0 => {
    //   block [0x82DE36E0..0x82DE36E4)
	// 82DE36E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE36E8 size=8
    let mut pc: u32 = 0x82DE36E8;
    'dispatch: loop {
        match pc {
            0x82DE36E8 => {
    //   block [0x82DE36E8..0x82DE36F0)
	// 82DE36E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DE36EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE36F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE36F0 size=20
    let mut pc: u32 = 0x82DE36F0;
    'dispatch: loop {
        match pc {
            0x82DE36F0 => {
    //   block [0x82DE36F0..0x82DE3704)
	// 82DE36F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE36F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DE36F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE36FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3700: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE3704 size=4
    let mut pc: u32 = 0x82DE3704;
    'dispatch: loop {
        match pc {
            0x82DE3704 => {
    //   block [0x82DE3704..0x82DE3708)
	// 82DE3704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE3708 size=32
    let mut pc: u32 = 0x82DE3708;
    'dispatch: loop {
        match pc {
            0x82DE3708 => {
    //   block [0x82DE3708..0x82DE3728)
	// 82DE3708: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DE370C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE3728 size=20
    let mut pc: u32 = 0x82DE3728;
    'dispatch: loop {
        match pc {
            0x82DE3728 => {
    //   block [0x82DE3728..0x82DE373C)
	// 82DE3728: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE372C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82DE3730: 409A000C  bne cr6, 0x82de373c
	if !ctx.cr[6].eq {
		sub_82DE373C(ctx, base);
		return;
	}
	// 82DE3734: D0430018  stfs f2, 0x18(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DE3738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE373C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE373C size=32
    let mut pc: u32 = 0x82DE373C;
    'dispatch: loop {
        match pc {
            0x82DE373C => {
    //   block [0x82DE373C..0x82DE375C)
	// 82DE373C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE3740: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82DE3744: C00A0EE0  lfs f0, 0xee0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE3748: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE3760 size=240
    let mut pc: u32 = 0x82DE3760;
    'dispatch: loop {
        match pc {
            0x82DE3760 => {
    //   block [0x82DE3760..0x82DE3850)
	// 82DE3760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE3764: 4BEC5CA1  bl 0x82ca9404
	ctx.lr = 0x82DE3768;
	sub_82CA93D0(ctx, base);
	// 82DE3768: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE376C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE3770: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE3774: 396B42E0  addi r11, r11, 0x42e0
	ctx.r[11].s64 = ctx.r[11].s64 + 17120;
	// 82DE3778: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE377C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE3780: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DE3784: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE3788: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DE378C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE3790: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DE3794: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3798: 83BC0000  lwz r29, 0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE379C: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE37A0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE37A4: 4BF73BCD  bl 0x82d57370
	ctx.lr = 0x82DE37A8;
	sub_82D57370(ctx, base);
	// 82DE37A8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE37AC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DE37B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE37B4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82DE37B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DE37BC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DE37C0: 409A000C  bne cr6, 0x82de37cc
	if !ctx.cr[6].eq {
	pc = 0x82DE37CC; continue 'dispatch;
	}
	// 82DE37C4: 4803505D  bl 0x82e18820
	ctx.lr = 0x82DE37C8;
	sub_82E18820(ctx, base);
	// 82DE37C8: 48000008  b 0x82de37d0
	pc = 0x82DE37D0; continue 'dispatch;
	// 82DE37CC: 48034E6D  bl 0x82e18638
	ctx.lr = 0x82DE37D0;
	sub_82E18638(ctx, base);
	// 82DE37D0: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82DE37D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE3850 size=164
    let mut pc: u32 = 0x82DE3850;
    'dispatch: loop {
        match pc {
            0x82DE3850 => {
    //   block [0x82DE3850..0x82DE38F4)
	// 82DE3850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE3854: 4BEC5BB1  bl 0x82ca9404
	ctx.lr = 0x82DE3858;
	sub_82CA93D0(ctx, base);
	// 82DE3858: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE385C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE3860: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3864: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DE3868: 80A40008  lwz r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE386C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DE3870: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DE3874: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3878: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE387C: 4BF73AF5  bl 0x82d57370
	ctx.lr = 0x82DE3880;
	sub_82D57370(ctx, base);
	// 82DE3880: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DE3884: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DE3888: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DE388C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DE3890: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DE3894: 48037A25  bl 0x82e1b2b8
	ctx.lr = 0x82DE3898;
	sub_82E1B2B8(ctx, base);
	// 82DE3898: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DE389C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE38A0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DE38A4: C1A1005C  lfs f13, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE38F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE38F8 size=556
    let mut pc: u32 = 0x82DE38F8;
    'dispatch: loop {
        match pc {
            0x82DE38F8 => {
    //   block [0x82DE38F8..0x82DE3B24)
	// 82DE38F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE38FC: 4BEC5AF5  bl 0x82ca93f0
	ctx.lr = 0x82DE3900;
	sub_82CA93D0(ctx, base);
	// 82DE3900: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3904: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3908: 3AE00008  li r23, 8
	ctx.r[23].s64 = 8;
	// 82DE390C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DE3910: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DE3914: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DE3918: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DE391C: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DE3920: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3924: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3928: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE392C: 4098002C  bge cr6, 0x82de3958
	if !ctx.cr[6].lt {
	pc = 0x82DE3958; continue 'dispatch;
	}
	// 82DE3930: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE3934: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DE3938: 3929432C  addi r9, r9, 0x432c
	ctx.r[9].s64 = ctx.r[9].s64 + 17196;
	// 82DE393C: 39084324  addi r8, r8, 0x4324
	ctx.r[8].s64 = ctx.r[8].s64 + 17188;
	// 82DE3940: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE3944: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DE3948: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE394C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DE3950: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE3954: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE3958: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82DE395C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3960: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3964: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE3968: 40980020  bge cr6, 0x82de3988
	if !ctx.cr[6].lt {
	pc = 0x82DE3988; continue 'dispatch;
	}
	// 82DE396C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE3970: 39294318  addi r9, r9, 0x4318
	ctx.r[9].s64 = ctx.r[9].s64 + 17176;
	// 82DE3974: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE3978: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE397C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE3980: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE3984: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE3988: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 82DE398C: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3990: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3994: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3998: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE399C: 4BF739D5  bl 0x82d57370
	ctx.lr = 0x82DE39A0;
	sub_82D57370(ctx, base);
	// 82DE39A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE39A4: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 82DE39A8: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82DE39AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE39B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE39B4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DE39B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE39BC: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82DE39C0: 88BF0008  lbz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE39C4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82DE39C8: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DE39CC: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82DE39D0: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE39D4: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82DE39D8: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DE39DC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DE39E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82DE39E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE39E8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE39EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE39F0: 4E800421  bctrl
	ctx.lr = 0x82DE39F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE39F4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DE39F8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE39FC: 38C10140  addi r6, r1, 0x140
	ctx.r[6].s64 = ctx.r[1].s64 + 320;
	// 82DE3A00: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE3A04: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DE3A08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE3A0C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DE3A10: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE3A14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3A18: 4E800421  bctrl
	ctx.lr = 0x82DE3A1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3A1C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DE3A20: 38C101E0  addi r6, r1, 0x1e0
	ctx.r[6].s64 = ctx.r[1].s64 + 480;
	// 82DE3A24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE3A28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE3A2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DE3A30: 48037889  bl 0x82e1b2b8
	ctx.lr = 0x82DE3A34;
	sub_82E1B2B8(ctx, base);
	// 82DE3A34: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82DE3A38: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DE3A3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE3A40: 419A0010  beq cr6, 0x82de3a50
	if ctx.cr[6].eq {
	pc = 0x82DE3A50; continue 'dispatch;
	}
	// 82DE3A44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE3A48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DE3A4C: 48036235  bl 0x82e19c80
	ctx.lr = 0x82DE3A50;
	sub_82E19C80(ctx, base);
	// 82DE3A50: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82DE3A54: 409A006C  bne cr6, 0x82de3ac0
	if !ctx.cr[6].eq {
	pc = 0x82DE3AC0; continue 'dispatch;
	}
	// 82DE3A58: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DE3A5C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3A60: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DE3A64: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE3A68: C1A1005C  lfs f13, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE3A6C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DE3A70: C1BD0010  lfs f13, 0x10(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE3B28 size=420
    let mut pc: u32 = 0x82DE3B28;
    'dispatch: loop {
        match pc {
            0x82DE3B28 => {
    //   block [0x82DE3B28..0x82DE3CCC)
	// 82DE3B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE3B2C: 4BEC58D1  bl 0x82ca93fc
	ctx.lr = 0x82DE3B30;
	sub_82CA93D0(ctx, base);
	// 82DE3B30: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3B34: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3B38: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DE3B3C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DE3B40: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DE3B44: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DE3B48: 7D5DE02E  lwzx r10, r29, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DE3B4C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3B50: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3B54: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE3B58: 40980020  bge cr6, 0x82de3b78
	if !ctx.cr[6].lt {
	pc = 0x82DE3B78; continue 'dispatch;
	}
	// 82DE3B5C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE3B60: 39293C70  addi r9, r9, 0x3c70
	ctx.r[9].s64 = ctx.r[9].s64 + 15472;
	// 82DE3B64: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE3B68: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE3B6C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE3B70: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE3B74: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE3B78: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DE3B7C: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3B80: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3B84: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3B88: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3B8C: 4BF737E5  bl 0x82d57370
	ctx.lr = 0x82DE3B90;
	sub_82D57370(ctx, base);
	// 82DE3B90: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3B94: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DE3B98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE3B9C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82DE3BA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE3BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE3BA8: 409A000C  bne cr6, 0x82de3bb4
	if !ctx.cr[6].eq {
	pc = 0x82DE3BB4; continue 'dispatch;
	}
	// 82DE3BAC: 48034C75  bl 0x82e18820
	ctx.lr = 0x82DE3BB0;
	sub_82E18820(ctx, base);
	// 82DE3BB0: 48000008  b 0x82de3bb8
	pc = 0x82DE3BB8; continue 'dispatch;
	// 82DE3BB4: 48034A85  bl 0x82e18638
	ctx.lr = 0x82DE3BB8;
	sub_82E18638(ctx, base);
	// 82DE3BB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DE3BBC: 8961005A  lbz r11, 0x5a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82DE3BC0: 88A10058  lbz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DE3BC4: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 82DE3BC8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE3BCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE3BD0: 914100C0  stw r10, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 82DE3BD4: 89410059  lbz r10, 0x59(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82DE3BD8: 90A100B0  stw r5, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 82DE3BDC: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82DE3BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE3BE4: 914100C4  stw r10, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 82DE3BE8: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE3BEC: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DE3BF0: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 82DE3BF4: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 82DE3BF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3BFC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE3C00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3C04: 4E800421  bctrl
	ctx.lr = 0x82DE3C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3C08: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3C0C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82DE3C10: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DE3C14: 80A100B4  lwz r5, 0xb4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82DE3C18: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 82DE3C1C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE3C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE3C24: 81290034  lwz r9, 0x34(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE3C28: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE3C2C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DE3C30: 4E800421  bctrl
	ctx.lr = 0x82DE3C34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3C34: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DE3C38: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DE3C3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE3C40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE3C44: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DE3C48: 48037671  bl 0x82e1b2b8
	ctx.lr = 0x82DE3C4C;
	sub_82E1B2B8(ctx, base);
	// 82DE3C4C: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DE3C50: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3C54: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3C58: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE3C5C: 40980020  bge cr6, 0x82de3c7c
	if !ctx.cr[6].lt {
	pc = 0x82DE3C7C; continue 'dispatch;
	}
	// 82DE3C60: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE3C64: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DE3C68: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE3C6C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE3C70: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DE3C74: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE3C78: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE3C7C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE3C80: 409A0028  bne cr6, 0x82de3ca8
	if !ctx.cr[6].eq {
	pc = 0x82DE3CA8; continue 'dispatch;
	}
	// 82DE3C84: C1A1006C  lfs f13, 0x6c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE3C88: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE3C8C: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE3C90: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DE3C94: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE3C98: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DE3C9C: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE3CA0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DE3CA4: 40980020  bge cr6, 0x82de3cc4
	if !ctx.cr[6].lt {
	pc = 0x82DE3CC4; continue 'dispatch;
	}
	// 82DE3CA8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3CAC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DE3CB0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DE3CB4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DE3CB8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3CBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3CC0: 4E800421  bctrl
	ctx.lr = 0x82DE3CC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3CC4: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DE3CC8: 4BEC5784  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE3CD0 size=740
    let mut pc: u32 = 0x82DE3CD0;
    'dispatch: loop {
        match pc {
            0x82DE3CD0 => {
    //   block [0x82DE3CD0..0x82DE3FB4)
	// 82DE3CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE3CD4: 4BEC5721  bl 0x82ca93f4
	ctx.lr = 0x82DE3CD8;
	sub_82CA93D0(ctx, base);
	// 82DE3CD8: 9421FCF0  stwu r1, -0x310(r1)
	ea = ctx.r[1].u32.wrapping_add(-784 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3CDC: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3CE0: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DE3CE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE3CE8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DE3CEC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DE3CF0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82DE3CF4: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DE3CF8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE3CFC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3D00: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE3D04: 40980020  bge cr6, 0x82de3d24
	if !ctx.cr[6].lt {
	pc = 0x82DE3D24; continue 'dispatch;
	}
	// 82DE3D08: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE3D0C: 39293C70  addi r9, r9, 0x3c70
	ctx.r[9].s64 = ctx.r[9].s64 + 15472;
	// 82DE3D10: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE3D14: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE3D18: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DE3D1C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE3D20: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE3D24: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82DE3D28: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3D2C: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3D30: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3D34: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3D38: 4BF73639  bl 0x82d57370
	ctx.lr = 0x82DE3D3C;
	sub_82D57370(ctx, base);
	// 82DE3D3C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE3D40: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 82DE3D44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DE3D48: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82DE3D4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE3D50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE3D54: 409A000C  bne cr6, 0x82de3d60
	if !ctx.cr[6].eq {
	pc = 0x82DE3D60; continue 'dispatch;
	}
	// 82DE3D58: 48034AC9  bl 0x82e18820
	ctx.lr = 0x82DE3D5C;
	sub_82E18820(ctx, base);
	// 82DE3D5C: 48000008  b 0x82de3d64
	pc = 0x82DE3D64; continue 'dispatch;
	// 82DE3D60: 480348D9  bl 0x82e18638
	ctx.lr = 0x82DE3D64;
	sub_82E18638(ctx, base);
	// 82DE3D64: 89410059  lbz r10, 0x59(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82DE3D68: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DE3D6C: 8961005A  lbz r11, 0x5a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82DE3D70: 38C10160  addi r6, r1, 0x160
	ctx.r[6].s64 = ctx.r[1].s64 + 352;
	// 82DE3D74: 88A10058  lbz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DE3D78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE3D7C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3D80: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3D84: 91410144  stw r10, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 82DE3D88: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE3D8C: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DE3D90: 93610150  stw r27, 0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[27].u32 ) };
	// 82DE3D94: 90A10140  stw r5, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[5].u32 ) };
	// 82DE3D98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE3D9C: 93610154  stw r27, 0x154(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), ctx.r[27].u32 ) };
	// 82DE3DA0: 91410148  stw r10, 0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[10].u32 ) };
	// 82DE3DA4: 9161014C  stw r11, 0x14c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), ctx.r[11].u32 ) };
	// 82DE3DA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3DAC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE3DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE3DB4: 4E800421  bctrl
	ctx.lr = 0x82DE3DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3DB8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3DBC: 81610140  lwz r11, 0x140(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 82DE3DC0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DE3DC4: 80A10144  lwz r5, 0x144(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 82DE3DC8: 38C10220  addi r6, r1, 0x220
	ctx.r[6].s64 = ctx.r[1].s64 + 544;
	// 82DE3DCC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE3DD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE3DD4: 81290034  lwz r9, 0x34(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE3DD8: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE3DDC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DE3DE0: 4E800421  bctrl
	ctx.lr = 0x82DE3DE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE3DE4: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82DE3DE8: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 82DE3DEC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DE3DF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE3DF4: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 82DE3DF8: 480374C1  bl 0x82e1b2b8
	ctx.lr = 0x82DE3DFC;
	sub_82E1B2B8(ctx, base);
	// 82DE3DFC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE3E00: 409A00EC  bne cr6, 0x82de3eec
	if !ctx.cr[6].eq {
	pc = 0x82DE3EEC; continue 'dispatch;
	}
	// 82DE3E04: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DE3E08: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82DE3E0C: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 82DE3E10: 48035FC1  bl 0x82e19dd0
	ctx.lr = 0x82DE3E14;
	sub_82E19DD0(ctx, base);
	// 82DE3E14: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DE3E18: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DE3E1C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3E20: 4BF72621  bl 0x82d56440
	ctx.lr = 0x82DE3E24;
	sub_82D56440(ctx, base);
	// 82DE3E24: C1A10090  lfs f13, 0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE3E28: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE3E2C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DE3E30: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE3E34: C19A0004  lfs f12, 4(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE3E38: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DE3E3C: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82DE3E40: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DE3E44: 409800A8  bge cr6, 0x82de3eec
	if !ctx.cr[6].lt {
	pc = 0x82DE3EEC; continue 'dispatch;
	}
	// 82DE3E48: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DE3E4C: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE3E50: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DE3E54: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DE3E58: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DE3E5C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DE3E60: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82DE3E64: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE3FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE3FB8 size=512
    let mut pc: u32 = 0x82DE3FB8;
    'dispatch: loop {
        match pc {
            0x82DE3FB8 => {
    //   block [0x82DE3FB8..0x82DE41B8)
	// 82DE3FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE3FBC: 4BEC543D  bl 0x82ca93f8
	ctx.lr = 0x82DE3FC0;
	sub_82CA93D0(ctx, base);
	// 82DE3FC0: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE3FC4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DE3FC8: 83860000  lwz r28, 0(r6)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3FCC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DE3FD0: 80A60008  lwz r5, 8(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3FD4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DE3FD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE3FDC: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DE3FE0: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE3FE4: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82DE3FE8: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE3FEC: 4BF73385  bl 0x82d57370
	ctx.lr = 0x82DE3FF0;
	sub_82D57370(ctx, base);
	// 82DE3FF0: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82DE3FF4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82DE3FF8: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 82DE3FFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE4000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE4004: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82DE4008: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DE400C: 88BF0008  lbz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE4010: 932100C0  stw r25, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[25].u32 ) };
	// 82DE4014: 932100C4  stw r25, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[25].u32 ) };
	// 82DE4018: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82DE401C: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE4020: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82DE4024: 90A100B0  stw r5, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 82DE4028: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 82DE402C: 916100BC  stw r11, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 82DE4030: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4034: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE4038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE403C: 4E800421  bctrl
	ctx.lr = 0x82DE4040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE4040: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82DE4044: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4048: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 82DE404C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE4050: 80A100B4  lwz r5, 0xb4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82DE4054: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DE4058: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DE405C: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE4060: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE4064: 4E800421  bctrl
	ctx.lr = 0x82DE4068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE4068: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DE406C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DE4070: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DE4074: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE4078: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DE407C: 4803723D  bl 0x82e1b2b8
	ctx.lr = 0x82DE4080;
	sub_82E1B2B8(ctx, base);
	// 82DE4080: 816100C4  lwz r11, 0xc4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82DE4084: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE4088: 419A0010  beq cr6, 0x82de4098
	if ctx.cr[6].eq {
	pc = 0x82DE4098; continue 'dispatch;
	}
	// 82DE408C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE4090: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DE4094: 48035BED  bl 0x82e19c80
	ctx.lr = 0x82DE4098;
	sub_82E19C80(ctx, base);
	// 82DE4098: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE409C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DE40A0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82DE40A4: 48035D2D  bl 0x82e19dd0
	ctx.lr = 0x82DE40A8;
	sub_82E19DD0(ctx, base);
	// 82DE40A8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE41B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE41B8 size=292
    let mut pc: u32 = 0x82DE41B8;
    'dispatch: loop {
        match pc {
            0x82DE41B8 => {
    //   block [0x82DE41B8..0x82DE42DC)
	// 82DE41B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE41BC: 4BEC5241  bl 0x82ca93fc
	ctx.lr = 0x82DE41C0;
	sub_82CA93D0(ctx, base);
	// 82DE41C0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE41C4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE41C8: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 82DE41CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DE41D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE41D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DE41D8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DE41DC: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DE41E0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DE41E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE41E8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE41EC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE41F0: 40980020  bge cr6, 0x82de4210
	if !ctx.cr[6].lt {
	pc = 0x82DE4210; continue 'dispatch;
	}
	// 82DE41F4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE41F8: 39293C70  addi r9, r9, 0x3c70
	ctx.r[9].s64 = ctx.r[9].s64 + 15472;
	// 82DE41FC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE4200: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE4204: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82DE4208: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE420C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE4210: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82DE4214: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE4218: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE421C: 4BF73155  bl 0x82d57370
	ctx.lr = 0x82DE4220;
	sub_82D57370(ctx, base);
	// 82DE4220: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4224: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DE4228: C01A0004  lfs f0, 4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE422C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DE4230: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DE4234: 389B000C  addi r4, r27, 0xc
	ctx.r[4].s64 = ctx.r[27].s64 + 12;
	// 82DE4238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE423C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DE4240: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4244: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DE4248: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE424C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DE4250: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82DE4254: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DE4258: 48037EC9  bl 0x82e1c120
	ctx.lr = 0x82DE425C;
	sub_82E1C120(ctx, base);
	// 82DE425C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DE4260: 409A0044  bne cr6, 0x82de42a4
	if !ctx.cr[6].eq {
	pc = 0x82DE42A4; continue 'dispatch;
	}
	// 82DE4264: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DE4268: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82DE426C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4270: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82DE4274: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE42E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE42E0 size=80
    let mut pc: u32 = 0x82DE42E0;
    'dispatch: loop {
        match pc {
            0x82DE42E0 => {
    //   block [0x82DE42E0..0x82DE4330)
	// 82DE42E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE42E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE42E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE42EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE42F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE42F4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE42F8: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82DE42FC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE4300: 48032109  bl 0x82e16408
	ctx.lr = 0x82DE4304;
	sub_82E16408(ctx, base);
	// 82DE4304: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4308: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DE430C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE4310: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4314: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE4318: 4E800421  bctrl
	ctx.lr = 0x82DE431C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE431C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE4320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE4324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE4328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE432C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4330 size=16
    let mut pc: u32 = 0x82DE4330;
    'dispatch: loop {
        match pc {
            0x82DE4330 => {
    //   block [0x82DE4330..0x82DE4340)
	// 82DE4330: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82DE4334: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE4338: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE433C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4340 size=40
    let mut pc: u32 = 0x82DE4340;
    'dispatch: loop {
        match pc {
            0x82DE4340 => {
    //   block [0x82DE4340..0x82DE4368)
	// 82DE4340: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DE4344: 39630036  addi r11, r3, 0x36
	ctx.r[11].s64 = ctx.r[3].s64 + 54;
	// 82DE4348: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE434C: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DE4350: 419A0018  beq cr6, 0x82de4368
	if ctx.cr[6].eq {
		sub_82DE4368(ctx, base);
		return;
	}
	// 82DE4354: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DE4358: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE435C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE4360: 4198FFE8  blt cr6, 0x82de4348
	if ctx.cr[6].lt {
	pc = 0x82DE4348; continue 'dispatch;
	}
	// 82DE4364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4368 size=12
    let mut pc: u32 = 0x82DE4368;
    'dispatch: loop {
        match pc {
            0x82DE4368 => {
    //   block [0x82DE4368..0x82DE4374)
	// 82DE4368: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82DE436C: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82DE4370: 48032FE8  b 0x82e17358
	sub_82E17358(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4378 size=16
    let mut pc: u32 = 0x82DE4378;
    'dispatch: loop {
        match pc {
            0x82DE4378 => {
    //   block [0x82DE4378..0x82DE4388)
	// 82DE4378: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82DE437C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE4380: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE4384: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4388 size=36
    let mut pc: u32 = 0x82DE4388;
    'dispatch: loop {
        match pc {
            0x82DE4388 => {
    //   block [0x82DE4388..0x82DE43AC)
	// 82DE4388: 39430036  addi r10, r3, 0x36
	ctx.r[10].s64 = ctx.r[3].s64 + 54;
	// 82DE438C: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4390: 2B08FFFF  cmplwi cr6, r8, 0xffff
	ctx.cr[6].compare_u32(ctx.r[8].u32, 65535 as u32, &mut ctx.xer);
	// 82DE4394: 419A0018  beq cr6, 0x82de43ac
	if ctx.cr[6].eq {
		sub_82DE43AC(ctx, base);
		return;
	}
	// 82DE4398: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE439C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DE43A0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE43A4: 4198FFE8  blt cr6, 0x82de438c
	if ctx.cr[6].lt {
	pc = 0x82DE438C; continue 'dispatch;
	}
	// 82DE43A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE43AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE43AC size=16
    let mut pc: u32 = 0x82DE43AC;
    'dispatch: loop {
        match pc {
            0x82DE43AC => {
    //   block [0x82DE43AC..0x82DE43BC)
	// 82DE43AC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE43B0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DE43B4: B08B0036  sth r4, 0x36(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(54 as u32), ctx.r[4].u16 ) };
	// 82DE43B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE43C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE43C0 size=20
    let mut pc: u32 = 0x82DE43C0;
    'dispatch: loop {
        match pc {
            0x82DE43C0 => {
    //   block [0x82DE43C0..0x82DE43D4)
	// 82DE43C0: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82DE43C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DE43C8: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DE43CC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE43D0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE43D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE43D4 size=40
    let mut pc: u32 = 0x82DE43D4;
    'dispatch: loop {
        match pc {
            0x82DE43D4 => {
    //   block [0x82DE43D4..0x82DE43FC)
	// 82DE43D4: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DE43D8: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82DE43DC: A0CB0002  lhz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE43E0: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DE43E4: 419A0018  beq cr6, 0x82de43fc
	if ctx.cr[6].eq {
		sub_82DE43FC(ctx, base);
		return;
	}
	// 82DE43E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DE43EC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE43F0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE43F4: 4198FFE8  blt cr6, 0x82de43dc
	if ctx.cr[6].lt {
	pc = 0x82DE43DC; continue 'dispatch;
	}
	// 82DE43F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE43FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE43FC size=12
    let mut pc: u32 = 0x82DE43FC;
    'dispatch: loop {
        match pc {
            0x82DE43FC => {
    //   block [0x82DE43FC..0x82DE4408)
	// 82DE43FC: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82DE4400: 98EB0001  stb r7, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 82DE4404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE4408 size=68
    let mut pc: u32 = 0x82DE4408;
    'dispatch: loop {
        match pc {
            0x82DE4408 => {
    //   block [0x82DE4408..0x82DE444C)
	// 82DE4408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE440C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE4410: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE4414: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE4418: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE441C: 4BFFF345  bl 0x82de3760
	ctx.lr = 0x82DE4420;
	sub_82DE3760(ctx, base);
	// 82DE4420: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE4424: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE4428: 396B4354  addi r11, r11, 0x4354
	ctx.r[11].s64 = ctx.r[11].s64 + 17236;
	// 82DE442C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE4430: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE4434: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DE4438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE443C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE4440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE4444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE4448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE4450 size=156
    let mut pc: u32 = 0x82DE4450;
    'dispatch: loop {
        match pc {
            0x82DE4450 => {
    //   block [0x82DE4450..0x82DE44EC)
	// 82DE4450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE4454: 4BEC4FB5  bl 0x82ca9408
	ctx.lr = 0x82DE4458;
	sub_82CA93D0(ctx, base);
	// 82DE4458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE445C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4460: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DE4464: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DE4468: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE446C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DE4470: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DE4474: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE4478: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DE447C: 419A0048  beq cr6, 0x82de44c4
	if ctx.cr[6].eq {
	pc = 0x82DE44C4; continue 'dispatch;
	}
	// 82DE4480: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82DE4484: 4BF70DC5  bl 0x82d55248
	ctx.lr = 0x82DE4488;
	sub_82D55248(ctx, base);
	// 82DE4488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE448C: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 82DE4490: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DE4494: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE4498: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE449C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DE44A0: 4BFFF2C1  bl 0x82de3760
	ctx.lr = 0x82DE44A4;
	sub_82DE3760(ctx, base);
	// 82DE44A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE44A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE44AC: 396B4354  addi r11, r11, 0x4354
	ctx.r[11].s64 = ctx.r[11].s64 + 17236;
	// 82DE44B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE44B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE44B8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DE44BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE44C0: 4BEC4F98  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82DE44C4: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82DE44C8: 4BF70D81  bl 0x82d55248
	ctx.lr = 0x82DE44CC;
	sub_82D55248(ctx, base);
	// 82DE44CC: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82DE44D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DE44D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE44D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE44DC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DE44E0: 4BFFF281  bl 0x82de3760
	ctx.lr = 0x82DE44E4;
	sub_82DE3760(ctx, base);
	// 82DE44E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE44E8: 4BEC4F70  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE44F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE44F0 size=64
    let mut pc: u32 = 0x82DE44F0;
    'dispatch: loop {
        match pc {
            0x82DE44F0 => {
    //   block [0x82DE44F0..0x82DE4530)
	// 82DE44F0: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DE44F4: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DE44F8: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DE44FC: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DE4500: 38EB4B00  addi r7, r11, 0x4b00
	ctx.r[7].s64 = ctx.r[11].s64 + 19200;
	// 82DE4504: 39084450  addi r8, r8, 0x4450
	ctx.r[8].s64 = ctx.r[8].s64 + 17488;
	// 82DE4508: 39293B28  addi r9, r9, 0x3b28
	ctx.r[9].s64 = ctx.r[9].s64 + 15144;
	// 82DE450C: 394A3CD0  addi r10, r10, 0x3cd0
	ctx.r[10].s64 = ctx.r[10].s64 + 15568;
	// 82DE4510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE4514: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DE4518: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE451C: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE4520: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DE4524: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82DE4528: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82DE452C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE4530 size=104
    let mut pc: u32 = 0x82DE4530;
    'dispatch: loop {
        match pc {
            0x82DE4530 => {
    //   block [0x82DE4530..0x82DE4598)
	// 82DE4530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE4534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE4538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE453C: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DE4540: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DE4544: 396B4B00  addi r11, r11, 0x4b00
	ctx.r[11].s64 = ctx.r[11].s64 + 19200;
	// 82DE4548: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DE454C: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DE4550: 39084450  addi r8, r8, 0x4450
	ctx.r[8].s64 = ctx.r[8].s64 + 17488;
	// 82DE4554: 39293B28  addi r9, r9, 0x3b28
	ctx.r[9].s64 = ctx.r[9].s64 + 15144;
	// 82DE4558: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DE455C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE4560: 394A3CD0  addi r10, r10, 0x3cd0
	ctx.r[10].s64 = ctx.r[10].s64 + 15568;
	// 82DE4564: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DE4568: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE456C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DE4570: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE4574: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DE4578: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82DE457C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DE4580: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82DE4584: 4BFDCBE5  bl 0x82dc1168
	ctx.lr = 0x82DE4588;
	sub_82DC1168(ctx, base);
	// 82DE4588: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE458C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE4590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE4594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE4598 size=124
    let mut pc: u32 = 0x82DE4598;
    'dispatch: loop {
        match pc {
            0x82DE4598 => {
    //   block [0x82DE4598..0x82DE4614)
	// 82DE4598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE45A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE45A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE45A8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE45AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE45B0: 90810070  stw r4, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[4].u32 ) };
	// 82DE45B4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DE45B8: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE45BC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82DE45C0: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82DE45C4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DE45C8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE45CC: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE45D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82DE45D4: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82DE45D8: 4BF72D99  bl 0x82d57370
	ctx.lr = 0x82DE45DC;
	sub_82D57370(ctx, base);
	// 82DE45DC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82DE45E0: 391F0020  addi r8, r31, 0x20
	ctx.r[8].s64 = ctx.r[31].s64 + 32;
	// 82DE45E4: 38FF0030  addi r7, r31, 0x30
	ctx.r[7].s64 = ctx.r[31].s64 + 48;
	// 82DE45E8: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 82DE45EC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82DE45F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE45F4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DE45F8: 48044779  bl 0x82e28d70
	ctx.lr = 0x82DE45FC;
	sub_82E28D70(ctx, base);
	// 82DE45FC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82DE4600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE4604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE4608: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE460C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE4610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE4618 size=456
    let mut pc: u32 = 0x82DE4618;
    'dispatch: loop {
        match pc {
            0x82DE4618 => {
    //   block [0x82DE4618..0x82DE47E0)
	// 82DE4618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE461C: 4BEC4DE1  bl 0x82ca93fc
	ctx.lr = 0x82DE4620;
	sub_82CA93D0(ctx, base);
	// 82DE4620: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE4624: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4628: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DE462C: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE4630: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4634: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE4638: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE463C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE4640: 4098002C  bge cr6, 0x82de466c
	if !ctx.cr[6].lt {
	pc = 0x82DE466C; continue 'dispatch;
	}
	// 82DE4644: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE4648: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DE464C: 3929438C  addi r9, r9, 0x438c
	ctx.r[9].s64 = ctx.r[9].s64 + 17292;
	// 82DE4650: 39083714  addi r8, r8, 0x3714
	ctx.r[8].s64 = ctx.r[8].s64 + 14100;
	// 82DE4654: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE4658: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DE465C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE4660: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DE4664: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE4668: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE466C: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE4670: 3B630018  addi r27, r3, 0x18
	ctx.r[27].s64 = ctx.r[3].s64 + 24;
	// 82DE4674: C1A60050  lfs f13, 0x50(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE4678: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DE467C: 419A00F0  beq cr6, 0x82de476c
	if ctx.cr[6].eq {
	pc = 0x82DE476C; continue 'dispatch;
	}
	// 82DE4680: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE4684: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE4688: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE468C: C1860004  lfs f12, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE4690: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
	// 82DE4694: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82DE4698: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82DE469C: C0060058  lfs f0, 0x58(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE46A0: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 82DE46A4: C1AB00A0  lfs f13, 0xa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE46A8: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 82DE46AC: C18B009C  lfs f12, 0x9c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE46B0: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 82DE46B4: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE47E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE47E0 size=796
    let mut pc: u32 = 0x82DE47E0;
    'dispatch: loop {
        match pc {
            0x82DE47E0 => {
    //   block [0x82DE47E0..0x82DE4AFC)
	// 82DE47E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE47E4: 4BEC4C0D  bl 0x82ca93f0
	ctx.lr = 0x82DE47E8;
	sub_82CA93D0(ctx, base);
	// 82DE47E8: DBC1FF98  stfd f30, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 82DE47EC: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82DE47F0: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4B00 size=828
    let mut pc: u32 = 0x82DE4B00;
    'dispatch: loop {
        match pc {
            0x82DE4B00 => {
    //   block [0x82DE4B00..0x82DE4E3C)
	// 82DE4B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE4B04: 4BEC48ED  bl 0x82ca93f0
	ctx.lr = 0x82DE4B08;
	sub_82CA93D0(ctx, base);
	// 82DE4B08: DBA1FF90  stfd f29, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[29].u64 ) };
	// 82DE4B0C: DBC1FF98  stfd f30, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 82DE4B10: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82DE4B14: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4E40 size=12
    let mut pc: u32 = 0x82DE4E40;
    'dispatch: loop {
        match pc {
            0x82DE4E40 => {
    //   block [0x82DE4E40..0x82DE4E4C)
	// 82DE4E40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE4E44: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DE4E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE4E50 size=28
    let mut pc: u32 = 0x82DE4E50;
    'dispatch: loop {
        match pc {
            0x82DE4E50 => {
    //   block [0x82DE4E50..0x82DE4E6C)
	// 82DE4E50: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE4E54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE4E58: 419A0014  beq cr6, 0x82de4e6c
	if ctx.cr[6].eq {
		sub_82DE4E6C(ctx, base);
		return;
	}
	// 82DE4E5C: C004001C  lfs f0, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE4E60: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE4E64: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DE4E68: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4E6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4E6C size=44
    let mut pc: u32 = 0x82DE4E6C;
    'dispatch: loop {
        match pc {
            0x82DE4E6C => {
    //   block [0x82DE4E6C..0x82DE4E98)
	// 82DE4E6C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE4E70: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82DE4E74: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DE4E78: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4E98 size=64
    let mut pc: u32 = 0x82DE4E98;
    'dispatch: loop {
        match pc {
            0x82DE4E98 => {
    //   block [0x82DE4E98..0x82DE4ED8)
	// 82DE4E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE4E9C: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE4EA0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DE4EA4: 99430022  stb r10, 0x22(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(34 as u32), ctx.r[10].u8 ) };
	// 82DE4EA8: 89430021  lbz r10, 0x21(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DE4EAC: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DE4EB0: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82DE4EB4: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82DE4EB8: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4EBC: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82DE4EC0: A14A0002  lhz r10, 2(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE4EC4: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 82DE4EC8: 89630021  lbz r11, 0x21(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DE4ECC: 396B00FF  addi r11, r11, 0xff
	ctx.r[11].s64 = ctx.r[11].s64 + 255;
	// 82DE4ED0: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82DE4ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4ED8 size=52
    let mut pc: u32 = 0x82DE4ED8;
    'dispatch: loop {
        match pc {
            0x82DE4ED8 => {
    //   block [0x82DE4ED8..0x82DE4F0C)
	// 82DE4ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE4EDC: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82DE4EE0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82DE4EE4: B163000A  sth r11, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82DE4EE8: B163000E  sth r11, 0xe(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82DE4EEC: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 82DE4EF0: B1630016  sth r11, 0x16(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[11].u16 ) };
	// 82DE4EF4: B163001A  sth r11, 0x1a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 82DE4EF8: B163001E  sth r11, 0x1e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u16 ) };
	// 82DE4EFC: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82DE4F00: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82DE4F04: 99630022  stb r11, 0x22(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(34 as u32), ctx.r[11].u8 ) };
	// 82DE4F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE4F10 size=128
    let mut pc: u32 = 0x82DE4F10;
    'dispatch: loop {
        match pc {
            0x82DE4F10 => {
    //   block [0x82DE4F10..0x82DE4F90)
	// 82DE4F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE4F14: 4BEC44F9  bl 0x82ca940c
	ctx.lr = 0x82DE4F18;
	sub_82CA93D0(ctx, base);
	// 82DE4F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE4F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE4F20: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DE4F24: 8BBF0021  lbz r29, 0x21(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DE4F28: 2B1D0008  cmplwi cr6, r29, 8
	ctx.cr[6].compare_u32(ctx.r[29].u32, 8 as u32, &mut ctx.xer);
	// 82DE4F2C: 41990058  bgt cr6, 0x82de4f84
	if ctx.cr[6].gt {
	pc = 0x82DE4F84; continue 'dispatch;
	}
	// 82DE4F30: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DE4F34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE4F38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE4F3C: 48000055  bl 0x82de4f90
	ctx.lr = 0x82DE4F40;
	sub_82DE4F90(ctx, base);
	// 82DE4F40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4F44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE4F48: 409A003C  bne cr6, 0x82de4f84
	if !ctx.cr[6].eq {
	pc = 0x82DE4F84; continue 'dispatch;
	}
	// 82DE4F4C: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 82DE4F50: 40980034  bge cr6, 0x82de4f84
	if !ctx.cr[6].lt {
	pc = 0x82DE4F84; continue 'dispatch;
	}
	// 82DE4F54: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE4F58: A15E0000  lhz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4F5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE4F60: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DE4F64: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82DE4F68: A15E0002  lhz r10, 2(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE4F6C: B14B0002  sth r10, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 82DE4F70: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DE4F74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE4F78: 997F0021  stb r11, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82DE4F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE4F80: 4BEC44DC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82DE4F84: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE4F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE4F8C: 4BEC44D0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4F90 size=84
    let mut pc: u32 = 0x82DE4F90;
    'dispatch: loop {
        match pc {
            0x82DE4F90 => {
    //   block [0x82DE4F90..0x82DE4FE4)
	// 82DE4F90: 89640021  lbz r11, 0x21(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DE4F94: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82DE4F98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE4F9C: 4198003C  blt cr6, 0x82de4fd8
	if ctx.cr[6].lt {
	pc = 0x82DE4FD8; continue 'dispatch;
	}
	// 82DE4FA0: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE4FA4: 89250000  lbz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4FA8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DE4FAC: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4FB0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE4FB4: 409A0014  bne cr6, 0x82de4fc8
	if !ctx.cr[6].eq {
	pc = 0x82DE4FC8; continue 'dispatch;
	}
	// 82DE4FB8: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE4FBC: 88E50001  lbz r7, 1(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE4FC0: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DE4FC4: 419A0020  beq cr6, 0x82de4fe4
	if ctx.cr[6].eq {
		sub_82DE4FE4(ctx, base);
		return;
	}
	// 82DE4FC8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DE4FCC: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82DE4FD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE4FD4: 4098FFD8  bge cr6, 0x82de4fac
	if !ctx.cr[6].lt {
	pc = 0x82DE4FAC; continue 'dispatch;
	}
	// 82DE4FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE4FDC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DE4FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4FE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4FE4 size=12
    let mut pc: u32 = 0x82DE4FE4;
    'dispatch: loop {
        match pc {
            0x82DE4FE4 => {
    //   block [0x82DE4FE4..0x82DE4FF0)
	// 82DE4FE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DE4FE8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DE4FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE4FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE4FF0 size=44
    let mut pc: u32 = 0x82DE4FF0;
    'dispatch: loop {
        match pc {
            0x82DE4FF0 => {
    //   block [0x82DE4FF0..0x82DE501C)
	// 82DE4FF0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4FF4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DE4FF8: 39200017  li r9, 0x17
	ctx.r[9].s64 = 23;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE5020 size=36
    let mut pc: u32 = 0x82DE5020;
    'dispatch: loop {
        match pc {
            0x82DE5020 => {
    //   block [0x82DE5020..0x82DE5044)
	// 82DE5020: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE5024: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE5048 size=4
    let mut pc: u32 = 0x82DE5048;
    'dispatch: loop {
        match pc {
            0x82DE5048 => {
    //   block [0x82DE5048..0x82DE504C)
	// 82DE5048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE5050 size=352
    let mut pc: u32 = 0x82DE5050;
    'dispatch: loop {
        match pc {
            0x82DE5050 => {
    //   block [0x82DE5050..0x82DE51B0)
	// 82DE5050: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE5054: 80E40030  lwz r7, 0x30(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE5058: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82DE505C: C1A40020  lfs f13, 0x20(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE5060: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE5064: 80C40034  lwz r6, 0x34(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE5068: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82DE506C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DE5070: 89470001  lbz r10, 1(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(1 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE51B0 size=376
    let mut pc: u32 = 0x82DE51B0;
    'dispatch: loop {
        match pc {
            0x82DE51B0 => {
    //   block [0x82DE51B0..0x82DE5328)
	// 82DE51B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE51B4: 4BEC4239  bl 0x82ca93ec
	ctx.lr = 0x82DE51B8;
	sub_82CA93D0(ctx, base);
	// 82DE51B8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE51BC: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE51C0: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82DE51C4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE51C8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE51CC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DE51D0: 41980150  blt cr6, 0x82de5320
	if ctx.cr[6].lt {
	pc = 0x82DE5320; continue 'dispatch;
	}
	// 82DE51D4: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 82DE51D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DE51DC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DE51E0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DE51E4: 3BCA4C40  addi r30, r10, 0x4c40
	ctx.r[30].s64 = ctx.r[10].s64 + 19520;
	// 82DE51E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE51EC: 3B030008  addi r24, r3, 8
	ctx.r[24].s64 = ctx.r[3].s64 + 8;
	// 82DE51F0: C1A70C14  lfs f13, 0xc14(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE51F4: 3AC40034  addi r22, r4, 0x34
	ctx.r[22].s64 = ctx.r[4].s64 + 52;
	// 82DE51F8: C0080C38  lfs f0, 0xc38(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE51FC: 3AA40030  addi r21, r4, 0x30
	ctx.r[21].s64 = ctx.r[4].s64 + 48;
	// 82DE5200: 3B240008  addi r25, r4, 8
	ctx.r[25].s64 = ctx.r[4].s64 + 8;
	// 82DE5204: 3AE40004  addi r23, r4, 4
	ctx.r[23].s64 = ctx.r[4].s64 + 4;
	// 82DE5208: 3BAA2AF0  addi r29, r10, 0x2af0
	ctx.r[29].s64 = ctx.r[10].s64 + 10992;
	// 82DE520C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82DE5210: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 82DE5214: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82DE5218: 3B40000D  li r26, 0xd
	ctx.r[26].s64 = 13;
	// 82DE521C: 80950000  lwz r4, 0(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE5328 size=412
    let mut pc: u32 = 0x82DE5328;
    'dispatch: loop {
        match pc {
            0x82DE5328 => {
    //   block [0x82DE5328..0x82DE54C4)
	// 82DE5328: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DE532C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE5330: 38E30010  addi r7, r3, 0x10
	ctx.r[7].s64 = ctx.r[3].s64 + 16;
	// 82DE5334: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE5338: 81440030  lwz r10, 0x30(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE533C: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82DE5340: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE5344: C1A40020  lfs f13, 0x20(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE5348: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82DE534C: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE54C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE54C8 size=460
    let mut pc: u32 = 0x82DE54C8;
    'dispatch: loop {
        match pc {
            0x82DE54C8 => {
    //   block [0x82DE54C8..0x82DE5694)
	// 82DE54C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE54CC: 4BEC3F41  bl 0x82ca940c
	ctx.lr = 0x82DE54D0;
	sub_82CA93D0(ctx, base);
	// 82DE54D0: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 82DE54D4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE54D8: 81440030  lwz r10, 0x30(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE54DC: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82DE54E0: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE54E4: C1A40020  lfs f13, 0x20(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE54E8: C184001C  lfs f12, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE54EC: 3BC4001C  addi r30, r4, 0x1c
	ctx.r[30].s64 = ctx.r[4].s64 + 28;
	// 82DE54F0: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE5698 size=348
    let mut pc: u32 = 0x82DE5698;
    'dispatch: loop {
        match pc {
            0x82DE5698 => {
    //   block [0x82DE5698..0x82DE57F4)
	// 82DE5698: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 82DE569C: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE56A0: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82DE56A4: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE56A8: C1A40020  lfs f13, 0x20(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE56AC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE57F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE57F8 size=380
    let mut pc: u32 = 0x82DE57F8;
    'dispatch: loop {
        match pc {
            0x82DE57F8 => {
    //   block [0x82DE57F8..0x82DE5974)
	// 82DE57F8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE57FC: 38E30020  addi r7, r3, 0x20
	ctx.r[7].s64 = ctx.r[3].s64 + 32;
	// 82DE5800: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE5804: 81440030  lwz r10, 0x30(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE5808: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82DE580C: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE5810: C1A40020  lfs f13, 0x20(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE5814: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE5978 size=348
    let mut pc: u32 = 0x82DE5978;
    'dispatch: loop {
        match pc {
            0x82DE5978 => {
    //   block [0x82DE5978..0x82DE5AD4)
	// 82DE5978: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 82DE597C: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE5980: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82DE5984: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE5988: C1A40020  lfs f13, 0x20(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE598C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE5AD8 size=440
    let mut pc: u32 = 0x82DE5AD8;
    'dispatch: loop {
        match pc {
            0x82DE5AD8 => {
    //   block [0x82DE5AD8..0x82DE5C90)
	// 82DE5AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE5ADC: 4BEC3931  bl 0x82ca940c
	ctx.lr = 0x82DE5AE0;
	sub_82CA93D0(ctx, base);
	// 82DE5AE0: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 82DE5AE4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE5AE8: 81440030  lwz r10, 0x30(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE5AEC: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82DE5AF0: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE5AF4: C184001C  lfs f12, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE5AF8: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 82DE5AFC: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE5C90 size=456
    let mut pc: u32 = 0x82DE5C90;
    'dispatch: loop {
        match pc {
            0x82DE5C90 => {
    //   block [0x82DE5C90..0x82DE5E58)
	// 82DE5C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE5C94: 4BEC3775  bl 0x82ca9408
	ctx.lr = 0x82DE5C98;
	sub_82CA93D0(ctx, base);
	// 82DE5C98: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82DE5C9C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE5CA0: 81440030  lwz r10, 0x30(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE5CA4: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
	// 82DE5CA8: 81040034  lwz r8, 0x34(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE5CAC: C1A40020  lfs f13, 0x20(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE5CB0: C184001C  lfs f12, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE5CB4: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 82DE5CB8: 3BC4001C  addi r30, r4, 0x1c
	ctx.r[30].s64 = ctx.r[4].s64 + 28;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE5E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE5E58 size=704
    let mut pc: u32 = 0x82DE5E58;
    'dispatch: loop {
        match pc {
            0x82DE5E58 => {
    //   block [0x82DE5E58..0x82DE6118)
	// 82DE5E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE5E5C: 4BEC3589  bl 0x82ca93e4
	ctx.lr = 0x82DE5E60;
	sub_82CA93D0(ctx, base);
	// 82DE5E60: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82DE5E64: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE5E68: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DE5E6C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DE5E70: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82DE5E74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DE5E78: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DE5E7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE5E80: 835E0030  lwz r26, 0x30(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE5E84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DE5E88: 837E0034  lwz r27, 0x34(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE5E8C: 48000A85  bl 0x82de6910
	ctx.lr = 0x82DE5E90;
	sub_82DE6910(ctx, base);
	// 82DE5E90: C13F000C  lfs f9, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DE5E94: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DE5E98: C11F003C  lfs f8, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DE5E9C: 3ABF000C  addi r21, r31, 0xc
	ctx.r[21].s64 = ctx.r[31].s64 + 12;
	// 82DE5EA0: C0FF006C  lfs f7, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82DE5EA4: 3A9F003C  addi r20, r31, 0x3c
	ctx.r[20].s64 = ctx.r[31].s64 + 60;
	// 82DE5EA8: D1210060  stfs f9, 0x60(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DE5EAC: 3A7F006C  addi r19, r31, 0x6c
	ctx.r[19].s64 = ctx.r[31].s64 + 108;
	// 82DE5EB0: D1010064  stfs f8, 0x64(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DE5EB4: 3ADE001C  addi r22, r30, 0x1c
	ctx.r[22].s64 = ctx.r[30].s64 + 28;
	// 82DE5EB8: D0E10068  stfs f7, 0x68(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DE5EBC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DE5EC0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE6118 size=196
    let mut pc: u32 = 0x82DE6118;
    'dispatch: loop {
        match pc {
            0x82DE6118 => {
    //   block [0x82DE6118..0x82DE61DC)
	// 82DE6118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE611C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE6120: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE6124: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DE6128: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DE612C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE61E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE61E0 size=92
    let mut pc: u32 = 0x82DE61E0;
    'dispatch: loop {
        match pc {
            0x82DE61E0 => {
    //   block [0x82DE61E0..0x82DE623C)
	// 82DE61E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE61E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE61E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE61EC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DE61F0: 89230002  lbz r9, 2(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE61F4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DE61F8: 5529203E  rotlwi r9, r9, 4
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(4)) as u64;
	// 82DE61FC: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82DE6200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6240 size=28
    let mut pc: u32 = 0x82DE6240;
    'dispatch: loop {
        match pc {
            0x82DE6240 => {
    //   block [0x82DE6240..0x82DE625C)
	// 82DE6240: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6244: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82DE6248: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82DE624C: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82DE6250: 986B0004  stb r3, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u8 ) };
	// 82DE6254: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE6258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6260 size=16
    let mut pc: u32 = 0x82DE6260;
    'dispatch: loop {
        match pc {
            0x82DE6260 => {
    //   block [0x82DE6260..0x82DE6270)
	// 82DE6260: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6264: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DE6268: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE626C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6270 size=36
    let mut pc: u32 = 0x82DE6270;
    'dispatch: loop {
        match pc {
            0x82DE6270 => {
    //   block [0x82DE6270..0x82DE6294)
	// 82DE6270: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6274: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE6278: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82DE627C: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 82DE6280: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE6284: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE6288: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE628C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE6290: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6298 size=108
    let mut pc: u32 = 0x82DE6298;
    'dispatch: loop {
        match pc {
            0x82DE6298 => {
    //   block [0x82DE6298..0x82DE6304)
	// 82DE6298: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6308 size=232
    let mut pc: u32 = 0x82DE6308;
    'dispatch: loop {
        match pc {
            0x82DE6308 => {
    //   block [0x82DE6308..0x82DE63F0)
	// 82DE6308: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DE630C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE6310: 39450030  addi r10, r5, 0x30
	ctx.r[10].s64 = ctx.r[5].s64 + 48;
	// 82DE6314: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DE6318: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82DE631C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82DE6320: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82DE6324: 39030040  addi r8, r3, 0x40
	ctx.r[8].s64 = ctx.r[3].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE63F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE63F0 size=136
    let mut pc: u32 = 0x82DE63F0;
    'dispatch: loop {
        match pc {
            0x82DE63F0 => {
    //   block [0x82DE63F0..0x82DE6478)
	// 82DE63F0: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82DE63F4: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DE63F8: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 82DE63FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82DE6400: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82DE6404: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6478 size=180
    let mut pc: u32 = 0x82DE6478;
    'dispatch: loop {
        match pc {
            0x82DE6478 => {
    //   block [0x82DE6478..0x82DE652C)
	// 82DE6478: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE647C: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DE6480: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82DE6484: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82DE6488: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82DE648C: 39050030  addi r8, r5, 0x30
	ctx.r[8].s64 = ctx.r[5].s64 + 48;
	// 82DE6490: 39230040  addi r9, r3, 0x40
	ctx.r[9].s64 = ctx.r[3].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE6530 size=176
    let mut pc: u32 = 0x82DE6530;
    'dispatch: loop {
        match pc {
            0x82DE6530 => {
    //   block [0x82DE6530..0x82DE65E0)
	// 82DE6530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE6534: 4BEC2ED5  bl 0x82ca9408
	ctx.lr = 0x82DE6538;
	sub_82CA93D0(ctx, base);
	// 82DE6538: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE653C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE6540: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE6544: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DE6548: 394A43C0  addi r10, r10, 0x43c0
	ctx.r[10].s64 = ctx.r[10].s64 + 17344;
	// 82DE654C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82DE6550: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82DE6554: 38EA0002  addi r7, r10, 2
	ctx.r[7].s64 = ctx.r[10].s64 + 2;
	// 82DE6558: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DE655C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE6560: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE6564: 7D4B40AE  lbzx r10, r11, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DE6568: 5568203E  rotlwi r8, r11, 4
	ctx.r[8].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE656C: 7D6B38AE  lbzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DE6570: 7FAA3214  add r29, r10, r6
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82DE6574: 7F8B3214  add r28, r11, r6
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[6].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE65E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE65E0 size=168
    let mut pc: u32 = 0x82DE65E0;
    'dispatch: loop {
        match pc {
            0x82DE65E0 => {
    //   block [0x82DE65E0..0x82DE6688)
	// 82DE65E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE65E4: 4BEC2E19  bl 0x82ca93fc
	ctx.lr = 0x82DE65E8;
	sub_82CA93D0(ctx, base);
	// 82DE65E8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE65EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE65F0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DE65F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DE65F8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DE65FC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DE6600: 8BDD0002  lbz r30, 2(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE6604: 897D0003  lbz r11, 3(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE6608: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DE660C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DE6610: 40980070  bge cr6, 0x82de6680
	if !ctx.cr[6].lt {
	pc = 0x82DE6680; continue 'dispatch;
	}
	// 82DE6614: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE6618: 396B43C0  addi r11, r11, 0x43c0
	ctx.r[11].s64 = ctx.r[11].s64 + 17344;
	// 82DE661C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE6620: 7FFE5A14  add r31, r30, r11
	ctx.r[31].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82DE6624: 897FFFFF  lbz r11, -1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82DE6628: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82DE662C: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6630: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DE6634: 893F0001  lbz r9, 1(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE6638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE6688 size=180
    let mut pc: u32 = 0x82DE6688;
    'dispatch: loop {
        match pc {
            0x82DE6688 => {
    //   block [0x82DE6688..0x82DE673C)
	// 82DE6688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE668C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE6690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE6694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE6698: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE669C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82DE66A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE66A4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE66A8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DE66AC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE66B0: 3909FFF0  addi r8, r9, -0x10
	ctx.r[8].s64 = ctx.r[9].s64 + -16;
	// 82DE66B4: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DE66B8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE66BC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DE66C0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DE66C4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE66C8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DE66CC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DE66D0: 4200FFF0  bdnz 0x82de66c0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DE66C0; continue 'dispatch;
	}
	// 82DE66D4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE66D8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE66DC: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 82DE66E0: 4198000C  blt cr6, 0x82de66ec
	if ctx.cr[6].lt {
	pc = 0x82DE66EC; continue 'dispatch;
	}
	// 82DE66E4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE66E8: 4BFFFFF0  b 0x82de66d8
	pc = 0x82DE66D8; continue 'dispatch;
	// 82DE66EC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE66F0: 4800D101  bl 0x82df37f0
	ctx.lr = 0x82DE66F4;
	sub_82DF37F0(ctx, base);
	// 82DE66F4: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE66F8: A09E0012  lhz r4, 0x12(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DE66FC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DE6700: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6704: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 82DE6708: 409A0014  bne cr6, 0x82de671c
	if !ctx.cr[6].eq {
	pc = 0x82DE671C; continue 'dispatch;
	}
	// 82DE670C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE6710: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE6714: 4800D0F5  bl 0x82df3808
	ctx.lr = 0x82DE6718;
	sub_82DF3808(ctx, base);
	// 82DE6718: 4800000C  b 0x82de6724
	pc = 0x82DE6724; continue 'dispatch;
	// 82DE671C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE6720: 48002901  bl 0x82de9020
	ctx.lr = 0x82DE6724;
	sub_82DE9020(ctx, base);
	// 82DE6724: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DE6728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE672C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE6730: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE6734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE6738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE6740 size=460
    let mut pc: u32 = 0x82DE6740;
    'dispatch: loop {
        match pc {
            0x82DE6740 => {
    //   block [0x82DE6740..0x82DE690C)
	// 82DE6740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE6744: 4BEC2C9D  bl 0x82ca93e0
	ctx.lr = 0x82DE6748;
	sub_82CA93D0(ctx, base);
	// 82DE6748: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DE674C: 83E50030  lwz r31, 0x30(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE6750: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82DE6754: 83C50034  lwz r30, 0x34(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE6758: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE675C: 394B6E60  addi r10, r11, 0x6e60
	ctx.r[10].s64 = ctx.r[11].s64 + 28256;
	// 82DE6760: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6764: 3B450020  addi r26, r5, 0x20
	ctx.r[26].s64 = ctx.r[5].s64 + 32;
	// 82DE6768: 3B25001C  addi r25, r5, 0x1c
	ctx.r[25].s64 = ctx.r[5].s64 + 28;
	// 82DE676C: C1680C38  lfs f11, 0xc38(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3128 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE6770: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DE6774: 3B1F0040  addi r24, r31, 0x40
	ctx.r[24].s64 = ctx.r[31].s64 + 64;
	// 82DE6778: 3AFE0040  addi r23, r30, 0x40
	ctx.r[23].s64 = ctx.r[30].s64 + 64;
	// 82DE677C: 3ADF0001  addi r22, r31, 1
	ctx.r[22].s64 = ctx.r[31].s64 + 1;
	// 82DE6780: 3A5E0001  addi r18, r30, 1
	ctx.r[18].s64 = ctx.r[30].s64 + 1;
	// 82DE6784: 3BBE0030  addi r29, r30, 0x30
	ctx.r[29].s64 = ctx.r[30].s64 + 48;
	// 82DE6788: 3B9F0030  addi r28, r31, 0x30
	ctx.r[28].s64 = ctx.r[31].s64 + 48;
	// 82DE678C: 3B694C40  addi r27, r9, 0x4c40
	ctx.r[27].s64 = ctx.r[9].s64 + 19520;
	// 82DE6790: 3A800010  li r20, 0x10
	ctx.r[20].s64 = 16;
	// 82DE6794: 3AA00020  li r21, 0x20
	ctx.r[21].s64 = 32;
	// 82DE6798: 3A600005  li r19, 5
	ctx.r[19].s64 = 5;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE6910 size=448
    let mut pc: u32 = 0x82DE6910;
    'dispatch: loop {
        match pc {
            0x82DE6910 => {
    //   block [0x82DE6910..0x82DE6AD0)
	// 82DE6910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE6914: 4BEC2AD5  bl 0x82ca93e8
	ctx.lr = 0x82DE6918;
	sub_82CA93D0(ctx, base);
	// 82DE6918: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DE691C: 83E50030  lwz r31, 0x30(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE6920: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82DE6924: 80E50034  lwz r7, 0x34(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE6928: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE692C: 394B6E60  addi r10, r11, 0x6e60
	ctx.r[10].s64 = ctx.r[11].s64 + 28256;
	// 82DE6930: 3BC50020  addi r30, r5, 0x20
	ctx.r[30].s64 = ctx.r[5].s64 + 32;
	// 82DE6934: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82DE6938: C1680C38  lfs f11, 0xc38(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3128 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE693C: 38A5001C  addi r5, r5, 0x1c
	ctx.r[5].s64 = ctx.r[5].s64 + 28;
	// 82DE6940: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82DE6944: 3BBF0040  addi r29, r31, 0x40
	ctx.r[29].s64 = ctx.r[31].s64 + 64;
	// 82DE6948: 3B870040  addi r28, r7, 0x40
	ctx.r[28].s64 = ctx.r[7].s64 + 64;
	// 82DE694C: 3B7F0001  addi r27, r31, 1
	ctx.r[27].s64 = ctx.r[31].s64 + 1;
	// 82DE6950: 3A870001  addi r20, r7, 1
	ctx.r[20].s64 = ctx.r[7].s64 + 1;
	// 82DE6954: 3B470030  addi r26, r7, 0x30
	ctx.r[26].s64 = ctx.r[7].s64 + 48;
	// 82DE6958: 3B3F0030  addi r25, r31, 0x30
	ctx.r[25].s64 = ctx.r[31].s64 + 48;
	// 82DE695C: 3AC94C40  addi r22, r9, 0x4c40
	ctx.r[22].s64 = ctx.r[9].s64 + 19520;
	// 82DE6960: 3AA0FFF0  li r21, -0x10
	ctx.r[21].s64 = -16;
	// 82DE6964: 3AE00010  li r23, 0x10
	ctx.r[23].s64 = 16;
	// 82DE6968: 3B000020  li r24, 0x20
	ctx.r[24].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE6AD0 size=352
    let mut pc: u32 = 0x82DE6AD0;
    'dispatch: loop {
        match pc {
            0x82DE6AD0 => {
    //   block [0x82DE6AD0..0x82DE6C30)
	// 82DE6AD0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE6AD4: 81040030  lwz r8, 0x30(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE6AD8: C003001C  lfs f0, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE6ADC: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6C30 size=112
    let mut pc: u32 = 0x82DE6C30;
    'dispatch: loop {
        match pc {
            0x82DE6C30 => {
    //   block [0x82DE6C30..0x82DE6CA0)
	// 82DE6C30: 81240030  lwz r9, 0x30(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6CA0 size=160
    let mut pc: u32 = 0x82DE6CA0;
    'dispatch: loop {
        match pc {
            0x82DE6CA0 => {
    //   block [0x82DE6CA0..0x82DE6D40)
	// 82DE6CA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE6CA4: 39480050  addi r10, r8, 0x50
	ctx.r[10].s64 = ctx.r[8].s64 + 80;
	// 82DE6CA8: 396B2AF0  addi r11, r11, 0x2af0
	ctx.r[11].s64 = ctx.r[11].s64 + 10992;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6D40 size=112
    let mut pc: u32 = 0x82DE6D40;
    'dispatch: loop {
        match pc {
            0x82DE6D40 => {
    //   block [0x82DE6D40..0x82DE6DB0)
	// 82DE6D40: 81240030  lwz r9, 0x30(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6DB0 size=160
    let mut pc: u32 = 0x82DE6DB0;
    'dispatch: loop {
        match pc {
            0x82DE6DB0 => {
    //   block [0x82DE6DB0..0x82DE6E50)
	// 82DE6DB0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE6DB4: 39480050  addi r10, r8, 0x50
	ctx.r[10].s64 = ctx.r[8].s64 + 80;
	// 82DE6DB8: 396B2AF0  addi r11, r11, 0x2af0
	ctx.r[11].s64 = ctx.r[11].s64 + 10992;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE6E50 size=96
    let mut pc: u32 = 0x82DE6E50;
    'dispatch: loop {
        match pc {
            0x82DE6E50 => {
    //   block [0x82DE6E50..0x82DE6EB0)
	// 82DE6E50: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6E54: 3940000E  li r10, 0xe
	ctx.r[10].s64 = 14;
	// 82DE6E58: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE6E5C: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE6E60: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DE6E64: D005001C  stfs f0, 0x1c(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DE6E68: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82DE6E6C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE6E70: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE6E74: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DE6E78: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DE6E7C: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DE6E80: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE6E84: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE6E88: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DE6E8C: D00B0024  stfs f0, 0x24(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82DE6E90: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE6E94: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE6E98: D00B0028  stfs f0, 0x28(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82DE6E9C: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE6EA0: D00B002C  stfs f0, 0x2c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82DE6EA4: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE6EA8: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DE6EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE6EB0 size=468
    let mut pc: u32 = 0x82DE6EB0;
    'dispatch: loop {
        match pc {
            0x82DE6EB0 => {
    //   block [0x82DE6EB0..0x82DE7084)
	// 82DE6EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE6EB4: 4BEC252D  bl 0x82ca93e0
	ctx.lr = 0x82DE6EB8;
	sub_82CA93D0(ctx, base);
	// 82DE6EB8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DE6EBC: 80640030  lwz r3, 0x30(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE6EC0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82DE6EC4: 83E40034  lwz r31, 0x34(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE6EC8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE6ECC: 394B6E60  addi r10, r11, 0x6e60
	ctx.r[10].s64 = ctx.r[11].s64 + 28256;
	// 82DE6ED0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE6ED4: 3BA50030  addi r29, r5, 0x30
	ctx.r[29].s64 = ctx.r[5].s64 + 48;
	// 82DE6ED8: 3B640020  addi r27, r4, 0x20
	ctx.r[27].s64 = ctx.r[4].s64 + 32;
	// 82DE6EDC: C1680C38  lfs f11, 0xc38(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3128 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE6EE0: 3B44001C  addi r26, r4, 0x1c
	ctx.r[26].s64 = ctx.r[4].s64 + 28;
	// 82DE6EE4: 3BC60030  addi r30, r6, 0x30
	ctx.r[30].s64 = ctx.r[6].s64 + 48;
	// 82DE6EE8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82DE6EEC: 3B230040  addi r25, r3, 0x40
	ctx.r[25].s64 = ctx.r[3].s64 + 64;
	// 82DE6EF0: 3B1F0040  addi r24, r31, 0x40
	ctx.r[24].s64 = ctx.r[31].s64 + 64;
	// 82DE6EF4: 3AE30001  addi r23, r3, 1
	ctx.r[23].s64 = ctx.r[3].s64 + 1;
	// 82DE6EF8: 3A5F0001  addi r18, r31, 1
	ctx.r[18].s64 = ctx.r[31].s64 + 1;
	// 82DE6EFC: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82DE6F00: 3B830030  addi r28, r3, 0x30
	ctx.r[28].s64 = ctx.r[3].s64 + 48;
	// 82DE6F04: 3AC94C40  addi r22, r9, 0x4c40
	ctx.r[22].s64 = ctx.r[9].s64 + 19520;
	// 82DE6F08: 3A800010  li r20, 0x10
	ctx.r[20].s64 = 16;
	// 82DE6F0C: 3AA00020  li r21, 0x20
	ctx.r[21].s64 = 32;
	// 82DE6F10: 3A600005  li r19, 5
	ctx.r[19].s64 = 5;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE7088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE7088 size=112
    let mut pc: u32 = 0x82DE7088;
    'dispatch: loop {
        match pc {
            0x82DE7088 => {
    //   block [0x82DE7088..0x82DE70F8)
	// 82DE7088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE708C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE7090: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE7094: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82DE7098: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82DE709C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE70A0: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82DE70A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE70F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE70F8 size=100
    let mut pc: u32 = 0x82DE70F8;
    'dispatch: loop {
        match pc {
            0x82DE70F8 => {
    //   block [0x82DE70F8..0x82DE715C)
	// 82DE70F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE70FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE7100: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE7104: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82DE7108: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE710C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DE7110: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82DE7114: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DE7118: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE7160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE7160 size=340
    let mut pc: u32 = 0x82DE7160;
    'dispatch: loop {
        match pc {
            0x82DE7160 => {
    //   block [0x82DE7160..0x82DE72B4)
	// 82DE7160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE7164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE7168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE716C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE7170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE7174: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE7178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE717C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82DE7180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE7184: 419A0114  beq cr6, 0x82de7298
	if ctx.cr[6].eq {
	pc = 0x82DE7298; continue 'dispatch;
	}
	// 82DE7188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE718C: 89630003  lbz r11, 3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE7190: C01E0040  lfs f0, 0x40(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE7194: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82DE7198: 394A43C0  addi r10, r10, 0x43c0
	ctx.r[10].s64 = ctx.r[10].s64 + 17344;
	// 82DE719C: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE71A0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DE71A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE71A8: 392A0002  addi r9, r10, 2
	ctx.r[9].s64 = ctx.r[10].s64 + 2;
	// 82DE71AC: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82DE71B0: 7D4B48AE  lbzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DE71B4: 7D2B40AE  lbzx r9, r11, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DE71B8: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE71BC: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE72B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE72B8 size=376
    let mut pc: u32 = 0x82DE72B8;
    'dispatch: loop {
        match pc {
            0x82DE72B8 => {
    //   block [0x82DE72B8..0x82DE7430)
	// 82DE72B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE72BC: 4BEC2151  bl 0x82ca940c
	ctx.lr = 0x82DE72C0;
	sub_82CA93D0(ctx, base);
	// 82DE72C0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82DE72C4: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE7430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE7430 size=356
    let mut pc: u32 = 0x82DE7430;
    'dispatch: loop {
        match pc {
            0x82DE7430 => {
    //   block [0x82DE7430..0x82DE7594)
	// 82DE7430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE7434: 4BEC1FD5  bl 0x82ca9408
	ctx.lr = 0x82DE7438;
	sub_82CA93D0(ctx, base);
	// 82DE7438: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82DE743C: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE7598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE7598 size=500
    let mut pc: u32 = 0x82DE7598;
    'dispatch: loop {
        match pc {
            0x82DE7598 => {
    //   block [0x82DE7598..0x82DE778C)
	// 82DE7598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE759C: 4BEC1E6D  bl 0x82ca9408
	ctx.lr = 0x82DE75A0;
	sub_82CA93D0(ctx, base);
	// 82DE75A0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82DE75A4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82DE75A8: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE7790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE7790 size=156
    let mut pc: u32 = 0x82DE7790;
    'dispatch: loop {
        match pc {
            0x82DE7790 => {
    //   block [0x82DE7790..0x82DE782C)
	// 82DE7790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE7794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE7798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE779C: 89630002  lbz r11, 2(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE77A0: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82DE77A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE77A8: 419A0058  beq cr6, 0x82de7800
	if ctx.cr[6].eq {
	pc = 0x82DE7800; continue 'dispatch;
	}
	// 82DE77AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE77B0: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE77B4: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE77B8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DE77BC: 419A0044  beq cr6, 0x82de7800
	if ctx.cr[6].eq {
	pc = 0x82DE7800; continue 'dispatch;
	}
	// 82DE77C0: 89230004  lbz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE77C4: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82DE77C8: 89630003  lbz r11, 3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE77CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DE77D0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DE77D4: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE77D8: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82DE77DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE77E0: 8124004C  lwz r9, 0x4c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DE77E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DE77E8: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DE77EC: 4BFFD9C5  bl 0x82de51b0
	ctx.lr = 0x82DE77F0;
	sub_82DE51B0(ctx, base);
	// 82DE77F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE77F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE77F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE77FC: 4E800020  blr
	return;
	// 82DE7800: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE7804: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82DE7808: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE780C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DE7810: 992B0003  stb r9, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 82DE7814: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82DE7818: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE781C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE7820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE7824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE7828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE7830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE7830 size=876
    let mut pc: u32 = 0x82DE7830;
    'dispatch: loop {
        match pc {
            0x82DE7830 => {
    //   block [0x82DE7830..0x82DE7B9C)
	// 82DE7830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE7834: 4BEC1BC1  bl 0x82ca93f4
	ctx.lr = 0x82DE7838;
	sub_82CA93D0(ctx, base);
	// 82DE7838: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE783C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE7840: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DE7844: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DE7848: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DE784C: 3B1D004C  addi r24, r29, 0x4c
	ctx.r[24].s64 = ctx.r[29].s64 + 76;
	// 82DE7850: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE7854: A0FF0006  lhz r7, 6(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DE7858: 7D480734  extsh r8, r10
	ctx.r[8].s64 = ctx.r[10].s16 as i64;
	// 82DE785C: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DE7860: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE7864: 7CEA0734  extsh r10, r7
	ctx.r[10].s64 = ctx.r[7].s16 as i64;
	// 82DE7868: 7EE85A14  add r23, r8, r11
	ctx.r[23].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DE786C: 7F4A5A14  add r26, r10, r11
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DE7870: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DE7874: 419A0304  beq cr6, 0x82de7b78
	if ctx.cr[6].eq {
	pc = 0x82DE7B78; continue 'dispatch;
	}
	// 82DE7878: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE787C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE7880: 419A02F8  beq cr6, 0x82de7b78
	if ctx.cr[6].eq {
	pc = 0x82DE7B78; continue 'dispatch;
	}
	// 82DE7884: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE7888: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE788C: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82DE7890: 394A43C0  addi r10, r10, 0x43c0
	ctx.r[10].s64 = ctx.r[10].s64 + 17344;
	// 82DE7894: 5569203E  rotlwi r9, r11, 4
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE7898: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82DE789C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DE78A0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82DE78A4: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 82DE78A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE78AC: 7D0B40AE  lbzx r8, r11, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DE78B0: 3BDF0003  addi r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 3;
	// 82DE78B4: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE78B8: 7D68E214  add r11, r8, r28
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[28].u64;
	// 82DE78BC: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE7BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE7BA0 size=1200
    let mut pc: u32 = 0x82DE7BA0;
    'dispatch: loop {
        match pc {
            0x82DE7BA0 => {
    //   block [0x82DE7BA0..0x82DE8050)
	// 82DE7BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE7BA4: 4BEC182D  bl 0x82ca93d0
	ctx.lr = 0x82DE7BA8;
	sub_82CA93D0(ctx, base);
	// 82DE7BA8: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82DE7BAC: 4BEC6129  bl 0x82cadcd4
	ctx.lr = 0x82DE7BB0;
	sub_82CADCA0(ctx, base);
	// 82DE7BB0: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE8050 size=168
    let mut pc: u32 = 0x82DE8050;
    'dispatch: loop {
        match pc {
            0x82DE8050 => {
    //   block [0x82DE8050..0x82DE80F8)
	// 82DE8050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE8054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE8058: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE805C: 89430002  lbz r10, 2(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE8060: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DE8064: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DE8068: 419A0064  beq cr6, 0x82de80cc
	if ctx.cr[6].eq {
	pc = 0x82DE80CC; continue 'dispatch;
	}
	// 82DE806C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE8070: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8074: C1AA0C18  lfs f13, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE8078: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DE807C: 40990050  ble cr6, 0x82de80cc
	if !ctx.cr[6].gt {
	pc = 0x82DE80CC; continue 'dispatch;
	}
	// 82DE8080: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82DE8084: 89430003  lbz r10, 3(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE8088: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82DE808C: 554A203E  rotlwi r10, r10, 4
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 82DE8090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE80F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE80F8 size=792
    let mut pc: u32 = 0x82DE80F8;
    'dispatch: loop {
        match pc {
            0x82DE80F8 => {
    //   block [0x82DE80F8..0x82DE8410)
	// 82DE80F8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DE80FC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE8100: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE8410 size=776
    let mut pc: u32 = 0x82DE8410;
    'dispatch: loop {
        match pc {
            0x82DE8410 => {
    //   block [0x82DE8410..0x82DE8718)
	// 82DE8410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE8414: 4BEC0FF9  bl 0x82ca940c
	ctx.lr = 0x82DE8418;
	sub_82CA93D0(ctx, base);
	// 82DE8418: 3BC00030  li r30, 0x30
	ctx.r[30].s64 = 48;
	// 82DE841C: C1A30030  lfs f13, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE8420: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82DE8424: C0030034  lfs f0, 0x34(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8428: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 82DE842C: 81440030  lwz r10, 0x30(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE8430: 3901FF30  addi r8, r1, -0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + -208;
	// 82DE8434: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE8438: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE8718 size=56
    let mut pc: u32 = 0x82DE8718;
    'dispatch: loop {
        match pc {
            0x82DE8718 => {
    //   block [0x82DE8718..0x82DE8750)
	// 82DE8718: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE871C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DE8720: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DE8724: 81230050  lwz r9, 0x50(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE8728: 81030048  lwz r8, 0x48(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE872C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE8730: 98EB0003  stb r7, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 82DE8734: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DE8738: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DE873C: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DE8740: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DE8744: 98AB0001  stb r5, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[5].u8 ) };
	// 82DE8748: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DE874C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE8750 size=608
    let mut pc: u32 = 0x82DE8750;
    'dispatch: loop {
        match pc {
            0x82DE8750 => {
    //   block [0x82DE8750..0x82DE89B0)
	// 82DE8750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE8754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE8758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE875C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE8760: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE8764: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DE8768: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE876C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82DE8770: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82DE8774: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8778: 38EAFFF0  addi r7, r10, -0x10
	ctx.r[7].s64 = ctx.r[10].s64 + -16;
	// 82DE877C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DE8780: 90E60000  stw r7, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DE8784: C00B0020  lfs f0, 0x20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8788: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82DE878C: E90A0000  ld r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DE8790: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DE8794: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DE8798: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DE879C: 4200FFF0  bdnz 0x82de878c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DE878C; continue 'dispatch;
	}
	// 82DE87A0: C1A40020  lfs f13, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE87A4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE87A8: C1840040  lfs f12, 0x40(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE87AC: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DE87B0: C1640044  lfs f11, 0x44(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE87B4: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82DE87B8: EDAC0032  fmuls f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DE87BC: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DE87C0: D1A10090  stfs f13, 0x90(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82DE87C4: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82DE87C8: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE87CC: 2B090017  cmplwi cr6, r9, 0x17
	ctx.cr[6].compare_u32(ctx.r[9].u32, 23 as u32, &mut ctx.xer);
	// 82DE87D0: 4198000C  blt cr6, 0x82de87dc
	if ctx.cr[6].lt {
	pc = 0x82DE87DC; continue 'dispatch;
	}
	// 82DE87D4: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE87D8: 4BFFFFF0  b 0x82de87c8
	pc = 0x82DE87C8; continue 'dispatch;
	// 82DE87DC: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DE87E0: 3BCA0030  addi r30, r10, 0x30
	ctx.r[30].s64 = ctx.r[10].s64 + 48;
	// 82DE87E4: A3EA0004  lhz r31, 4(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE87E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE87EC: 5529283E  rotlwi r9, r9, 5
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(5)) as u64;
	// 82DE87F0: 88AA000A  lbz r5, 0xa(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DE87F4: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DE87F8: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DE87FC: 392A0030  addi r9, r10, 0x30
	ctx.r[9].s64 = ctx.r[10].s64 + 48;
	// 82DE8800: 41980108  blt cr6, 0x82de8908
	if ctx.cr[6].lt {
	pc = 0x82DE8908; continue 'dispatch;
	}
	// 82DE8804: 395FFFFC  addi r10, r31, -4
	ctx.r[10].s64 = ctx.r[31].s64 + -4;
	// 82DE8808: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE880C: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82DE8810: 395E003C  addi r10, r30, 0x3c
	ctx.r[10].s64 = ctx.r[30].s64 + 60;
	// 82DE8814: 5503103A  slwi r3, r8, 2
	ctx.r[3].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DE8818: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE881C: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE8820: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DE8824: C18B0024  lfs f12, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE8828: C1640004  lfs f11, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE882C: C00AFFE0  lfs f0, -0x20(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8830: C1490010  lfs f10, 0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DE8834: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DE8838: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DE883C: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DE8840: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82DE8844: 40980008  bge cr6, 0x82de884c
	if !ctx.cr[6].lt {
	pc = 0x82DE884C; continue 'dispatch;
	}
	// 82DE8848: D0090010  stfs f0, 0x10(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DE884C: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8850: 7D254A14  add r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82DE8854: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE8858: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DE885C: C18B0024  lfs f12, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE8860: C1640004  lfs f11, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE8864: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8868: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DE886C: C1890010  lfs f12, 0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE8870: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DE8874: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DE8878: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DE887C: 40980008  bge cr6, 0x82de8884
	if !ctx.cr[6].lt {
	pc = 0x82DE8884; continue 'dispatch;
	}
	// 82DE8880: D0090010  stfs f0, 0x10(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DE8884: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8888: 7D254A14  add r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82DE888C: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE8890: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DE8894: C18B0024  lfs f12, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE8898: C1640004  lfs f11, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE889C: C00A0020  lfs f0, 0x20(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE88A0: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DE88A4: C1890010  lfs f12, 0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE88A8: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DE88AC: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DE88B0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DE88B4: 40980008  bge cr6, 0x82de88bc
	if !ctx.cr[6].lt {
	pc = 0x82DE88BC; continue 'dispatch;
	}
	// 82DE88B8: D0090010  stfs f0, 0x10(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DE88BC: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE88C0: 7D254A14  add r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82DE88C4: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE88C8: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DE88CC: C18B0024  lfs f12, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE88D0: C1640004  lfs f11, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE88D4: C00A0040  lfs f0, 0x40(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE88D8: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DE88DC: C1890010  lfs f12, 0x10(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE88E0: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DE88E4: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DE88E8: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DE88EC: 40980008  bge cr6, 0x82de88f4
	if !ctx.cr[6].lt {
	pc = 0x82DE88F4; continue 'dispatch;
	}
	// 82DE88F0: D0090010  stfs f0, 0x10(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DE88F4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DE88F8: 7D254A14  add r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82DE88FC: 394A0080  addi r10, r10, 0x80
	ctx.r[10].s64 = ctx.r[10].s64 + 128;
	// 82DE8900: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DE8904: 409AFF14  bne cr6, 0x82de8818
	if !ctx.cr[6].eq {
	pc = 0x82DE8818; continue 'dispatch;
	}
	// 82DE8908: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DE890C: 40980060  bge cr6, 0x82de896c
	if !ctx.cr[6].lt {
	pc = 0x82DE896C; continue 'dispatch;
	}
	// 82DE8910: 546A2834  slwi r10, r3, 5
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE8914: 7D0AF214  add r8, r10, r30
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DE8918: 39490010  addi r10, r9, 0x10
	ctx.r[10].s64 = ctx.r[9].s64 + 16;
	// 82DE891C: 3928001C  addi r9, r8, 0x1c
	ctx.r[9].s64 = ctx.r[8].s64 + 28;
	// 82DE8920: 7D03F850  subf r8, r3, r31
	ctx.r[8].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82DE8924: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8928: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE892C: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DE8930: C18B0024  lfs f12, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DE8934: C1640004  lfs f11, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE8938: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE893C: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DE8940: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DE8944: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DE8948: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DE894C: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82DE8950: 40980008  bge cr6, 0x82de8958
	if !ctx.cr[6].lt {
	pc = 0x82DE8958; continue 'dispatch;
	}
	// 82DE8954: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DE8958: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DE895C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82DE8960: 39290020  addi r9, r9, 0x20
	ctx.r[9].s64 = ctx.r[9].s64 + 32;
	// 82DE8964: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DE8968: 409AFFBC  bne cr6, 0x82de8924
	if !ctx.cr[6].eq {
	pc = 0x82DE8924; continue 'dispatch;
	}
	// 82DE896C: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE8970: A08B0012  lhz r4, 0x12(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DE8974: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8978: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 82DE897C: 409A0014  bne cr6, 0x82de8990
	if !ctx.cr[6].eq {
	pc = 0x82DE8990; continue 'dispatch;
	}
	// 82DE8980: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE8984: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE8988: 4800AE81  bl 0x82df3808
	ctx.lr = 0x82DE898C;
	sub_82DF3808(ctx, base);
	// 82DE898C: 4800000C  b 0x82de8998
	pc = 0x82DE8998; continue 'dispatch;
	// 82DE8990: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE8994: 4800068D  bl 0x82de9020
	ctx.lr = 0x82DE8998;
	sub_82DE9020(ctx, base);
	// 82DE8998: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DE899C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE89A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE89A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE89A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE89AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE89B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE89B0 size=488
    let mut pc: u32 = 0x82DE89B0;
    'dispatch: loop {
        match pc {
            0x82DE89B0 => {
    //   block [0x82DE89B0..0x82DE8B98)
	// 82DE89B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE89B4: 4BEC0A4D  bl 0x82ca9400
	ctx.lr = 0x82DE89B8;
	sub_82CA93D0(ctx, base);
	// 82DE89B8: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE89BC: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE89C0: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82DE89C4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DE89C8: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82DE89CC: C0030024  lfs f0, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE89D0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE89D4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DE89D8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DE89DC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DE89E0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DE89E4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE89E8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DE89EC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DE89F0: 4200FFF0  bdnz 0x82de89e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DE89E0; continue 'dispatch;
	}
	// 82DE89F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE89F8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DE89FC: 83A30014  lwz r29, 0x14(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE8A00: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DE8A04: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DE8A08: A3830012  lhz r28, 0x12(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DE8A0C: 810100F4  lwz r8, 0xf4(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 82DE8A10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE8A14: 80E100F0  lwz r7, 0xf0(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 82DE8A18: 3B6B0010  addi r27, r11, 0x10
	ctx.r[27].s64 = ctx.r[11].s64 + 16;
	// 82DE8A1C: 80C100E8  lwz r6, 0xe8(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(232 as u32) ) } as u64;
	// 82DE8A20: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DE8A24: 3BC10100  addi r30, r1, 0x100
	ctx.r[30].s64 = ctx.r[1].s64 + 256;
	// 82DE8A28: 9B4B0003  stb r26, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[26].u8 ) };
	// 82DE8A2C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82DE8A30: B10B0006  sth r8, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DE8A34: B0EB0004  sth r7, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82DE8A38: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82DE8A3C: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DE8A40: 986B0001  stb r3, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[3].u8 ) };
	// 82DE8A44: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DE8A48: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE8A4C: 80C40034  lwz r6, 0x34(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE8A50: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82DE8A54: 38E60030  addi r7, r6, 0x30
	ctx.r[7].s64 = ctx.r[6].s64 + 48;
	// 82DE8A58: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82DE8A5C: F88A0000  std r4, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 82DE8A60: E9080008  ld r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 82DE8A64: F90A0008  std r8, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 82DE8A68: E9470000  ld r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 82DE8A6C: F9490000  std r10, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82DE8A70: E9470008  ld r10, 8(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 82DE8A74: F9490008  std r10, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82DE8A78: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82DE8A7C: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DE8A80: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE8A84: F95E0000  std r10, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82DE8A88: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82DE8A8C: 4200FFF0  bdnz 0x82de8a7c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DE8A7C; continue 'dispatch;
	}
	// 82DE8A90: 39610180  addi r11, r1, 0x180
	ctx.r[11].s64 = ctx.r[1].s64 + 384;
	// 82DE8A94: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DE8A98: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DE8A9C: E9460000  ld r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DE8AA0: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 82DE8AA4: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82DE8AA8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE8AAC: 4200FFF0  bdnz 0x82de8a9c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DE8A9C; continue 'dispatch;
	}
	// 82DE8AB0: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DE8AB4: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82DE8AB8: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82DE8ABC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DE8AC0: 390101B0  addi r8, r1, 0x1b0
	ctx.r[8].s64 = ctx.r[1].s64 + 432;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE8B98 size=676
    let mut pc: u32 = 0x82DE8B98;
    'dispatch: loop {
        match pc {
            0x82DE8B98 => {
    //   block [0x82DE8B98..0x82DE8E3C)
	// 82DE8B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE8B9C: 4BEC0869  bl 0x82ca9404
	ctx.lr = 0x82DE8BA0;
	sub_82CA93D0(ctx, base);
	// 82DE8BA0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE8BA4: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82DE8BA8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DE8BAC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DE8BB0: 390100F0  addi r8, r1, 0xf0
	ctx.r[8].s64 = ctx.r[1].s64 + 240;
	// 82DE8BB4: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82DE8BB8: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DE8BBC: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82DE8BC0: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82DE8BC4: F8CB0000  std r6, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82DE8BC8: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82DE8BCC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82DE8BD0: E9690000  ld r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DE8BD4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DE8BD8: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82DE8BDC: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82DE8BE0: 4200FFF0  bdnz 0x82de8bd0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DE8BD0; continue 'dispatch;
	}
	// 82DE8BE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8BE8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE8BEC: 83E30014  lwz r31, 0x14(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE8BF0: 3BA00070  li r29, 0x70
	ctx.r[29].s64 = 112;
	// 82DE8BF4: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DE8BF8: 3B810070  addi r28, r1, 0x70
	ctx.r[28].s64 = ctx.r[1].s64 + 112;
	// 82DE8BFC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE8C00: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE8C04: A13F0006  lhz r9, 6(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DE8C08: 895F000A  lbz r10, 0xa(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DE8C0C: 5527283E  rotlwi r7, r9, 5
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(5)) as u64;
	// 82DE8C10: 81040050  lwz r8, 0x50(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE8C14: 81240054  lwz r9, 0x54(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DE8C18: 7CE7FA14  add r7, r7, r31
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82DE8C1C: 80C40048  lwz r6, 0x48(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE8C20: 98AB0003  stb r5, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 82DE8C24: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82DE8C28: 38E70030  addi r7, r7, 0x30
	ctx.r[7].s64 = ctx.r[7].s64 + 48;
	// 82DE8C2C: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82DE8C30: B10B0004  sth r8, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82DE8C34: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82DE8C38: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DE8C3C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DE8C40: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DE8C44: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82DE8C48: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DE8C4C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DE8C50: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82DE8C54: 81640050  lwz r11, 0x50(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE8C58: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DE8C5C: 556ADFFE  rlwinm r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DE8C60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE8C64: 409A0088  bne cr6, 0x82de8cec
	if !ctx.cr[6].eq {
	pc = 0x82DE8CEC; continue 'dispatch;
	}
	// 82DE8C68: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE8C6C: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82DE8C70: 8B6B0000  lbz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8C74: 9B610070  stb r27, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u8 ) };
	// 82DE8C78: 8B6B0001  lbz r27, 1(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE8C7C: 9B610071  stb r27, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[27].u8 ) };
	// 82DE8C80: 836B0004  lwz r27, 4(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE8C84: 93610074  stw r27, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE8E40 size=56
    let mut pc: u32 = 0x82DE8E40;
    'dispatch: loop {
        match pc {
            0x82DE8E40 => {
    //   block [0x82DE8E40..0x82DE8E78)
	// 82DE8E40: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8E44: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DE8E48: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DE8E4C: 81230050  lwz r9, 0x50(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE8E50: 81030048  lwz r8, 0x48(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE8E54: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE8E58: 98EB0003  stb r7, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 82DE8E5C: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82DE8E60: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DE8E64: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DE8E68: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DE8E6C: 98CB0001  stb r6, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[6].u8 ) };
	// 82DE8E70: 90640000  stw r3, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DE8E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE8E78 size=56
    let mut pc: u32 = 0x82DE8E78;
    'dispatch: loop {
        match pc {
            0x82DE8E78 => {
    //   block [0x82DE8E78..0x82DE8EB0)
	// 82DE8E78: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8E7C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DE8E80: 81430054  lwz r10, 0x54(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DE8E84: 81230050  lwz r9, 0x50(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DE8E88: 81030048  lwz r8, 0x48(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE8E8C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DE8E90: 98EB0003  stb r7, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 82DE8E94: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82DE8E98: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82DE8E9C: B12B0004  sth r9, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82DE8EA0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DE8EA4: 98CB0001  stb r6, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[6].u8 ) };
	// 82DE8EA8: 90640000  stw r3, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DE8EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE8EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE8EB0 size=368
    let mut pc: u32 = 0x82DE8EB0;
    'dispatch: loop {
        match pc {
            0x82DE8EB0 => {
    //   block [0x82DE8EB0..0x82DE9020)
	// 82DE8EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE8EB4: 4BEC0541  bl 0x82ca93f4
	ctx.lr = 0x82DE8EB8;
	sub_82CA93D0(ctx, base);
	// 82DE8EB8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE8EBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE8EC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE8EC4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DE8EC8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DE8ECC: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE8ED0: A11F0006  lhz r8, 6(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DE8ED4: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82DE8ED8: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DE8EDC: 88FF0002  lbz r7, 2(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE8EE0: 7D0A0734  extsh r10, r8
	ctx.r[10].s64 = ctx.r[8].s16 as i64;
	// 82DE8EE4: 7EE95A14  add r23, r9, r11
	ctx.r[23].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DE8EE8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DE8EEC: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DE8EF0: 419A010C  beq cr6, 0x82de8ffc
	if ctx.cr[6].eq {
	pc = 0x82DE8FFC; continue 'dispatch;
	}
	// 82DE8EF4: 831F000C  lwz r24, 0xc(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE8EF8: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82DE8EFC: 419A0100  beq cr6, 0x82de8ffc
	if ctx.cr[6].eq {
	pc = 0x82DE8FFC; continue 'dispatch;
	}
	// 82DE8F00: 89770000  lbz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8F04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE8F08: 409A000C  bne cr6, 0x82de8f14
	if !ctx.cr[6].eq {
	pc = 0x82DE8F14; continue 'dispatch;
	}
	// 82DE8F0C: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8F10: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DE8F14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE8F18: 895F0003  lbz r10, 3(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE8F1C: 83790000  lwz r27, 0(r25)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE8F20: 3B450030  addi r26, r5, 0x30
	ctx.r[26].s64 = ctx.r[5].s64 + 48;
	// 82DE8F24: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DE8F28: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DE8F2C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DE8F30: C00B0C18  lfs f0, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE8F34: 554B203E  rotlwi r11, r10, 4
	ctx.r[11].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 82DE8F38: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DE8F3C: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DE8F40: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DE8F44: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DE8F48: 48002581  bl 0x82deb4c8
	ctx.lr = 0x82DE8F4C;
	sub_82DEB4C8(ctx, base);
	// 82DE8F4C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DE8F50: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE9020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE9020 size=8616
    //   switch @ 0x82DE9304: r10 with 27 label(s)
    //       case  0 → 0x82DE9374
    //       case  1 → 0x82DE938C
    //       case  2 → 0x82DE93D0
    //       case  3 → 0x82DE94A8
    //       case  4 → 0x82DE952C
    //       case  5 → 0x82DE95D4
    //       case  6 → 0x82DE960C
    //       case  7 → 0x82DE96C8
    //       case  8 → 0x82DE9730
    //       case  9 → 0x82DE97A8
    //       case 10 → 0x82DE9818
    //       case 11 → 0x82DE98A8
    //       case 12 → 0x82DE9A04
    //       case 13 → 0x82DE9AB4
    //       case 14 → 0x82DE9B64
    //       case 15 → 0x82DE9E1C
    //       case 16 → 0x82DE9C8C
    //       case 17 → 0x82DEA064
    //       case 18 → 0x82DEA0FC
    //       case 19 → 0x82DEA404
    //       case 20 → 0x82DEA84C
    //       case 21 → 0x82DEB19C
    //       case 22 → 0x82DEB19C
    //       case 23 → 0x82DEAB54
    //       case 24 → 0x82DEAE1C
    //       case 25 → 0x82DEAD88
    //       case 26 → 0x82DEAFF8
    let mut pc: u32 = 0x82DE9020;
    'dispatch: loop {
        match pc {
            0x82DE9020 => {
    //   block [0x82DE9020..0x82DE9374)
	// 82DE9020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE9024: 4BEC03AD  bl 0x82ca93d0
	ctx.lr = 0x82DE9028;
	sub_82CA93D0(ctx, base);
	// 82DE9028: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82DE902C: 4BEC4C7D  bl 0x82cadca8
	ctx.lr = 0x82DE9030;
	sub_82CADCA0(ctx, base);
	// 82DE9030: 3980FEC0  li r12, -0x140
	ctx.r[12].s64 = -320;
	pc = 0x82DE9374; continue 'dispatch;
            }
            0x82DE9374 => {
    //   block [0x82DE9374..0x82DE938C)
	// 82DE9374: 395F000F  addi r10, r31, 0xf
	ctx.r[10].s64 = ctx.r[31].s64 + 15;
	// 82DE9378: 555F0036  rlwinm r31, r10, 0, 0, 0x1b
	ctx.r[31].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82DE937C: A15F0000  lhz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE9380: 2B0A001A  cmplwi cr6, r10, 0x1a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 26 as u32, &mut ctx.xer);
	// 82DE9384: 4099FF6C  ble cr6, 0x82de92f0
	if !ctx.cr[6].gt {
	pc = 0x82DE92F0; continue 'dispatch;
	}
	// 82DE9388: 48001E14  b 0x82deb19c
	pc = 0x82DEB19C; continue 'dispatch;
            }
            0x82DE938C => {
    //   block [0x82DE938C..0x82DE93D0)
	// 82DE938C: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE9390: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DE9394: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DE9398: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DE939C: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE93A0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE93A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE93A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE93AC: 4E800421  bctrl
	ctx.lr = 0x82DE93B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE93B0: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DE93B4: 814100B8  lwz r10, 0xb8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82DE93B8: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82DE93BC: 7D6BAA14  add r11, r11, r21
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 82DE93C0: 7D4AAA14  add r10, r10, r21
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[21].u64;
	// 82DE93C4: 916100AC  stw r11, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82DE93C8: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 82DE93CC: 48001DD0  b 0x82deb19c
	pc = 0x82DEB19C; continue 'dispatch;
            }
            0x82DE93D0 => {
    //   block [0x82DE93D0..0x82DE94A8)
	// 82DE93D0: 80E10098  lwz r7, 0x98(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82DE93D4: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82DE93D8: 390101C0  addi r8, r1, 0x1c0
	ctx.r[8].s64 = ctx.r[1].s64 + 448;
	// 82DE93DC: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	pc = 0x82DE94A8; continue 'dispatch;
            }
            0x82DE94A8 => {
    //   block [0x82DE94A8..0x82DE952C)
	// 82DE94A8: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82DE94AC: 81010098  lwz r8, 0x98(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82DE94B0: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	pc = 0x82DE952C; continue 'dispatch;
            }
            0x82DE952C => {
    //   block [0x82DE952C..0x82DE95D4)
	// 82DE952C: 80E10098  lwz r7, 0x98(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82DE9530: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82DE9534: 390101C0  addi r8, r1, 0x1c0
	ctx.r[8].s64 = ctx.r[1].s64 + 448;
	// 82DE9538: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	pc = 0x82DE95D4; continue 'dispatch;
            }
            0x82DE95D4 => {
    //   block [0x82DE95D4..0x82DE960C)
	// 82DE95D4: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82DE95D8: 38C10130  addi r6, r1, 0x130
	ctx.r[6].s64 = ctx.r[1].s64 + 304;
	// 82DE95DC: 38A10190  addi r5, r1, 0x190
	ctx.r[5].s64 = ctx.r[1].s64 + 400;
	// 82DE95E0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DE95E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE95E8: 4BFFD8C9  bl 0x82de6eb0
	ctx.lr = 0x82DE95EC;
	sub_82DE6EB0(ctx, base);
	// 82DE95EC: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DE95F0: 814100B8  lwz r10, 0xb8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82DE95F4: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82DE95F8: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 82DE95FC: 394A0018  addi r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 + 24;
	// 82DE9600: 916100AC  stw r11, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82DE9604: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 82DE9608: 48001B94  b 0x82deb19c
	pc = 0x82DEB19C; continue 'dispatch;
            }
            0x82DE960C => {
    //   block [0x82DE960C..0x82DE96C8)
	// 82DE960C: 392101C0  addi r9, r1, 0x1c0
	ctx.r[9].s64 = ctx.r[1].s64 + 448;
	// 82DE9610: 394104C0  addi r10, r1, 0x4c0
	ctx.r[10].s64 = ctx.r[1].s64 + 1216;
	// 82DE9614: 39610114  addi r11, r1, 0x114
	ctx.r[11].s64 = ctx.r[1].s64 + 276;
	pc = 0x82DE96C8; continue 'dispatch;
            }
            0x82DE96C8 => {
    //   block [0x82DE96C8..0x82DE9730)
	// 82DE96C8: 390101C0  addi r8, r1, 0x1c0
	ctx.r[8].s64 = ctx.r[1].s64 + 448;
	// 82DE96CC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE96D0: 39210570  addi r9, r1, 0x570
	ctx.r[9].s64 = ctx.r[1].s64 + 1392;
	// 82DE96D4: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE96D8: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82DE96DC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	pc = 0x82DE9730; continue 'dispatch;
            }
            0x82DE9730 => {
    //   block [0x82DE9730..0x82DE97A8)
	// 82DE9730: 390101C0  addi r8, r1, 0x1c0
	ctx.r[8].s64 = ctx.r[1].s64 + 448;
	// 82DE9734: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE9738: 392104F0  addi r9, r1, 0x4f0
	ctx.r[9].s64 = ctx.r[1].s64 + 1264;
	// 82DE973C: D0010520  stfs f0, 0x520(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1312 as u32), tmp.u32 ) };
	// 82DE9740: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE9744: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE9748: D0010524  stfs f0, 0x524(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1316 as u32), tmp.u32 ) };
	// 82DE974C: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82DE9750: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	pc = 0x82DE97A8; continue 'dispatch;
            }
            0x82DE97A8 => {
    //   block [0x82DE97A8..0x82DE9818)
	// 82DE97A8: 394101C0  addi r10, r1, 0x1c0
	ctx.r[10].s64 = ctx.r[1].s64 + 448;
	// 82DE97AC: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE97B0: 39610530  addi r11, r1, 0x530
	ctx.r[11].s64 = ctx.r[1].s64 + 1328;
	// 82DE97B4: D0010560  stfs f0, 0x560(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1376 as u32), tmp.u32 ) };
	// 82DE97B8: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE97BC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DE97C0: D0010564  stfs f0, 0x564(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1380 as u32), tmp.u32 ) };
	// 82DE97C4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DE97C8: 38610530  addi r3, r1, 0x530
	ctx.r[3].s64 = ctx.r[1].s64 + 1328;
	pc = 0x82DE9818; continue 'dispatch;
            }
            0x82DE9818 => {
    //   block [0x82DE9818..0x82DE98A8)
	// 82DE9818: 895F0002  lbz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE981C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DE9820: 419A0054  beq cr6, 0x82de9874
	if ctx.cr[6].eq {
	pc = 0x82DE9874; continue 'dispatch;
	}
	// 82DE9824: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE9828: FF00A000  fcmpu cr6, f0, f20
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[20].f64);
	// 82DE982C: 40990048  ble cr6, 0x82de9874
	if !ctx.cr[6].gt {
	pc = 0x82DE9874; continue 'dispatch;
	}
	// 82DE9830: 38E101C0  addi r7, r1, 0x1c0
	ctx.r[7].s64 = ctx.r[1].s64 + 448;
	// 82DE9834: 895F0003  lbz r10, 3(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE9838: 39010490  addi r8, r1, 0x490
	ctx.r[8].s64 = ctx.r[1].s64 + 1168;
	// 82DE983C: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82DE9840: 554A203E  rotlwi r10, r10, 4
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 82DE9844: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	pc = 0x82DE98A8; continue 'dispatch;
            }
            0x82DE98A8 => {
    //   block [0x82DE98A8..0x82DE9A04)
	// 82DE98A8: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE98AC: A11F0006  lhz r8, 6(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DE98B0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82DE98B4: 88FF0002  lbz r7, 2(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE98B8: 7D0A0734  extsh r10, r8
	ctx.r[10].s64 = ctx.r[8].s16 as i64;
	// 82DE98BC: 7F695A14  add r27, r9, r11
	ctx.r[27].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DE98C0: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DE98C4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DE98C8: 419A0108  beq cr6, 0x82de99d0
	if ctx.cr[6].eq {
	pc = 0x82DE99D0; continue 'dispatch;
	}
	// 82DE98CC: 839F000C  lwz r28, 0xc(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE98D0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DE98D4: 419A00FC  beq cr6, 0x82de99d0
	if ctx.cr[6].eq {
	pc = 0x82DE99D0; continue 'dispatch;
	}
	// 82DE98D8: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE98DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE98E0: 409A000C  bne cr6, 0x82de98ec
	if !ctx.cr[6].eq {
	pc = 0x82DE98EC; continue 'dispatch;
	}
	// 82DE98E4: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE98E8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DE98EC: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE98F0: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82DE98F4: 83B70000  lwz r29, 0(r23)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE98F8: 38E10220  addi r7, r1, 0x220
	ctx.r[7].s64 = ctx.r[1].s64 + 544;
	// 82DE98FC: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE9900: D2810228  stfs f20, 0x228(r1)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(552 as u32), tmp.u32 ) };
	// 82DE9904: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DE9908: D281022C  stfs f20, 0x22c(r1)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(556 as u32), tmp.u32 ) };
	// 82DE990C: 388101C0  addi r4, r1, 0x1c0
	ctx.r[4].s64 = ctx.r[1].s64 + 448;
	// 82DE9910: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE9914: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DE9918: 48001BB1  bl 0x82deb4c8
	ctx.lr = 0x82DE991C;
	sub_82DEB4C8(ctx, base);
	// 82DE991C: 39610160  addi r11, r1, 0x160
	ctx.r[11].s64 = ctx.r[1].s64 + 352;
	// 82DE9920: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DE9924: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE9928: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82DE992C: 39210100  addi r9, r1, 0x100
	ctx.r[9].s64 = ctx.r[1].s64 + 256;
	// 82DE9930: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	pc = 0x82DE9A04; continue 'dispatch;
            }
            0x82DE9A04 => {
    //   block [0x82DE9A04..0x82DE9AB4)
	// 82DE9A04: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE9A08: 395A0001  addi r10, r26, 1
	ctx.r[10].s64 = ctx.r[26].s64 + 1;
	// 82DE9A0C: 38FA0002  addi r7, r26, 2
	ctx.r[7].s64 = ctx.r[26].s64 + 2;
	// 82DE9A10: 557E203E  rotlwi r30, r11, 4
	ctx.r[30].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE9A14: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 82DE9A18: 39010130  addi r8, r1, 0x130
	ctx.r[8].s64 = ctx.r[1].s64 + 304;
	// 82DE9A1C: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE9A20: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82DE9A24: 7D6B38AE  lbzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DE9A28: 38E10330  addi r7, r1, 0x330
	ctx.r[7].s64 = ctx.r[1].s64 + 816;
	// 82DE9A2C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	pc = 0x82DE9AB4; continue 'dispatch;
            }
            0x82DE9AB4 => {
    //   block [0x82DE9AB4..0x82DE9B64)
	// 82DE9AB4: 8BBF0002  lbz r29, 2(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE9AB8: 3B9F0003  addi r28, r31, 3
	ctx.r[28].s64 = ctx.r[31].s64 + 3;
	// 82DE9ABC: 895F0003  lbz r10, 3(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE9AC0: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82DE9AC4: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DE9AC8: 40980078  bge cr6, 0x82de9b40
	if !ctx.cr[6].lt {
	pc = 0x82DE9B40; continue 'dispatch;
	}
	// 82DE9ACC: 397A0001  addi r11, r26, 1
	ctx.r[11].s64 = ctx.r[26].s64 + 1;
	// 82DE9AD0: 7FDD5A14  add r30, r29, r11
	ctx.r[30].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82DE9AD4: 897EFFFF  lbz r11, -1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82DE9AD8: 39410190  addi r10, r1, 0x190
	ctx.r[10].s64 = ctx.r[1].s64 + 400;
	// 82DE9ADC: 893E0000  lbz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE9AE0: 39010190  addi r8, r1, 0x190
	ctx.r[8].s64 = ctx.r[1].s64 + 400;
	// 82DE9AE4: 88FE0001  lbz r7, 1(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE9AE8: 38C10130  addi r6, r1, 0x130
	ctx.r[6].s64 = ctx.r[1].s64 + 304;
	// 82DE9AEC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DE9AF0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	pc = 0x82DE9B64; continue 'dispatch;
            }
            0x82DE9B64 => {
    //   block [0x82DE9B64..0x82DE9C8C)
	// 82DE9B64: 895F0002  lbz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE9B68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DE9B6C: 419A00E0  beq cr6, 0x82de9c4c
	if ctx.cr[6].eq {
	pc = 0x82DE9C4C; continue 'dispatch;
	}
	// 82DE9B70: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE9B74: 391A0002  addi r8, r26, 2
	ctx.r[8].s64 = ctx.r[26].s64 + 2;
	// 82DE9B78: 38BA0001  addi r5, r26, 1
	ctx.r[5].s64 = ctx.r[26].s64 + 1;
	// 82DE9B7C: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE9B80: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82DE9B84: C1A100A0  lfs f13, 0xa0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE9B88: 38E10190  addi r7, r1, 0x190
	ctx.r[7].s64 = ctx.r[1].s64 + 400;
	// 82DE9B8C: C2DF0004  lfs f22, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82DE9B90: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82DE9B94: C2BF0008  lfs f21, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 82DE9B98: 7D0B40AE  lbzx r8, r11, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DE9B9C: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 82DE9BA0: 7CAB28AE  lbzx r5, r11, r5
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82DE9BA4: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE9BA8: 388100FC  addi r4, r1, 0xfc
	ctx.r[4].s64 = ctx.r[1].s64 + 252;
	// 82DE9BAC: EF200372  fmuls f25, f0, f13
	ctx.f[25].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DE9BB0: 386100F4  addi r3, r1, 0xf4
	ctx.r[3].s64 = ctx.r[1].s64 + 244;
	pc = 0x82DE9C8C; continue 'dispatch;
            }
            0x82DE9C8C => {
    //   block [0x82DE9C8C..0x82DE9E1C)
	// 82DE9C8C: 895F0003  lbz r10, 3(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE9C90: 39010130  addi r8, r1, 0x130
	ctx.r[8].s64 = ctx.r[1].s64 + 304;
	// 82DE9C94: 893F0004  lbz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE9C98: 38E10190  addi r7, r1, 0x190
	ctx.r[7].s64 = ctx.r[1].s64 + 400;
	// 82DE9C9C: 554A203E  rotlwi r10, r10, 4
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 82DE9CA0: 5529203E  rotlwi r9, r9, 4
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(4)) as u64;
	// 82DE9CA4: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DE9CA8: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DE9CAC: 39010108  addi r8, r1, 0x108
	ctx.r[8].s64 = ctx.r[1].s64 + 264;
	pc = 0x82DE9E1C; continue 'dispatch;
            }
            0x82DE9E1C => {
    //   block [0x82DE9E1C..0x82DEA064)
	// 82DE9E1C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE9E20: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82DE9E24: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 82DE9E28: 895F0004  lbz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE9E2C: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DE9E30: C2DF0008  lfs f22, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82DE9E34: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE9E38: C2BF000C  lfs f21, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 82DE9E3C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DE9E40: 38810120  addi r4, r1, 0x120
	ctx.r[4].s64 = ctx.r[1].s64 + 288;
	// 82DE9E44: 554A203E  rotlwi r10, r10, 4
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	pc = 0x82DEA064; continue 'dispatch;
            }
            0x82DEA064 => {
    //   block [0x82DEA064..0x82DEA0FC)
	// 82DEA064: 895F0002  lbz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DEA068: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DEA06C: 419A004C  beq cr6, 0x82dea0b8
	if ctx.cr[6].eq {
	pc = 0x82DEA0B8; continue 'dispatch;
	}
	// 82DEA070: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEA074: FF00A000  fcmpu cr6, f0, f20
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[20].f64);
	// 82DEA078: 419A0040  beq cr6, 0x82dea0b8
	if ctx.cr[6].eq {
	pc = 0x82DEA0B8; continue 'dispatch;
	}
	// 82DEA07C: 895F0003  lbz r10, 3(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DEA080: 39210190  addi r9, r1, 0x190
	ctx.r[9].s64 = ctx.r[1].s64 + 400;
	// 82DEA084: 891F0004  lbz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEA088: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DEA08C: 554A203E  rotlwi r10, r10, 4
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 82DEA090: D00102A8  stfs f0, 0x2a8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(680 as u32), tmp.u32 ) };
	// 82DEA094: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DEA098: 916102A4  stw r11, 0x2a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(676 as u32), ctx.r[11].u32 ) };
	// 82DEA09C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DEA0A0: 386102A0  addi r3, r1, 0x2a0
	ctx.r[3].s64 = ctx.r[1].s64 + 672;
	// 82DEA0A4: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82DEA0A8: 910102AC  stw r8, 0x2ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(684 as u32), ctx.r[8].u32 ) };
	// 82DEA0AC: 914102A0  stw r10, 0x2a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(672 as u32), ctx.r[10].u32 ) };
	// 82DEA0B0: 4BFFB101  bl 0x82de51b0
	ctx.lr = 0x82DEA0B4;
	sub_82DE51B0(ctx, base);
	// 82DEA0B4: 48000020  b 0x82dea0d4
	pc = 0x82DEA0D4; continue 'dispatch;
	// 82DEA0B8: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEA0BC: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82DEA0C0: 895F0004  lbz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEA0C4: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82DEA0C8: 9B0B0003  stb r24, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[24].u8 ) };
	// 82DEA0CC: 994B0004  stb r10, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82DEA0D0: 91370000  stw r9, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DEA0D4: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEA0D8: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82DEA0DC: 812100B8  lwz r9, 0xb8(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82DEA0E0: 556A183E  rotlwi r10, r11, 3
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 82DEA0E4: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DEA0E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DEA0EC: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DEA0F0: 916100AC  stw r11, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82DEA0F4: 914100B8  stw r10, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 82DEA0F8: 480010A4  b 0x82deb19c
	pc = 0x82DEB19C; continue 'dispatch;
            }
            0x82DEA0FC => {
    //   block [0x82DEA0FC..0x82DEA404)
	// 82DEA0FC: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEA100: A0FF0006  lhz r7, 6(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DEA104: 7D280734  extsh r8, r9
	ctx.r[8].s64 = ctx.r[9].s16 as i64;
	// 82DEA108: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DEA10C: 7CE90734  extsh r9, r7
	ctx.r[9].s64 = ctx.r[7].s16 as i64;
	// 82DEA110: 7F685A14  add r27, r8, r11
	ctx.r[27].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DEA114: 7FA95A14  add r29, r9, r11
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DEA118: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DEA11C: 419A02B4  beq cr6, 0x82dea3d0
	if ctx.cr[6].eq {
	pc = 0x82DEA3D0; continue 'dispatch;
	}
	// 82DEA120: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DEA124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEA128: 419A02A8  beq cr6, 0x82dea3d0
	if ctx.cr[6].eq {
	pc = 0x82DEA3D0; continue 'dispatch;
	}
	// 82DEA12C: 38C10110  addi r6, r1, 0x110
	ctx.r[6].s64 = ctx.r[1].s64 + 272;
	// 82DEA130: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DEA134: 393A0001  addi r9, r26, 1
	ctx.r[9].s64 = ctx.r[26].s64 + 1;
	// 82DEA138: 389A0002  addi r4, r26, 2
	ctx.r[4].s64 = ctx.r[26].s64 + 2;
	// 82DEA13C: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82DEA140: 38A10190  addi r5, r1, 0x190
	ctx.r[5].s64 = ctx.r[1].s64 + 400;
	// 82DEA144: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82DEA148: 38E10130  addi r7, r1, 0x130
	ctx.r[7].s64 = ctx.r[1].s64 + 304;
	// 82DEA14C: 7CCB48AE  lbzx r6, r11, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DEA150: 5569203E  rotlwi r9, r11, 4
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82DEA154: 7D4B20AE  lbzx r10, r11, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DEA158: 39010130  addi r8, r1, 0x130
	ctx.r[8].s64 = ctx.r[1].s64 + 304;
	// 82DEA15C: 7D662A14  add r11, r6, r5
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 82DEA160: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DEA164: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 82DEA168: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DEA16C: 3BDF0003  addi r30, r31, 3
	ctx.r[30].s64 = ctx.r[31].s64 + 3;
	pc = 0x82DEA404; continue 'dispatch;
            }
            0x82DEA404 => {
    //   block [0x82DEA404..0x82DEA84C)
	// 82DEA404: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEA408: A11F0006  lhz r8, 6(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DEA40C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82DEA410: 88FF0002  lbz r7, 2(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DEA414: 7D0A0734  extsh r10, r8
	ctx.r[10].s64 = ctx.r[8].s16 as i64;
	// 82DEA418: 7FC95A14  add r30, r9, r11
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DEA41C: 7F2A5A14  add r25, r10, r11
	ctx.r[25].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEA420: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DEA424: 419A03D8  beq cr6, 0x82dea7fc
	if ctx.cr[6].eq {
	pc = 0x82DEA7FC; continue 'dispatch;
	}
	// 82DEA428: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 82DEA42C: 8081009C  lwz r4, 0x9c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 82DEA430: 38610630  addi r3, r1, 0x630
	ctx.r[3].s64 = ctx.r[1].s64 + 1584;
	// 82DEA434: 4BF753C5  bl 0x82d5f7f8
	ctx.lr = 0x82DEA438;
	sub_82D5F7F8(ctx, base);
	// 82DEA438: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DEA43C: D2810248  stfs f20, 0x248(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(584 as u32), tmp.u32 ) };
	// 82DEA440: 92C10258  stw r22, 0x258(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(600 as u32), ctx.r[22].u32 ) };
	// 82DEA444: D281024C  stfs f20, 0x24c(r1)
	tmp.f32 = (ctx.f[20].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(588 as u32), tmp.u32 ) };
	// 82DEA448: 9201025C  stw r16, 0x25c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(604 as u32), ctx.r[16].u32 ) };
	// 82DEA44C: 92010268  stw r16, 0x268(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(616 as u32), ctx.r[16].u32 ) };
	// 82DEA450: 7FD5F378  mr r21, r30
	ctx.r[21].u64 = ctx.r[30].u64;
	// 82DEA454: 92C1026C  stw r22, 0x26c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(620 as u32), ctx.r[22].u32 ) };
	// 82DEA458: 3A9F0040  addi r20, r31, 0x40
	ctx.r[20].s64 = ctx.r[31].s64 + 64;
	// 82DEA45C: 91610244  stw r11, 0x244(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(580 as u32), ctx.r[11].u32 ) };
	// 82DEA460: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82DEA464: 92C10270  stw r22, 0x270(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(624 as u32), ctx.r[22].u32 ) };
	// 82DEA468: 7EDDB378  mr r29, r22
	ctx.r[29].u64 = ctx.r[22].u64;
	// 82DEA46C: 91610260  stw r11, 0x260(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(608 as u32), ctx.r[11].u32 ) };
	// 82DEA470: 81140000  lwz r8, 0(r20)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEA474: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DEA478: 419A0350  beq cr6, 0x82dea7c8
	if ctx.cr[6].eq {
	pc = 0x82DEA7C8; continue 'dispatch;
	}
	// 82DEA47C: 39610258  addi r11, r1, 0x258
	ctx.r[11].s64 = ctx.r[1].s64 + 600;
	// 82DEA480: 39210130  addi r9, r1, 0x130
	ctx.r[9].s64 = ctx.r[1].s64 + 304;
	// 82DEA484: 38E10190  addi r7, r1, 0x190
	ctx.r[7].s64 = ctx.r[1].s64 + 400;
	// 82DEA488: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DEA48C: 38C10268  addi r6, r1, 0x268
	ctx.r[6].s64 = ctx.r[1].s64 + 616;
	// 82DEA490: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DEA494: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82DEA498: 39410130  addi r10, r1, 0x130
	ctx.r[10].s64 = ctx.r[1].s64 + 304;
	// 82DEA49C: 557E2036  slwi r30, r11, 4
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82DEA4A0: 39610190  addi r11, r1, 0x190
	ctx.r[11].s64 = ctx.r[1].s64 + 400;
	// 82DEA4A4: 7F7E4A14  add r27, r30, r9
	ctx.r[27].u64 = ctx.r[30].u64 + ctx.r[9].u64;
	// 82DEA4A8: 7CDD302E  lwzx r6, r29, r6
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82DEA4AC: 392103E0  addi r9, r1, 0x3e0
	ctx.r[9].s64 = ctx.r[1].s64 + 992;
	// 82DEA4B0: 54DC2036  slwi r28, r6, 4
	ctx.r[28].u32 = ctx.r[6].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	pc = 0x82DEA84C; continue 'dispatch;
            }
            0x82DEA84C => {
    //   block [0x82DEA84C..0x82DEAB54)
	// 82DEA84C: 392101C0  addi r9, r1, 0x1c0
	ctx.r[9].s64 = ctx.r[1].s64 + 448;
	pc = 0x82DEAB54; continue 'dispatch;
            }
            0x82DEAB54 => {
    //   block [0x82DEAB54..0x82DEAD88)
	// 82DEAB54: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEAB58: 394105D0  addi r10, r1, 0x5d0
	ctx.r[10].s64 = ctx.r[1].s64 + 1488;
	// 82DEAB5C: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DEAB60: 390BFFF0  addi r8, r11, -0x10
	ctx.r[8].s64 = ctx.r[11].s64 + -16;
	// 82DEAB64: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DEAB68: 91170000  stw r8, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DEAB6C: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAB70: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DEAB74: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DEAB78: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEAB7C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DEAB80: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DEAB84: 4200FFF0  bdnz 0x82deab74
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DEAB74; continue 'dispatch;
	}
	// 82DEAB88: C1A10080  lfs f13, 0x80(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEAB8C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEAB90: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEAB94: D1A105F0  stfs f13, 0x5f0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1520 as u32), tmp.u32 ) };
	// 82DEAB98: C1A100A0  lfs f13, 0xa0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEAB9C: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEABA0: D1A10610  stfs f13, 0x610(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1552 as u32), tmp.u32 ) };
	// 82DEABA4: C1A100A4  lfs f13, 0xa4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEABA8: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEABAC: D0010614  stfs f0, 0x614(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1556 as u32), tmp.u32 ) };
	// 82DEABB0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEABB4: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 82DEABB8: 4198000C  blt cr6, 0x82deabc4
	if ctx.cr[6].lt {
	pc = 0x82DEABC4; continue 'dispatch;
	}
	// 82DEABBC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEABC0: 4BFFFFF0  b 0x82deabb0
	pc = 0x82DEABB0; continue 'dispatch;
	// 82DEABC4: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DEABC8: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82DEABCC: A0CB0004  lhz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEABD0: 7EC7B378  mr r7, r22
	ctx.r[7].u64 = ctx.r[22].u64;
	// 82DEABD4: 554A283E  rotlwi r10, r10, 5
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(5)) as u64;
	// 82DEABD8: 890B000A  lbz r8, 0xa(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DEABDC: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82DEABE0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEABE4: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 82DEABE8: 41980108  blt cr6, 0x82deacf0
	if ctx.cr[6].lt {
	pc = 0x82DEACF0; continue 'dispatch;
	}
	// 82DEABEC: 3966FFFC  addi r11, r6, -4
	ctx.r[11].s64 = ctx.r[6].s64 + -4;
	// 82DEABF0: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DEABF4: 3965003C  addi r11, r5, 0x3c
	ctx.r[11].s64 = ctx.r[5].s64 + 60;
	// 82DEABF8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DEABFC: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DEAC00: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAC04: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEAC08: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DEAC0C: C1410064  lfs f10, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DEAC10: C19F0024  lfs f12, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEAC14: C00BFFE0  lfs f0, -0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAC18: C16A0010  lfs f11, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEAC1C: EDAD02B2  fmuls f13, f13, f10
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 82DEAC20: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEAC24: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DEAC28: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DEAC2C: 40980008  bge cr6, 0x82deac34
	if !ctx.cr[6].lt {
	pc = 0x82DEAC34; continue 'dispatch;
	}
	// 82DEAC30: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DEAC34: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAC38: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DEAC3C: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEAC40: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DEAC44: C1610064  lfs f11, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEAC48: C19F0024  lfs f12, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEAC4C: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAC50: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DEAC54: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEAC58: C18A0010  lfs f12, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEAC5C: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DEAC60: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DEAC64: 40980008  bge cr6, 0x82deac6c
	if !ctx.cr[6].lt {
	pc = 0x82DEAC6C; continue 'dispatch;
	}
	// 82DEAC68: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DEAC6C: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAC70: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DEAC74: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEAC78: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DEAC7C: C1610064  lfs f11, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEAC80: C19F0024  lfs f12, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEAC84: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAC88: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DEAC8C: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEAC90: C18A0010  lfs f12, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEAC94: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DEAC98: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DEAC9C: 40980008  bge cr6, 0x82deaca4
	if !ctx.cr[6].lt {
	pc = 0x82DEACA4; continue 'dispatch;
	}
	// 82DEACA0: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DEACA4: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEACA8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DEACAC: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEACB0: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DEACB4: C1610064  lfs f11, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEACB8: C19F0024  lfs f12, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEACBC: C00B0040  lfs f0, 0x40(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEACC0: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DEACC4: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEACC8: C18A0010  lfs f12, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEACCC: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DEACD0: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DEACD4: 40980008  bge cr6, 0x82deacdc
	if !ctx.cr[6].lt {
	pc = 0x82DEACDC; continue 'dispatch;
	}
	// 82DEACD8: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DEACDC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DEACE0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DEACE4: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 82DEACE8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DEACEC: 409AFF14  bne cr6, 0x82deac00
	if !ctx.cr[6].eq {
	pc = 0x82DEAC00; continue 'dispatch;
	}
	// 82DEACF0: 7F073000  cmpw cr6, r7, r6
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DEACF4: 40980060  bge cr6, 0x82dead54
	if !ctx.cr[6].lt {
	pc = 0x82DEAD54; continue 'dispatch;
	}
	// 82DEACF8: 54EB2834  slwi r11, r7, 5
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEACFC: 7D2B2A14  add r9, r11, r5
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DEAD00: 396A0010  addi r11, r10, 0x10
	ctx.r[11].s64 = ctx.r[10].s64 + 16;
	// 82DEAD04: 3949001C  addi r10, r9, 0x1c
	ctx.r[10].s64 = ctx.r[9].s64 + 28;
	// 82DEAD08: 7D273050  subf r9, r7, r6
	ctx.r[9].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 82DEAD0C: C01F0020  lfs f0, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAD10: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEAD14: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DEAD18: C1410064  lfs f10, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DEAD1C: C19F0024  lfs f12, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEAD20: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAD24: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEAD28: EDAD02B2  fmuls f13, f13, f10
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[10].f64) as f32) as f64);
	// 82DEAD2C: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEAD30: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DEAD34: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DEAD38: 40980008  bge cr6, 0x82dead40
	if !ctx.cr[6].lt {
	pc = 0x82DEAD40; continue 'dispatch;
	}
	// 82DEAD3C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DEAD40: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DEAD44: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DEAD48: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DEAD4C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DEAD50: 409AFFBC  bne cr6, 0x82dead0c
	if !ctx.cr[6].eq {
	pc = 0x82DEAD0C; continue 'dispatch;
	}
	// 82DEAD54: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEAD58: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82DEAD5C: A09F0012  lhz r4, 0x12(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DEAD60: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEAD64: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 82DEAD68: 409A0014  bne cr6, 0x82dead7c
	if !ctx.cr[6].eq {
	pc = 0x82DEAD7C; continue 'dispatch;
	}
	// 82DEAD6C: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 82DEAD70: 388105D0  addi r4, r1, 0x5d0
	ctx.r[4].s64 = ctx.r[1].s64 + 1488;
	// 82DEAD74: 48008A95  bl 0x82df3808
	ctx.lr = 0x82DEAD78;
	sub_82DF3808(ctx, base);
	// 82DEAD78: 4800041C  b 0x82deb194
	pc = 0x82DEB194; continue 'dispatch;
	// 82DEAD7C: 38A105D0  addi r5, r1, 0x5d0
	ctx.r[5].s64 = ctx.r[1].s64 + 1488;
	// 82DEAD80: 4BFFE2A1  bl 0x82de9020
	ctx.lr = 0x82DEAD84;
	sub_82DE9020(ctx, base);
	// 82DEAD84: 48000410  b 0x82deb194
	pc = 0x82DEB194; continue 'dispatch;
            }
            0x82DEAD88 => {
    //   block [0x82DEAD88..0x82DEAE1C)
	// 82DEAD88: 81370000  lwz r9, 0(r23)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEAD8C: 39410740  addi r10, r1, 0x740
	ctx.r[10].s64 = ctx.r[1].s64 + 1856;
	// 82DEAD90: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DEAD94: 3909FFF0  addi r8, r9, -0x10
	ctx.r[8].s64 = ctx.r[9].s64 + -16;
	// 82DEAD98: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DEAD9C: 91170000  stw r8, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DEADA0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DEADA4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DEADA8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEADAC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DEADB0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DEADB4: 4200FFF0  bdnz 0x82deada4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DEADA4; continue 'dispatch;
	}
	// 82DEADB8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEADBC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEADC0: 2B0A0017  cmplwi cr6, r10, 0x17
	ctx.cr[6].compare_u32(ctx.r[10].u32, 23 as u32, &mut ctx.xer);
	// 82DEADC4: 4198000C  blt cr6, 0x82deadd0
	if ctx.cr[6].lt {
	pc = 0x82DEADD0; continue 'dispatch;
	}
	// 82DEADC8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEADCC: 4BFFFFF0  b 0x82deadbc
	pc = 0x82DEADBC; continue 'dispatch;
	// 82DEADD0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DEADD4: 48008A1D  bl 0x82df37f0
	ctx.lr = 0x82DEADD8;
	sub_82DF37F0(ctx, base);
	// 82DEADD8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEADDC: A09F0012  lhz r4, 0x12(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DEADE0: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82DEADE4: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEADE8: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 82DEADEC: 409A001C  bne cr6, 0x82deae08
	if !ctx.cr[6].eq {
	pc = 0x82DEAE08; continue 'dispatch;
	}
	// 82DEADF0: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 82DEADF4: 38810740  addi r4, r1, 0x740
	ctx.r[4].s64 = ctx.r[1].s64 + 1856;
	// 82DEADF8: 48008A11  bl 0x82df3808
	ctx.lr = 0x82DEADFC;
	sub_82DF3808(ctx, base);
	// 82DEADFC: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DEAE00: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82DEAE04: 48000398  b 0x82deb19c
	pc = 0x82DEB19C; continue 'dispatch;
	// 82DEAE08: 38A10740  addi r5, r1, 0x740
	ctx.r[5].s64 = ctx.r[1].s64 + 1856;
	// 82DEAE0C: 4BFFE215  bl 0x82de9020
	ctx.lr = 0x82DEAE10;
	sub_82DE9020(ctx, base);
	// 82DEAE10: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82DEAE14: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82DEAE18: 48000384  b 0x82deb19c
	pc = 0x82DEB19C; continue 'dispatch;
            }
            0x82DEAE1C => {
    //   block [0x82DEAE1C..0x82DEAFF8)
	// 82DEAE1C: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAE20: 39410660  addi r10, r1, 0x660
	ctx.r[10].s64 = ctx.r[1].s64 + 1632;
	// 82DEAE24: D00100C0  stfs f0, 0xc0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82DEAE28: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DEAE2C: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEAE30: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DEAE34: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82DEAE38: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82DEAE3C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DEAE40: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEAE44: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DEAE48: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DEAE4C: 4200FFF0  bdnz 0x82deae3c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DEAE3C; continue 'dispatch;
	}
	// 82DEAE50: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEAE54: 394101E0  addi r10, r1, 0x1e0
	ctx.r[10].s64 = ctx.r[1].s64 + 480;
	// 82DEAE58: 810100B4  lwz r8, 0xb4(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82DEAE5C: 39210200  addi r9, r1, 0x200
	ctx.r[9].s64 = ctx.r[1].s64 + 512;
	// 82DEAE60: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DEAE64: 80E100B0  lwz r7, 0xb0(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82DEAE68: 80C100A8  lwz r6, 0xa8(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 82DEAE6C: 38810820  addi r4, r1, 0x820
	ctx.r[4].s64 = ctx.r[1].s64 + 2080;
	// 82DEAE70: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEAE74: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82DEAE78: A3BF0012  lhz r29, 0x12(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(18 as u32) ) } as u64;
	// 82DEAE7C: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 82DEAE80: 9A0B0003  stb r16, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[16].u8 ) };
	// 82DEAE84: B10B0006  sth r8, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82DEAE88: 92CB0008  stw r22, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 82DEAE8C: B0EB0004  sth r7, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 82DEAE90: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82DEAE94: 9ACB0001  stb r22, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[22].u8 ) };
	// 82DEAE98: 80E10090  lwz r7, 0x90(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DEAE9C: 90770000  stw r3, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82DEAEA0: 39670030  addi r11, r7, 0x30
	ctx.r[11].s64 = ctx.r[7].s64 + 48;
	// 82DEAEA4: 80C10094  lwz r6, 0x94(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82DEAEA8: 39060030  addi r8, r6, 0x30
	ctx.r[8].s64 = ctx.r[6].s64 + 48;
	// 82DEAEAC: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DEAEB0: F86A0000  std r3, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82DEAEB4: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82DEAEB8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DEAEBC: E9680000  ld r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82DEAEC0: F9690000  std r11, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82DEAEC4: E9680008  ld r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 82DEAEC8: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DEAECC: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82DEAED0: E9670000  ld r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	// 82DEAED4: 38E70008  addi r7, r7, 8
	ctx.r[7].s64 = ctx.r[7].s64 + 8;
	// 82DEAED8: F9640000  std r11, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82DEAEDC: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 82DEAEE0: 4200FFF0  bdnz 0x82deaed0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DEAED0; continue 'dispatch;
	}
	// 82DEAEE4: 396107A0  addi r11, r1, 0x7a0
	ctx.r[11].s64 = ctx.r[1].s64 + 1952;
	// 82DEAEE8: 7E6A9B78  mr r10, r19
	ctx.r[10].u64 = ctx.r[19].u64;
	// 82DEAEEC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82DEAEF0: E9460000  ld r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82DEAEF4: 38C60008  addi r6, r6, 8
	ctx.r[6].s64 = ctx.r[6].s64 + 8;
	// 82DEAEF8: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82DEAEFC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DEAF00: 4200FFF0  bdnz 0x82deaef0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DEAEF0; continue 'dispatch;
	}
	// 82DEAF04: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82DEAF08: 38E101E0  addi r7, r1, 0x1e0
	ctx.r[7].s64 = ctx.r[1].s64 + 480;
	// 82DEAF0C: 39210850  addi r9, r1, 0x850
	ctx.r[9].s64 = ctx.r[1].s64 + 2128;
	// 82DEAF10: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DEAF14: 390107D0  addi r8, r1, 0x7d0
	ctx.r[8].s64 = ctx.r[1].s64 + 2000;
	pc = 0x82DEAFF8; continue 'dispatch;
            }
            0x82DEAFF8 => {
    //   block [0x82DEAFF8..0x82DEB19C)
	// 82DEAFF8: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82DEAFFC: 396102F0  addi r11, r1, 0x2f0
	ctx.r[11].s64 = ctx.r[1].s64 + 752;
	// 82DEB000: 390108A0  addi r8, r1, 0x8a0
	ctx.r[8].s64 = ctx.r[1].s64 + 2208;
	// 82DEB004: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82DEB008: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82DEB00C: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DEB010: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82DEB014: F8CB0000  std r6, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82DEB018: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82DEB01C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82DEB020: E9690000  ld r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DEB024: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82DEB028: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82DEB02C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82DEB030: 4200FFF0  bdnz 0x82deb020
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DEB020; continue 'dispatch;
	}
	// 82DEB034: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEB038: 386106C0  addi r3, r1, 0x6c0
	ctx.r[3].s64 = ctx.r[1].s64 + 1728;
	// 82DEB03C: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEB040: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DEB044: 80E100B4  lwz r7, 0xb4(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82DEB048: 80C100B0  lwz r6, 0xb0(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82DEB04C: 810100A8  lwz r8, 0xa8(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 82DEB050: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82DEB054: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEB058: A13E0006  lhz r9, 6(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DEB05C: 895E000A  lbz r10, 0xa(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DEB060: 5529283E  rotlwi r9, r9, 5
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(5)) as u64;
	// 82DEB064: 9A0B0003  stb r16, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[16].u8 ) };
	// 82DEB068: B0CB0004  sth r6, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82DEB06C: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82DEB070: B0EB0006  sth r7, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DEB074: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DEB078: 39290030  addi r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 + 48;
	// 82DEB07C: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82DEB080: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82DEB084: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82DEB088: 90B70000  stw r5, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DEB08C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DEB090: 557DDFFE  rlwinm r29, r11, 0x1b, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DEB094: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DEB098: 409A0020  bne cr6, 0x82deb0b8
	if !ctx.cr[6].eq {
	pc = 0x82DEB0B8; continue 'dispatch;
	}
	// 82DEB09C: 80810090  lwz r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DEB0A0: 4BFBC0C9  bl 0x82da7168
	ctx.lr = 0x82DEB0A4;
	sub_82DA7168(ctx, base);
	// 82DEB0A4: 396106C0  addi r11, r1, 0x6c0
	ctx.r[11].s64 = ctx.r[1].s64 + 1728;
	// 82DEB0A8: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82DEB0AC: 396102F0  addi r11, r1, 0x2f0
	ctx.r[11].s64 = ctx.r[1].s64 + 752;
	pc = 0x82DEB19C; continue 'dispatch;
            }
            0x82DEB19C => {
    //   block [0x82DEB19C..0x82DEB1C8)
	// 82DEB19C: 81410104  lwz r10, 0x104(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DEB1A0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DEB1A4: 4198E13C  blt cr6, 0x82de92e0
	if ctx.cr[6].lt {
	pc = 0x82DE92E0; continue 'dispatch;
	}
	// 82DEB1A8: 38210A40  addi r1, r1, 0xa40
	ctx.r[1].s64 = ctx.r[1].s64 + 2624;
	// 82DEB1AC: 3800FEC0  li r0, -0x140
	ctx.r[0].s64 = -320;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEB1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEB1C8 size=276
    let mut pc: u32 = 0x82DEB1C8;
    'dispatch: loop {
        match pc {
            0x82DEB1C8 => {
    //   block [0x82DEB1C8..0x82DEB2DC)
	// 82DEB1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEB1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DEB1D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DEB1D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DEB1D8: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEB2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEB2E0 size=20
    let mut pc: u32 = 0x82DEB2E0;
    'dispatch: loop {
        match pc {
            0x82DEB2E0 => {
    //   block [0x82DEB2E0..0x82DEB2F4)
	// 82DEB2E0: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DEB2E4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DEB2E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DEB2EC: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82DEB2F0: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEB2F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DEB2F4 size=468
    let mut pc: u32 = 0x82DEB2F4;
    'dispatch: loop {
        match pc {
            0x82DEB2F4 => {
    //   block [0x82DEB2F4..0x82DEB31C)
	// 82DEB2F4: 3D8082DF  lis r12, -0x7d21
	ctx.r[12].s64 = -2099314688;
	// 82DEB2F8: 398CB30C  addi r12, r12, -0x4cf4
	ctx.r[12].s64 = ctx.r[12].s64 + -19700;
	// 82DEB2FC: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DEB300: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82DEB304: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82DEB308: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82DEB31C; continue 'dispatch;
		},
		1 => {
	pc = 0x82DEB3C8; continue 'dispatch;
		},
		2 => {
	pc = 0x82DEB410; continue 'dispatch;
		},
		3 => {
	pc = 0x82DEB4BC; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82DEB30C: 82DEB31C  lwz r22, -0x4ce4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19684 as u32) ) } as u64;
	// 82DEB310: 82DEB3C8  lwz r22, -0x4c38(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19512 as u32) ) } as u64;
	// 82DEB314: 82DEB410  lwz r22, -0x4bf0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19440 as u32) ) } as u64;
	// 82DEB318: 82DEB4BC  lwz r22, -0x4b44(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19268 as u32) ) } as u64;
            }
            0x82DEB31C => {
    //   block [0x82DEB31C..0x82DEB3C8)
	// 82DEB31C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEB320: C1A3001C  lfs f13, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB324: C1630020  lfs f11, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEB328: C1840014  lfs f12, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEB32C: C1240010  lfs f9, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DEB330: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB334: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DEB338: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DEB33C: ED6D0332  fmuls f11, f13, f12
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEB340: EDAC5828  fsubs f13, f12, f11
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82DEB344: FD406A10  fabs f10, f13
	ctx.f[10].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82DEB348: FF0A0000  fcmpu cr6, f10, f0
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[0].f64);
	// 82DEB34C: 40990020  ble cr6, 0x82deb36c
	if !ctx.cr[6].gt {
	pc = 0x82DEB36C; continue 'dispatch;
	}
	// 82DEB350: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DEB354: C14B0C18  lfs f10, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DEB358: FF0D5000  fcmpu cr6, f13, f10
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[10].f64);
	// 82DEB35C: 41990008  bgt cr6, 0x82deb364
	if ctx.cr[6].gt {
	pc = 0x82DEB364; continue 'dispatch;
	}
	// 82DEB360: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DEB364: EDA0582A  fadds f13, f0, f11
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64;
	// 82DEB368: 48000008  b 0x82deb370
	pc = 0x82DEB370; continue 'dispatch;
	// 82DEB36C: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	// 82DEB370: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82DEB374: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB378: FD606050  fneg f11, f12
	ctx.f[11].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DEB37C: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DEB380: EC0B0028  fsubs f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DEB384: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DEB388: 4098000C  bge cr6, 0x82deb394
	if !ctx.cr[6].lt {
	pc = 0x82DEB394; continue 'dispatch;
	}
	// 82DEB38C: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82DEB390: 48000010  b 0x82deb3a0
	pc = 0x82DEB3A0; continue 'dispatch;
	// 82DEB394: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82DEB398: 40990008  ble cr6, 0x82deb3a0
	if !ctx.cr[6].gt {
	pc = 0x82DEB3A0; continue 'dispatch;
	}
	// 82DEB39C: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	// 82DEB3A0: ED8D482A  fadds f12, f13, f9
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[9].f64) as f32) as f64;
	// 82DEB3A4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEB3A8: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB3AC: C104000C  lfs f8, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DEB3B0: C1430014  lfs f10, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DEB3B4: C1A30018  lfs f13, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB3B8: C16B0010  lfs f11, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEB3BC: ED6C02F2  fmuls f11, f12, f11
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 82DEB3C0: FD800050  fneg f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DEB3C4: 480000DC  b 0x82deb4a0
	pc = 0x82DEB4A0; continue 'dispatch;
            }
            0x82DEB3C8 => {
    //   block [0x82DEB3C8..0x82DEB410)
	// 82DEB3C8: 8963001C  lbz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DEB3CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEB3D0: 419A002C  beq cr6, 0x82deb3fc
	if ctx.cr[6].eq {
	pc = 0x82DEB3FC; continue 'dispatch;
	}
	// 82DEB3D4: C1A40010  lfs f13, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB3D8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEB3DC: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB3E0: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DEB3E4: C104000C  lfs f8, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DEB3E8: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB3EC: ED600372  fmuls f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEB3F0: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB3F4: FD406890  fmr f10, f13
	ctx.f[10].f64 = ctx.f[13].f64;
	// 82DEB3F8: 480000A0  b 0x82deb498
	pc = 0x82DEB498; continue 'dispatch;
	// 82DEB3FC: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB400: C1630018  lfs f11, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEB404: FD406890  fmr f10, f13
	ctx.f[10].f64 = ctx.f[13].f64;
	// 82DEB408: C104000C  lfs f8, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DEB40C: 4800008C  b 0x82deb498
	pc = 0x82DEB498; continue 'dispatch;
            }
            0x82DEB410 => {
    //   block [0x82DEB410..0x82DEB4BC)
	// 82DEB410: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DEB414: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEB418: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB41C: C1840014  lfs f12, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEB420: C1440010  lfs f10, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DEB424: ED0C502A  fadds f8, f12, f10
	ctx.f[8].f64 = ((ctx.f[12].f64 + ctx.f[10].f64) as f32) as f64;
	// 82DEB428: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB42C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DEB430: ED2D0024  fdivs f9, f13, f0
	ctx.f[9].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DEB434: C18A0000  lfs f12, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEB438: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB43C: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEB440: C16B0C18  lfs f11, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DEB444: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEB448: EC000272  fmuls f0, f0, f9
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 82DEB44C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DEB450: 4098000C  bge cr6, 0x82deb45c
	if !ctx.cr[6].lt {
	pc = 0x82DEB45C; continue 'dispatch;
	}
	// 82DEB454: FD405890  fmr f10, f11
	ctx.f[10].f64 = ctx.f[11].f64;
	// 82DEB458: 48000018  b 0x82deb470
	pc = 0x82DEB470; continue 'dispatch;
	// 82DEB45C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DEB460: 4099000C  ble cr6, 0x82deb46c
	if !ctx.cr[6].gt {
	pc = 0x82DEB46C; continue 'dispatch;
	}
	// 82DEB464: FD406890  fmr f10, f13
	ctx.f[10].f64 = ctx.f[13].f64;
	// 82DEB468: 48000008  b 0x82deb470
	pc = 0x82DEB470; continue 'dispatch;
	// 82DEB46C: FD400090  fmr f10, f0
	ctx.f[10].f64 = ctx.f[0].f64;
	// 82DEB470: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB474: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82DEB478: EC000272  fmuls f0, f0, f9
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[9].f64) as f32) as f64);
	// 82DEB47C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DEB480: 4098000C  bge cr6, 0x82deb48c
	if !ctx.cr[6].lt {
	pc = 0x82DEB48C; continue 'dispatch;
	}
	// 82DEB484: FDA05890  fmr f13, f11
	ctx.f[13].f64 = ctx.f[11].f64;
	// 82DEB488: 48000010  b 0x82deb498
	pc = 0x82DEB498; continue 'dispatch;
	// 82DEB48C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DEB490: 41990008  bgt cr6, 0x82deb498
	if ctx.cr[6].gt {
	pc = 0x82DEB498; continue 'dispatch;
	}
	// 82DEB494: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 82DEB498: C183000C  lfs f12, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEB49C: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB4A0: D1650004  stfs f11, 4(r5)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DEB4A4: D1050000  stfs f8, 0(r5)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DEB4A8: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DEB4AC: D185000C  stfs f12, 0xc(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DEB4B0: D1450010  stfs f10, 0x10(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DEB4B4: D1A50014  stfs f13, 0x14(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DEB4B8: 4E800020  blr
	return;
            }
            0x82DEB4BC => {
    //   block [0x82DEB4BC..0x82DEB4C8)
	// 82DEB4BC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DEB4C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DEB4C4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEB4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEB4C8 size=348
    let mut pc: u32 = 0x82DEB4C8;
    'dispatch: loop {
        match pc {
            0x82DEB4C8 => {
    //   block [0x82DEB4C8..0x82DEB624)
	// 82DEB4C8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DEB4CC: 81650030  lwz r11, 0x30(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DEB4D0: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82DEB4D4: 81450034  lwz r10, 0x34(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(52 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEB628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DEB628 size=96
    let mut pc: u32 = 0x82DEB628;
    'dispatch: loop {
        match pc {
            0x82DEB628 => {
    //   block [0x82DEB628..0x82DEB688)
	// 82DEB628: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEB62C: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 82DEB630: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB634: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB638: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEB63C: D005000C  stfs f0, 0xc(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DEB640: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82DEB644: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB648: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB64C: 394B0050  addi r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 + 80;
	// 82DEB650: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEB654: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DEB658: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB65C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEB660: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEB664: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82DEB668: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB66C: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DEB670: D00B0038  stfs f0, 0x38(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82DEB674: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB678: D00B003C  stfs f0, 0x3c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82DEB67C: C0030014  lfs f0, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEB680: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82DEB684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEB688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEB688 size=1036
    let mut pc: u32 = 0x82DEB688;
    'dispatch: loop {
        match pc {
            0x82DEB688 => {
    //   block [0x82DEB688..0x82DEBA94)
	// 82DEB688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEB68C: 4BEBDD4D  bl 0x82ca93d8
	ctx.lr = 0x82DEB690;
	sub_82CA93D0(ctx, base);
	// 82DEB690: DBA1FF60  stfd f29, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[29].u64 ) };
	// 82DEB694: DBC1FF68  stfd f30, -0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[30].u64 ) };
	// 82DEB698: DBE1FF70  stfd f31, -0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 82DEB69C: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEBA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEBA98 size=1208
    let mut pc: u32 = 0x82DEBA98;
    'dispatch: loop {
        match pc {
            0x82DEBA98 => {
    //   block [0x82DEBA98..0x82DEBF50)
	// 82DEBA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEBA9C: 4BEBD935  bl 0x82ca93d0
	ctx.lr = 0x82DEBAA0;
	sub_82CA93D0(ctx, base);
	// 82DEBAA0: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82DEBAA4: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DEBAA8: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DEBAAC: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEBF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DEBF50 size=308
    let mut pc: u32 = 0x82DEBF50;
    'dispatch: loop {
        match pc {
            0x82DEBF50 => {
    //   block [0x82DEBF50..0x82DEC084)
	// 82DEBF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEBF54: 4BEBD4AD  bl 0x82ca9400
	ctx.lr = 0x82DEBF58;
	sub_82CA93D0(ctx, base);
	// 82DEBF58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DEBF5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DEBF60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEBF64: C12B0C18  lfs f9, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DEBF68: FD604890  fmr f11, f9
	ctx.f[11].f64 = ctx.f[9].f64;
	// 82DEBF6C: FC004890  fmr f0, f9
	ctx.f[0].f64 = ctx.f[9].f64;
	// 82DEBF70: 40990110  ble cr6, 0x82dec080
	if !ctx.cr[6].gt {
	pc = 0x82DEC080; continue 'dispatch;
	}
	// 82DEBF74: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DEBF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DEBF7C: 3BA3FFFF  addi r29, r3, -1
	ctx.r[29].s64 = ctx.r[3].s64 + -1;
	// 82DEBF80: 3BCA4C40  addi r30, r10, 0x4c40
	ctx.r[30].s64 = ctx.r[10].s64 + 19520;
	// 82DEBF84: 38880004  addi r4, r8, 4
	ctx.r[4].s64 = ctx.r[8].s64 + 4;
	// 82DEBF88: C1890C38  lfs f12, 0xc38(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3128 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEBF8C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 82DEBF90: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82DEBF94: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DEBF98: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82DEBF9C: 3B80FFF0  li r28, -0x10
	ctx.r[28].s64 = -16;
	// 82DEBFA0: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82DEBFA4: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DEC088 size=476
    let mut pc: u32 = 0x82DEC088;
    'dispatch: loop {
        match pc {
            0x82DEC088 => {
    //   block [0x82DEC088..0x82DEC264)
	// 82DEC088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEC08C: 4BEBD36D  bl 0x82ca93f8
	ctx.lr = 0x82DEC090;
	sub_82CA93D0(ctx, base);
	// 82DEC090: 83410054  lwz r26, 0x54(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DEC094: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82DEC098: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82DEC09C: 39230001  addi r9, r3, 1
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	// 82DEC0A0: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82DEC0A4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DEC0A8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC0AC: 3B2B0004  addi r25, r11, 4
	ctx.r[25].s64 = ctx.r[11].s64 + 4;
	// 82DEC0B0: 98CB0003  stb r6, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[6].u8 ) };
	// 82DEC0B4: 1CC50044  mulli r6, r5, 0x44
	ctx.r[6].s64 = ctx.r[5].s64 * 68;
	// 82DEC0B8: D02B0008  stfs f1, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DEC0BC: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82DEC0C0: D04B000C  stfs f2, 0xc(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DEC0C4: 38C60027  addi r6, r6, 0x27
	ctx.r[6].s64 = ctx.r[6].s64 + 39;
	// 82DEC0C8: 54C60036  rlwinm r6, r6, 0, 0, 0x1b
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82DEC0CC: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82DEC0D0: 4099002C  ble cr6, 0x82dec0fc
	if !ctx.cr[6].gt {
	pc = 0x82DEC0FC; continue 'dispatch;
	}
	// 82DEC0D4: 554A32B2  rlwinm r10, r10, 6, 0xa, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 82DEC0D8: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82DEC0DC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DEC0E0: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82DEC0E4: 80C70000  lwz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC0E8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DEC0EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DEC0F0: 7CCA392E  stwx r6, r10, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.r[6].u32) };
	// 82DEC0F4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82DEC0F8: 409AFFEC  bne cr6, 0x82dec0e4
	if !ctx.cr[6].eq {
	pc = 0x82DEC0E4; continue 'dispatch;
	}
	// 82DEC0FC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DEC100: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC104: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DEC108: 5547083E  rotlwi r7, r10, 1
	ctx.r[7].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82DEC10C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DEC110: 7CEA3A14  add r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DEC114: C1290C18  lfs f9, 0xc18(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DEC118: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC11C: FD604890  fmr f11, f9
	ctx.f[11].f64 = ctx.f[9].f64;
	// 82DEC120: 38A90010  addi r5, r9, 0x10
	ctx.r[5].s64 = ctx.r[9].s64 + 16;
	// 82DEC124: FC004890  fmr f0, f9
	ctx.f[0].f64 = ctx.f[9].f64;
	// 82DEC128: 5549303E  rotlwi r9, r10, 6
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(6)) as u64;
	// 82DEC12C: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DEC130: 54E92036  slwi r9, r7, 4
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DEC134: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82DEC138: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DEC13C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DEC140: 40990110  ble cr6, 0x82dec250
	if !ctx.cr[6].gt {
	pc = 0x82DEC250; continue 'dispatch;
	}
	// 82DEC144: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DEC148: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 82DEC14C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82DEC150: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82DEC154: 3BCA4C40  addi r30, r10, 0x4c40
	ctx.r[30].s64 = ctx.r[10].s64 + 19520;
	// 82DEC158: C1890C38  lfs f12, 0xc38(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3128 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEC15C: 3BA3FFFF  addi r29, r3, -1
	ctx.r[29].s64 = ctx.r[3].s64 + -1;
	// 82DEC160: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 82DEC164: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82DEC168: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82DEC16C: 3B80FFF0  li r28, -0x10
	ctx.r[28].s64 = -16;
	// 82DEC170: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82DEC174: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC268 size=8
    let mut pc: u32 = 0x82DEC268;
    'dispatch: loop {
        match pc {
            0x82DEC268 => {
    //   block [0x82DEC268..0x82DEC270)
	// 82DEC268: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DEC26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC270 size=8
    let mut pc: u32 = 0x82DEC270;
    'dispatch: loop {
        match pc {
            0x82DEC270 => {
    //   block [0x82DEC270..0x82DEC278)
	// 82DEC270: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82DEC274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC278 size=8
    let mut pc: u32 = 0x82DEC278;
    'dispatch: loop {
        match pc {
            0x82DEC278 => {
    //   block [0x82DEC278..0x82DEC280)
	// 82DEC278: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 82DEC27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC280 size=24
    let mut pc: u32 = 0x82DEC280;
    'dispatch: loop {
        match pc {
            0x82DEC280 => {
    //   block [0x82DEC280..0x82DEC298)
	// 82DEC280: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEC284: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82DEC288: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEC28C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DEC290: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82DEC294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC298 size=4
    let mut pc: u32 = 0x82DEC298;
    'dispatch: loop {
        match pc {
            0x82DEC298 => {
    //   block [0x82DEC298..0x82DEC29C)
	// 82DEC298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC2A0 size=4
    let mut pc: u32 = 0x82DEC2A0;
    'dispatch: loop {
        match pc {
            0x82DEC2A0 => {
    //   block [0x82DEC2A0..0x82DEC2A4)
	// 82DEC2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC2A8 size=112
    let mut pc: u32 = 0x82DEC2A8;
    'dispatch: loop {
        match pc {
            0x82DEC2A8 => {
    //   block [0x82DEC2A8..0x82DEC318)
	// 82DEC2A8: 39670040  addi r11, r7, 0x40
	ctx.r[11].s64 = ctx.r[7].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC318 size=28
    let mut pc: u32 = 0x82DEC318;
    'dispatch: loop {
        match pc {
            0x82DEC318 => {
    //   block [0x82DEC318..0x82DEC334)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DEC338 size=888
    let mut pc: u32 = 0x82DEC338;
    'dispatch: loop {
        match pc {
            0x82DEC338 => {
    //   block [0x82DEC338..0x82DEC6B0)
	// 82DEC338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEC33C: 4BEBD095  bl 0x82ca93d0
	ctx.lr = 0x82DEC340;
	sub_82CA93D0(ctx, base);
	// 82DEC340: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DEC344: A1660000  lhz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC348: 90E10034  stw r7, 0x34(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(52 as u32), ctx.r[7].u32 ) };
	// 82DEC34C: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 82DEC350: 5569083E  rotlwi r9, r11, 1
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(1)) as u64;
	// 82DEC354: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DEC358: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DEC35C: C00A0C18  lfs f0, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEC360: 1D4B003C  mulli r10, r11, 0x3c
	ctx.r[10].s64 = ctx.r[11].s64 * 60;
	// 82DEC364: 90E1FF14  stw r7, -0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-236 as u32), ctx.r[7].u32 ) };
	// 82DEC368: 93E1FF18  stw r31, -0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-232 as u32), ctx.r[31].u32 ) };
	// 82DEC36C: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82DEC370: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DEC374: 3A4A0014  addi r18, r10, 0x14
	ctx.r[18].s64 = ctx.r[10].s64 + 20;
	// 82DEC378: 556A303E  rotlwi r10, r11, 6
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(6)) as u64;
	// 82DEC37C: 3A60FFF0  li r19, -0x10
	ctx.r[19].s64 = -16;
	// 82DEC380: 3B400030  li r26, 0x30
	ctx.r[26].s64 = 48;
	// 82DEC384: 7D6A3214  add r11, r10, r6
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82DEC388: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEC38C: 3A2B0014  addi r17, r11, 0x14
	ctx.r[17].s64 = ctx.r[11].s64 + 20;
	// 82DEC390: D012FFFC  stfs f0, -4(r18)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DEC394: 7D6A3214  add r11, r10, r6
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82DEC398: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82DEC39C: 9081FF10  stw r4, -0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-240 as u32), ctx.r[4].u32 ) };
	// 82DEC3A0: 409901E0  ble cr6, 0x82dec580
	if !ctx.cr[6].gt {
	pc = 0x82DEC580; continue 'dispatch;
	}
	// 82DEC3A4: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DEC3A8: 3AE6000C  addi r23, r6, 0xc
	ctx.r[23].s64 = ctx.r[6].s64 + 12;
	// 82DEC3AC: 3AC60008  addi r22, r6, 8
	ctx.r[22].s64 = ctx.r[6].s64 + 8;
	// 82DEC3B0: 3AA30040  addi r21, r3, 0x40
	ctx.r[21].s64 = ctx.r[3].s64 + 64;
	// 82DEC3B4: 7E4A9378  mr r10, r18
	ctx.r[10].u64 = ctx.r[18].u64;
	// 82DEC3B8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DEC3BC: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 82DEC3C0: 7F328850  subf r25, r18, r17
	ctx.r[25].s64 = ctx.r[17].s64 - ctx.r[18].s64;
	// 82DEC3C4: 3A894C40  addi r20, r9, 0x4c40
	ctx.r[20].s64 = ctx.r[9].s64 + 19520;
	// 82DEC3C8: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82DEC3CC: 3B000020  li r24, 0x20
	ctx.r[24].s64 = 32;
	// 82DEC3D0: 7D0ACA14  add r8, r10, r25
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[25].u64;
	// 82DEC3D4: 7D2AC82E  lwzx r9, r10, r25
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[25].u32)) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEC6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DEC6B0 size=1900
    let mut pc: u32 = 0x82DEC6B0;
    'dispatch: loop {
        match pc {
            0x82DEC6B0 => {
    //   block [0x82DEC6B0..0x82DECE1C)
	// 82DEC6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEC6B4: 4BEBCD1D  bl 0x82ca93d0
	ctx.lr = 0x82DEC6B8;
	sub_82CA93D0(ctx, base);
	// 82DEC6B8: A1660000  lhz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC6BC: 3BC60010  addi r30, r6, 0x10
	ctx.r[30].s64 = ctx.r[6].s64 + 16;
	// 82DEC6C0: 90E10034  stw r7, 0x34(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(52 as u32), ctx.r[7].u32 ) };
	// 82DEC6C4: 3B00FFD0  li r24, -0x30
	ctx.r[24].s64 = -48;
	// 82DEC6C8: 556A183E  rotlwi r10, r11, 3
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 82DEC6CC: 5569183E  rotlwi r9, r11, 3
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 82DEC6D0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DEC6D4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DEC6D8: 93C1FE78  stw r30, -0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-392 as u32), ctx.r[30].u32 ) };
	// 82DEC6DC: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DEC6E0: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82DEC6E4: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82DEC6E8: 1D6B0130  mulli r11, r11, 0x130
	ctx.r[11].s64 = ctx.r[11].s64 * 304;
	// 82DEC6EC: 93A1FE48  stw r29, -0x1b8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-440 as u32), ctx.r[29].u32 ) };
	// 82DEC6F0: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DEC6F4: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DEC6F8: 3B8AFFF0  addi r28, r10, -0x10
	ctx.r[28].s64 = ctx.r[10].s64 + -16;
	// 82DEC6FC: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82DEC700: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 82DEC704: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82DEC708: 38890010  addi r4, r9, 0x10
	ctx.r[4].s64 = ctx.r[9].s64 + 16;
	// 82DEC70C: 9141FE68  stw r10, -0x198(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-408 as u32), ctx.r[10].u32 ) };
	// 82DEC710: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DECE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DECE20 size=52
    let mut pc: u32 = 0x82DECE20;
    'dispatch: loop {
        match pc {
            0x82DECE20 => {
    //   block [0x82DECE20..0x82DECE54)
	// 82DECE20: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82DECE24: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82DECE28: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DECE2C: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82DECE30: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DECE34: E9090000  ld r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DECE38: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 82DECE3C: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82DECE40: F92A0008  std r9, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 82DECE44: 409A0008  bne cr6, 0x82dece4c
	if !ctx.cr[6].eq {
	pc = 0x82DECE4C; continue 'dispatch;
	}
	// 82DECE48: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82DECE4C: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DECE50: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DECE54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DECE54 size=192
    //   switch @ 0x82DECE7C: r10 with 4 label(s)
    //       case  0 → 0x82DECEB8
    //       case  1 → 0x82DECE90
    //       case  2 → 0x82DECE90
    //       case  3 → 0x82DECF10
    let mut pc: u32 = 0x82DECE54;
    'dispatch: loop {
        match pc {
            0x82DECE54 => {
    //   block [0x82DECE54..0x82DECE90)
	// 82DECE54: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	pc = 0x82DECE90; continue 'dispatch;
            }
            0x82DECE90 => {
    //   block [0x82DECE90..0x82DECEB8)
	// 82DECE90: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DECE94: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82DECE98: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	pc = 0x82DECEB8; continue 'dispatch;
            }
            0x82DECEB8 => {
    //   block [0x82DECEB8..0x82DECF10)
	// 82DECEB8: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DECEBC: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82DECEC0: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DECEC4: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	pc = 0x82DECF10; continue 'dispatch;
            }
            0x82DECF10 => {
    //   block [0x82DECF10..0x82DECF14)
	// 82DECF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DECF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DECF18 size=1736
    //   switch @ 0x82DED008: r10 with 4 label(s)
    //       case  0 → 0x82DED2FC
    //       case  1 → 0x82DED278
    //       case  2 → 0x82DED01C
    //       case  3 → 0x82DED2F4
    let mut pc: u32 = 0x82DECF18;
    'dispatch: loop {
        match pc {
            0x82DECF18 => {
    //   block [0x82DECF18..0x82DED01C)
	// 82DECF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DECF1C: 4BEBC4D5  bl 0x82ca93f0
	ctx.lr = 0x82DECF20;
	sub_82CA93D0(ctx, base);
	// 82DECF20: C0030120  lfs f0, 0x120(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DECF24: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82DECF28: C1A30040  lfs f13, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DECF2C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DECF30: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DECF34: D001FDE0  stfs f0, -0x220(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-544 as u32), tmp.u32 ) };
	// 82DECF38: 3901FDE0  addi r8, r1, -0x220
	ctx.r[8].s64 = ctx.r[1].s64 + -544;
	// 82DECF3C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DECF40: E8A90008  ld r5, 8(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 82DECF44: 3941FDE0  addi r10, r1, -0x220
	ctx.r[10].s64 = ctx.r[1].s64 + -544;
	pc = 0x82DED01C; continue 'dispatch;
            }
            0x82DED01C => {
    //   block [0x82DED01C..0x82DED278)
	// 82DED01C: C1430000  lfs f10, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DED020: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED024: D141FEF0  stfs f10, -0x110(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-272 as u32), tmp.u32 ) };
	// 82DED028: 38E1FEF0  addi r7, r1, -0x110
	ctx.r[7].s64 = ctx.r[1].s64 + -272;
	// 82DED02C: D001FE80  stfs f0, -0x180(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-384 as u32), tmp.u32 ) };
	// 82DED030: 1D08001C  mulli r8, r8, 0x1c
	ctx.r[8].s64 = ctx.r[8].s64 * 28;
	// 82DED034: D001FE84  stfs f0, -0x17c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-380 as u32), tmp.u32 ) };
	// 82DED038: D001FE88  stfs f0, -0x178(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-376 as u32), tmp.u32 ) };
	// 82DED03C: D001FE8C  stfs f0, -0x174(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-372 as u32), tmp.u32 ) };
	pc = 0x82DED278; continue 'dispatch;
            }
            0x82DED278 => {
    //   block [0x82DED278..0x82DED2F4)
	// 82DED278: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	pc = 0x82DED2F4; continue 'dispatch;
            }
            0x82DED2F4 => {
    //   block [0x82DED2F4..0x82DED2FC)
	// 82DED2F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DED2F8: 4BEBC148  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DED2FC => {
    //   block [0x82DED2FC..0x82DED5E0)
	// 82DED2FC: D001FE70  stfs f0, -0x190(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-400 as u32), tmp.u32 ) };
	// 82DED300: 13FF038C  vspltisw v31, -1
	for i in 0..4 {
		ctx.v[31].u32[i] = 4294967295;
	}
	// 82DED304: D001FE74  stfs f0, -0x18c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-396 as u32), tmp.u32 ) };
	// 82DED308: 38A30030  addi r5, r3, 0x30
	ctx.r[5].s64 = ctx.r[3].s64 + 48;
	// 82DED30C: D001FE78  stfs f0, -0x188(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-392 as u32), tmp.u32 ) };
	// 82DED310: 5744063E  clrlwi r4, r26, 0x18
	ctx.r[4].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	// 82DED314: D001FE7C  stfs f0, -0x184(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-388 as u32), tmp.u32 ) };
	// 82DED318: 3941FE70  addi r10, r1, -0x190
	ctx.r[10].s64 = ctx.r[1].s64 + -400;
	// 82DED31C: D001FEB0  stfs f0, -0x150(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-336 as u32), tmp.u32 ) };
	// 82DED320: D001FEB4  stfs f0, -0x14c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-332 as u32), tmp.u32 ) };
	// 82DED324: D001FEB8  stfs f0, -0x148(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-328 as u32), tmp.u32 ) };
	// 82DED328: D001FEBC  stfs f0, -0x144(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-324 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DED5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DED5E0 size=1696
    let mut pc: u32 = 0x82DED5E0;
    'dispatch: loop {
        match pc {
            0x82DED5E0 => {
    //   block [0x82DED5E0..0x82DEDC80)
	// 82DED5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DED5E4: 4BEBBE0D  bl 0x82ca93f0
	ctx.lr = 0x82DED5E8;
	sub_82CA93D0(ctx, base);
	// 82DED5E8: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82DED5EC: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DED5F0: A1660004  lhz r11, 4(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DED5F4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DED5F8: 39460020  addi r10, r6, 0x20
	ctx.r[10].s64 = ctx.r[6].s64 + 32;
	// 82DED5FC: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82DED600: 1D2B04F0  mulli r9, r11, 0x4f0
	ctx.r[9].s64 = ctx.r[11].s64 * 1264;
	// 82DED604: C0060018  lfs f0, 0x18(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DED608: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82DED60C: C006001C  lfs f0, 0x1c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DED610: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82DED614: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DED618: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82DED61C: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 82DED620: 1D0B00F0  mulli r8, r11, 0xf0
	ctx.r[8].s64 = ctx.r[11].s64 * 240;
	// 82DED624: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 82DED628: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82DED62C: 1CEB04F4  mulli r7, r11, 0x4f4
	ctx.r[7].s64 = ctx.r[11].s64 * 1268;
	// 82DED630: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82DED634: 39280020  addi r9, r8, 0x20
	ctx.r[9].s64 = ctx.r[8].s64 + 32;
	// 82DED638: 7CE73214  add r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82DED63C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82DED640: 556A183E  rotlwi r10, r11, 3
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 82DED644: 1CAB04B0  mulli r5, r11, 0x4b0
	ctx.r[5].s64 = ctx.r[11].s64 * 1200;
	// 82DED648: 91210080  stw r9, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82DED64C: 92C1007C  stw r22, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[22].u32 ) };
	// 82DED650: 39270044  addi r9, r7, 0x44
	ctx.r[9].s64 = ctx.r[7].s64 + 68;
	// 82DED654: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DED658: 7CA53214  add r5, r5, r6
	ctx.r[5].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 82DED65C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DED660: 1C6B04D0  mulli r3, r11, 0x4d0
	ctx.r[3].s64 = ctx.r[11].s64 * 1232;
	// 82DED664: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82DED668: 39250020  addi r9, r5, 0x20
	ctx.r[9].s64 = ctx.r[5].s64 + 32;
	// 82DED66C: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82DED670: 7CC33214  add r6, r3, r6
	ctx.r[6].u64 = ctx.r[3].u64 + ctx.r[6].u64;
	// 82DED674: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DED678: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DED67C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED680: 91210088  stw r9, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[9].u32 ) };
	// 82DED684: 39260040  addi r9, r6, 0x40
	ctx.r[9].s64 = ctx.r[6].s64 + 64;
	// 82DED688: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82DED68C: 9121008C  stw r9, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 82DED690: 4099004C  ble cr6, 0x82ded6dc
	if !ctx.cr[6].gt {
	pc = 0x82DED6DC; continue 'dispatch;
	}
	// 82DED694: 57E93032  slwi r9, r31, 6
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DED698: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82DED69C: 81010084  lwz r8, 0x84(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED6A0: C00A001C  lfs f0, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DED6A4: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DED6A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DED6AC: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82DED6B0: 39290014  addi r9, r9, 0x14
	ctx.r[9].s64 = ctx.r[9].s64 + 20;
	// 82DED6B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DED6B8: C188000C  lfs f12, 0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DED6BC: EC0C037A  fmadds f0, f12, f13, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82DED6C0: D00A001C  stfs f0, 0x1c(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DED6C4: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DED6C8: 409AFFD4  bne cr6, 0x82ded69c
	if !ctx.cr[6].eq {
	pc = 0x82DED69C; continue 'dispatch;
	}
	// 82DED6CC: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DED6D0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DED6D4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DED6D8: 4198FFBC  blt cr6, 0x82ded694
	if ctx.cr[6].lt {
	pc = 0x82DED694; continue 'dispatch;
	}
	// 82DED6DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED6E0: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82DED6E4: 480071F5  bl 0x82df48d8
	ctx.lr = 0x82DED6E8;
	sub_82DF48D8(ctx, base);
	// 82DED6E8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DED6EC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DED6F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DED6F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DED6F8: C3EB0C18  lfs f31, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DED6FC: 40990080  ble cr6, 0x82ded77c
	if !ctx.cr[6].gt {
	pc = 0x82DED77C; continue 'dispatch;
	}
	// 82DED700: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82DED704: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DED708: 57BC3032  slwi r28, r29, 6
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DED70C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DED710: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED714: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DED718: 7D5C28AE  lbzx r10, r28, r5
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82DED71C: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DED720: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82DED724: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82DED728: 409A0018  bne cr6, 0x82ded740
	if !ctx.cr[6].eq {
	pc = 0x82DED740; continue 'dispatch;
	}
	// 82DED72C: 7D7BFA14  add r11, r27, r31
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 82DED730: 8141008C  lwz r10, 0x8c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82DED734: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DED738: 7FEB552E  stfsx f31, r11, r10
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82DED73C: 4800001C  b 0x82ded758
	pc = 0x82DED758; continue 'dispatch;
	// 82DED740: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DED744: 419A0014  beq cr6, 0x82ded758
	if ctx.cr[6].eq {
	pc = 0x82DED758; continue 'dispatch;
	}
	// 82DED748: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DED74C: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82DED750: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DED754: 48006D0D  bl 0x82df4460
	ctx.lr = 0x82DED758;
	sub_82DF4460(ctx, base);
	// 82DED758: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82DED75C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DED760: 2F1E0006  cmpwi cr6, r30, 6
	ctx.cr[6].compare_i32(ctx.r[30].s32, 6, &mut ctx.xer);
	// 82DED764: 4198FFAC  blt cr6, 0x82ded710
	if ctx.cr[6].lt {
	pc = 0x82DED710; continue 'dispatch;
	}
	// 82DED768: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DED76C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82DED770: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82DED774: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DED778: 4198FF8C  blt cr6, 0x82ded704
	if ctx.cr[6].lt {
	pc = 0x82DED704; continue 'dispatch;
	}
	// 82DED77C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED780: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DED784: 4800755D  bl 0x82df4ce0
	ctx.lr = 0x82DED788;
	sub_82DF4CE0(ctx, base);
	// 82DED788: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82DED78C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DED790: 80E10084  lwz r7, 0x84(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED794: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED798: D3E10058  stfs f31, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DED79C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED7A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED7A4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DED7A8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DED7AC: 48007865  bl 0x82df5010
	ctx.lr = 0x82DED7B0;
	sub_82DF5010(ctx, base);
	// 82DED7B0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED7B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DED7B8: 41980064  blt cr6, 0x82ded81c
	if ctx.cr[6].lt {
	pc = 0x82DED81C; continue 'dispatch;
	}
	// 82DED7BC: 80C10080  lwz r6, 0x80(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DED7C0: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED7C4: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DED7C8: 48008379  bl 0x82df5b40
	ctx.lr = 0x82DED7CC;
	sub_82DF5B40(ctx, base);
	// 82DED7CC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DED7D0: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82DED7D4: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DED7D8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED7DC: 480073DD  bl 0x82df4bb8
	ctx.lr = 0x82DED7E0;
	sub_82DF4BB8(ctx, base);
	// 82DED7E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED7E4: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DED7E8: 480074F9  bl 0x82df4ce0
	ctx.lr = 0x82DED7EC;
	sub_82DF4CE0(ctx, base);
	// 82DED7EC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DED7F0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED7F4: 80E10084  lwz r7, 0x84(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED7F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED7FC: D3E10058  stfs f31, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DED800: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED804: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DED808: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DED80C: 48007805  bl 0x82df5010
	ctx.lr = 0x82DED810;
	sub_82DF5010(ctx, base);
	// 82DED810: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED814: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DED818: 4098FFA4  bge cr6, 0x82ded7bc
	if !ctx.cr[6].lt {
	pc = 0x82DED7BC; continue 'dispatch;
	}
	// 82DED81C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED820: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DED824: 480074BD  bl 0x82df4ce0
	ctx.lr = 0x82DED828;
	sub_82DF4CE0(ctx, base);
	// 82DED828: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DED82C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED830: 80E10084  lwz r7, 0x84(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED834: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED838: D3E10058  stfs f31, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DED83C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED840: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DED844: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DED848: 48007571  bl 0x82df4db8
	ctx.lr = 0x82DED84C;
	sub_82DF4DB8(ctx, base);
	// 82DED84C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED850: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DED854: 41980064  blt cr6, 0x82ded8b8
	if ctx.cr[6].lt {
	pc = 0x82DED8B8; continue 'dispatch;
	}
	// 82DED858: 80C10080  lwz r6, 0x80(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DED85C: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED860: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DED864: 480082DD  bl 0x82df5b40
	ctx.lr = 0x82DED868;
	sub_82DF5B40(ctx, base);
	// 82DED868: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82DED86C: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED870: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DED874: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED878: 48006BE9  bl 0x82df4460
	ctx.lr = 0x82DED87C;
	sub_82DF4460(ctx, base);
	// 82DED87C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED880: 80810088  lwz r4, 0x88(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DED884: 4800745D  bl 0x82df4ce0
	ctx.lr = 0x82DED888;
	sub_82DF4CE0(ctx, base);
	// 82DED888: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82DED88C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DED890: 80E10084  lwz r7, 0x84(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DED894: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DED898: D3E10058  stfs f31, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82DED89C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DED8A0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DED8A4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DED8A8: 48007511  bl 0x82df4db8
	ctx.lr = 0x82DED8AC;
	sub_82DF4DB8(ctx, base);
	// 82DED8AC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DED8B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DED8B4: 4098FFA4  bge cr6, 0x82ded858
	if !ctx.cr[6].lt {
	pc = 0x82DED858; continue 'dispatch;
	}
	// 82DED8B8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DED8BC: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82DED8C0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DED8C4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEDC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DEDC80 size=7024
    let mut pc: u32 = 0x82DEDC80;
    'dispatch: loop {
        match pc {
            0x82DEDC80 => {
    //   block [0x82DEDC80..0x82DEDD7C)
	// 82DEDC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEDC84: 4BEBB74D  bl 0x82ca93d0
	ctx.lr = 0x82DEDC88;
	sub_82CA93D0(ctx, base);
	// 82DEDC88: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DEDC8C: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DEDC90: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
	pc = 0x82DEDD7C; continue 'dispatch;
            }
            0x82DEDD7C => {
    //   block [0x82DEDD7C..0x82DEDE1C)
	// 82DEDD7C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82DEDD80: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DEDD84: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DEDD88: 2B0B001C  cmplwi cr6, r11, 0x1c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 28 as u32, &mut ctx.xer);
	// 82DEDD8C: 41991A40  bgt cr6, 0x82def7cc
	if ctx.cr[6].gt {
	pc = 0x82DEF7CC; continue 'dispatch;
	}
	// 82DEDD90: 3D8082DF  lis r12, -0x7d21
	ctx.r[12].s64 = -2099314688;
	// 82DEDD94: 398CDDA8  addi r12, r12, -0x2258
	ctx.r[12].s64 = ctx.r[12].s64 + -8792;
	// 82DEDD98: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DEDD9C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82DEDDA0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82DEDDA4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82DEF7A8; continue 'dispatch;
		},
		1 => {
	pc = 0x82DEE750; continue 'dispatch;
		},
		2 => {
	pc = 0x82DEF624; continue 'dispatch;
		},
		3 => {
	pc = 0x82DEDD7C; continue 'dispatch;
		},
		4 => {
	pc = 0x82DEF7CC; continue 'dispatch;
		},
		5 => {
	pc = 0x82DEED24; continue 'dispatch;
		},
		6 => {
	pc = 0x82DEEDE4; continue 'dispatch;
		},
		7 => {
	pc = 0x82DEF0C8; continue 'dispatch;
		},
		8 => {
	pc = 0x82DEEFCC; continue 'dispatch;
		},
		9 => {
	pc = 0x82DEF228; continue 'dispatch;
		},
		10 => {
	pc = 0x82DEF498; continue 'dispatch;
		},
		11 => {
	pc = 0x82DEECA0; continue 'dispatch;
		},
		12 => {
	pc = 0x82DEEA08; continue 'dispatch;
		},
		13 => {
	pc = 0x82DEE93C; continue 'dispatch;
		},
		14 => {
	pc = 0x82DEEB28; continue 'dispatch;
		},
		15 => {
	pc = 0x82DEE200; continue 'dispatch;
		},
		16 => {
	pc = 0x82DEE200; continue 'dispatch;
		},
		17 => {
	pc = 0x82DEE2D0; continue 'dispatch;
		},
		18 => {
	pc = 0x82DEE2D0; continue 'dispatch;
		},
		19 => {
	pc = 0x82DEDE1C; continue 'dispatch;
		},
		20 => {
	pc = 0x82DEDE1C; continue 'dispatch;
		},
		21 => {
	pc = 0x82DEE784; continue 'dispatch;
		},
		22 => {
	pc = 0x82DEE500; continue 'dispatch;
		},
		23 => {
	pc = 0x82DEF544; continue 'dispatch;
		},
		24 => {
	pc = 0x82DEF58C; continue 'dispatch;
		},
		25 => {
	pc = 0x82DEF7CC; continue 'dispatch;
		},
		26 => {
	pc = 0x82DEF630; continue 'dispatch;
		},
		27 => {
	pc = 0x82DEF6A8; continue 'dispatch;
		},
		28 => {
	pc = 0x82DEF728; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82DEDDA8: 82DEF7A8  lwz r22, -0x858(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2136 as u32) ) } as u64;
	// 82DEDDAC: 82DEE750  lwz r22, -0x18b0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6320 as u32) ) } as u64;
	// 82DEDDB0: 82DEF624  lwz r22, -0x9dc(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2524 as u32) ) } as u64;
	// 82DEDDB4: 82DEDD7C  lwz r22, -0x2284(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8836 as u32) ) } as u64;
	// 82DEDDB8: 82DEF7CC  lwz r22, -0x834(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2100 as u32) ) } as u64;
	// 82DEDDBC: 82DEED24  lwz r22, -0x12dc(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4828 as u32) ) } as u64;
	// 82DEDDC0: 82DEEDE4  lwz r22, -0x121c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4636 as u32) ) } as u64;
	// 82DEDDC4: 82DEF0C8  lwz r22, -0xf38(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-3896 as u32) ) } as u64;
	// 82DEDDC8: 82DEEFCC  lwz r22, -0x1034(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4148 as u32) ) } as u64;
	// 82DEDDCC: 82DEF228  lwz r22, -0xdd8(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-3544 as u32) ) } as u64;
	// 82DEDDD0: 82DEF498  lwz r22, -0xb68(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2920 as u32) ) } as u64;
	// 82DEDDD4: 82DEECA0  lwz r22, -0x1360(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4960 as u32) ) } as u64;
	// 82DEDDD8: 82DEEA08  lwz r22, -0x15f8(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-5624 as u32) ) } as u64;
	// 82DEDDDC: 82DEE93C  lwz r22, -0x16c4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-5828 as u32) ) } as u64;
	// 82DEDDE0: 82DEEB28  lwz r22, -0x14d8(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-5336 as u32) ) } as u64;
	// 82DEDDE4: 82DEE200  lwz r22, -0x1e00(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-7680 as u32) ) } as u64;
	// 82DEDDE8: 82DEE200  lwz r22, -0x1e00(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-7680 as u32) ) } as u64;
	// 82DEDDEC: 82DEE2D0  lwz r22, -0x1d30(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-7472 as u32) ) } as u64;
	// 82DEDDF0: 82DEE2D0  lwz r22, -0x1d30(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-7472 as u32) ) } as u64;
	// 82DEDDF4: 82DEDE1C  lwz r22, -0x21e4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8676 as u32) ) } as u64;
	// 82DEDDF8: 82DEDE1C  lwz r22, -0x21e4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8676 as u32) ) } as u64;
	// 82DEDDFC: 82DEE784  lwz r22, -0x187c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6268 as u32) ) } as u64;
	// 82DEDE00: 82DEE500  lwz r22, -0x1b00(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-6912 as u32) ) } as u64;
	// 82DEDE04: 82DEF544  lwz r22, -0xabc(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2748 as u32) ) } as u64;
	// 82DEDE08: 82DEF58C  lwz r22, -0xa74(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2676 as u32) ) } as u64;
	// 82DEDE0C: 82DEF7CC  lwz r22, -0x834(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2100 as u32) ) } as u64;
	// 82DEDE10: 82DEF630  lwz r22, -0x9d0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2512 as u32) ) } as u64;
	// 82DEDE14: 82DEF6A8  lwz r22, -0x958(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2392 as u32) ) } as u64;
	// 82DEDE18: 82DEF728  lwz r22, -0x8d8(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2264 as u32) ) } as u64;
            }
            0x82DEDE1C => {
    //   block [0x82DEDE1C..0x82DEE200)
	pc = 0x82DEE200; continue 'dispatch;
            }
            0x82DEE200 => {
    //   block [0x82DEE200..0x82DEE2D0)
	pc = 0x82DEE2D0; continue 'dispatch;
            }
            0x82DEE2D0 => {
    //   block [0x82DEE2D0..0x82DEE500)
	pc = 0x82DEE500; continue 'dispatch;
            }
            0x82DEE500 => {
    //   block [0x82DEE500..0x82DEE750)
	pc = 0x82DEE750; continue 'dispatch;
            }
            0x82DEE750 => {
    //   block [0x82DEE750..0x82DEE784)
	// 82DEE750: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEE754: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82DEE758: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82DEE75C: 7C005A2C  dcbt 0, r11
	// 82DEE760: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DEE764: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82DEE768: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82DEE76C: 7C005A2C  dcbt 0, r11
	pc = 0x82DEE784; continue 'dispatch;
            }
            0x82DEE784 => {
    //   block [0x82DEE784..0x82DEE93C)
	pc = 0x82DEE93C; continue 'dispatch;
            }
            0x82DEE93C => {
    //   block [0x82DEE93C..0x82DEEA08)
	pc = 0x82DEEA08; continue 'dispatch;
            }
            0x82DEEA08 => {
    //   block [0x82DEEA08..0x82DEEB28)
	// 82DEEA08: 39540008  addi r10, r20, 8
	ctx.r[10].s64 = ctx.r[20].s64 + 8;
	// 82DEEA0C: 39370050  addi r9, r23, 0x50
	ctx.r[9].s64 = ctx.r[23].s64 + 80;
	// 82DEEA10: 39160050  addi r8, r22, 0x50
	ctx.r[8].s64 = ctx.r[22].s64 + 80;
	// 82DEEA14: 38F40040  addi r7, r20, 0x40
	ctx.r[7].s64 = ctx.r[20].s64 + 64;
	pc = 0x82DEEB28; continue 'dispatch;
            }
            0x82DEEB28 => {
    //   block [0x82DEEB28..0x82DEECA0)
	// 82DEEB28: 39370050  addi r9, r23, 0x50
	ctx.r[9].s64 = ctx.r[23].s64 + 80;
	// 82DEEB2C: 39160050  addi r8, r22, 0x50
	ctx.r[8].s64 = ctx.r[22].s64 + 80;
	// 82DEEB30: 38F40040  addi r7, r20, 0x40
	ctx.r[7].s64 = ctx.r[20].s64 + 64;
	// 82DEEB34: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82DEEB38: 38C0FFEC  li r6, -0x14
	ctx.r[6].s64 = -20;
	// 82DEEB3C: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEEB40: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DEEB44: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEEB48: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DEEB4C: EC006FFA  fmadds f0, f0, f31, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DEEB50: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	pc = 0x82DEECA0; continue 'dispatch;
            }
            0x82DEECA0 => {
    //   block [0x82DEECA0..0x82DEED24)
	pc = 0x82DEED24; continue 'dispatch;
            }
            0x82DEED24 => {
    //   block [0x82DEED24..0x82DEEDE4)
	pc = 0x82DEEDE4; continue 'dispatch;
            }
            0x82DEEDE4 => {
    //   block [0x82DEEDE4..0x82DEEFCC)
	// 82DEEDE4: 38740040  addi r3, r20, 0x40
	ctx.r[3].s64 = ctx.r[20].s64 + 64;
	// 82DEEDE8: 39760040  addi r11, r22, 0x40
	ctx.r[11].s64 = ctx.r[22].s64 + 64;
	// 82DEEDEC: 39570040  addi r10, r23, 0x40
	ctx.r[10].s64 = ctx.r[23].s64 + 64;
	// 82DEEDF0: 39360050  addi r9, r22, 0x50
	ctx.r[9].s64 = ctx.r[22].s64 + 80;
	// 82DEEDF4: 39170050  addi r8, r23, 0x50
	ctx.r[8].s64 = ctx.r[23].s64 + 80;
	pc = 0x82DEEFCC; continue 'dispatch;
            }
            0x82DEEFCC => {
    //   block [0x82DEEFCC..0x82DEF0C8)
	pc = 0x82DEF0C8; continue 'dispatch;
            }
            0x82DEF0C8 => {
    //   block [0x82DEF0C8..0x82DEF228)
	pc = 0x82DEF228; continue 'dispatch;
            }
            0x82DEF228 => {
    //   block [0x82DEF228..0x82DEF498)
	// 82DEF228: 3BB40040  addi r29, r20, 0x40
	ctx.r[29].s64 = ctx.r[20].s64 + 64;
	// 82DEF22C: 38D60040  addi r6, r22, 0x40
	ctx.r[6].s64 = ctx.r[22].s64 + 64;
	// 82DEF230: 38B70040  addi r5, r23, 0x40
	ctx.r[5].s64 = ctx.r[23].s64 + 64;
	// 82DEF234: 38960050  addi r4, r22, 0x50
	ctx.r[4].s64 = ctx.r[22].s64 + 80;
	// 82DEF238: 38770050  addi r3, r23, 0x50
	ctx.r[3].s64 = ctx.r[23].s64 + 80;
	// 82DEF23C: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 82DEF240: 3A80FFDC  li r20, -0x24
	ctx.r[20].s64 = -36;
	// 82DEF244: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEF248: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEF24C: 39410170  addi r10, r1, 0x170
	ctx.r[10].s64 = ctx.r[1].s64 + 368;
	// 82DEF250: EC006FFA  fmadds f0, f0, f31, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DEF254: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	pc = 0x82DEF498; continue 'dispatch;
            }
            0x82DEF498 => {
    //   block [0x82DEF498..0x82DEF544)
	pc = 0x82DEF544; continue 'dispatch;
            }
            0x82DEF544 => {
    //   block [0x82DEF544..0x82DEF58C)
	// 82DEF544: 39770030  addi r11, r23, 0x30
	ctx.r[11].s64 = ctx.r[23].s64 + 48;
	pc = 0x82DEF58C; continue 'dispatch;
            }
            0x82DEF58C => {
    //   block [0x82DEF58C..0x82DEF624)
	pc = 0x82DEF624; continue 'dispatch;
            }
            0x82DEF624 => {
    //   block [0x82DEF624..0x82DEF630)
	// 82DEF624: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DEF628: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DEF62C: 4BFFE754  b 0x82dedd80
	pc = 0x82DEDD80; continue 'dispatch;
            }
            0x82DEF630 => {
    //   block [0x82DEF630..0x82DEF6A8)
	// 82DEF630: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DEF6A8; continue 'dispatch;
            }
            0x82DEF6A8 => {
    //   block [0x82DEF6A8..0x82DEF728)
	// 82DEF6A8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DEF728; continue 'dispatch;
            }
            0x82DEF728 => {
    //   block [0x82DEF728..0x82DEF7A8)
	// 82DEF728: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DEF7A8; continue 'dispatch;
            }
            0x82DEF7A8 => {
    //   block [0x82DEF7A8..0x82DEF7CC)
	pc = 0x82DEF7CC; continue 'dispatch;
            }
            0x82DEF7CC => {
    //   block [0x82DEF7CC..0x82DEF7F0)
	// 82DEF7CC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82DEF7D0: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82DEF7D4: 3800FF30  li r0, -0xd0
	ctx.r[0].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DEF7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DEF7F0 size=9124
    //   switch @ 0x82DEF8C8: r10 with 4 label(s)
    //       case  0 → 0x82DEF904
    //       case  1 → 0x82DEF8DC
    //       case  2 → 0x82DEF8DC
    //       case  3 → 0x82DEF95C
    let mut pc: u32 = 0x82DEF7F0;
    'dispatch: loop {
        match pc {
            0x82DEF7F0 => {
    //   block [0x82DEF7F0..0x82DEF8DC)
	// 82DEF7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DEF7F4: 4BEB9BDD  bl 0x82ca93d0
	ctx.lr = 0x82DEF7F8;
	sub_82CA93D0(ctx, base);
	// 82DEF7F8: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82DEF7FC: 4BEBE4D5  bl 0x82cadcd0
	ctx.lr = 0x82DEF800;
	sub_82CADCA0(ctx, base);
	// 82DEF800: 3981FF30  addi r12, r1, -0xd0
	ctx.r[12].s64 = ctx.r[1].s64 + -208;
	// 82DEF804: 482171D1  bl 0x830069d4
	ctx.lr = 0x82DEF808;
	sub_83006760(ctx, base);
	// 82DEF808: 9421FB60  stwu r1, -0x4a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1184 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DEF80C: 3FE08334  lis r31, -0x7ccc
	ctx.r[31].s64 = -2093744128;
	// 82DEF810: 908104BC  stw r4, 0x4bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1212 as u32), ctx.r[4].u32 ) };
	// 82DEF814: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DEF818: 90C104CC  stw r6, 0x4cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1228 as u32), ctx.r[6].u32 ) };
	// 82DEF81C: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 82DEF820: 897FAC52  lbz r11, -0x53ae(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(-21422 as u32) ) } as u64;
	// 82DEF824: 92E104B4  stw r23, 0x4b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1204 as u32), ctx.r[23].u32 ) };
	// 82DEF828: 926104C4  stw r19, 0x4c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1220 as u32), ctx.r[19].u32 ) };
	// 82DEF82C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DEF830: 409A0024  bne cr6, 0x82def854
	if !ctx.cr[6].eq {
	pc = 0x82DEF854; continue 'dispatch;
	}
	// 82DEF834: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF838: 48007C01  bl 0x82df7438
	ctx.lr = 0x82DEF83C;
	sub_82DF7438(ctx, base);
	// 82DEF83C: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82DEF840: 987FAC52  stb r3, -0x53ae(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-21422 as u32), ctx.r[3].u8 ) };
	// 82DEF844: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF848: 409A000C  bne cr6, 0x82def854
	if !ctx.cr[6].eq {
	pc = 0x82DEF854; continue 'dispatch;
	}
	// 82DEF84C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DEF850: 48002038  b 0x82df1888
	pc = 0x82DF1888; continue 'dispatch;
	// 82DEF854: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 82DEF858: 39370010  addi r9, r23, 0x10
	ctx.r[9].s64 = ctx.r[23].s64 + 16;
            }
            0x82DEF8DC => {
    //   block [0x82DEF8DC..0x82DEF904)
	// 82DEF8DC: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DEF8E0: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82DEF8E4: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	pc = 0x82DEF904; continue 'dispatch;
            }
            0x82DEF904 => {
    //   block [0x82DEF904..0x82DEF95C)
	// 82DEF904: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	// 82DEF908: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82DEF90C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DEF910: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	pc = 0x82DEF95C; continue 'dispatch;
            }
            0x82DEF95C => {
    //   block [0x82DEF95C..0x82DEFA90)
	// 82DEF95C: 81770114  lwz r11, 0x114(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(276 as u32) ) } as u64;
	// 82DEF960: C0170120  lfs f0, 0x120(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DEF964: C1B70040  lfs f13, 0x40(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(64 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEF968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DEF96C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DEF970: D00100E0  stfs f0, 0xe0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 82DEF974: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DEF978: 396100E0  addi r11, r1, 0xe0
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	// 82DEF97C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	pc = 0x82DEFA90; continue 'dispatch;
            }
            0x82DEFA90 => {
    //   block [0x82DEFA90..0x82DEFB30)
	// 82DEFA90: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82DEFA94: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DEFA98: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DEFA9C: 2B0B001C  cmplwi cr6, r11, 0x1c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 28 as u32, &mut ctx.xer);
	// 82DEFAA0: 41991A54  bgt cr6, 0x82df14f4
	if ctx.cr[6].gt {
	pc = 0x82DF14F4; continue 'dispatch;
	}
	// 82DEFAA4: 3D8082DF  lis r12, -0x7d21
	ctx.r[12].s64 = -2099314688;
	// 82DEFAA8: 398CFABC  addi r12, r12, -0x544
	ctx.r[12].s64 = ctx.r[12].s64 + -1348;
	// 82DEFAAC: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DEFAB0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82DEFAB4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82DEFAB8: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82DF14D0; continue 'dispatch;
		},
		1 => {
	pc = 0x82DF0464; continue 'dispatch;
		},
		2 => {
	pc = 0x82DF134C; continue 'dispatch;
		},
		3 => {
	pc = 0x82DEFA90; continue 'dispatch;
		},
		4 => {
	pc = 0x82DF14F4; continue 'dispatch;
		},
		5 => {
	pc = 0x82DF0A34; continue 'dispatch;
		},
		6 => {
	pc = 0x82DF0AF4; continue 'dispatch;
		},
		7 => {
	pc = 0x82DF0DD8; continue 'dispatch;
		},
		8 => {
	pc = 0x82DF0CDC; continue 'dispatch;
		},
		9 => {
	pc = 0x82DF0F38; continue 'dispatch;
		},
		10 => {
	pc = 0x82DF11A8; continue 'dispatch;
		},
		11 => {
	pc = 0x82DF09B0; continue 'dispatch;
		},
		12 => {
	pc = 0x82DF0718; continue 'dispatch;
		},
		13 => {
	pc = 0x82DF0650; continue 'dispatch;
		},
		14 => {
	pc = 0x82DF0838; continue 'dispatch;
		},
		15 => {
	pc = 0x82DEFF14; continue 'dispatch;
		},
		16 => {
	pc = 0x82DEFF14; continue 'dispatch;
		},
		17 => {
	pc = 0x82DEFFE4; continue 'dispatch;
		},
		18 => {
	pc = 0x82DEFFE4; continue 'dispatch;
		},
		19 => {
	pc = 0x82DEFB30; continue 'dispatch;
		},
		20 => {
	pc = 0x82DEFB30; continue 'dispatch;
		},
		21 => {
	pc = 0x82DF0498; continue 'dispatch;
		},
		22 => {
	pc = 0x82DF0214; continue 'dispatch;
		},
		23 => {
	pc = 0x82DF1254; continue 'dispatch;
		},
		24 => {
	pc = 0x82DF129C; continue 'dispatch;
		},
		25 => {
	pc = 0x82DF14F4; continue 'dispatch;
		},
		26 => {
	pc = 0x82DF1358; continue 'dispatch;
		},
		27 => {
	pc = 0x82DF13D0; continue 'dispatch;
		},
		28 => {
	pc = 0x82DF1450; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82DEFABC: 82DF14D0  lwz r22, 0x14d0(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5328 as u32) ) } as u64;
	// 82DEFAC0: 82DF0464  lwz r22, 0x464(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1124 as u32) ) } as u64;
	// 82DEFAC4: 82DF134C  lwz r22, 0x134c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4940 as u32) ) } as u64;
	// 82DEFAC8: 82DEFA90  lwz r22, -0x570(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1392 as u32) ) } as u64;
	// 82DEFACC: 82DF14F4  lwz r22, 0x14f4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5364 as u32) ) } as u64;
	// 82DEFAD0: 82DF0A34  lwz r22, 0xa34(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2612 as u32) ) } as u64;
	// 82DEFAD4: 82DF0AF4  lwz r22, 0xaf4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2804 as u32) ) } as u64;
	// 82DEFAD8: 82DF0DD8  lwz r22, 0xdd8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3544 as u32) ) } as u64;
	// 82DEFADC: 82DF0CDC  lwz r22, 0xcdc(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3292 as u32) ) } as u64;
	// 82DEFAE0: 82DF0F38  lwz r22, 0xf38(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3896 as u32) ) } as u64;
	// 82DEFAE4: 82DF11A8  lwz r22, 0x11a8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4520 as u32) ) } as u64;
	// 82DEFAE8: 82DF09B0  lwz r22, 0x9b0(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2480 as u32) ) } as u64;
	// 82DEFAEC: 82DF0718  lwz r22, 0x718(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1816 as u32) ) } as u64;
	// 82DEFAF0: 82DF0650  lwz r22, 0x650(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1616 as u32) ) } as u64;
	// 82DEFAF4: 82DF0838  lwz r22, 0x838(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2104 as u32) ) } as u64;
	// 82DEFAF8: 82DEFF14  lwz r22, -0xec(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-236 as u32) ) } as u64;
	// 82DEFAFC: 82DEFF14  lwz r22, -0xec(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-236 as u32) ) } as u64;
	// 82DEFB00: 82DEFFE4  lwz r22, -0x1c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82DEFB04: 82DEFFE4  lwz r22, -0x1c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82DEFB08: 82DEFB30  lwz r22, -0x4d0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1232 as u32) ) } as u64;
	// 82DEFB0C: 82DEFB30  lwz r22, -0x4d0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1232 as u32) ) } as u64;
	// 82DEFB10: 82DF0498  lwz r22, 0x498(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1176 as u32) ) } as u64;
	// 82DEFB14: 82DF0214  lwz r22, 0x214(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(532 as u32) ) } as u64;
	// 82DEFB18: 82DF1254  lwz r22, 0x1254(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4692 as u32) ) } as u64;
	// 82DEFB1C: 82DF129C  lwz r22, 0x129c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4764 as u32) ) } as u64;
	// 82DEFB20: 82DF14F4  lwz r22, 0x14f4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5364 as u32) ) } as u64;
	// 82DEFB24: 82DF1358  lwz r22, 0x1358(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4952 as u32) ) } as u64;
	// 82DEFB28: 82DF13D0  lwz r22, 0x13d0(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5072 as u32) ) } as u64;
	// 82DEFB2C: 82DF1450  lwz r22, 0x1450(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5200 as u32) ) } as u64;
            }
            0x82DEFB30 => {
    //   block [0x82DEFB30..0x82DEFF14)
	pc = 0x82DEFF14; continue 'dispatch;
            }
            0x82DEFF14 => {
    //   block [0x82DEFF14..0x82DEFFE4)
	pc = 0x82DEFFE4; continue 'dispatch;
            }
            0x82DEFFE4 => {
    //   block [0x82DEFFE4..0x82DF0214)
	pc = 0x82DF0214; continue 'dispatch;
            }
            0x82DF0214 => {
    //   block [0x82DF0214..0x82DF0464)
	pc = 0x82DF0464; continue 'dispatch;
            }
            0x82DF0464 => {
    //   block [0x82DF0464..0x82DF0498)
	// 82DF0464: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF0468: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82DF046C: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82DF0470: 7C005A2C  dcbt 0, r11
	// 82DF0474: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DF0478: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82DF047C: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82DF0480: 7C005A2C  dcbt 0, r11
	pc = 0x82DF0498; continue 'dispatch;
            }
            0x82DF0498 => {
    //   block [0x82DF0498..0x82DF0650)
	pc = 0x82DF0650; continue 'dispatch;
            }
            0x82DF0650 => {
    //   block [0x82DF0650..0x82DF0718)
	pc = 0x82DF0718; continue 'dispatch;
            }
            0x82DF0718 => {
    //   block [0x82DF0718..0x82DF0838)
	// 82DF0718: 39570008  addi r10, r23, 8
	ctx.r[10].s64 = ctx.r[23].s64 + 8;
	// 82DF071C: 39360050  addi r9, r22, 0x50
	ctx.r[9].s64 = ctx.r[22].s64 + 80;
	// 82DF0720: 39150050  addi r8, r21, 0x50
	ctx.r[8].s64 = ctx.r[21].s64 + 80;
	pc = 0x82DF0838; continue 'dispatch;
            }
            0x82DF0838 => {
    //   block [0x82DF0838..0x82DF09B0)
	// 82DF0838: 39360050  addi r9, r22, 0x50
	ctx.r[9].s64 = ctx.r[22].s64 + 80;
	// 82DF083C: 39150050  addi r8, r21, 0x50
	ctx.r[8].s64 = ctx.r[21].s64 + 80;
	// 82DF0840: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82DF0844: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF0848: 3880FFEC  li r4, -0x14
	ctx.r[4].s64 = -20;
	// 82DF084C: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF0850: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DF0854: EC006F7A  fmadds f0, f0, f29, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[29].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF0858: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	pc = 0x82DF09B0; continue 'dispatch;
            }
            0x82DF09B0 => {
    //   block [0x82DF09B0..0x82DF0A34)
	pc = 0x82DF0A34; continue 'dispatch;
            }
            0x82DF0A34 => {
    //   block [0x82DF0A34..0x82DF0AF4)
	pc = 0x82DF0AF4; continue 'dispatch;
            }
            0x82DF0AF4 => {
    //   block [0x82DF0AF4..0x82DF0CDC)
	// 82DF0AF4: 39750040  addi r11, r21, 0x40
	ctx.r[11].s64 = ctx.r[21].s64 + 64;
	// 82DF0AF8: 39560040  addi r10, r22, 0x40
	ctx.r[10].s64 = ctx.r[22].s64 + 64;
	// 82DF0AFC: 39350050  addi r9, r21, 0x50
	ctx.r[9].s64 = ctx.r[21].s64 + 80;
	// 82DF0B00: 39160050  addi r8, r22, 0x50
	ctx.r[8].s64 = ctx.r[22].s64 + 80;
	pc = 0x82DF0CDC; continue 'dispatch;
            }
            0x82DF0CDC => {
    //   block [0x82DF0CDC..0x82DF0DD8)
	pc = 0x82DF0DD8; continue 'dispatch;
            }
            0x82DF0DD8 => {
    //   block [0x82DF0DD8..0x82DF0F38)
	pc = 0x82DF0F38; continue 'dispatch;
            }
            0x82DF0F38 => {
    //   block [0x82DF0F38..0x82DF11A8)
	// 82DF0F38: 38D50040  addi r6, r21, 0x40
	ctx.r[6].s64 = ctx.r[21].s64 + 64;
	// 82DF0F3C: 38B60040  addi r5, r22, 0x40
	ctx.r[5].s64 = ctx.r[22].s64 + 64;
	// 82DF0F40: 38950050  addi r4, r21, 0x50
	ctx.r[4].s64 = ctx.r[21].s64 + 80;
	// 82DF0F44: 38760050  addi r3, r22, 0x50
	ctx.r[3].s64 = ctx.r[22].s64 + 80;
	// 82DF0F48: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 82DF0F4C: 3A60FFDC  li r19, -0x24
	ctx.r[19].s64 = -36;
	// 82DF0F50: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF0F54: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF0F58: 39410370  addi r10, r1, 0x370
	ctx.r[10].s64 = ctx.r[1].s64 + 880;
	// 82DF0F5C: EC006F7A  fmadds f0, f0, f29, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[29].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF0F60: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	pc = 0x82DF11A8; continue 'dispatch;
            }
            0x82DF11A8 => {
    //   block [0x82DF11A8..0x82DF1254)
	pc = 0x82DF1254; continue 'dispatch;
            }
            0x82DF1254 => {
    //   block [0x82DF1254..0x82DF129C)
	// 82DF1254: 39760030  addi r11, r22, 0x30
	ctx.r[11].s64 = ctx.r[22].s64 + 48;
	pc = 0x82DF129C; continue 'dispatch;
            }
            0x82DF129C => {
    //   block [0x82DF129C..0x82DF134C)
	// 82DF129C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	pc = 0x82DF134C; continue 'dispatch;
            }
            0x82DF134C => {
    //   block [0x82DF134C..0x82DF1358)
	// 82DF134C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF1350: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF1354: 4BFFE740  b 0x82defa94
	pc = 0x82DEFA94; continue 'dispatch;
            }
            0x82DF1358 => {
    //   block [0x82DF1358..0x82DF13D0)
	// 82DF1358: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DF13D0; continue 'dispatch;
            }
            0x82DF13D0 => {
    //   block [0x82DF13D0..0x82DF1450)
	// 82DF13D0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DF1450; continue 'dispatch;
            }
            0x82DF1450 => {
    //   block [0x82DF1450..0x82DF14D0)
	// 82DF1450: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DF14D0; continue 'dispatch;
            }
            0x82DF14D0 => {
    //   block [0x82DF14D0..0x82DF14F4)
	pc = 0x82DF14F4; continue 'dispatch;
            }
            0x82DF14F4 => {
    //   block [0x82DF14F4..0x82DF1578)
	// 82DF14F4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82DF14F8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DF14FC: FFA0F890  fmr f29, f31
	ctx.f[29].f64 = ctx.f[31].f64;
	// 82DF1500: D3A100AC  stfs f29, 0xac(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82DF1504: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF1508: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF150C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DF1510: 4199E528  bgt cr6, 0x82defa38
	if ctx.cr[6].gt {
	pc = 0x82DEFA38; continue 'dispatch;
	}
	// 82DF1514: 39770114  addi r11, r23, 0x114
	ctx.r[11].s64 = ctx.r[23].s64 + 276;
	// 82DF1518: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DF151C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF1520: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1524: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF1528: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF152C: 409A0008  bne cr6, 0x82df1534
	if !ctx.cr[6].eq {
	pc = 0x82DF1534; continue 'dispatch;
	}
	// 82DF1530: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF1534: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DF1538: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 82DF153C: 7F135040  cmplw cr6, r19, r10
	ctx.cr[6].compare_u32(ctx.r[19].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF1540: 40980328  bge cr6, 0x82df1868
	if !ctx.cr[6].lt {
	pc = 0x82DF1868; continue 'dispatch;
	}
	// 82DF1544: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF1548: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 82DF154C: 41990310  bgt cr6, 0x82df185c
	if ctx.cr[6].gt {
	pc = 0x82DF185C; continue 'dispatch;
	}
	// 82DF1550: 3D8082DF  lis r12, -0x7d21
	ctx.r[12].s64 = -2099314688;
	// 82DF1554: 398C1568  addi r12, r12, 0x1568
	ctx.r[12].s64 = ctx.r[12].s64 + 5480;
	// 82DF1558: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DF155C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82DF1560: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82DF1564: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x82DF18A0; continue 'dispatch;
		},
		1 => {
	pc = 0x82DF17E4; continue 'dispatch;
		},
		2 => {
	pc = 0x82DF1578; continue 'dispatch;
		},
		3 => {
	pc = 0x82DF1868; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82DF1568: 82DF18A0  lwz r22, 0x18a0(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6304 as u32) ) } as u64;
	// 82DF156C: 82DF17E4  lwz r22, 0x17e4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 82DF1570: 82DF1578  lwz r22, 0x1578(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5496 as u32) ) } as u64;
	// 82DF1574: 82DF1868  lwz r22, 0x1868(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6248 as u32) ) } as u64;
            }
            0x82DF1578 => {
    //   block [0x82DF1578..0x82DF17E4)
	// 82DF1578: 80E10070  lwz r7, 0x70(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82DF157C: C0170000  lfs f0, 0(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF1580: D0010270  stfs f0, 0x270(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(624 as u32), tmp.u32 ) };
	// 82DF1584: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF1588: D3C101C0  stfs f30, 0x1c0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 82DF158C: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF1590: D3C101C4  stfs f30, 0x1c4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), tmp.u32 ) };
	// 82DF1594: 1D08001C  mulli r8, r8, 0x1c
	ctx.r[8].s64 = ctx.r[8].s64 * 28;
	// 82DF1598: D3C101C8  stfs f30, 0x1c8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(456 as u32), tmp.u32 ) };
	// 82DF159C: 7D08BA14  add r8, r8, r23
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[23].u64;
	pc = 0x82DF17E4; continue 'dispatch;
            }
            0x82DF17E4 => {
    //   block [0x82DF17E4..0x82DF1868)
	// 82DF17E4: 394B0040  addi r10, r11, 0x40
	ctx.r[10].s64 = ctx.r[11].s64 + 64;
	pc = 0x82DF1868; continue 'dispatch;
            }
            0x82DF1868 => {
    //   block [0x82DF1868..0x82DF18A0)
	// 82DF1868: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DF186C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82DF1870: 39770114  addi r11, r23, 0x114
	ctx.r[11].s64 = ctx.r[23].s64 + 276;
	// 82DF1874: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DF1878: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF187C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF1880: 4198E19C  blt cr6, 0x82defa1c
	if ctx.cr[6].lt {
	pc = 0x82DEFA1C; continue 'dispatch;
	}
	// 82DF1884: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF1888: 382104A0  addi r1, r1, 0x4a0
	ctx.r[1].s64 = ctx.r[1].s64 + 1184;
	// 82DF188C: 3981FF30  addi r12, r1, -0xd0
	ctx.r[12].s64 = ctx.r[1].s64 + -208;
	// 82DF1890: 482153DD  bl 0x83006c6c
	ctx.lr = 0x82DF1894;
	sub_830069F8(ctx, base);
	// 82DF1894: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82DF1898: 4BEBC485  bl 0x82cadd1c
	ctx.lr = 0x82DF189C;
	sub_82CADCEC(ctx, base);
	// 82DF189C: 4BEB7B84  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            0x82DF18A0 => {
    //   block [0x82DF18A0..0x82DF1B94)
	// 82DF18A0: D3C10140  stfs f30, 0x140(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), tmp.u32 ) };
	// 82DF18A4: 13FF038C  vspltisw v31, -1
	for i in 0..4 {
		ctx.v[31].u32[i] = 4294967295;
	}
	// 82DF18A8: D3C10144  stfs f30, 0x144(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), tmp.u32 ) };
	// 82DF18AC: 38B70030  addi r5, r23, 0x30
	ctx.r[5].s64 = ctx.r[23].s64 + 48;
	// 82DF18B0: D3C10148  stfs f30, 0x148(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), tmp.u32 ) };
	// 82DF18B4: 5464063E  clrlwi r4, r3, 0x18
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82DF18B8: D3C1014C  stfs f30, 0x14c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 82DF18BC: 39410140  addi r10, r1, 0x140
	ctx.r[10].s64 = ctx.r[1].s64 + 320;
	// 82DF18C0: D3C10150  stfs f30, 0x150(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), tmp.u32 ) };
	// 82DF18C4: D3C10154  stfs f30, 0x154(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), tmp.u32 ) };
	// 82DF18C8: D3C10158  stfs f30, 0x158(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(344 as u32), tmp.u32 ) };
	// 82DF18CC: D3C1015C  stfs f30, 0x15c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF1B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF1B98 size=7136
    let mut pc: u32 = 0x82DF1B98;
    'dispatch: loop {
        match pc {
            0x82DF1B98 => {
    //   block [0x82DF1B98..0x82DF1CE4)
	// 82DF1B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF1B9C: 4BEB7835  bl 0x82ca93d0
	ctx.lr = 0x82DF1BA0;
	sub_82CA93D0(ctx, base);
	// 82DF1BA0: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82DF1BA4: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DF1BA8: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DF1BAC: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
	pc = 0x82DF1CE4; continue 'dispatch;
            }
            0x82DF1CE4 => {
    //   block [0x82DF1CE4..0x82DF1D84)
	// 82DF1CE4: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82DF1CE8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DF1CEC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF1CF0: 2B0B001C  cmplwi cr6, r11, 0x1c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 28 as u32, &mut ctx.xer);
	// 82DF1CF4: 41991A58  bgt cr6, 0x82df374c
	if ctx.cr[6].gt {
	pc = 0x82DF374C; continue 'dispatch;
	}
	// 82DF1CF8: 3D8082DF  lis r12, -0x7d21
	ctx.r[12].s64 = -2099314688;
	// 82DF1CFC: 398C1D10  addi r12, r12, 0x1d10
	ctx.r[12].s64 = ctx.r[12].s64 + 7440;
	// 82DF1D00: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DF1D04: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82DF1D08: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82DF1D0C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82DF3728; continue 'dispatch;
		},
		1 => {
	pc = 0x82DF26B8; continue 'dispatch;
		},
		2 => {
	pc = 0x82DF35A4; continue 'dispatch;
		},
		3 => {
	pc = 0x82DF1CE4; continue 'dispatch;
		},
		4 => {
	pc = 0x82DF374C; continue 'dispatch;
		},
		5 => {
	pc = 0x82DF2C8C; continue 'dispatch;
		},
		6 => {
	pc = 0x82DF2D4C; continue 'dispatch;
		},
		7 => {
	pc = 0x82DF3030; continue 'dispatch;
		},
		8 => {
	pc = 0x82DF2F34; continue 'dispatch;
		},
		9 => {
	pc = 0x82DF3190; continue 'dispatch;
		},
		10 => {
	pc = 0x82DF3400; continue 'dispatch;
		},
		11 => {
	pc = 0x82DF2C08; continue 'dispatch;
		},
		12 => {
	pc = 0x82DF2970; continue 'dispatch;
		},
		13 => {
	pc = 0x82DF28A4; continue 'dispatch;
		},
		14 => {
	pc = 0x82DF2A90; continue 'dispatch;
		},
		15 => {
	pc = 0x82DF2168; continue 'dispatch;
		},
		16 => {
	pc = 0x82DF2168; continue 'dispatch;
		},
		17 => {
	pc = 0x82DF2238; continue 'dispatch;
		},
		18 => {
	pc = 0x82DF2238; continue 'dispatch;
		},
		19 => {
	pc = 0x82DF1D84; continue 'dispatch;
		},
		20 => {
	pc = 0x82DF1D84; continue 'dispatch;
		},
		21 => {
	pc = 0x82DF26EC; continue 'dispatch;
		},
		22 => {
	pc = 0x82DF2468; continue 'dispatch;
		},
		23 => {
	pc = 0x82DF34AC; continue 'dispatch;
		},
		24 => {
	pc = 0x82DF34F4; continue 'dispatch;
		},
		25 => {
	pc = 0x82DF374C; continue 'dispatch;
		},
		26 => {
	pc = 0x82DF35B0; continue 'dispatch;
		},
		27 => {
	pc = 0x82DF3628; continue 'dispatch;
		},
		28 => {
	pc = 0x82DF36A8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82DF1D10: 82DF3728  lwz r22, 0x3728(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14120 as u32) ) } as u64;
	// 82DF1D14: 82DF26B8  lwz r22, 0x26b8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9912 as u32) ) } as u64;
	// 82DF1D18: 82DF35A4  lwz r22, 0x35a4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13732 as u32) ) } as u64;
	// 82DF1D1C: 82DF1CE4  lwz r22, 0x1ce4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7396 as u32) ) } as u64;
	// 82DF1D20: 82DF374C  lwz r22, 0x374c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14156 as u32) ) } as u64;
	// 82DF1D24: 82DF2C8C  lwz r22, 0x2c8c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11404 as u32) ) } as u64;
	// 82DF1D28: 82DF2D4C  lwz r22, 0x2d4c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11596 as u32) ) } as u64;
	// 82DF1D2C: 82DF3030  lwz r22, 0x3030(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12336 as u32) ) } as u64;
	// 82DF1D30: 82DF2F34  lwz r22, 0x2f34(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12084 as u32) ) } as u64;
	// 82DF1D34: 82DF3190  lwz r22, 0x3190(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12688 as u32) ) } as u64;
	// 82DF1D38: 82DF3400  lwz r22, 0x3400(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13312 as u32) ) } as u64;
	// 82DF1D3C: 82DF2C08  lwz r22, 0x2c08(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(11272 as u32) ) } as u64;
	// 82DF1D40: 82DF2970  lwz r22, 0x2970(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10608 as u32) ) } as u64;
	// 82DF1D44: 82DF28A4  lwz r22, 0x28a4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10404 as u32) ) } as u64;
	// 82DF1D48: 82DF2A90  lwz r22, 0x2a90(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10896 as u32) ) } as u64;
	// 82DF1D4C: 82DF2168  lwz r22, 0x2168(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8552 as u32) ) } as u64;
	// 82DF1D50: 82DF2168  lwz r22, 0x2168(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8552 as u32) ) } as u64;
	// 82DF1D54: 82DF2238  lwz r22, 0x2238(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8760 as u32) ) } as u64;
	// 82DF1D58: 82DF2238  lwz r22, 0x2238(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8760 as u32) ) } as u64;
	// 82DF1D5C: 82DF1D84  lwz r22, 0x1d84(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7556 as u32) ) } as u64;
	// 82DF1D60: 82DF1D84  lwz r22, 0x1d84(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7556 as u32) ) } as u64;
	// 82DF1D64: 82DF26EC  lwz r22, 0x26ec(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9964 as u32) ) } as u64;
	// 82DF1D68: 82DF2468  lwz r22, 0x2468(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(9320 as u32) ) } as u64;
	// 82DF1D6C: 82DF34AC  lwz r22, 0x34ac(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13484 as u32) ) } as u64;
	// 82DF1D70: 82DF34F4  lwz r22, 0x34f4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13556 as u32) ) } as u64;
	// 82DF1D74: 82DF374C  lwz r22, 0x374c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14156 as u32) ) } as u64;
	// 82DF1D78: 82DF35B0  lwz r22, 0x35b0(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13744 as u32) ) } as u64;
	// 82DF1D7C: 82DF3628  lwz r22, 0x3628(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13864 as u32) ) } as u64;
	// 82DF1D80: 82DF36A8  lwz r22, 0x36a8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(13992 as u32) ) } as u64;
            }
            0x82DF1D84 => {
    //   block [0x82DF1D84..0x82DF2168)
	pc = 0x82DF2168; continue 'dispatch;
            }
            0x82DF2168 => {
    //   block [0x82DF2168..0x82DF2238)
	pc = 0x82DF2238; continue 'dispatch;
            }
            0x82DF2238 => {
    //   block [0x82DF2238..0x82DF2468)
	pc = 0x82DF2468; continue 'dispatch;
            }
            0x82DF2468 => {
    //   block [0x82DF2468..0x82DF26B8)
	pc = 0x82DF26B8; continue 'dispatch;
            }
            0x82DF26B8 => {
    //   block [0x82DF26B8..0x82DF26EC)
	// 82DF26B8: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF26BC: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82DF26C0: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82DF26C4: 7C005A2C  dcbt 0, r11
	// 82DF26C8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DF26CC: 556B383E  rotlwi r11, r11, 7
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 82DF26D0: 7D6B9A14  add r11, r11, r19
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[19].u64;
	// 82DF26D4: 7C005A2C  dcbt 0, r11
	pc = 0x82DF26EC; continue 'dispatch;
            }
            0x82DF26EC => {
    //   block [0x82DF26EC..0x82DF28A4)
	pc = 0x82DF28A4; continue 'dispatch;
            }
            0x82DF28A4 => {
    //   block [0x82DF28A4..0x82DF2970)
	pc = 0x82DF2970; continue 'dispatch;
            }
            0x82DF2970 => {
    //   block [0x82DF2970..0x82DF2A90)
	// 82DF2970: 39540008  addi r10, r20, 8
	ctx.r[10].s64 = ctx.r[20].s64 + 8;
	// 82DF2974: 39370050  addi r9, r23, 0x50
	ctx.r[9].s64 = ctx.r[23].s64 + 80;
	// 82DF2978: 39160050  addi r8, r22, 0x50
	ctx.r[8].s64 = ctx.r[22].s64 + 80;
	// 82DF297C: 38F40040  addi r7, r20, 0x40
	ctx.r[7].s64 = ctx.r[20].s64 + 64;
	pc = 0x82DF2A90; continue 'dispatch;
            }
            0x82DF2A90 => {
    //   block [0x82DF2A90..0x82DF2C08)
	// 82DF2A90: 39370050  addi r9, r23, 0x50
	ctx.r[9].s64 = ctx.r[23].s64 + 80;
	// 82DF2A94: 39160050  addi r8, r22, 0x50
	ctx.r[8].s64 = ctx.r[22].s64 + 80;
	// 82DF2A98: 38F40040  addi r7, r20, 0x40
	ctx.r[7].s64 = ctx.r[20].s64 + 64;
	// 82DF2A9C: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82DF2AA0: 38C0FFEC  li r6, -0x14
	ctx.r[6].s64 = -20;
	// 82DF2AA4: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF2AA8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DF2AAC: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF2AB0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DF2AB4: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF2AB8: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	pc = 0x82DF2C08; continue 'dispatch;
            }
            0x82DF2C08 => {
    //   block [0x82DF2C08..0x82DF2C8C)
	pc = 0x82DF2C8C; continue 'dispatch;
            }
            0x82DF2C8C => {
    //   block [0x82DF2C8C..0x82DF2D4C)
	pc = 0x82DF2D4C; continue 'dispatch;
            }
            0x82DF2D4C => {
    //   block [0x82DF2D4C..0x82DF2F34)
	// 82DF2D4C: 38740040  addi r3, r20, 0x40
	ctx.r[3].s64 = ctx.r[20].s64 + 64;
	// 82DF2D50: 39760040  addi r11, r22, 0x40
	ctx.r[11].s64 = ctx.r[22].s64 + 64;
	// 82DF2D54: 39570040  addi r10, r23, 0x40
	ctx.r[10].s64 = ctx.r[23].s64 + 64;
	// 82DF2D58: 39360050  addi r9, r22, 0x50
	ctx.r[9].s64 = ctx.r[22].s64 + 80;
	// 82DF2D5C: 39170050  addi r8, r23, 0x50
	ctx.r[8].s64 = ctx.r[23].s64 + 80;
	pc = 0x82DF2F34; continue 'dispatch;
            }
            0x82DF2F34 => {
    //   block [0x82DF2F34..0x82DF3030)
	pc = 0x82DF3030; continue 'dispatch;
            }
            0x82DF3030 => {
    //   block [0x82DF3030..0x82DF3190)
	pc = 0x82DF3190; continue 'dispatch;
            }
            0x82DF3190 => {
    //   block [0x82DF3190..0x82DF3400)
	// 82DF3190: 3BB40040  addi r29, r20, 0x40
	ctx.r[29].s64 = ctx.r[20].s64 + 64;
	// 82DF3194: 38D60040  addi r6, r22, 0x40
	ctx.r[6].s64 = ctx.r[22].s64 + 64;
	// 82DF3198: 38B70040  addi r5, r23, 0x40
	ctx.r[5].s64 = ctx.r[23].s64 + 64;
	// 82DF319C: 38960050  addi r4, r22, 0x50
	ctx.r[4].s64 = ctx.r[22].s64 + 80;
	// 82DF31A0: 38770050  addi r3, r23, 0x50
	ctx.r[3].s64 = ctx.r[23].s64 + 80;
	// 82DF31A4: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 82DF31A8: 3A80FFDC  li r20, -0x24
	ctx.r[20].s64 = -36;
	// 82DF31AC: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF31B0: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF31B4: 39410180  addi r10, r1, 0x180
	ctx.r[10].s64 = ctx.r[1].s64 + 384;
	// 82DF31B8: EC006FBA  fmadds f0, f0, f30, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[30].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF31BC: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	pc = 0x82DF3400; continue 'dispatch;
            }
            0x82DF3400 => {
    //   block [0x82DF3400..0x82DF34AC)
	pc = 0x82DF34AC; continue 'dispatch;
            }
            0x82DF34AC => {
    //   block [0x82DF34AC..0x82DF34F4)
	// 82DF34AC: 39770030  addi r11, r23, 0x30
	ctx.r[11].s64 = ctx.r[23].s64 + 48;
	pc = 0x82DF34F4; continue 'dispatch;
            }
            0x82DF34F4 => {
    //   block [0x82DF34F4..0x82DF35A4)
	// 82DF34F4: 8141029C  lwz r10, 0x29c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(668 as u32) ) } as u64;
	pc = 0x82DF35A4; continue 'dispatch;
            }
            0x82DF35A4 => {
    //   block [0x82DF35A4..0x82DF35B0)
	// 82DF35A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF35A8: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF35AC: 4BFFE73C  b 0x82df1ce8
	pc = 0x82DF1CE8; continue 'dispatch;
            }
            0x82DF35B0 => {
    //   block [0x82DF35B0..0x82DF3628)
	// 82DF35B0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DF3628; continue 'dispatch;
            }
            0x82DF3628 => {
    //   block [0x82DF3628..0x82DF36A8)
	// 82DF3628: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DF36A8; continue 'dispatch;
            }
            0x82DF36A8 => {
    //   block [0x82DF36A8..0x82DF3728)
	// 82DF36A8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	pc = 0x82DF3728; continue 'dispatch;
            }
            0x82DF3728 => {
    //   block [0x82DF3728..0x82DF374C)
	pc = 0x82DF374C; continue 'dispatch;
            }
            0x82DF374C => {
    //   block [0x82DF374C..0x82DF3778)
	// 82DF374C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82DF3750: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF3754: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82DF3758: 3800FF30  li r0, -0xd0
	ctx.r[0].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF3778 size=116
    let mut pc: u32 = 0x82DF3778;
    'dispatch: loop {
        match pc {
            0x82DF3778 => {
    //   block [0x82DF3778..0x82DF37EC)
	// 82DF3778: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DF377C: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF3780: C1640004  lfs f11, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF3784: FD405850  fneg f10, f11
	ctx.f[10].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DF3788: ED610028  fsubs f11, f1, f0
	ctx.f[11].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DF378C: C1ABC234  lfs f13, -0x3dcc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15820 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF3790: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DF3794: FD806850  fneg f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82DF3798: C1ABC230  lfs f13, -0x3dd0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15824 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF379C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF37A0: EDAD00B2  fmuls f13, f13, f2
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[2].f64) as f32) as f64);
	// 82DF37A4: ED4A5828  fsubs f10, f10, f11
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[11].f64) as f32) as f64);
	// 82DF37A8: C12B0C4C  lfs f9, 0xc4c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3148 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DF37AC: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DF37B0: ED0C6828  fsubs f8, f12, f13
	ctx.f[8].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DF37B4: FD88636E  fsel f12, f8, f13, f12
	ctx.f[12].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 82DF37B8: EDAC6A7A  fmadds f13, f12, f9, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[9].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF37BC: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DF37C0: EC2B6028  fsubs f1, f11, f12
	ctx.f[1].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DF37C4: FF0A6800  fcmpu cr6, f10, f13
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[13].f64);
	// 82DF37C8: 4099000C  ble cr6, 0x82df37d4
	if !ctx.cr[6].gt {
	pc = 0x82DF37D4; continue 'dispatch;
	}
	// 82DF37CC: EC005028  fsubs f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82DF37D0: EC2A082A  fadds f1, f10, f1
	ctx.f[1].f64 = ((ctx.f[10].f64 + ctx.f[1].f64) as f32) as f64;
	// 82DF37D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF37D8: C1AB0A5C  lfs f13, 0xa5c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2652 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF37DC: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DF37E0: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82DF37E4: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DF37E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF37F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF37F0 size=24
    let mut pc: u32 = 0x82DF37F0;
    'dispatch: loop {
        match pc {
            0x82DF37F0 => {
    //   block [0x82DF37F0..0x82DF3808)
	// 82DF37F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF37F4: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF37F8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DF37FC: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DF3800: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DF3804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF3808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF3808 size=2892
    let mut pc: u32 = 0x82DF3808;
    'dispatch: loop {
        match pc {
            0x82DF3808 => {
    //   block [0x82DF3808..0x82DF4354)
	// 82DF3808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF380C: 4BEB5BC5  bl 0x82ca93d0
	ctx.lr = 0x82DF3810;
	sub_82CA93D0(ctx, base);
	// 82DF3810: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82DF3814: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DF3818: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DF381C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DF3820: A0E30004  lhz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF3824: 3961FF40  addi r11, r1, -0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + -192;
	// 82DF3828: 90C1002C  stw r6, 0x2c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82DF382C: 8923000A  lbz r9, 0xa(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 82DF3830: 7CA50774  extsb r5, r5
	ctx.r[5].s64 = ctx.r[5].s8 as i64;
	// 82DF3834: 83E40030  lwz r31, 0x30(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DF3838: 83C40034  lwz r30, 0x34(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DF383C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DF3840: C0480C18  lfs f2, 0xc18(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82DF3844: 39030010  addi r8, r3, 0x10
	ctx.r[8].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF4358 size=12
    let mut pc: u32 = 0x82DF4358;
    'dispatch: loop {
        match pc {
            0x82DF4358 => {
    //   block [0x82DF4358..0x82DF4364)
	// 82DF4358: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF435C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4360: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF4364 size=152
    let mut pc: u32 = 0x82DF4364;
    'dispatch: loop {
        match pc {
            0x82DF4364 => {
    //   block [0x82DF4364..0x82DF43FC)
	// 82DF4364: 39050090  addi r8, r5, 0x90
	ctx.r[8].s64 = ctx.r[5].s64 + 144;
	// 82DF4368: 39650070  addi r11, r5, 0x70
	ctx.r[11].s64 = ctx.r[5].s64 + 112;
	// 82DF436C: 39460080  addi r10, r6, 0x80
	ctx.r[10].s64 = ctx.r[6].s64 + 128;
	// 82DF4370: 7CA53050  subf r5, r5, r6
	ctx.r[5].s64 = ctx.r[6].s64 - ctx.r[5].s64;
	// 82DF4374: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82DF4378: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DF437C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF4380: C1A60C14  lfs f13, 0xc14(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF4384: C0070C18  lfs f0, 0xc18(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF4388: 88C30000  lbz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF438C: 5527063E  clrlwi r7, r9, 0x18
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82DF4390: 7CC73C30  srw r7, r6, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[6].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF4394: 54E707BE  clrlwi r7, r7, 0x1e
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	// 82DF4398: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DF439C: 419A0044  beq cr6, 0x82df43e0
	if ctx.cr[6].eq {
	pc = 0x82DF43E0; continue 'dispatch;
	}
	// 82DF43A0: 7C870774  extsb r7, r4
	ctx.r[7].s64 = ctx.r[4].s8 as i64;
	// 82DF43A4: D00BFFF0  stfs f0, -0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82DF43A8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF43AC: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DF43B0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF43B4: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DF43B8: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DF43BC: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82DF43C0: D1A80000  stfs f13, 0(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF43C4: 419A001C  beq cr6, 0x82df43e0
	if ctx.cr[6].eq {
	pc = 0x82DF43E0; continue 'dispatch;
	}
	// 82DF43C8: D00AFFE0  stfs f0, -0x20(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82DF43CC: 7C055D2E  stfsx f0, r5, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82DF43D0: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF43D4: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DF43D8: D00A0020  stfs f0, 0x20(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82DF43DC: D00A0030  stfs f0, 0x30(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82DF43E0: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82DF43E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF43E8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DF43EC: 39080014  addi r8, r8, 0x14
	ctx.r[8].s64 = ctx.r[8].s64 + 20;
	// 82DF43F0: 2F090006  cmpwi cr6, r9, 6
	ctx.cr[6].compare_i32(ctx.r[9].s32, 6, &mut ctx.xer);
	// 82DF43F4: 4198FF94  blt cr6, 0x82df4388
	if ctx.cr[6].lt {
	pc = 0x82DF4388; continue 'dispatch;
	}
	// 82DF43F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF4400 size=12
    let mut pc: u32 = 0x82DF4400;
    'dispatch: loop {
        match pc {
            0x82DF4400 => {
    //   block [0x82DF4400..0x82DF440C)
	// 82DF4400: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4408: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF440C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF440C size=80
    let mut pc: u32 = 0x82DF440C;
    'dispatch: loop {
        match pc {
            0x82DF440C => {
    //   block [0x82DF440C..0x82DF445C)
	// 82DF440C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF4410: 39640090  addi r11, r4, 0x90
	ctx.r[11].s64 = ctx.r[4].s64 + 144;
	// 82DF4414: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4418: 5549063E  clrlwi r9, r10, 0x18
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82DF441C: 7D094C30  srw r9, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[8].u32) >> ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF4420: 552907BE  clrlwi r9, r9, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 82DF4424: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF4428: 419A0020  beq cr6, 0x82df4448
	if ctx.cr[6].eq {
	pc = 0x82DF4448; continue 'dispatch;
	}
	// 82DF442C: 392BFFA0  addi r9, r11, -0x60
	ctx.r[9].s64 = ctx.r[11].s64 + -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF4460 size=76
    let mut pc: u32 = 0x82DF4460;
    'dispatch: loop {
        match pc {
            0x82DF4460 => {
    //   block [0x82DF4460..0x82DF44AC)
	// 82DF4460: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF4464: 546A2834  slwi r10, r3, 5
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF4468: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DF446C: 7D2A3214  add r9, r10, r6
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 82DF4470: 54880E3C  rlwinm r8, r4, 1, 0x18, 0x1e
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82DF4474: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4478: 7D4A4430  srw r10, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF447C: 554A07BE  clrlwi r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82DF4480: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DF4484: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF4488: 7D445214  add r10, r4, r10
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82DF448C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF4490: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DF4494: 409A0018  bne cr6, 0x82df44ac
	if !ctx.cr[6].eq {
		sub_82DF44AC(ctx, base);
		return;
	}
	// 82DF4498: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF449C: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82DF44A0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF44A4: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82DF44A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF44AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF44AC size=20
    let mut pc: u32 = 0x82DF44AC;
    'dispatch: loop {
        match pc {
            0x82DF44AC => {
    //   block [0x82DF44AC..0x82DF44C0)
	// 82DF44AC: C00B0008  lfs f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF44B0: 39640004  addi r11, r4, 4
	ctx.r[11].s64 = ctx.r[4].s64 + 4;
	// 82DF44B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF44B8: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82DF44BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF44C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF44C0 size=1048
    let mut pc: u32 = 0x82DF44C0;
    'dispatch: loop {
        match pc {
            0x82DF44C0 => {
    //   block [0x82DF44C0..0x82DF48D8)
	// 82DF44C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF44C4: 4BEB4F31  bl 0x82ca93f4
	ctx.lr = 0x82DF44C8;
	sub_82CA93D0(ctx, base);
	// 82DF44C8: 9421FC50  stwu r1, -0x3b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-944 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF44CC: 39640300  addi r11, r4, 0x300
	ctx.r[11].s64 = ctx.r[4].s64 + 768;
	// 82DF44D0: 908103CC  stw r4, 0x3cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(972 as u32), ctx.r[4].u32 ) };
	// 82DF44D4: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82DF44D8: 90E103E4  stw r7, 0x3e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(996 as u32), ctx.r[7].u32 ) };
	// 82DF44DC: 3BC00020  li r30, 0x20
	ctx.r[30].s64 = 32;
	// 82DF44E0: 90C103DC  stw r6, 0x3dc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(988 as u32), ctx.r[6].u32 ) };
	// 82DF44E4: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82DF44E8: 3B000040  li r24, 0x40
	ctx.r[24].s64 = 64;
	// 82DF44EC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DF44F0: 39640240  addi r11, r4, 0x240
	ctx.r[11].s64 = ctx.r[4].s64 + 576;
	// 82DF44F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DF44F8: 914103D4  stw r10, 0x3d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(980 as u32), ctx.r[10].u32 ) };
	// 82DF44FC: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82DF4500: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DF4504: 3B200050  li r25, 0x50
	ctx.r[25].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF48D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF48D8 size=732
    let mut pc: u32 = 0x82DF48D8;
    'dispatch: loop {
        match pc {
            0x82DF48D8 => {
    //   block [0x82DF48D8..0x82DF4BB4)
	// 82DF48D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF48DC: 4BEB4AF5  bl 0x82ca93d0
	ctx.lr = 0x82DF48E0;
	sub_82CA93D0(ctx, base);
	// 82DF48E0: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82DF48E4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF48E8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF48EC: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF48F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF48F4: 9281FF0C  stw r20, -0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-244 as u32), ctx.r[20].u32 ) };
	// 82DF48F8: 409902B8  ble cr6, 0x82df4bb0
	if !ctx.cr[6].gt {
	pc = 0x82DF4BB0; continue 'dispatch;
	}
	// 82DF48FC: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82DF4900: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DF4904: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DF4908: 396B4C40  addi r11, r11, 0x4c40
	ctx.r[11].s64 = ctx.r[11].s64 + 19520;
	// 82DF490C: 3A630018  addi r19, r3, 0x18
	ctx.r[19].s64 = ctx.r[3].s64 + 24;
	// 82DF4910: 3A43001C  addi r18, r3, 0x1c
	ctx.r[18].s64 = ctx.r[3].s64 + 28;
	// 82DF4914: 3A230024  addi r17, r3, 0x24
	ctx.r[17].s64 = ctx.r[3].s64 + 36;
	// 82DF4918: C1AA0C18  lfs f13, 0xc18(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF491C: 7E95A378  mr r21, r20
	ctx.r[21].u64 = ctx.r[20].u64;
	// 82DF4920: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82DF4924: 9161FF10  stw r11, -0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-240 as u32), ctx.r[11].u32 ) };
	// 82DF4928: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82DF492C: 3AC00020  li r22, 0x20
	ctx.r[22].s64 = 32;
	// 82DF4930: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82DF4934: 81730000  lwz r11, 0(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4938: 3B81FF20  addi r28, r1, -0xe0
	ctx.r[28].s64 = ctx.r[1].s64 + -224;
	// 82DF493C: 80F10000  lwz r7, 0(r17)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4940: 3B41FF40  addi r26, r1, -0xc0
	ctx.r[26].s64 = ctx.r[1].s64 + -192;
	// 82DF4944: 7D755A14  add r11, r21, r11
	ctx.r[11].u64 = ctx.r[21].u64 + ctx.r[11].u64;
	// 82DF4948: 81520000  lwz r10, 0(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF494C: 7CE7A214  add r7, r7, r20
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[20].u64;
	// 82DF4950: 3B21FF50  addi r25, r1, -0xb0
	ctx.r[25].s64 = ctx.r[1].s64 + -176;
	// 82DF4954: 38870010  addi r4, r7, 0x10
	ctx.r[4].s64 = ctx.r[7].s64 + 16;
	// 82DF4958: 3B61FF30  addi r27, r1, -0xd0
	ctx.r[27].s64 = ctx.r[1].s64 + -208;
	// 82DF495C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4960: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82DF4964: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4968: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82DF496C: 7D6A2A14  add r11, r10, r5
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82DF4970: 7D465214  add r10, r6, r10
	ctx.r[10].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 82DF4974: 38EB0040  addi r7, r11, 0x40
	ctx.r[7].s64 = ctx.r[11].s64 + 64;
	// 82DF4978: 38AB0050  addi r5, r11, 0x50
	ctx.r[5].s64 = ctx.r[11].s64 + 80;
	// 82DF497C: 38CA0040  addi r6, r10, 0x40
	ctx.r[6].s64 = ctx.r[10].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF4BB8 size=296
    let mut pc: u32 = 0x82DF4BB8;
    'dispatch: loop {
        match pc {
            0x82DF4BB8 => {
    //   block [0x82DF4BB8..0x82DF4CE0)
	// 82DF4BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4BBC: 4BEB483D  bl 0x82ca93f8
	ctx.lr = 0x82DF4BC0;
	sub_82CA93D0(ctx, base);
	// 82DF4BC0: 81050024  lwz r8, 0x24(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF4BC4: 54693032  slwi r9, r3, 6
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF4BC8: 80E50018  lwz r7, 0x18(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF4BCC: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF4BD0: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DF4BD4: 8145001C  lwz r10, 0x1c(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF4BD8: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82DF4BDC: 83C50014  lwz r30, 0x14(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF4BE0: 5469083C  slwi r9, r3, 1
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF4BE4: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 82DF4BE8: 7FE34A14  add r31, r3, r9
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[9].u64;
	// 82DF4BEC: 5489103A  slwi r9, r4, 2
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF4BF0: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4BF4: 7FFF2214  add r31, r31, r4
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 82DF4BF8: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4BFC: 7D644A14  add r11, r4, r9
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[9].u64;
	// 82DF4C00: 57E92834  slwi r9, r31, 5
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF4C04: 557F103A  slwi r31, r11, 2
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DF4C08: 7D7C5214  add r11, r28, r10
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82DF4C0C: 7D5D5214  add r10, r29, r10
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[10].u64;
	// 82DF4C10: 3B600050  li r27, 0x50
	ctx.r[27].s64 = 80;
	// 82DF4C14: 7D1F4214  add r8, r31, r8
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[8].u64;
	// 82DF4C18: 3B01FFB0  addi r24, r1, -0x50
	ctx.r[24].s64 = ctx.r[1].s64 + -80;
	// 82DF4C1C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4CE0 size=216
    let mut pc: u32 = 0x82DF4CE0;
    'dispatch: loop {
        match pc {
            0x82DF4CE0 => {
    //   block [0x82DF4CE0..0x82DF4DB8)
	// 82DF4CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4CE4: 4BEB4719  bl 0x82ca93fc
	ctx.lr = 0x82DF4CE8;
	sub_82CA93D0(ctx, base);
	// 82DF4CE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4CEC: 3BA40020  addi r29, r4, 0x20
	ctx.r[29].s64 = ctx.r[4].s64 + 32;
	// 82DF4CF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4CF4: 397DFFE0  addi r11, r29, -0x20
	ctx.r[11].s64 = ctx.r[29].s64 + -32;
	// 82DF4CF8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DF4CFC: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF4DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF4DB8 size=600
    let mut pc: u32 = 0x82DF4DB8;
    'dispatch: loop {
        match pc {
            0x82DF4DB8 => {
    //   block [0x82DF4DB8..0x82DF5010)
	// 82DF4DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF4DBC: 4BEB4631  bl 0x82ca93ec
	ctx.lr = 0x82DF4DC0;
	sub_82CA93D0(ctx, base);
	// 82DF4DC0: DBC1FF90  stfd f30, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[30].u64 ) };
	// 82DF4DC4: DBE1FF98  stfd f31, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82DF4DC8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF4DCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF4DD0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DF4DD4: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82DF4DD8: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 82DF4DDC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DF4DE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF4DE4: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5010 size=1396
    let mut pc: u32 = 0x82DF5010;
    'dispatch: loop {
        match pc {
            0x82DF5010 => {
    //   block [0x82DF5010..0x82DF5584)
	// 82DF5010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5014: 4BEB43BD  bl 0x82ca93d0
	ctx.lr = 0x82DF5018;
	sub_82CA93D0(ctx, base);
	// 82DF5018: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82DF501C: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82DF5020: 3981FF50  addi r12, r1, -0xb0
	ctx.r[12].s64 = ctx.r[1].s64 + -176;
	// 82DF5024: 482119B1  bl 0x830069d4
	ctx.lr = 0x82DF5028;
	sub_83006760(ctx, base);
	// 82DF5028: 9421FD00  stwu r1, -0x300(r1)
	ea = ctx.r[1].u32.wrapping_add(-768 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF502C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF5030: 9081031C  stw r4, 0x31c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(796 as u32), ctx.r[4].u32 ) };
	// 82DF5034: 394100D0  addi r10, r1, 0xd0
	ctx.r[10].s64 = ctx.r[1].s64 + 208;
	// 82DF5038: 90A10324  stw r5, 0x324(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(804 as u32), ctx.r[5].u32 ) };
	// 82DF503C: 392100E0  addi r9, r1, 0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	// 82DF5040: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DF5044: 386101F0  addi r3, r1, 0x1f0
	ctx.r[3].s64 = ctx.r[1].s64 + 496;
	// 82DF5048: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5588 size=1460
    let mut pc: u32 = 0x82DF5588;
    'dispatch: loop {
        match pc {
            0x82DF5588 => {
    //   block [0x82DF5588..0x82DF5B3C)
	// 82DF5588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF558C: 4BEB3E49  bl 0x82ca93d4
	ctx.lr = 0x82DF5590;
	sub_82CA93D0(ctx, base);
	// 82DF5590: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5594: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82DF5598: 90A10214  stw r5, 0x214(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(532 as u32), ctx.r[5].u32 ) };
	// 82DF559C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82DF55A0: 90610204  stw r3, 0x204(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(516 as u32), ctx.r[3].u32 ) };
	// 82DF55A4: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82DF55A8: 90C1021C  stw r6, 0x21c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(540 as u32), ctx.r[6].u32 ) };
	// 82DF55AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DF55B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82DF55B4: 9381023C  stw r28, 0x23c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(572 as u32), ctx.r[28].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF5B40 size=480
    let mut pc: u32 = 0x82DF5B40;
    'dispatch: loop {
        match pc {
            0x82DF5B40 => {
    //   block [0x82DF5B40..0x82DF5D20)
	// 82DF5B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5B44: 4BEB38C1  bl 0x82ca9404
	ctx.lr = 0x82DF5B48;
	sub_82CA93D0(ctx, base);
	// 82DF5B48: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5B4C: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 82DF5B50: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DF5B54: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF5B58: 4098000C  bge cr6, 0x82df5b64
	if !ctx.cr[6].lt {
	pc = 0x82DF5B64; continue 'dispatch;
	}
	// 82DF5B5C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82DF5B60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF5B64: 419A00C0  beq cr6, 0x82df5c24
	if ctx.cr[6].eq {
	pc = 0x82DF5C24; continue 'dispatch;
	}
	// 82DF5B68: 1D6A03C0  mulli r11, r10, 0x3c0
	ctx.r[11].s64 = ctx.r[10].s64 * 960;
	// 82DF5B6C: 7D2B3214  add r9, r11, r6
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82DF5B70: 3BE00010  li r31, 0x10
	ctx.r[31].s64 = 16;
	// 82DF5B74: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82DF5B78: 3969FF40  addi r11, r9, -0xc0
	ctx.r[11].s64 = ctx.r[9].s64 + -192;
	// 82DF5B7C: 3929FDC0  addi r9, r9, -0x240
	ctx.r[9].s64 = ctx.r[9].s64 + -576;
	// 82DF5B80: 390B0060  addi r8, r11, 0x60
	ctx.r[8].s64 = ctx.r[11].s64 + 96;
	// 82DF5B84: 38EB0090  addi r7, r11, 0x90
	ctx.r[7].s64 = ctx.r[11].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF5D20 size=336
    let mut pc: u32 = 0x82DF5D20;
    'dispatch: loop {
        match pc {
            0x82DF5D20 => {
    //   block [0x82DF5D20..0x82DF5E70)
	// 82DF5D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5D24: 4BEB36D9  bl 0x82ca93fc
	ctx.lr = 0x82DF5D28;
	sub_82CA93D0(ctx, base);
	// 82DF5D28: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF5D2C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DF5D30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF5D34: 38E0001C  li r7, 0x1c
	ctx.r[7].s64 = 28;
	// 82DF5D38: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5D3C: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5D40: C01E0008  lfs f0, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF5D44: 815E002C  lwz r10, 0x2c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF5D48: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF5D4C: 57AB043E  clrlwi r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 82DF5D50: 833E0024  lwz r25, 0x24(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF5D54: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DF5D58: 391D0001  addi r8, r29, 1
	ctx.r[8].s64 = ctx.r[29].s64 + 1;
	// 82DF5D5C: 98FF0003  stb r7, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[7].u8 ) };
	// 82DF5D60: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82DF5D64: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DF5D68: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DF5D6C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF5D70: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF5D74: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DF5D78: 1D6A0534  mulli r11, r10, 0x534
	ctx.r[11].s64 = ctx.r[10].s64 * 1332;
	// 82DF5D7C: 38EB0053  addi r7, r11, 0x53
	ctx.r[7].s64 = ctx.r[11].s64 + 83;
	// 82DF5D80: 1D6A04F0  mulli r11, r10, 0x4f0
	ctx.r[11].s64 = ctx.r[10].s64 * 1264;
	// 82DF5D84: 54EA0036  rlwinm r10, r7, 0, 0, 0x1b
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF5D88: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF5D8C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82DF5D90: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DF5D94: 40990020  ble cr6, 0x82df5db4
	if !ctx.cr[6].gt {
	pc = 0x82DF5DB4; continue 'dispatch;
	}
	// 82DF5D98: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5D9C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DF5DA0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DF5DA4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF5DA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DF5DAC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF5DB0: 409AFFE8  bne cr6, 0x82df5d98
	if !ctx.cr[6].eq {
	pc = 0x82DF5D98; continue 'dispatch;
	}
	// 82DF5DB4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5DB8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DF5DBC: 1D6B04F4  mulli r11, r11, 0x4f4
	ctx.r[11].s64 = ctx.r[11].s64 * 1268;
	// 82DF5DC0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82DF5DC4: 3B6B0044  addi r27, r11, 0x44
	ctx.r[27].s64 = ctx.r[11].s64 + 68;
	// 82DF5DC8: 4099002C  ble cr6, 0x82df5df4
	if !ctx.cr[6].gt {
	pc = 0x82DF5DF4; continue 'dispatch;
	}
	// 82DF5DCC: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 82DF5DD0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82DF5DD4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DF5DD8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DF5DDC: 4BEB36A5  bl 0x82ca9480
	ctx.lr = 0x82DF5DE0;
	sub_82CA9480(ctx, base);
	// 82DF5DE0: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 82DF5DE4: 3B7B0040  addi r27, r27, 0x40
	ctx.r[27].s64 = ctx.r[27].s64 + 64;
	// 82DF5DE8: 3B390040  addi r25, r25, 0x40
	ctx.r[25].s64 = ctx.r[25].s64 + 64;
	// 82DF5DEC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82DF5DF0: 409AFFE0  bne cr6, 0x82df5dd0
	if !ctx.cr[6].eq {
	pc = 0x82DF5DD0; continue 'dispatch;
	}
	// 82DF5DF4: 1D1D03C0  mulli r8, r29, 0x3c0
	ctx.r[8].s64 = ctx.r[29].s64 * 960;
	// 82DF5DF8: C01E0028  lfs f0, 0x28(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF5DFC: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DF5E00: 815E002C  lwz r10, 0x2c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF5E04: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5E08: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 82DF5E0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82DF5E10: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DF5E14: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E18: 57A81838  slwi r8, r29, 3
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF5E1C: 813E0020  lwz r9, 0x20(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF5E20: 1D4B00F0  mulli r10, r11, 0xf0
	ctx.r[10].s64 = ctx.r[11].s64 * 240;
	// 82DF5E24: 80BE0024  lwz r5, 0x24(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF5E28: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E2C: 7CFD4214  add r7, r29, r8
	ctx.r[7].u64 = ctx.r[29].u64 + ctx.r[8].u64;
	// 82DF5E30: 1D0B04F0  mulli r8, r11, 0x4f0
	ctx.r[8].s64 = ctx.r[11].s64 * 1264;
	// 82DF5E34: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF5E38: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82DF5E3C: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82DF5E40: 7D08FA14  add r8, r8, r31
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82DF5E44: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DF5E48: 38EB0020  addi r7, r11, 0x20
	ctx.r[7].s64 = ctx.r[11].s64 + 32;
	// 82DF5E4C: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82DF5E50: 38C60020  addi r6, r6, 0x20
	ctx.r[6].s64 = ctx.r[6].s64 + 32;
	// 82DF5E54: 4BFFF735  bl 0x82df5588
	ctx.lr = 0x82DF5E58;
	sub_82DF5588(ctx, base);
	// 82DF5E58: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF5E5C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E60: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DF5E64: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF5E68: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF5E6C: 4BEB35E0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF5E70 size=8
    let mut pc: u32 = 0x82DF5E70;
    'dispatch: loop {
        match pc {
            0x82DF5E70 => {
    //   block [0x82DF5E70..0x82DF5E78)
	// 82DF5E70: D0240004  stfs f1, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF5E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF5E78 size=12
    let mut pc: u32 = 0x82DF5E78;
    'dispatch: loop {
        match pc {
            0x82DF5E78 => {
    //   block [0x82DF5E78..0x82DF5E84)
	// 82DF5E78: D0250000  stfs f1, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF5E7C: D0450004  stfs f2, 4(r5)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF5E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF5E88 size=20
    let mut pc: u32 = 0x82DF5E88;
    'dispatch: loop {
        match pc {
            0x82DF5E88 => {
    //   block [0x82DF5E88..0x82DF5E9C)
	// 82DF5E88: D0270000  stfs f1, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF5E8C: D0470004  stfs f2, 4(r7)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF5E90: D0670008  stfs f3, 8(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DF5E94: D087000C  stfs f4, 0xc(r7)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DF5E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF5EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF5EA0 size=3352
    let mut pc: u32 = 0x82DF5EA0;
    'dispatch: loop {
        match pc {
            0x82DF5EA0 => {
    //   block [0x82DF5EA0..0x82DF5FE4)
	// 82DF5EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF5EA4: 4BEB3531  bl 0x82ca93d4
	ctx.lr = 0x82DF5EA8;
	sub_82CA93D0(ctx, base);
	// 82DF5EA8: 81630114  lwz r11, 0x114(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 82DF5EAC: C003010C  lfs f0, 0x10c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF5EB0: C1A30050  lfs f13, 0x50(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF5EB4: 3903010C  addi r8, r3, 0x10c
	ctx.r[8].s64 = ctx.r[3].s64 + 268;
	// 82DF5EB8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DF5EBC: ECCD0032  fmuls f6, f13, f0
	ctx.f[6].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DF5EC0: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82DF5EC4: C1830058  lfs f12, 0x58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF5EC8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DF5ECC: 90C1002C  stw r6, 0x2c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82DF5ED0: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DF5ED4: ECEC0032  fmuls f7, f12, f0
	ctx.f[7].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DF5ED8: 9101FEA0  stw r8, -0x160(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-352 as u32), ctx.r[8].u32 ) };
	// 82DF5EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF5EE0: F961FEE0  std r11, -0x120(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-288 as u32), ctx.r[11].u64 ) };
	// 82DF5EE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82DF5EE8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF5EEC: FD200090  fmr f9, f0
	ctx.f[9].f64 = ctx.f[0].f64;
	// 82DF5EF0: 3A8B4C40  addi r20, r11, 0x4c40
	ctx.r[20].s64 = ctx.r[11].s64 + 19520;
	// 82DF5EF4: 93E10024  stw r31, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 82DF5EF8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DF5EFC: C1070A98  lfs f8, 0xa98(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2712 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DF5F00: 9101FE80  stw r8, -0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-384 as u32), ctx.r[8].u32 ) };
	// 82DF5F04: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DF5F08: 388BC238  addi r4, r11, -0x3dc8
	ctx.r[4].s64 = ctx.r[11].s64 + -15816;
	// 82DF5F0C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DF5F10: 3A000050  li r16, 0x50
	ctx.r[16].s64 = 80;
	// 82DF5F14: 3B2BC258  addi r25, r11, -0x3da8
	ctx.r[25].s64 = ctx.r[11].s64 + -15784;
	// 82DF5F18: C0A80BDC  lfs f5, 0xbdc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3036 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82DF5F1C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82DF5F20: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
	// 82DF5F24: 3A400010  li r18, 0x10
	ctx.r[18].s64 = 16;
	// 82DF5F28: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82DF5F2C: 9321FED8  stw r25, -0x128(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-296 as u32), ctx.r[25].u32 ) };
	// 82DF5F30: 3B40FFF4  li r26, -0xc
	ctx.r[26].s64 = -12;
	// 82DF5F34: C9A1FEE0  lfd f13, -0x120(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-288 as u32) ) };
	// 82DF5F38: 9081FEE0  stw r4, -0x120(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-288 as u32), ctx.r[4].u32 ) };
	// 82DF5F3C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DF5F40: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82DF5F44: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DF5F48: A10A0004  lhz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5F4C: A06A0006  lhz r3, 6(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DF5F50: 5507383E  rotlwi r7, r8, 7
	ctx.r[7].u64 = ((ctx.r[8].u32).rotate_left(7)) as u64;
	// 82DF5F54: 8B6A0001  lbz r27, 1(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF5F58: 5468383E  rotlwi r8, r3, 7
	ctx.r[8].u64 = ((ctx.r[3].u32).rotate_left(7)) as u64;
	// 82DF5F5C: 806A000C  lwz r3, 0xc(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DF5F60: 7FA73214  add r29, r7, r6
	ctx.r[29].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82DF5F64: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF5F68: 7FC83214  add r30, r8, r6
	ctx.r[30].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82DF5F6C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DF5F70: 9361FEE8  stw r27, -0x118(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-280 as u32), ctx.r[27].u32 ) };
	// 82DF5F74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF5F78: 9061FED4  stw r3, -0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-300 as u32), ctx.r[3].u32 ) };
	// 82DF5F7C: 93A1FEA8  stw r29, -0x158(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-344 as u32), ctx.r[29].u32 ) };
	// 82DF5F80: 93C1FEB0  stw r30, -0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-336 as u32), ctx.r[30].u32 ) };
	// 82DF5F84: 409A004C  bne cr6, 0x82df5fd0
	if !ctx.cr[6].eq {
	pc = 0x82DF5FD0; continue 'dispatch;
	}
	// 82DF5F88: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DF5F8C: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82DF5F90: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82DF5F94: 4099003C  ble cr6, 0x82df5fd0
	if !ctx.cr[6].gt {
	pc = 0x82DF5FD0; continue 'dispatch;
	}
	// 82DF5F98: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82DF5F9C: 2F070019  cmpwi cr6, r7, 0x19
	ctx.cr[6].compare_i32(ctx.r[7].s32, 25, &mut ctx.xer);
	// 82DF5FA0: 40980030  bge cr6, 0x82df5fd0
	if !ctx.cr[6].lt {
	pc = 0x82DF5FD0; continue 'dispatch;
	}
	// 82DF5FA4: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82DF5FA8: 7D030774  extsb r3, r8
	ctx.r[3].s64 = ctx.r[8].s8 as i64;
	// 82DF5FAC: 7D0720AE  lbzx r8, r7, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82DF5FB0: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DF5FB4: 7D03C8AE  lbzx r8, r3, r25
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DF5FB8: 5508103E  rotlwi r8, r8, 2
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(2)) as u64;
	// 82DF5FBC: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82DF5FC0: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DF5FC4: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82DF5FC8: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82DF5FCC: 4199FFCC  bgt cr6, 0x82df5f98
	if ctx.cr[6].gt {
	pc = 0x82DF5F98; continue 'dispatch;
	}
	// 82DF5FD0: 8101FE80  lwz r8, -0x180(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-384 as u32) ) } as u64;
	// 82DF5FD4: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82DF5FD8: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF5FDC: 7EE8FA14  add r23, r8, r31
	ctx.r[23].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 82DF5FE0: 92E1FEB8  stw r23, -0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-328 as u32), ctx.r[23].u32 ) };
	pc = 0x82DF5FE4; continue 'dispatch;
            }
            0x82DF5FE4 => {
    //   block [0x82DF5FE4..0x82DF6080)
	// 82DF5FE4: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DF5FE8: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82DF5FEC: 2B07001C  cmplwi cr6, r7, 0x1c
	ctx.cr[6].compare_u32(ctx.r[7].u32, 28 as u32, &mut ctx.xer);
	// 82DF5FF0: 4199FFF4  bgt cr6, 0x82df5fe4
	if ctx.cr[6].gt {
	pc = 0x82DF5FE4; continue 'dispatch;
	}
	// 82DF5FF4: 3D8082DF  lis r12, -0x7d21
	ctx.r[12].s64 = -2099314688;
	// 82DF5FF8: 398C600C  addi r12, r12, 0x600c
	ctx.r[12].s64 = ctx.r[12].s64 + 24588;
	// 82DF5FFC: 54E0103A  slwi r0, r7, 2
	ctx.r[0].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DF6000: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82DF6004: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82DF6008: 4E800420  bctr
	match ctx.r[7].u64 {
		0 => {
	pc = 0x82DF6BA8; continue 'dispatch;
		},
		1 => {
	pc = 0x82DF6B9C; continue 'dispatch;
		},
		2 => {
	pc = 0x82DF68D4; continue 'dispatch;
		},
		3 => {
	pc = 0x82DF6080; continue 'dispatch;
		},
		4 => {
	pc = 0x82DF5FE4; continue 'dispatch;
		},
		5 => {
	pc = 0x82DF61A4; continue 'dispatch;
		},
		6 => {
	pc = 0x82DF6198; continue 'dispatch;
		},
		7 => {
	pc = 0x82DF6198; continue 'dispatch;
		},
		8 => {
	pc = 0x82DF6238; continue 'dispatch;
		},
		9 => {
	pc = 0x82DF6210; continue 'dispatch;
		},
		10 => {
	pc = 0x82DF63B8; continue 'dispatch;
		},
		11 => {
	pc = 0x82DF62BC; continue 'dispatch;
		},
		12 => {
	pc = 0x82DF62C8; continue 'dispatch;
		},
		13 => {
	pc = 0x82DF6348; continue 'dispatch;
		},
		14 => {
	pc = 0x82DF6320; continue 'dispatch;
		},
		15 => {
	pc = 0x82DF61A4; continue 'dispatch;
		},
		16 => {
	pc = 0x82DF61A4; continue 'dispatch;
		},
		17 => {
	pc = 0x82DF614C; continue 'dispatch;
		},
		18 => {
	pc = 0x82DF614C; continue 'dispatch;
		},
		19 => {
	pc = 0x82DF6098; continue 'dispatch;
		},
		20 => {
	pc = 0x82DF6098; continue 'dispatch;
		},
		21 => {
	pc = 0x82DF6430; continue 'dispatch;
		},
		22 => {
	pc = 0x82DF64F0; continue 'dispatch;
		},
		23 => {
	pc = 0x82DF671C; continue 'dispatch;
		},
		24 => {
	pc = 0x82DF65F4; continue 'dispatch;
		},
		25 => {
	pc = 0x82DF5FE4; continue 'dispatch;
		},
		26 => {
	pc = 0x82DF6724; continue 'dispatch;
		},
		27 => {
	pc = 0x82DF67F8; continue 'dispatch;
		},
		28 => {
	pc = 0x82DF68E0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82DF600C: 82DF6BA8  lwz r22, 0x6ba8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27560 as u32) ) } as u64;
	// 82DF6010: 82DF6B9C  lwz r22, 0x6b9c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27548 as u32) ) } as u64;
	// 82DF6014: 82DF68D4  lwz r22, 0x68d4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26836 as u32) ) } as u64;
	// 82DF6018: 82DF6080  lwz r22, 0x6080(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24704 as u32) ) } as u64;
	// 82DF601C: 82DF5FE4  lwz r22, 0x5fe4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24548 as u32) ) } as u64;
	// 82DF6020: 82DF61A4  lwz r22, 0x61a4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24996 as u32) ) } as u64;
	// 82DF6024: 82DF6198  lwz r22, 0x6198(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24984 as u32) ) } as u64;
	// 82DF6028: 82DF6198  lwz r22, 0x6198(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24984 as u32) ) } as u64;
	// 82DF602C: 82DF6238  lwz r22, 0x6238(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25144 as u32) ) } as u64;
	// 82DF6030: 82DF6210  lwz r22, 0x6210(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25104 as u32) ) } as u64;
	// 82DF6034: 82DF63B8  lwz r22, 0x63b8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25528 as u32) ) } as u64;
	// 82DF6038: 82DF62BC  lwz r22, 0x62bc(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25276 as u32) ) } as u64;
	// 82DF603C: 82DF62C8  lwz r22, 0x62c8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25288 as u32) ) } as u64;
	// 82DF6040: 82DF6348  lwz r22, 0x6348(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25416 as u32) ) } as u64;
	// 82DF6044: 82DF6320  lwz r22, 0x6320(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25376 as u32) ) } as u64;
	// 82DF6048: 82DF61A4  lwz r22, 0x61a4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24996 as u32) ) } as u64;
	// 82DF604C: 82DF61A4  lwz r22, 0x61a4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24996 as u32) ) } as u64;
	// 82DF6050: 82DF614C  lwz r22, 0x614c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24908 as u32) ) } as u64;
	// 82DF6054: 82DF614C  lwz r22, 0x614c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24908 as u32) ) } as u64;
	// 82DF6058: 82DF6098  lwz r22, 0x6098(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24728 as u32) ) } as u64;
	// 82DF605C: 82DF6098  lwz r22, 0x6098(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24728 as u32) ) } as u64;
	// 82DF6060: 82DF6430  lwz r22, 0x6430(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25648 as u32) ) } as u64;
	// 82DF6064: 82DF64F0  lwz r22, 0x64f0(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(25840 as u32) ) } as u64;
	// 82DF6068: 82DF671C  lwz r22, 0x671c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26396 as u32) ) } as u64;
	// 82DF606C: 82DF65F4  lwz r22, 0x65f4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26100 as u32) ) } as u64;
	// 82DF6070: 82DF5FE4  lwz r22, 0x5fe4(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24548 as u32) ) } as u64;
	// 82DF6074: 82DF6724  lwz r22, 0x6724(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26404 as u32) ) } as u64;
	// 82DF6078: 82DF67F8  lwz r22, 0x67f8(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26616 as u32) ) } as u64;
	// 82DF607C: 82DF68E0  lwz r22, 0x68e0(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26848 as u32) ) } as u64;
            }
            0x82DF6080 => {
    //   block [0x82DF6080..0x82DF6098)
	// 82DF6080: 88EA0004  lbz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6084: 7F680734  extsh r8, r27
	ctx.r[8].s64 = ctx.r[27].s16 as i64;
	// 82DF6088: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DF608C: 7D0741D6  mullw r8, r7, r8
	ctx.r[8].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82DF6090: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DF6094: 4BFFFF50  b 0x82df5fe4
	pc = 0x82DF5FE4; continue 'dispatch;
            }
            0x82DF6098 => {
    //   block [0x82DF6098..0x82DF614C)
	// 82DF6098: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x82DF614C; continue 'dispatch;
            }
            0x82DF614C => {
    //   block [0x82DF614C..0x82DF6198)
	// 82DF614C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF6150: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF6154: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 82DF6158: C188002C  lfs f12, 0x2c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF615C: ED8C0172  fmuls f12, f12, f5
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[5].f64) as f32) as f64);
	// 82DF6160: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82DF6164: 40990048  ble cr6, 0x82df61ac
	if !ctx.cr[6].gt {
	pc = 0x82DF61AC; continue 'dispatch;
	}
	// 82DF6168: 8081FE80  lwz r4, -0x180(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-384 as u32) ) } as u64;
	// 82DF616C: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82DF6170: 3AF70010  addi r23, r23, 0x10
	ctx.r[23].s64 = ctx.r[23].s64 + 16;
	// 82DF6174: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82DF6178: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DF617C: 92E1FEB8  stw r23, -0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-328 as u32), ctx.r[23].u32 ) };
	// 82DF6180: 9081FE80  stw r4, -0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-384 as u32), ctx.r[4].u32 ) };
	// 82DF6184: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF6188: 90870000  stw r4, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DF618C: 8081FED4  lwz r4, -0x12c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-300 as u32) ) } as u64;
	// 82DF6190: 90870004  stw r4, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DF6194: 48000018  b 0x82df61ac
	pc = 0x82DF61AC; continue 'dispatch;
            }
            0x82DF6198 => {
    //   block [0x82DF6198..0x82DF61A4)
	// 82DF6198: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF619C: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 82DF61A0: 4800000C  b 0x82df61ac
	pc = 0x82DF61AC; continue 'dispatch;
            }
            0x82DF61A4 => {
    //   block [0x82DF61A4..0x82DF6210)
	// 82DF61A4: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF61A8: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	pc = 0x82DF6210; continue 'dispatch;
            }
            0x82DF6210 => {
    //   block [0x82DF6210..0x82DF6238)
	// 82DF6210: C00A000C  lfs f0, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF6214: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF6218: C1690008  lfs f11, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF621C: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF6220: EC0B002A  fadds f0, f11, f0
	ctx.f[0].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DF6224: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DF6228: 88EA0003  lbz r7, 3(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DF622C: 394A0050  addi r10, r10, 0x50
	ctx.r[10].s64 = ctx.r[10].s64 + 80;
	// 82DF6230: 7CE70774  extsb r7, r7
	ctx.r[7].s64 = ctx.r[7].s8 as i64;
	// 82DF6234: 48000014  b 0x82df6248
	pc = 0x82DF6248; continue 'dispatch;
            }
            0x82DF6238 => {
    //   block [0x82DF6238..0x82DF62BC)
	// 82DF6238: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82DF623C: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF6240: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF6244: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	pc = 0x82DF62BC; continue 'dispatch;
            }
            0x82DF62BC => {
    //   block [0x82DF62BC..0x82DF62C8)
	// 82DF62BC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF62C0: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DF62C4: 4800000C  b 0x82df62d0
	pc = 0x82DF62D0; continue 'dispatch;
            }
            0x82DF62C8 => {
    //   block [0x82DF62C8..0x82DF6320)
	// 82DF62C8: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF62CC: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	pc = 0x82DF6320; continue 'dispatch;
            }
            0x82DF6320 => {
    //   block [0x82DF6320..0x82DF6348)
	// 82DF6320: C00A001C  lfs f0, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF6324: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF6328: C1690008  lfs f11, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF632C: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF6330: EC0B002A  fadds f0, f11, f0
	ctx.f[0].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DF6334: D00A001C  stfs f0, 0x1c(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82DF6338: 88EA0003  lbz r7, 3(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DF633C: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 82DF6340: 7CE70774  extsb r7, r7
	ctx.r[7].s64 = ctx.r[7].s8 as i64;
	// 82DF6344: 48000014  b 0x82df6358
	pc = 0x82DF6358; continue 'dispatch;
            }
            0x82DF6348 => {
    //   block [0x82DF6348..0x82DF63B8)
	// 82DF6348: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 82DF634C: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF6350: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF6354: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	pc = 0x82DF63B8; continue 'dispatch;
            }
            0x82DF63B8 => {
    //   block [0x82DF63B8..0x82DF6430)
	// 82DF63B8: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x82DF6430; continue 'dispatch;
            }
            0x82DF6430 => {
    //   block [0x82DF6430..0x82DF64F0)
	// 82DF6430: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x82DF64F0; continue 'dispatch;
            }
            0x82DF64F0 => {
    //   block [0x82DF64F0..0x82DF65F4)
	// 82DF64F0: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x82DF65F4; continue 'dispatch;
            }
            0x82DF65F4 => {
    //   block [0x82DF65F4..0x82DF671C)
	// 82DF65F4: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82DF65F8: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DF65FC: 80E80010  lwz r7, 0x10(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DF6600: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF6604: 409A007C  bne cr6, 0x82df6680
	if !ctx.cr[6].eq {
	pc = 0x82DF6680; continue 'dispatch;
	}
	// 82DF6608: 88FD0000  lbz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF660C: 98E1FEF0  stb r7, -0x110(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-272 as u32), ctx.r[7].u8 ) };
	// 82DF6610: 88FD0001  lbz r7, 1(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF6614: 98E1FEF1  stb r7, -0x10f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-271 as u32), ctx.r[7].u8 ) };
	// 82DF6618: 80FD0004  lwz r7, 4(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF661C: 90E1FEF4  stw r7, -0x10c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-268 as u32), ctx.r[7].u32 ) };
	// 82DF6620: 38E1FF00  addi r7, r1, -0x100
	ctx.r[7].s64 = ctx.r[1].s64 + -256;
	pc = 0x82DF671C; continue 'dispatch;
            }
            0x82DF671C => {
    //   block [0x82DF671C..0x82DF6724)
	// 82DF671C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82DF6720: 4BFFF8C4  b 0x82df5fe4
	pc = 0x82DF5FE4; continue 'dispatch;
            }
            0x82DF6724 => {
    //   block [0x82DF6724..0x82DF67F8)
	// 82DF6724: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 82DF6728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF672C: 419A00B4  beq cr6, 0x82df67e0
	if ctx.cr[6].eq {
	pc = 0x82DF67E0; continue 'dispatch;
	}
	// 82DF6730: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6734: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF6738: 54E3303E  rotlwi r3, r7, 6
	ctx.r[3].u64 = ((ctx.r[7].u32).rotate_left(6)) as u64;
	// 82DF673C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82DF6740: 7CE35214  add r7, r3, r10
	ctx.r[7].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 82DF6744: 38E70014  addi r7, r7, 0x14
	ctx.r[7].s64 = ctx.r[7].s64 + 20;
	// 82DF6748: 409900A4  ble cr6, 0x82df67ec
	if !ctx.cr[6].gt {
	pc = 0x82DF67EC; continue 'dispatch;
	}
	// 82DF674C: 80670004  lwz r3, 4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x82DF67F8; continue 'dispatch;
            }
            0x82DF67F8 => {
    //   block [0x82DF67F8..0x82DF68D4)
	// 82DF67F8: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 82DF67FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6800: 419A00C0  beq cr6, 0x82df68c0
	if ctx.cr[6].eq {
	pc = 0x82DF68C0; continue 'dispatch;
	}
	// 82DF6804: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6808: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF680C: 1C670130  mulli r3, r7, 0x130
	ctx.r[3].s64 = ctx.r[7].s64 * 304;
	// 82DF6810: 54FF083E  rotlwi r31, r7, 1
	ctx.r[31].u64 = ((ctx.r[7].u32).rotate_left(1)) as u64;
	// 82DF6814: 7C635214  add r3, r3, r10
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 82DF6818: 7FE7FA15  add. r31, r7, r31
	ctx.r[31].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DF681C: 38E30020  addi r7, r3, 0x20
	ctx.r[7].s64 = ctx.r[3].s64 + 32;
	// 82DF6820: 408100B4  ble 0x82df68d4
	if !ctx.cr[0].gt {
	pc = 0x82DF68D4; continue 'dispatch;
	}
	// 82DF6824: 80670004  lwz r3, 4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x82DF68D4; continue 'dispatch;
            }
            0x82DF68D4 => {
    //   block [0x82DF68D4..0x82DF68E0)
	// 82DF68D4: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF68D8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82DF68DC: 4BFFF708  b 0x82df5fe4
	pc = 0x82DF5FE4; continue 'dispatch;
            }
            0x82DF68E0 => {
    //   block [0x82DF68E0..0x82DF6B9C)
	// 82DF68E0: A0EA0004  lhz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF68E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF68E8: 3A6A0004  addi r19, r10, 4
	ctx.r[19].s64 = ctx.r[10].s64 + 4;
	// 82DF68EC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DF68F0: 419A0038  beq cr6, 0x82df6928
	if ctx.cr[6].eq {
	pc = 0x82DF6928; continue 'dispatch;
	}
	// 82DF68F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DF68F8: A0930000  lhz r4, 0(r19)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF68FC: 806A0008  lwz r3, 8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF6900: 1C8404F4  mulli r4, r4, 0x4f4
	ctx.r[4].s64 = ctx.r[4].s64 * 1268;
	// 82DF6904: 7C843A14  add r4, r4, r7
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[7].u64;
	// 82DF6908: 38E70040  addi r7, r7, 0x40
	ctx.r[7].s64 = ctx.r[7].s64 + 64;
	// 82DF690C: 7C845214  add r4, r4, r10
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 82DF6910: 88840044  lbz r4, 0x44(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DF6914: 7C8341AE  stbx r4, r3, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[8].u32), ctx.r[4].u8) };
	// 82DF6918: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82DF691C: A0930000  lhz r4, 0(r19)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6920: 7F082000  cmpw cr6, r8, r4
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82DF6924: 4198FFD4  blt cr6, 0x82df68f8
	if ctx.cr[6].lt {
	pc = 0x82DF68F8; continue 'dispatch;
	}
	// 82DF6928: A1130000  lhz r8, 0(r19)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF692C: 388A0020  addi r4, r10, 0x20
	ctx.r[4].s64 = ctx.r[10].s64 + 32;
	// 82DF6930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF6934: 5507183E  rotlwi r7, r8, 3
	ctx.r[7].u64 = ((ctx.r[8].u32).rotate_left(3)) as u64;
	// 82DF6938: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82DF693C: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF6940: 7CE75214  add r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82DF6944: 38670020  addi r3, r7, 0x20
	ctx.r[3].s64 = ctx.r[7].s64 + 32;
	// 82DF6948: 419A0238  beq cr6, 0x82df6b80
	if ctx.cr[6].eq {
	pc = 0x82DF6B80; continue 'dispatch;
	}
	// 82DF694C: 5507043E  clrlwi r7, r8, 0x10
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82DF6950: 39E00000  li r15, 0
	ctx.r[15].s64 = 0;
	// 82DF6954: 1CE704F0  mulli r7, r7, 0x4f0
	ctx.r[7].s64 = ctx.r[7].s64 * 1264;
	// 82DF6958: 7CE75214  add r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82DF695C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DF6960: 38E70040  addi r7, r7, 0x40
	ctx.r[7].s64 = ctx.r[7].s64 + 64;
	// 82DF6964: 4099022C  ble cr6, 0x82df6b90
	if !ctx.cr[6].gt {
	pc = 0x82DF6B90; continue 'dispatch;
	}
	// 82DF6968: 7CF13B78  mr r17, r7
	ctx.r[17].u64 = ctx.r[7].u64;
	// 82DF696C: 5767083C  slwi r7, r27, 1
	ctx.r[7].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF6970: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 82DF6974: 7CFB3A14  add r7, r27, r7
	ctx.r[7].u64 = ctx.r[27].u64 + ctx.r[7].u64;
	// 82DF6978: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 82DF697C: 54E7083C  slwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF6980: 90E1FEDC  stw r7, -0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-292 as u32), ctx.r[7].u32 ) };
	// 82DF6984: 80F10004  lwz r7, 4(r17)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF6988: 3B2B0004  addi r25, r11, 4
	ctx.r[25].s64 = ctx.r[11].s64 + 4;
	// 82DF698C: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF6990: 55F83032  slwi r24, r15, 6
	ctx.r[24].u32 = ctx.r[15].u32.wrapping_shl(6);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 82DF6994: 7FC73214  add r30, r7, r6
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 82DF6998: 7C661A14  add r3, r6, r3
	ctx.r[3].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 82DF699C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82DF69A0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DF69A4: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 82DF69A8: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 82DF69AC: 3AE30040  addi r23, r3, 0x40
	ctx.r[23].s64 = ctx.r[3].s64 + 64;
	// 82DF69B0: 3ADE0040  addi r22, r30, 0x40
	ctx.r[22].s64 = ctx.r[30].s64 + 64;
	// 82DF69B4: 3B9E0050  addi r28, r30, 0x50
	ctx.r[28].s64 = ctx.r[30].s64 + 80;
	// 82DF69B8: 3B630050  addi r27, r3, 0x50
	ctx.r[27].s64 = ctx.r[3].s64 + 80;
	pc = 0x82DF6B9C; continue 'dispatch;
            }
            0x82DF6B9C => {
    //   block [0x82DF6B9C..0x82DF6BA8)
	// 82DF6B9C: 83E10024  lwz r31, 0x24(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF6BA0: 8081FEE0  lwz r4, -0x120(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-288 as u32) ) } as u64;
	// 82DF6BA4: 4BFFF3A4  b 0x82df5f48
	pc = 0x82DF5F48; continue 'dispatch;
            }
            0x82DF6BA8 => {
    //   block [0x82DF6BA8..0x82DF6BB8)
	// 82DF6BA8: 8161FE80  lwz r11, -0x180(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-384 as u32) ) } as u64;
	// 82DF6BAC: 81410024  lwz r10, 0x24(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF6BB0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DF6BB4: 4BEB2870  b 0x82ca9424
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF6BB8 size=4
    let mut pc: u32 = 0x82DF6BB8;
    'dispatch: loop {
        match pc {
            0x82DF6BB8 => {
    //   block [0x82DF6BB8..0x82DF6BBC)
	// 82DF6BB8: 4BFFF2E8  b 0x82df5ea0
	sub_82DF5EA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF6BC0 size=844
    let mut pc: u32 = 0x82DF6BC0;
    'dispatch: loop {
        match pc {
            0x82DF6BC0 => {
    //   block [0x82DF6BC0..0x82DF6F0C)
	// 82DF6BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF6BC4: 4BEB2825  bl 0x82ca93e8
	ctx.lr = 0x82DF6BC8;
	sub_82CA93D0(ctx, base);
	// 82DF6BC8: 3961FF50  addi r11, r1, -0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + -176;
	// 82DF6BCC: C1A40040  lfs f13, 0x40(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF6BD0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82DF6BD4: 39460110  addi r10, r6, 0x110
	ctx.r[10].s64 = ctx.r[6].s64 + 272;
	// 82DF6BD8: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82DF6BDC: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF6F10 size=304
    let mut pc: u32 = 0x82DF6F10;
    'dispatch: loop {
        match pc {
            0x82DF6F10 => {
    //   block [0x82DF6F10..0x82DF7040)
	// 82DF6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF6F14: 4BEB24F9  bl 0x82ca940c
	ctx.lr = 0x82DF6F18;
	sub_82CA93D0(ctx, base);
	// 82DF6F18: 392300E0  addi r9, r3, 0xe0
	ctx.r[9].s64 = ctx.r[3].s64 + 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF7040 size=180
    let mut pc: u32 = 0x82DF7040;
    'dispatch: loop {
        match pc {
            0x82DF7040 => {
    //   block [0x82DF7040..0x82DF70F4)
	// 82DF7040: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF70F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF70F8 size=756
    let mut pc: u32 = 0x82DF70F8;
    'dispatch: loop {
        match pc {
            0x82DF70F8 => {
    //   block [0x82DF70F8..0x82DF73EC)
	// 82DF70F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF70FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF7100: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DF7104: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF7108: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF710C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DF7110: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF7114: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DF7118: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DF711C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DF7120: 4BFFFF21  bl 0x82df7040
	ctx.lr = 0x82DF7124;
	sub_82DF7040(ctx, base);
	// 82DF7124: C01F001C  lfs f0, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7128: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF712C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DF7130: 40990020  ble cr6, 0x82df7150
	if !ctx.cr[6].gt {
	pc = 0x82DF7150; continue 'dispatch;
	}
	// 82DF7134: C1BF0020  lfs f13, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7138: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82DF713C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF7140: 41980010  blt cr6, 0x82df7150
	if ctx.cr[6].lt {
	pc = 0x82DF7150; continue 'dispatch;
	}
	// 82DF7144: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF7148: C02B0C18  lfs f1, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82DF714C: 48000288  b 0x82df73d4
	pc = 0x82DF73D4; continue 'dispatch;
	// 82DF7150: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DF7154: C1BF0014  lfs f13, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7158: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DF715C: C19F0020  lfs f12, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF7160: 39630140  addi r11, r3, 0x140
	ctx.r[11].s64 = ctx.r[3].s64 + 320;
	// 82DF7164: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82DF7168: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF73F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF73F0 size=4
    let mut pc: u32 = 0x82DF73F0;
    'dispatch: loop {
        match pc {
            0x82DF73F0 => {
    //   block [0x82DF73F0..0x82DF73F4)
	// 82DF73F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF73F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF73F8 size=20
    let mut pc: u32 = 0x82DF73F8;
    'dispatch: loop {
        match pc {
            0x82DF73F8 => {
    //   block [0x82DF73F8..0x82DF740C)
	// 82DF73F8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF73FC: 7C8A0774  extsb r10, r4
	ctx.r[10].s64 = ctx.r[4].s8 as i64;
	// 82DF7400: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF7404: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DF7408: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF740C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF740C size=32
    let mut pc: u32 = 0x82DF740C;
    'dispatch: loop {
        match pc {
            0x82DF740C => {
    //   block [0x82DF740C..0x82DF742C)
	// 82DF740C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF7410: 419A001C  beq cr6, 0x82df742c
	if ctx.cr[6].eq {
		sub_82DF742C(ctx, base);
		return;
	}
	// 82DF7414: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF7418: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF741C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF7420: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DF7424: 409AFFE8  bne cr6, 0x82df740c
	if !ctx.cr[6].eq {
	pc = 0x82DF740C; continue 'dispatch;
	}
	// 82DF7428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF742C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF742C size=8
    let mut pc: u32 = 0x82DF742C;
    'dispatch: loop {
        match pc {
            0x82DF742C => {
    //   block [0x82DF742C..0x82DF7434)
	// 82DF742C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF7430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7438 size=892
    let mut pc: u32 = 0x82DF7438;
    'dispatch: loop {
        match pc {
            0x82DF7438 => {
    //   block [0x82DF7438..0x82DF77B4)
	// 82DF7438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF743C: 4BEB1FA5  bl 0x82ca93e0
	ctx.lr = 0x82DF7440;
	sub_82CA93D0(ctx, base);
	// 82DF7440: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7444: 3F80820A  lis r28, -0x7df6
	ctx.r[28].s64 = -2113273856;
	// 82DF7448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF744C: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82DF7450: 3AEB44D4  addi r23, r11, 0x44d4
	ctx.r[23].s64 = ctx.r[11].s64 + 17620;
	// 82DF7454: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DF7458: 817CEF9C  lwz r11, -0x1064(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4196 as u32) ) } as u64;
	// 82DF745C: 7E749B78  mr r20, r19
	ctx.r[20].u64 = ctx.r[19].u64;
	// 82DF7460: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 82DF7464: 55720FFE  srwi r18, r11, 0x1f
	ctx.r[18].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[18].u64 = ctx.r[18].u32 as u64;
	// 82DF7468: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 82DF746C: 3B0BC278  addi r24, r11, -0x3d88
	ctx.r[24].s64 = ctx.r[11].s64 + -15752;
	// 82DF7470: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF7474: 3BCB4694  addi r30, r11, 0x4694
	ctx.r[30].s64 = ctx.r[11].s64 + 18068;
	// 82DF7478: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF747C: 3AAB468C  addi r21, r11, 0x468c
	ctx.r[21].s64 = ctx.r[11].s64 + 18060;
	// 82DF7480: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82DF7484: 3B4BEBDC  addi r26, r11, -0x1424
	ctx.r[26].s64 = ctx.r[11].s64 + -5156;
	// 82DF7488: 3D600209  lis r11, 0x209
	ctx.r[11].s64 = 34144256;
	// 82DF748C: 6176A5AD  ori r22, r11, 0xa5ad
	ctx.r[22].u64 = ctx.r[11].u64 | 42413;
	// 82DF7490: 897A0000  lbz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7494: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82DF7498: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF749C: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82DF74A0: 419A0024  beq cr6, 0x82df74c4
	if ctx.cr[6].eq {
	pc = 0x82DF74C4; continue 'dispatch;
	}
	// 82DF74A4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF74A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF74AC: 419A029C  beq cr6, 0x82df7748
	if ctx.cr[6].eq {
	pc = 0x82DF7748; continue 'dispatch;
	}
	// 82DF74B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF74B4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF74B8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF74BC: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 82DF74C0: 409AFFE8  bne cr6, 0x82df74a8
	if !ctx.cr[6].eq {
	pc = 0x82DF74A8; continue 'dispatch;
	}
	// 82DF74C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF74C8: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82DF74CC: 4B3EF2FD  bl 0x821e67c8
	ctx.lr = 0x82DF74D0;
	sub_821E67C8(ctx, base);
	// 82DF74D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF74D4: 419A0184  beq cr6, 0x82df7658
	if ctx.cr[6].eq {
	pc = 0x82DF7658; continue 'dispatch;
	}
	// 82DF74D8: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 82DF74DC: 419A000C  beq cr6, 0x82df74e8
	if ctx.cr[6].eq {
	pc = 0x82DF74E8; continue 'dispatch;
	}
	// 82DF74E0: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 82DF74E4: 409A0174  bne cr6, 0x82df7658
	if !ctx.cr[6].eq {
	pc = 0x82DF7658; continue 'dispatch;
	}
	// 82DF74E8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82DF74EC: 9A610050  stb r19, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[19].u8 ) };
	// 82DF74F0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82DF74F4: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF74F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF74FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7500: 409AFFF4  bne cr6, 0x82df74f4
	if !ctx.cr[6].eq {
	pc = 0x82DF74F4; continue 'dispatch;
	}
	// 82DF7504: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82DF7508: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF750C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF7510: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82DF7514: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF7518: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF751C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7520: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF7524: 409AFFF4  bne cr6, 0x82df7518
	if !ctx.cr[6].eq {
	pc = 0x82DF7518; continue 'dispatch;
	}
	// 82DF7528: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF752C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF7530: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7534: 216B00FE  subfic r11, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[11].s64 = (254 as i64) - ctx.r[11].s64;
	// 82DF7538: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF753C: 4098002C  bge cr6, 0x82df7568
	if !ctx.cr[6].lt {
	pc = 0x82DF7568; continue 'dispatch;
	}
	// 82DF7540: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DF7544: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF7548: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF754C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7550: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7554: 409AFFF4  bne cr6, 0x82df7548
	if !ctx.cr[6].eq {
	pc = 0x82DF7548; continue 'dispatch;
	}
	// 82DF7558: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF755C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF7560: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF7564: 4800002C  b 0x82df7590
	pc = 0x82DF7590; continue 'dispatch;
	// 82DF7568: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF756C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF7570: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7574: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7578: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF757C: 409AFFF4  bne cr6, 0x82df7570
	if !ctx.cr[6].eq {
	pc = 0x82DF7570; continue 'dispatch;
	}
	// 82DF7580: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF7584: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF7588: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF758C: 20AB00FE  subfic r5, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[5].s64 = (254 as i64) - ctx.r[11].s64;
	// 82DF7590: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF7594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7598: 4BEB8C01  bl 0x82cb0198
	ctx.lr = 0x82DF759C;
	sub_82CB0198(ctx, base);
	// 82DF759C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82DF75A0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82DF75A4: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF75A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF75AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF75B0: 409AFFF4  bne cr6, 0x82df75a4
	if !ctx.cr[6].eq {
	pc = 0x82DF75A4; continue 'dispatch;
	}
	// 82DF75B4: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82DF75B8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF75BC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF75C0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82DF75C4: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF75C8: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF75CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF75D0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF75D4: 409AFFF4  bne cr6, 0x82df75c8
	if !ctx.cr[6].eq {
	pc = 0x82DF75C8; continue 'dispatch;
	}
	// 82DF75D8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DF75DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF75E0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF75E4: 216B00FE  subfic r11, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[11].s64 = (254 as i64) - ctx.r[11].s64;
	// 82DF75E8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF75EC: 4098002C  bge cr6, 0x82df7618
	if !ctx.cr[6].lt {
	pc = 0x82DF7618; continue 'dispatch;
	}
	// 82DF75F0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DF75F4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF75F8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF75FC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7600: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7604: 409AFFF4  bne cr6, 0x82df75f8
	if !ctx.cr[6].eq {
	pc = 0x82DF75F8; continue 'dispatch;
	}
	// 82DF7608: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF760C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF7610: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF7614: 4800002C  b 0x82df7640
	pc = 0x82DF7640; continue 'dispatch;
	// 82DF7618: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF761C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DF7620: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7624: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7628: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF762C: 409AFFF4  bne cr6, 0x82df7620
	if !ctx.cr[6].eq {
	pc = 0x82DF7620; continue 'dispatch;
	}
	// 82DF7630: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82DF7634: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF7638: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF763C: 20AB00FE  subfic r5, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[5].s64 = (254 as i64) - ctx.r[11].s64;
	// 82DF7640: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7644: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7648: 7C8BC02E  lwzx r4, r11, r24
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DF764C: 4BEB8B4D  bl 0x82cb0198
	ctx.lr = 0x82DF7650;
	sub_82CB0198(ctx, base);
	// 82DF7650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DF7654: 4BF6052D  bl 0x82d57b80
	ctx.lr = 0x82DF7658;
	sub_82D57B80(ctx, base);
	// 82DF7658: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82DF765C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7660: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF7664: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 82DF7668: 419A0020  beq cr6, 0x82df7688
	if ctx.cr[6].eq {
	pc = 0x82DF7688; continue 'dispatch;
	}
	// 82DF766C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF7670: 419A00D8  beq cr6, 0x82df7748
	if ctx.cr[6].eq {
	pc = 0x82DF7748; continue 'dispatch;
	}
	// 82DF7674: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7678: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF767C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF7680: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 82DF7684: 409AFFE8  bne cr6, 0x82df766c
	if !ctx.cr[6].eq {
	pc = 0x82DF766C; continue 'dispatch;
	}
	// 82DF7688: 564A063E  clrlwi r10, r18, 0x18
	ctx.r[10].u64 = ctx.r[18].u32 as u64 & 0x000000FFu64;
	// 82DF768C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7690: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF7694: 419A0060  beq cr6, 0x82df76f4
	if ctx.cr[6].eq {
	pc = 0x82DF76F4; continue 'dispatch;
	}
	// 82DF7698: 817CEF9C  lwz r11, -0x1064(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4196 as u32) ) } as u64;
	// 82DF769C: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DF76A0: 7D7FEA78  xor r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 ^ ctx.r[29].u64;
	// 82DF76A4: 48000135  bl 0x82df77d8
	ctx.lr = 0x82DF76A8;
	sub_82DF77D8(ctx, base);
	// 82DF76A8: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 82DF76AC: 7F2B1800  cmpd cr6, r11, r3
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[3].s64, &mut ctx.xer);
	// 82DF76B0: 40990084  ble cr6, 0x82df7734
	if !ctx.cr[6].gt {
	pc = 0x82DF7734; continue 'dispatch;
	}
	// 82DF76B4: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82DF76B8: 7F2BB000  cmpd cr6, r11, r22
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[22].s64, &mut ctx.xer);
	// 82DF76BC: 40980078  bge cr6, 0x82df7734
	if !ctx.cr[6].lt {
	pc = 0x82DF7734; continue 'dispatch;
	}
	// 82DF76C0: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 82DF76C4: 419A000C  beq cr6, 0x82df76d0
	if ctx.cr[6].eq {
	pc = 0x82DF76D0; continue 'dispatch;
	}
	// 82DF76C8: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 82DF76CC: 409A0094  bne cr6, 0x82df7760
	if !ctx.cr[6].eq {
	pc = 0x82DF7760; continue 'dispatch;
	}
	// 82DF76D0: 3974FFFA  addi r11, r20, -6
	ctx.r[11].s64 = ctx.r[20].s64 + -6;
	// 82DF76D4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82DF76D8: 41990088  bgt cr6, 0x82df7760
	if ctx.cr[6].gt {
	pc = 0x82DF7760; continue 'dispatch;
	}
	// 82DF76DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF76E0: 386B4608  addi r3, r11, 0x4608
	ctx.r[3].s64 = ctx.r[11].s64 + 17928;
	// 82DF76E4: 4BF6049D  bl 0x82d57b80
	ctx.lr = 0x82DF76E8;
	sub_82D57B80(ctx, base);
	// 82DF76E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF76EC: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82DF76F0: 4BEB1D40  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
	// 82DF76F4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF76F8: 7E699B78  mr r9, r19
	ctx.r[9].u64 = ctx.r[19].u64;
	// 82DF76FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF7700: 419A0020  beq cr6, 0x82df7720
	if ctx.cr[6].eq {
	pc = 0x82DF7720; continue 'dispatch;
	}
	// 82DF7704: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF7708: 1D290017  mulli r9, r9, 0x17
	ctx.r[9].s64 = ctx.r[9].s64 * 23;
	// 82DF770C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7710: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF7714: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DF7718: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF771C: 409AFFE8  bne cr6, 0x82df7704
	if !ctx.cr[6].eq {
	pc = 0x82DF7704; continue 'dispatch;
	}
	// 82DF7720: 7D2BEA78  xor r11, r9, r29
	ctx.r[11].u64 = ctx.r[9].u64 ^ ctx.r[29].u64;
	// 82DF7724: 556A007E  clrlwi r10, r11, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DF7728: 817CEF9C  lwz r11, -0x1064(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4196 as u32) ) } as u64;
	// 82DF772C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF7730: 419A0064  beq cr6, 0x82df7794
	if ctx.cr[6].eq {
	pc = 0x82DF7794; continue 'dispatch;
	}
	// 82DF7734: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82DF7738: 39770030  addi r11, r23, 0x30
	ctx.r[11].s64 = ctx.r[23].s64 + 48;
	// 82DF773C: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82DF7740: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF7744: 4198FD4C  blt cr6, 0x82df7490
	if ctx.cr[6].lt {
	pc = 0x82DF7490; continue 'dispatch;
	}
	// 82DF7748: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF774C: 386B4588  addi r3, r11, 0x4588
	ctx.r[3].s64 = ctx.r[11].s64 + 17800;
	// 82DF7750: 4BF60431  bl 0x82d57b80
	ctx.lr = 0x82DF7754;
	sub_82D57B80(ctx, base);
	// 82DF7754: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF7758: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82DF775C: 4BEB1CD4  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
	// 82DF7760: 2F190004  cmpwi cr6, r25, 4
	ctx.cr[6].compare_i32(ctx.r[25].s32, 4, &mut ctx.xer);
	// 82DF7764: 419A0024  beq cr6, 0x82df7788
	if ctx.cr[6].eq {
	pc = 0x82DF7788; continue 'dispatch;
	}
	// 82DF7768: 2F14000A  cmpwi cr6, r20, 0xa
	ctx.cr[6].compare_i32(ctx.r[20].s32, 10, &mut ctx.xer);
	// 82DF776C: 4198001C  blt cr6, 0x82df7788
	if ctx.cr[6].lt {
	pc = 0x82DF7788; continue 'dispatch;
	}
	// 82DF7770: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF7774: 386B4508  addi r3, r11, 0x4508
	ctx.r[3].s64 = ctx.r[11].s64 + 17672;
	// 82DF7778: 4BF60409  bl 0x82d57b80
	ctx.lr = 0x82DF777C;
	sub_82D57B80(ctx, base);
	// 82DF777C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF7780: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82DF7784: 4BEB1CAC  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
	// 82DF7788: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF778C: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 82DF7790: 4BEB1CA0  b 0x82ca9430
	sub_82CA9420(ctx, base);
	return;
	// 82DF7794: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 82DF7798: 419A000C  beq cr6, 0x82df77a4
	if ctx.cr[6].eq {
	pc = 0x82DF77A4; continue 'dispatch;
	}
	// 82DF779C: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 82DF77A0: 409AFFC0  bne cr6, 0x82df7760
	if !ctx.cr[6].eq {
	pc = 0x82DF7760; continue 'dispatch;
	}
	// 82DF77A4: 3974FFFA  addi r11, r20, -6
	ctx.r[11].s64 = ctx.r[20].s64 + -6;
	// 82DF77A8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82DF77AC: 4099FF30  ble cr6, 0x82df76dc
	if !ctx.cr[6].gt {
	pc = 0x82DF76DC; continue 'dispatch;
	}
	// 82DF77B0: 4BFFFFB0  b 0x82df7760
	pc = 0x82DF7760; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF77B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF77B8 size=20
    let mut pc: u32 = 0x82DF77B8;
    'dispatch: loop {
        match pc {
            0x82DF77B8 => {
    //   block [0x82DF77B8..0x82DF77CC)
	// 82DF77B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82DF77BC: 814BAC54  lwz r10, -0x53ac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21420 as u32) ) } as u64;
	// 82DF77C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF77C4: 914BAC54  stw r10, -0x53ac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-21420 as u32), ctx.r[10].u32 ) };
	// 82DF77C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF77D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF77D0 size=4
    let mut pc: u32 = 0x82DF77D0;
    'dispatch: loop {
        match pc {
            0x82DF77D0 => {
    //   block [0x82DF77D0..0x82DF77D4)
	// 82DF77D0: 48210130  b 0x83007900
	sub_83007900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF77D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF77D8 size=8
    let mut pc: u32 = 0x82DF77D8;
    'dispatch: loop {
        match pc {
            0x82DF77D8 => {
    //   block [0x82DF77D8..0x82DF77E0)
	// 82DF77D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF77DC: 48210124  b 0x83007900
	sub_83007900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF77E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF77E0 size=272
    let mut pc: u32 = 0x82DF77E0;
    'dispatch: loop {
        match pc {
            0x82DF77E0 => {
    //   block [0x82DF77E0..0x82DF78F0)
	// 82DF77E0: 3866FFFF  addi r3, r6, -1
	ctx.r[3].s64 = ctx.r[6].s64 + -1;
	// 82DF77E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DF77E8: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82DF77EC: 41980094  blt cr6, 0x82df7880
	if ctx.cr[6].lt {
	pc = 0x82DF7880; continue 'dispatch;
	}
	// 82DF77F0: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 82DF77F4: 39440008  addi r10, r4, 8
	ctx.r[10].s64 = ctx.r[4].s64 + 8;
	// 82DF77F8: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF77FC: 39650004  addi r11, r5, 4
	ctx.r[11].s64 = ctx.r[5].s64 + 4;
	// 82DF7800: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF7804: 7D052050  subf r8, r5, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82DF7808: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF780C: 7DA85C2E  lfsx f13, r8, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7810: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DF7814: C00AFFF8  lfs f0, -8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7818: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF781C: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7820: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF7824: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF7828: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF782C: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DF7830: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF7834: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF7838: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF783C: 7C085C2E  lfsx f0, r8, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7840: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF7844: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF7848: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF784C: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7850: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7854: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF7858: EC00587A  fmadds f0, f0, f1, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[11].f64) as f32) as f64);
	// 82DF785C: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF7860: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7864: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7868: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DF786C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF7870: EC00507A  fmadds f0, f0, f1, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[10].f64) as f32) as f64);
	// 82DF7874: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DF7878: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF787C: 409AFF90  bne cr6, 0x82df780c
	if !ctx.cr[6].eq {
	pc = 0x82DF780C; continue 'dispatch;
	}
	// 82DF7880: 7F071800  cmpw cr6, r7, r3
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82DF7884: 40980040  bge cr6, 0x82df78c4
	if !ctx.cr[6].lt {
	pc = 0x82DF78C4; continue 'dispatch;
	}
	// 82DF7888: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF788C: 7D252050  subf r9, r5, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82DF7890: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DF7894: 7D471850  subf r10, r7, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[7].s64;
	// 82DF7898: 7D0B4A14  add r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DF789C: 7C0B4C2E  lfsx f0, r11, r9
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF78A0: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF78A4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF78A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF78AC: C1880004  lfs f12, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF78B0: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DF78B4: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF78B8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF78BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF78C0: 409AFFD8  bne cr6, 0x82df7898
	if !ctx.cr[6].eq {
	pc = 0x82DF7898; continue 'dispatch;
	}
	// 82DF78C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DF78C8: 54CB103A  slwi r11, r6, 2
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF78CC: C00A0C4C  lfs f0, 0xc4c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF78D0: 7D4B2A14  add r10, r11, r5
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82DF78D4: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DF78D8: C18AFFFC  lfs f12, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF78DC: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF78E0: EDAD0072  fmuls f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 82DF78E4: EC0D603A  fmadds f0, f13, f0, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF78E8: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF78EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF78F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF78F0 size=264
    let mut pc: u32 = 0x82DF78F0;
    'dispatch: loop {
        match pc {
            0x82DF78F0 => {
    //   block [0x82DF78F0..0x82DF79F8)
	// 82DF78F0: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 82DF78F4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DF78F8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82DF78FC: 41980098  blt cr6, 0x82df7994
	if ctx.cr[6].lt {
	pc = 0x82DF7994; continue 'dispatch;
	}
	// 82DF7900: 3966FFFB  addi r11, r6, -5
	ctx.r[11].s64 = ctx.r[6].s64 + -5;
	// 82DF7904: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 82DF7908: 5569F0BE  srwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF790C: 39650008  addi r11, r5, 8
	ctx.r[11].s64 = ctx.r[5].s64 + 8;
	// 82DF7910: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF7914: 7D052050  subf r8, r5, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 82DF7918: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF791C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82DF7920: C1ABFFFC  lfs f13, -4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7924: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82DF7928: C00BFFF8  lfs f0, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF792C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF7930: C1AAFFF4  lfs f13, -0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7934: 7D885C2E  lfsx f12, r8, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF7938: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF793C: C16AFFFC  lfs f11, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF7940: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DF7944: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF7948: D00AFFF4  stfs f0, -0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82DF794C: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7950: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7954: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF7958: EC00607A  fmadds f0, f0, f1, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF795C: 7C085D2E  stfsx f0, r8, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82DF7960: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7964: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7968: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF796C: EC00587A  fmadds f0, f0, f1, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[11].f64) as f32) as f64);
	// 82DF7970: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF7974: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7978: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF797C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF7980: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF7984: EC00507A  fmadds f0, f0, f1, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[10].f64) as f32) as f64);
	// 82DF7988: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF798C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DF7990: 409AFF90  bne cr6, 0x82df7920
	if !ctx.cr[6].eq {
	pc = 0x82DF7920; continue 'dispatch;
	}
	// 82DF7994: 7F073000  cmpw cr6, r7, r6
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DF7998: 40980040  bge cr6, 0x82df79d8
	if !ctx.cr[6].lt {
	pc = 0x82DF79D8; continue 'dispatch;
	}
	// 82DF799C: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF79A0: 7D242850  subf r9, r4, r5
	ctx.r[9].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 82DF79A4: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DF79A8: 7D473050  subf r10, r7, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 82DF79AC: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82DF79B0: 7C095C2E  lfsx f0, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF79B4: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF79B8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF79BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF79C0: C188FFFC  lfs f12, -4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF79C4: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 82DF79C8: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82DF79CC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF79D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF79D4: 409AFFD8  bne cr6, 0x82df79ac
	if !ctx.cr[6].eq {
	pc = 0x82DF79AC; continue 'dispatch;
	}
	// 82DF79D8: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF79DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF79E0: EDAD0072  fmuls f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 82DF79E4: C1840000  lfs f12, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF79E8: C00B0C4C  lfs f0, 0xc4c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF79EC: EC0D603A  fmadds f0, f13, f0, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF79F0: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF79F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF79F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF79F8 size=384
    let mut pc: u32 = 0x82DF79F8;
    'dispatch: loop {
        match pc {
            0x82DF79F8 => {
    //   block [0x82DF79F8..0x82DF7B78)
	// 82DF79F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF79FC: 4BEB1A05  bl 0x82ca9400
	ctx.lr = 0x82DF7A00;
	sub_82CA93D0(ctx, base);
	// 82DF7A00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7A04: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7A08: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82DF7A0C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF7A10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF7A14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF7A18: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82DF7A1C: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DF7A20: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DF7A24: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF7A28: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF7A2C: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DF7A30: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF7A34: 41990010  bgt cr6, 0x82df7a44
	if ctx.cr[6].gt {
	pc = 0x82DF7A44; continue 'dispatch;
	}
	// 82DF7A38: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DF7A3C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82DF7A40: 48000018  b 0x82df7a58
	pc = 0x82DF7A58; continue 'dispatch;
	// 82DF7A44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7A48: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF7A4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF7A50: 4E800421  bctrl
	ctx.lr = 0x82DF7A54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF7A54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF7A58: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DF7A5C: 40990014  ble cr6, 0x82df7a70
	if !ctx.cr[6].gt {
	pc = 0x82DF7A70; continue 'dispatch;
	}
	// 82DF7A60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF7A64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7A68: 57A5103A  slwi r5, r29, 2
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF7A6C: 4BEB1E9D  bl 0x82ca9908
	ctx.lr = 0x82DF7A70;
	sub_82CA9908(ctx, base);
	// 82DF7A70: 7FA60E70  srawi r6, r29, 1
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[29].s32 >> 1) as i64;
	// 82DF7A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF7A78: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82DF7A7C: 4198007C  blt cr6, 0x82df7af8
	if ctx.cr[6].lt {
	pc = 0x82DF7AF8; continue 'dispatch;
	}
	// 82DF7A80: 3966FFFC  addi r11, r6, -4
	ctx.r[11].s64 = ctx.r[6].s64 + -4;
	// 82DF7A84: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF7A88: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7A8C: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 82DF7A90: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82DF7A94: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82DF7A98: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82DF7A9C: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF7AA0: C00BFFF8  lfs f0, -8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7AA4: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82DF7AA8: D00AFFF8  stfs f0, -8(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DF7AAC: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7AB0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DF7AB4: D1A90000  stfs f13, 0(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF7AB8: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7ABC: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7AC0: D00AFFFC  stfs f0, -4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF7AC4: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7AC8: D1A90004  stfs f13, 4(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF7ACC: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7AD0: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF7AD4: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7AD8: D1A90008  stfs f13, 8(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DF7ADC: C1AB0014  lfs f13, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7AE0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82DF7AE4: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF7AE8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DF7AEC: D1A9000C  stfs f13, 0xc(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DF7AF0: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82DF7AF4: 409AFFAC  bne cr6, 0x82df7aa0
	if !ctx.cr[6].eq {
	pc = 0x82DF7AA0; continue 'dispatch;
	}
	// 82DF7AF8: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DF7AFC: 4098004C  bge cr6, 0x82df7b48
	if !ctx.cr[6].lt {
	pc = 0x82DF7B48; continue 'dispatch;
	}
	// 82DF7B00: 7D483214  add r10, r8, r6
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82DF7B04: 550B1838  slwi r11, r8, 3
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7B08: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF7B0C: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF7B10: 7CEAFA14  add r7, r10, r31
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82DF7B14: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DF7B18: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82DF7B1C: 7D483050  subf r10, r8, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[8].s64;
	// 82DF7B20: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7B24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF7B28: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7B2C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DF7B30: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF7B34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF7B38: D1A70000  stfs f13, 0(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF7B3C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DF7B40: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82DF7B44: 409AFFDC  bne cr6, 0x82df7b20
	if !ctx.cr[6].eq {
	pc = 0x82DF7B20; continue 'dispatch;
	}
	// 82DF7B48: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DF7B4C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DF7B50: 93C30020  stw r30, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82DF7B54: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7B58: 409A0018  bne cr6, 0x82df7b70
	if !ctx.cr[6].eq {
	pc = 0x82DF7B70; continue 'dispatch;
	}
	// 82DF7B5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7B60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF7B64: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF7B68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF7B6C: 4E800421  bctrl
	ctx.lr = 0x82DF7B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF7B70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF7B74: 4BEB18DC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF7B78 size=380
    let mut pc: u32 = 0x82DF7B78;
    'dispatch: loop {
        match pc {
            0x82DF7B78 => {
    //   block [0x82DF7B78..0x82DF7CF4)
	// 82DF7B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7B7C: 4BEB1885  bl 0x82ca9400
	ctx.lr = 0x82DF7B80;
	sub_82CA93D0(ctx, base);
	// 82DF7B80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7B84: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7B88: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82DF7B8C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DF7B90: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DF7B94: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DF7B98: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82DF7B9C: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DF7BA0: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DF7BA4: 83E30020  lwz r31, 0x20(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF7BA8: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF7BAC: 7D7F2214  add r11, r31, r4
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[4].u64;
	// 82DF7BB0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DF7BB4: 4199000C  bgt cr6, 0x82df7bc0
	if ctx.cr[6].gt {
	pc = 0x82DF7BC0; continue 'dispatch;
	}
	// 82DF7BB8: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DF7BBC: 48000018  b 0x82df7bd4
	pc = 0x82DF7BD4; continue 'dispatch;
	// 82DF7BC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7BC4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF7BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF7BCC: 4E800421  bctrl
	ctx.lr = 0x82DF7BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF7BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF7BD4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF7BD8: 40990014  ble cr6, 0x82df7bec
	if !ctx.cr[6].gt {
	pc = 0x82DF7BEC; continue 'dispatch;
	}
	// 82DF7BDC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF7BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DF7BE4: 57C5103A  slwi r5, r30, 2
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF7BE8: 4BEB1D21  bl 0x82ca9908
	ctx.lr = 0x82DF7BEC;
	sub_82CA9908(ctx, base);
	// 82DF7BEC: 7FC60E70  srawi r6, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 82DF7BF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DF7BF4: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82DF7BF8: 4198007C  blt cr6, 0x82df7c74
	if ctx.cr[6].lt {
	pc = 0x82DF7C74; continue 'dispatch;
	}
	// 82DF7BFC: 3966FFFC  addi r11, r6, -4
	ctx.r[11].s64 = ctx.r[6].s64 + -4;
	// 82DF7C00: 54C9103A  slwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF7C04: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7C08: 395F0008  addi r10, r31, 8
	ctx.r[10].s64 = ctx.r[31].s64 + 8;
	// 82DF7C0C: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82DF7C10: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82DF7C14: 397D0008  addi r11, r29, 8
	ctx.r[11].s64 = ctx.r[29].s64 + 8;
	// 82DF7C18: 54E8103A  slwi r8, r7, 2
	ctx.r[8].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF7C1C: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 82DF7C20: C00AFFF8  lfs f0, -8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7C24: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7C28: C18AFFFC  lfs f12, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF7C2C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DF7C30: C1690004  lfs f11, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF7C34: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DF7C38: C1290008  lfs f9, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DF7C3C: C10A0004  lfs f8, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DF7C40: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DF7C44: C0E9000C  lfs f7, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82DF7C48: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82DF7C4C: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DF7C50: D1ABFFFC  stfs f13, -4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF7C54: D18B0000  stfs f12, 0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF7C58: D16B0004  stfs f11, 4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF7C5C: D14B0008  stfs f10, 8(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DF7C60: D12B000C  stfs f9, 0xc(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82DF7C64: D10B0010  stfs f8, 0x10(r11)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DF7C68: D0EB0014  stfs f7, 0x14(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82DF7C6C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82DF7C70: 409AFFAC  bne cr6, 0x82df7c1c
	if !ctx.cr[6].eq {
	pc = 0x82DF7C1C; continue 'dispatch;
	}
	// 82DF7C74: 7F083000  cmpw cr6, r8, r6
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82DF7C78: 4098004C  bge cr6, 0x82df7cc4
	if !ctx.cr[6].lt {
	pc = 0x82DF7CC4; continue 'dispatch;
	}
	// 82DF7C7C: 7D483214  add r10, r8, r6
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82DF7C80: 550B1838  slwi r11, r8, 3
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7C84: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF7C88: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF7C8C: 7CEAFA14  add r7, r10, r31
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82DF7C90: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DF7C94: 7D29FA14  add r9, r9, r31
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82DF7C98: 7D483050  subf r10, r8, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[8].s64;
	// 82DF7C9C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF7CA0: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF7CA4: C1A70000  lfs f13, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF7CA8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DF7CAC: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF7CB0: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82DF7CB4: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF7CB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF7CBC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DF7CC0: 409AFFDC  bne cr6, 0x82df7c9c
	if !ctx.cr[6].eq {
	pc = 0x82DF7C9C; continue 'dispatch;
	}
	// 82DF7CC4: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DF7CC8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DF7CCC: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 82DF7CD0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF7CD4: 409A0018  bne cr6, 0x82df7cec
	if !ctx.cr[6].eq {
	pc = 0x82DF7CEC; continue 'dispatch;
	}
	// 82DF7CD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF7CDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF7CE0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF7CE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF7CE8: 4E800421  bctrl
	ctx.lr = 0x82DF7CEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF7CEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DF7CF0: 4BEB1760  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF7CF8 size=644
    let mut pc: u32 = 0x82DF7CF8;
    'dispatch: loop {
        match pc {
            0x82DF7CF8 => {
    //   block [0x82DF7CF8..0x82DF7F7C)
	// 82DF7CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7CFC: 4BEB170D  bl 0x82ca9408
	ctx.lr = 0x82DF7D00;
	sub_82CA93D0(ctx, base);
	// 82DF7D00: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82DF7D04: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82DF7D08: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7D0C: 3D008334  lis r8, -0x7ccc
	ctx.r[8].s64 = -2093744128;
	// 82DF7D10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF7D14: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DF7D18: 8128AD60  lwz r9, -0x52a0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-21152 as u32) ) } as u64;
	// 82DF7D1C: 552B07FE  clrlwi r11, r9, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82DF7D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF7D24: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82DF7D28: 394BACE0  addi r10, r11, -0x5320
	ctx.r[10].s64 = ctx.r[11].s64 + -21280;
	// 82DF7D2C: 409A00A8  bne cr6, 0x82df7dd4
	if !ctx.cr[6].eq {
	pc = 0x82DF7DD4; continue 'dispatch;
	}
	// 82DF7D30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF7D34: 61290001  ori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 | 1;
	// 82DF7D38: 396B1220  addi r11, r11, 0x1220
	ctx.r[11].s64 = ctx.r[11].s64 + 4640;
	// 82DF7D3C: 9128AD60  stw r9, -0x52a0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-21152 as u32), ctx.r[9].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF7F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF7F80 size=144
    let mut pc: u32 = 0x82DF7F80;
    'dispatch: loop {
        match pc {
            0x82DF7F80 => {
    //   block [0x82DF7F80..0x82DF8010)
	// 82DF7F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF7F84: 4BEB1489  bl 0x82ca940c
	ctx.lr = 0x82DF7F88;
	sub_82CA93D0(ctx, base);
	// 82DF7F88: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82DF7F8C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82DF7F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF7F94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF7F98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF7F9C: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 82DF7FA0: 40990060  ble cr6, 0x82df8000
	if !ctx.cr[6].gt {
	pc = 0x82DF8000; continue 'dispatch;
	}
	// 82DF7FA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DF7FA8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF7FAC: C3CA0A7C  lfs f30, 0xa7c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2684 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82DF7FB0: C3EB0C8C  lfs f31, 0xc8c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DF7FB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DF7FB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF7FBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF7FC0: 4BFFFA39  bl 0x82df79f8
	ctx.lr = 0x82DF7FC4;
	sub_82DF79F8(ctx, base);
	// 82DF7FC4: 57FFF87E  srwi r31, r31, 1
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shr(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DF7FC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF7FCC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DF7FD0: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF7FD4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DF7FD8: 7FABF214  add r29, r11, r30
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DF7FDC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF7FE0: 4BFFF801  bl 0x82df77e0
	ctx.lr = 0x82DF7FE4;
	sub_82DF77E0(ctx, base);
	// 82DF7FE4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DF7FE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF7FEC: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82DF7FF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DF7FF4: 4BFFF8FD  bl 0x82df78f0
	ctx.lr = 0x82DF7FF8;
	sub_82DF78F0(ctx, base);
	// 82DF7FF8: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 82DF7FFC: 4199FFB8  bgt cr6, 0x82df7fb4
	if ctx.cr[6].gt {
	pc = 0x82DF7FB4; continue 'dispatch;
	}
	// 82DF8000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF8004: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82DF8008: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82DF800C: 4BEB1450  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF8010 size=820
    let mut pc: u32 = 0x82DF8010;
    'dispatch: loop {
        match pc {
            0x82DF8010 => {
    //   block [0x82DF8010..0x82DF8344)
	// 82DF8010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8014: 4BEB13ED  bl 0x82ca9400
	ctx.lr = 0x82DF8018;
	sub_82CA93D0(ctx, base);
	// 82DF8018: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82DF801C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8020: 7CCB0774  extsb r11, r6
	ctx.r[11].s64 = ctx.r[6].s8 as i64;
	// 82DF8024: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DF8028: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DF802C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF8030: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8034: 419A0298  beq cr6, 0x82df82cc
	if ctx.cr[6].eq {
	pc = 0x82DF82CC; continue 'dispatch;
	}
	// 82DF8038: 7BEA0020  clrldi r10, r31, 0x20
	ctx.r[10].u64 = ctx.r[31].u64 & 0x00000000FFFFFFFFu64;
	// 82DF803C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82DF8040: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DF8044: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DF8048: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DF804C: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DF8050: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8054: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82DF8058: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DF805C: 419802DC  blt cr6, 0x82df8338
	if ctx.cr[6].lt {
	pc = 0x82DF8338; continue 'dispatch;
	}
	// 82DF8060: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8064: 419A0268  beq cr6, 0x82df82cc
	if ctx.cr[6].eq {
	pc = 0x82DF82CC; continue 'dispatch;
	}
	// 82DF8068: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF806C: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 82DF8070: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DF8074: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82DF8078: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DF807C: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 82DF8080: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DF8084: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82DF8088: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82DF808C: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82DF8090: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DF8094: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DF8098: 7D2A2214  add r9, r10, r4
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82DF809C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DF80A0: 4199000C  bgt cr6, 0x82df80ac
	if ctx.cr[6].gt {
	pc = 0x82DF80AC; continue 'dispatch;
	}
	// 82DF80A4: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82DF80A8: 4800001C  b 0x82df80c4
	pc = 0x82DF80C4; continue 'dispatch;
	// 82DF80AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF80B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DF80B4: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DF80B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF80BC: 4E800421  bctrl
	ctx.lr = 0x82DF80C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF80C0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF80C4: 7FEBF378  or r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 82DF80C8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DF80CC: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82DF80D0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DF80D4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82DF80D8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DF80DC: 40980024  bge cr6, 0x82df8100
	if !ctx.cr[6].lt {
	pc = 0x82DF8100; continue 'dispatch;
	}
	// 82DF80E0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF80E4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF80E8: 41980008  blt cr6, 0x82df80f0
	if ctx.cr[6].lt {
	pc = 0x82DF80F0; continue 'dispatch;
	}
	// 82DF80EC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82DF80F0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82DF80F4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DF80F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DF80FC: 4BF5EE15  bl 0x82d56f10
	ctx.lr = 0x82DF8100;
	sub_82D56F10(ctx, base);
	// 82DF8100: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82DF8104: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DF8108: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 82DF810C: 41980084  blt cr6, 0x82df8190
	if ctx.cr[6].lt {
	pc = 0x82DF8190; continue 'dispatch;
	}
	// 82DF8110: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 82DF8114: 20DD0004  subfic r6, r29, 4
	ctx.xer.ca = ctx.r[29].u32 <= 4 as u32;
	ctx.r[6].s64 = (4 as i64) - ctx.r[29].s64;
	// 82DF8118: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF811C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82DF8120: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82DF8124: 397D0008  addi r11, r29, 8
	ctx.r[11].s64 = ctx.r[29].s64 + 8;
	// 82DF8128: 20BDFFF8  subfic r5, r29, -8
	ctx.xer.ca = ctx.r[29].u32 <= -8 as u32;
	ctx.r[5].s64 = (-8 as i64) - ctx.r[29].s64;
	// 82DF812C: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DF8130: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF8134: 7D055A14  add r8, r5, r11
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82DF8138: C00BFFF8  lfs f0, -8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF813C: 7C665A14  add r3, r6, r11
	ctx.r[3].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82DF8140: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82DF8144: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF8148: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF814C: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 82DF8150: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF8154: C00BFFFC  lfs f0, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8158: 7D083A14  add r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82DF815C: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82DF8160: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF8164: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF8168: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF816C: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82DF8170: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82DF8174: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF8178: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF817C: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82DF8180: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82DF8184: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF8188: 7C03452E  stfsx f0, r3, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82DF818C: 409AFFA4  bne cr6, 0x82df8130
	if !ctx.cr[6].eq {
	pc = 0x82DF8130; continue 'dispatch;
	}
	// 82DF8190: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF8194: 4098002C  bge cr6, 0x82df81c0
	if !ctx.cr[6].lt {
	pc = 0x82DF81C0; continue 'dispatch;
	}
	// 82DF8198: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF819C: 7D44F850  subf r10, r4, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[4].s64;
	// 82DF81A0: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF81A4: 7C0BEC2E  lfsx f0, r11, r29
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF81A8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF81AC: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82DF81B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF81B4: 7C0B4D2E  stfsx f0, r11, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), tmp.u32) };
	// 82DF81B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF81BC: 409AFFE4  bne cr6, 0x82df81a0
	if !ctx.cr[6].eq {
	pc = 0x82DF81A0; continue 'dispatch;
	}
	// 82DF81C0: 7BEB0020  clrldi r11, r31, 0x20
	ctx.r[11].u64 = ctx.r[31].u64 & 0x00000000FFFFFFFFu64;
	// 82DF81C4: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 82DF81C8: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82DF81CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF81D0: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DF81D4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DF81D8: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DF81DC: C00B0C38  lfs f0, 0xc38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF81E0: EC0D07F8  fmsubs f0, f13, f31, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DF81E4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82DF81E8: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82DF81EC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DF81F0: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82DF81F4: 7F1EF800  cmpw cr6, r30, r31
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DF81F8: 41980008  blt cr6, 0x82df8200
	if ctx.cr[6].lt {
	pc = 0x82DF8200; continue 'dispatch;
	}
	// 82DF81FC: 3BDFFFFF  addi r30, r31, -1
	ctx.r[30].s64 = ctx.r[31].s64 + -1;
	// 82DF8200: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DF8204: 40980008  bge cr6, 0x82df820c
	if !ctx.cr[6].lt {
	pc = 0x82DF820C; continue 'dispatch;
	}
	// 82DF8208: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82DF820C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DF8210: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 82DF8214: 9B8B0000  stb r28, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82DF8218: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DF821C: 40990014  ble cr6, 0x82df8230
	if !ctx.cr[6].gt {
	pc = 0x82DF8230; continue 'dispatch;
	}
	// 82DF8220: 38BFFFFF  addi r5, r31, -1
	ctx.r[5].s64 = ctx.r[31].s64 + -1;
	// 82DF8224: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF8228: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DF822C: 4800011D  bl 0x82df8348
	ctx.lr = 0x82DF8230;
	sub_82DF8348(ctx, base);
	// 82DF8230: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF8234: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8238: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF823C: 7DAB542E  lfsx f13, r11, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8240: 419A0034  beq cr6, 0x82df8274
	if ctx.cr[6].eq {
	pc = 0x82DF8274; continue 'dispatch;
	}
	// 82DF8244: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DF8248: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82DF824C: C00A0C18  lfs f0, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8250: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF8254: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82DF8258: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 82DF825C: 41990008  bgt cr6, 0x82df8264
	if ctx.cr[6].gt {
	pc = 0x82DF8264; continue 'dispatch;
	}
	// 82DF8260: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8264: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 82DF8268: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF826C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF8270: 409AFFE0  bne cr6, 0x82df8250
	if !ctx.cr[6].eq {
	pc = 0x82DF8250; continue 'dispatch;
	}
	// 82DF8274: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DF8278: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DF827C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DF8280: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82DF8284: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF8288: 409A0014  bne cr6, 0x82df829c
	if !ctx.cr[6].eq {
	pc = 0x82DF829C; continue 'dispatch;
	}
	// 82DF828C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8290: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DF8294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DF8298: 4E800421  bctrl
	ctx.lr = 0x82DF829C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DF829C: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DF82A0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DF82A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF82A8: 409A0090  bne cr6, 0x82df8338
	if !ctx.cr[6].eq {
	pc = 0x82DF8338; continue 'dispatch;
	}
	// 82DF82AC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DF82B0: 7C7BD02E  lwzx r3, r27, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82DF82B4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DF82B8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DF82BC: 4BF5D00D  bl 0x82d552c8
	ctx.lr = 0x82DF82C0;
	sub_82D552C8(ctx, base);
	// 82DF82C0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DF82C4: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DF82C8: 4BEB1188  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82DF82CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF82D0: FD60F890  fmr f11, f31
	ctx.f[11].f64 = ctx.f[31].f64;
	// 82DF82D4: 57E9F87E  srwi r9, r31, 1
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF82D8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DF82DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DF82E0: C18B0C18  lfs f12, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF82E4: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 82DF82E8: 419A0050  beq cr6, 0x82df8338
	if ctx.cr[6].eq {
	pc = 0x82DF8338; continue 'dispatch;
	}
	// 82DF82EC: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF82F0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DF82F4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82DF82F8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF82FC: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8300: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82DF8304: FF0D5800  fcmpu cr6, f13, f11
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[11].f64);
	// 82DF8308: 40980014  bge cr6, 0x82df831c
	if !ctx.cr[6].lt {
	pc = 0x82DF831C; continue 'dispatch;
	}
	// 82DF830C: D18B0000  stfs f12, 0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8310: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DF8314: 40990008  ble cr6, 0x82df831c
	if !ctx.cr[6].gt {
	pc = 0x82DF831C; continue 'dispatch;
	}
	// 82DF8318: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DF831C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DF8320: 409A0010  bne cr6, 0x82df8330
	if !ctx.cr[6].eq {
	pc = 0x82DF8330; continue 'dispatch;
	}
	// 82DF8324: ED6B0028  fsubs f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DF8328: 5529F87E  srwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF832C: FC006090  fmr f0, f12
	ctx.f[0].f64 = ctx.f[12].f64;
	// 82DF8330: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF8334: 409AFFC0  bne cr6, 0x82df82f4
	if !ctx.cr[6].eq {
	pc = 0x82DF82F4; continue 'dispatch;
	}
	// 82DF8338: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DF833C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DF8340: 4BEB1110  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF8348 size=248
    let mut pc: u32 = 0x82DF8348;
    'dispatch: loop {
        match pc {
            0x82DF8348 => {
    //   block [0x82DF8348..0x82DF8440)
	// 82DF8348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF834C: 4BEB10BD  bl 0x82ca9408
	ctx.lr = 0x82DF8350;
	sub_82CA93D0(ctx, base);
	// 82DF8350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF8354: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DF8358: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF835C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF8360: 7D64EA14  add r11, r4, r29
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 82DF8364: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DF8368: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82DF836C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DF8370: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8374: 7C0BF42E  lfsx f0, r11, r30
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8378: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF837C: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DF8380: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8384: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF8388: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DF838C: 41980008  blt cr6, 0x82df8394
	if ctx.cr[6].lt {
	pc = 0x82DF8394; continue 'dispatch;
	}
	// 82DF8390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8394: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF8398: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF839C: 419A0010  beq cr6, 0x82df83ac
	if ctx.cr[6].eq {
	pc = 0x82DF83AC; continue 'dispatch;
	}
	// 82DF83A0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF83A4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DF83A8: 4BFFFFD8  b 0x82df8380
	pc = 0x82DF8380; continue 'dispatch;
	// 82DF83AC: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF83B0: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DF83B4: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF83B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DF83BC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF83C0: 41980008  blt cr6, 0x82df83c8
	if ctx.cr[6].lt {
	pc = 0x82DF83C8; continue 'dispatch;
	}
	// 82DF83C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF83C8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF83CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF83D0: 419A0010  beq cr6, 0x82df83e0
	if ctx.cr[6].eq {
	pc = 0x82DF83E0; continue 'dispatch;
	}
	// 82DF83D4: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82DF83D8: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 82DF83DC: 4BFFFFD8  b 0x82df83b4
	pc = 0x82DF83B4; continue 'dispatch;
	// 82DF83E0: 7F05F800  cmpw cr6, r5, r31
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DF83E4: 41980030  blt cr6, 0x82df8414
	if ctx.cr[6].lt {
	pc = 0x82DF8414; continue 'dispatch;
	}
	// 82DF83E8: 419A001C  beq cr6, 0x82df8404
	if ctx.cr[6].eq {
	pc = 0x82DF8404; continue 'dispatch;
	}
	// 82DF83EC: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF83F0: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF83F4: 7DAAF42E  lfsx f13, r10, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF83F8: 7D8BF42E  lfsx f12, r11, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF83FC: 7D8AF52E  stfsx f12, r10, r30
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 82DF8400: 7DABF52E  stfsx f13, r11, r30
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 82DF8404: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82DF8408: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF840C: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82DF8410: 4099FF68  ble cr6, 0x82df8378
	if !ctx.cr[6].gt {
	pc = 0x82DF8378; continue 'dispatch;
	}
	// 82DF8414: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82DF8418: 40980010  bge cr6, 0x82df8428
	if !ctx.cr[6].lt {
	pc = 0x82DF8428; continue 'dispatch;
	}
	// 82DF841C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DF8420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DF8424: 4BFFFF25  bl 0x82df8348
	ctx.lr = 0x82DF8428;
	sub_82DF8348(ctx, base);
	// 82DF8428: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DF842C: 4098000C  bge cr6, 0x82df8438
	if !ctx.cr[6].lt {
	pc = 0x82DF8438; continue 'dispatch;
	}
	// 82DF8430: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DF8434: 4BFFFF2C  b 0x82df8360
	pc = 0x82DF8360; continue 'dispatch;
	// 82DF8438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DF843C: 4BEB101C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF8440 size=20
    let mut pc: u32 = 0x82DF8440;
    'dispatch: loop {
        match pc {
            0x82DF8440 => {
    //   block [0x82DF8440..0x82DF8454)
	// 82DF8440: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8444: 7C8A0774  extsb r10, r4
	ctx.r[10].s64 = ctx.r[4].s8 as i64;
	// 82DF8448: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF844C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DF8450: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF8454 size=32
    let mut pc: u32 = 0x82DF8454;
    'dispatch: loop {
        match pc {
            0x82DF8454 => {
    //   block [0x82DF8454..0x82DF8474)
	// 82DF8454: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF8458: 419A001C  beq cr6, 0x82df8474
	if ctx.cr[6].eq {
		sub_82DF8474(ctx, base);
		return;
	}
	// 82DF845C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DF8460: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8464: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82DF8468: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DF846C: 409AFFE8  bne cr6, 0x82df8454
	if !ctx.cr[6].eq {
	pc = 0x82DF8454; continue 'dispatch;
	}
	// 82DF8470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DF8474 size=8
    let mut pc: u32 = 0x82DF8474;
    'dispatch: loop {
        match pc {
            0x82DF8474 => {
    //   block [0x82DF8474..0x82DF847C)
	// 82DF8474: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DF8478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DF8480 size=336
    let mut pc: u32 = 0x82DF8480;
    'dispatch: loop {
        match pc {
            0x82DF8480 => {
    //   block [0x82DF8480..0x82DF85D0)
	// 82DF8480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8484: 4BEB0F71  bl 0x82ca93f4
	ctx.lr = 0x82DF8488;
	sub_82CA93D0(ctx, base);
	// 82DF8488: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF848C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF8490: 3F60820A  lis r27, -0x7df6
	ctx.r[27].s64 = -2113273856;
	// 82DF8494: 3B0B47C4  addi r24, r11, 0x47c4
	ctx.r[24].s64 = ctx.r[11].s64 + 18372;
	// 82DF8498: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82DF849C: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82DF84A0: 3BCBEBDC  addi r30, r11, -0x1424
	ctx.r[30].s64 = ctx.r[11].s64 + -5156;
	// 82DF84A4: 80FBEF9C  lwz r7, -0x1064(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4196 as u32) ) } as u64;
	// 82DF84A8: 3D600209  lis r11, 0x209
	ctx.r[11].s64 = 34144256;
	// 82DF84AC: 3F808334  lis r28, -0x7ccc
	ctx.r[28].s64 = -2093744128;
	// 82DF84B0: 54F70FFE  srwi r23, r7, 0x1f
	ctx.r[23].u32 = ctx.r[7].u32.wrapping_shr(31);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 82DF84B4: 6179A5AD  ori r25, r11, 0xa5ad
	ctx.r[25].u64 = ctx.r[11].u64 | 42413;
	// 82DF84B8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DF84BC: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF84C0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DF84C4: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF84C8: 2B0A002E  cmplwi cr6, r10, 0x2e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 46 as u32, &mut ctx.xer);
	// 82DF84CC: 419A0024  beq cr6, 0x82df84f0
	if ctx.cr[6].eq {
	pc = 0x82DF84F0; continue 'dispatch;
	}
	// 82DF84D0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF84D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF84D8: 419A00A0  beq cr6, 0x82df8578
	if ctx.cr[6].eq {
	pc = 0x82DF8578; continue 'dispatch;
	}
	// 82DF84DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF84E0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF84E4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF84E8: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 82DF84EC: 409AFFE8  bne cr6, 0x82df84d4
	if !ctx.cr[6].eq {
	pc = 0x82DF84D4; continue 'dispatch;
	}
	// 82DF84F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF84F4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF84F8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF84FC: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 82DF8500: 419A0020  beq cr6, 0x82df8520
	if ctx.cr[6].eq {
	pc = 0x82DF8520; continue 'dispatch;
	}
	// 82DF8504: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF8508: 419A0070  beq cr6, 0x82df8578
	if ctx.cr[6].eq {
	pc = 0x82DF8578; continue 'dispatch;
	}
	// 82DF850C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF8510: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8514: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF8518: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 82DF851C: 409AFFE8  bne cr6, 0x82df8504
	if !ctx.cr[6].eq {
	pc = 0x82DF8504; continue 'dispatch;
	}
	// 82DF8520: 56EA063E  clrlwi r10, r23, 0x18
	ctx.r[10].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	// 82DF8524: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF8528: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF852C: 419A0060  beq cr6, 0x82df858c
	if ctx.cr[6].eq {
	pc = 0x82DF858C; continue 'dispatch;
	}
	// 82DF8530: 54EB007E  clrlwi r11, r7, 1
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x7FFFFFFFu64;
	// 82DF8534: 7D7F4278  xor r31, r11, r8
	ctx.r[31].u64 = ctx.r[11].u64 ^ ctx.r[8].u64;
	// 82DF8538: 480010C1  bl 0x82df95f8
	ctx.lr = 0x82DF853C;
	sub_82DF95F8(ctx, base);
	// 82DF853C: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 82DF8540: 7F2B1800  cmpd cr6, r11, r3
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[3].s64, &mut ctx.xer);
	// 82DF8544: 40990014  ble cr6, 0x82df8558
	if !ctx.cr[6].gt {
	pc = 0x82DF8558; continue 'dispatch;
	}
	// 82DF8548: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82DF854C: 7F2BC800  cmpd cr6, r11, r25
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[25].s64, &mut ctx.xer);
	// 82DF8550: 40980008  bge cr6, 0x82df8558
	if !ctx.cr[6].lt {
	pc = 0x82DF8558; continue 'dispatch;
	}
	// 82DF8554: 9B5CAD64  stb r26, -0x529c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(-21148 as u32), ctx.r[26].u8 ) };
	// 82DF8558: 80FBEF9C  lwz r7, -0x1064(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-4196 as u32) ) } as u64;
	// 82DF855C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82DF8560: 39780020  addi r11, r24, 0x20
	ctx.r[11].s64 = ctx.r[24].s64 + 32;
	// 82DF8564: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82DF8568: 4198FF54  blt cr6, 0x82df84bc
	if ctx.cr[6].lt {
	pc = 0x82DF84BC; continue 'dispatch;
	}
	// 82DF856C: 897CAD64  lbz r11, -0x529c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(-21148 as u32) ) } as u64;
	// 82DF8570: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8574: 409A0010  bne cr6, 0x82df8584
	if !ctx.cr[6].eq {
	pc = 0x82DF8584; continue 'dispatch;
	}
	// 82DF8578: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF857C: 386B47E8  addi r3, r11, 0x47e8
	ctx.r[3].s64 = ctx.r[11].s64 + 18408;
	// 82DF8580: 4BF5F601  bl 0x82d57b80
	ctx.lr = 0x82DF8584;
	sub_82D57B80(ctx, base);
	// 82DF8584: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF8588: 4BEB0EBC  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
	// 82DF858C: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8590: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF8594: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF8598: 419A0020  beq cr6, 0x82df85b8
	if ctx.cr[6].eq {
	pc = 0x82DF85B8; continue 'dispatch;
	}
	// 82DF859C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF85A0: 1D290017  mulli r9, r9, 0x17
	ctx.r[9].s64 = ctx.r[9].s64 * 23;
	// 82DF85A4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF85A8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82DF85AC: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82DF85B0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DF85B4: 409AFFE8  bne cr6, 0x82df859c
	if !ctx.cr[6].eq {
	pc = 0x82DF859C; continue 'dispatch;
	}
	// 82DF85B8: 7D2B4278  xor r11, r9, r8
	ctx.r[11].u64 = ctx.r[9].u64 ^ ctx.r[8].u64;
	// 82DF85BC: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82DF85C0: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DF85C4: 409AFF98  bne cr6, 0x82df855c
	if !ctx.cr[6].eq {
	pc = 0x82DF855C; continue 'dispatch;
	}
	// 82DF85C8: 9B5CAD64  stb r26, -0x529c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(-21148 as u32), ctx.r[26].u8 ) };
	// 82DF85CC: 4BFFFF90  b 0x82df855c
	pc = 0x82DF855C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF85D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF85D0 size=1348
    let mut pc: u32 = 0x82DF85D0;
    'dispatch: loop {
        match pc {
            0x82DF85D0 => {
    //   block [0x82DF85D0..0x82DF8B14)
	// 82DF85D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF85D4: 4BEB0E31  bl 0x82ca9404
	ctx.lr = 0x82DF85D8;
	sub_82CA93D0(ctx, base);
	// 82DF85D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF85DC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DF85E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DF85E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DF85E8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DF85EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DF85F0: 891B0001  lbz r8, 1(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF85F4: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82DF85F8: 41980108  blt cr6, 0x82df8700
	if ctx.cr[6].lt {
	pc = 0x82DF8700; continue 'dispatch;
	}
	// 82DF85FC: 3948FFFC  addi r10, r8, -4
	ctx.r[10].s64 = ctx.r[8].s64 + -4;
	// 82DF8600: 397D0008  addi r11, r29, 8
	ctx.r[11].s64 = ctx.r[29].s64 + 8;
	// 82DF8604: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8608: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF860C: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF8610: 893F0000  lbz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8614: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF8618: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF861C: 99210053  stb r9, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[9].u8 ) };
	// 82DF8620: 393F0001  addi r9, r31, 1
	ctx.r[9].s64 = ctx.r[31].s64 + 1;
	// 82DF8624: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8628: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF862C: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82DF8630: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8634: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8638: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82DF863C: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8640: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8644: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82DF8648: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF864C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8650: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8654: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82DF8658: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF865C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8660: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DF8664: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82DF8668: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF866C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8670: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82DF8674: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8678: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF867C: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82DF8680: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8684: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8688: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF868C: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82DF8690: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8694: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8698: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF869C: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82DF86A0: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF86A4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF86A8: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82DF86AC: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF86B0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF86B4: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82DF86B8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF86BC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF86C0: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF86C4: 98C10053  stb r6, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[6].u8 ) };
	// 82DF86C8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF86CC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF86D0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF86D4: 98C10052  stb r6, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[6].u8 ) };
	// 82DF86D8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF86DC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF86E0: 3BE90001  addi r31, r9, 1
	ctx.r[31].s64 = ctx.r[9].s64 + 1;
	// 82DF86E4: 98C10051  stb r6, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[6].u8 ) };
	// 82DF86E8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF86EC: 98C10050  stb r6, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u8 ) };
	// 82DF86F0: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF86F4: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF86F8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF86FC: 409AFF14  bne cr6, 0x82df8610
	if !ctx.cr[6].eq {
	pc = 0x82DF8610; continue 'dispatch;
	}
	// 82DF8700: 7F074000  cmpw cr6, r7, r8
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82DF8704: 40980058  bge cr6, 0x82df875c
	if !ctx.cr[6].lt {
	pc = 0x82DF875C; continue 'dispatch;
	}
	// 82DF8708: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF870C: 7D674050  subf r11, r7, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82DF8710: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82DF8714: 893F0000  lbz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8718: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF871C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8720: 99210053  stb r9, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[9].u8 ) };
	// 82DF8724: 393F0001  addi r9, r31, 1
	ctx.r[9].s64 = ctx.r[31].s64 + 1;
	// 82DF8728: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF872C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8730: 99010052  stb r8, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[8].u8 ) };
	// 82DF8734: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8738: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF873C: 3BE90001  addi r31, r9, 1
	ctx.r[31].s64 = ctx.r[9].s64 + 1;
	// 82DF8740: 99010051  stb r8, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[8].u8 ) };
	// 82DF8744: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8748: 99010050  stb r8, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u8 ) };
	// 82DF874C: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8750: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8754: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DF8758: 409AFFBC  bne cr6, 0x82df8714
	if !ctx.cr[6].eq {
	pc = 0x82DF8714; continue 'dispatch;
	}
	// 82DF875C: 3FC08334  lis r30, -0x7ccc
	ctx.r[30].s64 = -2093744128;
	// 82DF8760: 897EAD64  lbz r11, -0x529c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-21148 as u32) ) } as u64;
	// 82DF8764: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8768: 409A0014  bne cr6, 0x82df877c
	if !ctx.cr[6].eq {
	pc = 0x82DF877C; continue 'dispatch;
	}
	// 82DF876C: 4BFFFD15  bl 0x82df8480
	ctx.lr = 0x82DF8770;
	sub_82DF8480(ctx, base);
	// 82DF8770: 897EAD64  lbz r11, -0x529c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-21148 as u32) ) } as u64;
	// 82DF8774: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8778: 419A0394  beq cr6, 0x82df8b0c
	if ctx.cr[6].eq {
	pc = 0x82DF8B0C; continue 'dispatch;
	}
	// 82DF877C: 88FB0000  lbz r7, 0(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8780: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DF8784: C01B0004  lfs f0, 4(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8788: 394B4780  addi r10, r11, 0x4780
	ctx.r[10].s64 = ctx.r[11].s64 + 18304;
	// 82DF878C: 897B0001  lbz r11, 1(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF8790: 54E8103E  rotlwi r8, r7, 2
	ctx.r[8].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82DF8794: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DF8798: 7CABE050  subf r5, r11, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 82DF879C: 7C69EA14  add r3, r9, r29
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 82DF87A0: 2F070008  cmpwi cr6, r7, 8
	ctx.cr[6].compare_i32(ctx.r[7].s32, 8, &mut ctx.xer);
	// 82DF87A4: 7DA8542E  lfsx f13, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF87A8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DF87AC: 419A0254  beq cr6, 0x82df8a00
	if ctx.cr[6].eq {
	pc = 0x82DF8A00; continue 'dispatch;
	}
	// 82DF87B0: 2F070010  cmpwi cr6, r7, 0x10
	ctx.cr[6].compare_i32(ctx.r[7].s32, 16, &mut ctx.xer);
	// 82DF87B4: 419A00C8  beq cr6, 0x82df887c
	if ctx.cr[6].eq {
	pc = 0x82DF887C; continue 'dispatch;
	}
	// 82DF87B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DF87BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF87C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF87C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF87C8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DF87CC: 7D083830  slw r8, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF87D0: 3D080001  addis r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 65536;
	// 82DF87D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82DF87D8: 5504043E  clrlwi r4, r8, 0x10
	ctx.r[4].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82DF87DC: 419A0330  beq cr6, 0x82df8b0c
	if ctx.cr[6].eq {
	pc = 0x82DF8B0C; continue 'dispatch;
	}
	// 82DF87E0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DF87E4: C1A80BFC  lfs f13, 0xbfc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF87E8: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DF87EC: 41980058  blt cr6, 0x82df8844
	if ctx.cr[6].lt {
	pc = 0x82DF8844; continue 'dispatch;
	}
	// 82DF87F0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF87F4: 7D081A14  add r8, r8, r3
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82DF87F8: 7F092840  cmplw cr6, r9, r5
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82DF87FC: 40980030  bge cr6, 0x82df882c
	if !ctx.cr[6].lt {
	pc = 0x82DF882C; continue 'dispatch;
	}
	// 82DF8800: 5486043E  clrlwi r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DF8804: C19B0008  lfs f12, 8(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF8808: 7CC65038  and r6, r6, r10
	ctx.r[6].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82DF880C: 7CC607B4  extsw r6, r6
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 82DF8810: F8C10050  std r6, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u64 ) };
	// 82DF8814: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DF8818: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF881C: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF8820: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8824: ED8B603A  fmadds f12, f11, f0, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8828: D1880000  stfs f12, 0(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF882C: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82DF8830: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8834: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82DF8838: 7D4A3C30  srw r10, r10, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF883C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DF8840: 4098FFB8  bge cr6, 0x82df87f8
	if !ctx.cr[6].lt {
	pc = 0x82DF87F8; continue 'dispatch;
	}
	// 82DF8844: 391F0001  addi r8, r31, 1
	ctx.r[8].s64 = ctx.r[31].s64 + 1;
	// 82DF8848: 88DF0000  lbz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF884C: 7F092840  cmplw cr6, r9, r5
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82DF8850: 3BE80001  addi r31, r8, 1
	ctx.r[31].s64 = ctx.r[8].s64 + 1;
	// 82DF8854: 8BC80000  lbz r30, 0(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8858: 57C8403E  rotlwi r8, r30, 8
	ctx.r[8].u64 = ((ctx.r[30].u32).rotate_left(8)) as u64;
	// 82DF885C: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 82DF8860: 5508043E  clrlwi r8, r8, 0x10
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82DF8864: 7D085830  slw r8, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8868: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF886C: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 82DF8870: 4198FF78  blt cr6, 0x82df87e8
	if ctx.cr[6].lt {
	pc = 0x82DF87E8; continue 'dispatch;
	}
	// 82DF8874: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF8878: 4BEB0BDC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82DF887C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF8880: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DF8884: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82DF8888: C1AB0BFC  lfs f13, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF888C: 4198010C  blt cr6, 0x82df8998
	if ctx.cr[6].lt {
	pc = 0x82DF8998; continue 'dispatch;
	}
	// 82DF8890: 3945FFFC  addi r10, r5, -4
	ctx.r[10].s64 = ctx.r[5].s64 + -4;
	// 82DF8894: C19B0008  lfs f12, 8(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF8898: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82DF889C: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF88A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF88A4: 5547103A  slwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF88A8: 393F0001  addi r9, r31, 1
	ctx.r[9].s64 = ctx.r[31].s64 + 1;
	// 82DF88AC: 891F0000  lbz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF88B0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF88B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF88B8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF88BC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF88C0: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 82DF88C4: 7CC84378  or r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 82DF88C8: 5506043E  clrlwi r6, r8, 0x10
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82DF88CC: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF88D0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF88D4: F8C10050  std r6, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u64 ) };
	// 82DF88D8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF88DC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF88E0: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 82DF88E4: 7CC84378  or r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 82DF88E8: 5506043E  clrlwi r6, r8, 0x10
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82DF88EC: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF88F0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF88F4: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 82DF88F8: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF88FC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8900: 54C6403E  rotlwi r6, r6, 8
	ctx.r[6].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 82DF8904: 7CC84378  or r8, r6, r8
	ctx.r[8].u64 = ctx.r[6].u64 | ctx.r[8].u64;
	// 82DF8908: 5506043E  clrlwi r6, r8, 0x10
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82DF890C: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8910: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF8914: 3BE90001  addi r31, r9, 1
	ctx.r[31].s64 = ctx.r[9].s64 + 1;
	// 82DF8918: F8C10060  std r6, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u64 ) };
	// 82DF891C: 88C90000  lbz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8920: 54C9403E  rotlwi r9, r6, 8
	ctx.r[9].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 82DF8924: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82DF8928: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82DF892C: F9210068  std r9, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u64 ) };
	// 82DF8930: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DF8934: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF8938: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF893C: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8940: ED6B603A  fmadds f11, f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8944: D16BFFF8  stfs f11, -8(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DF8948: C9610058  lfd f11, 0x58(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DF894C: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF8950: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF8954: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8958: ED6B603A  fmadds f11, f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF895C: D16BFFFC  stfs f11, -4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF8960: C9610060  lfd f11, 0x60(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82DF8964: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF8968: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF896C: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8970: ED6B603A  fmadds f11, f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8974: D16B0000  stfs f11, 0(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8978: C9610068  lfd f11, 0x68(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82DF897C: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF8980: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF8984: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8988: ED6B603A  fmadds f11, f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF898C: D16B0004  stfs f11, 4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF8990: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF8994: 409AFF14  bne cr6, 0x82df88a8
	if !ctx.cr[6].eq {
	pc = 0x82DF88A8; continue 'dispatch;
	}
	// 82DF8998: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82DF899C: 40980170  bge cr6, 0x82df8b0c
	if !ctx.cr[6].lt {
	pc = 0x82DF8B0C; continue 'dispatch;
	}
	// 82DF89A0: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF89A4: C19B0008  lfs f12, 8(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF89A8: 7D672850  subf r11, r7, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 82DF89AC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82DF89B0: 393F0001  addi r9, r31, 1
	ctx.r[9].s64 = ctx.r[31].s64 + 1;
	// 82DF89B4: 891F0000  lbz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF89B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF89BC: 3BE90001  addi r31, r9, 1
	ctx.r[31].s64 = ctx.r[9].s64 + 1;
	// 82DF89C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF89C4: 88E90000  lbz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF89C8: 54E9403E  rotlwi r9, r7, 8
	ctx.r[9].u64 = ((ctx.r[7].u32).rotate_left(8)) as u64;
	// 82DF89CC: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82DF89D0: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82DF89D4: F9210068  std r9, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u64 ) };
	// 82DF89D8: C9610068  lfd f11, 0x68(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82DF89DC: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF89E0: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF89E4: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF89E8: ED6B603A  fmadds f11, f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF89EC: D16A0000  stfs f11, 0(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF89F0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DF89F4: 409AFFBC  bne cr6, 0x82df89b0
	if !ctx.cr[6].eq {
	pc = 0x82DF89B0; continue 'dispatch;
	}
	// 82DF89F8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF89FC: 4BEB0A58  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82DF8A00: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF8A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF8A08: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82DF8A0C: C1AB0BFC  lfs f13, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8A10: 419800B0  blt cr6, 0x82df8ac0
	if ctx.cr[6].lt {
	pc = 0x82DF8AC0; continue 'dispatch;
	}
	// 82DF8A14: 3945FFFC  addi r10, r5, -4
	ctx.r[10].s64 = ctx.r[5].s64 + -4;
	// 82DF8A18: C19B0008  lfs f12, 8(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF8A1C: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82DF8A20: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8A24: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF8A28: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF8A2C: 891F0000  lbz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8A30: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF8A34: 88FF0001  lbz r7, 1(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF8A38: 88DF0002  lbz r6, 2(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DF8A3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF8A40: 889F0003  lbz r4, 3(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DF8A44: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82DF8A48: F9010068  std r8, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u64 ) };
	// 82DF8A4C: F8E10060  std r7, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u64 ) };
	// 82DF8A50: F8C10058  std r6, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u64 ) };
	// 82DF8A54: F8810050  std r4, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u64 ) };
	// 82DF8A58: C9610068  lfd f11, 0x68(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82DF8A5C: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF8A60: C9410060  lfd f10, 0x60(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82DF8A64: C9210058  lfd f9, 0x58(r1)
	ctx.f[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DF8A68: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82DF8A6C: C9010050  lfd f8, 0x50(r1)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DF8A70: FD204E9C  fcfid f9, f9
	ctx.f[9].f64 = (ctx.f[9].s64 as f64);
	// 82DF8A74: FD00469C  fcfid f8, f8
	ctx.f[8].f64 = (ctx.f[8].s64 as f64);
	// 82DF8A78: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF8A7C: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82DF8A80: FD204818  frsp f9, f9
	ctx.f[9].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82DF8A84: FD004018  frsp f8, f8
	ctx.f[8].f64 = (ctx.f[8].f64 as f32) as f64;
	// 82DF8A88: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8A8C: ED4A682A  fadds f10, f10, f13
	ctx.f[10].f64 = ((ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8A90: ED29682A  fadds f9, f9, f13
	ctx.f[9].f64 = ((ctx.f[9].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8A94: ED08682A  fadds f8, f8, f13
	ctx.f[8].f64 = ((ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8A98: ED6B603A  fmadds f11, f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8A9C: D16BFFF8  stfs f11, -8(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DF8AA0: ED6A603A  fmadds f11, f10, f0, f12
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8AA4: D16BFFFC  stfs f11, -4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF8AA8: ED69603A  fmadds f11, f9, f0, f12
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8AAC: D16B0000  stfs f11, 0(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8AB0: ED68603A  fmadds f11, f8, f0, f12
	ctx.f[11].f64 = (((ctx.f[8].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8AB4: D16B0004  stfs f11, 4(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF8AB8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF8ABC: 409AFF70  bne cr6, 0x82df8a2c
	if !ctx.cr[6].eq {
	pc = 0x82DF8A2C; continue 'dispatch;
	}
	// 82DF8AC0: 7F092840  cmplw cr6, r9, r5
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82DF8AC4: 40980048  bge cr6, 0x82df8b0c
	if !ctx.cr[6].lt {
	pc = 0x82DF8B0C; continue 'dispatch;
	}
	// 82DF8AC8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8ACC: C19B0008  lfs f12, 8(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF8AD0: 7D692850  subf r11, r9, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 82DF8AD4: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82DF8AD8: 893F0000  lbz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8ADC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF8AE0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DF8AE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8AE8: F9210068  std r9, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u64 ) };
	// 82DF8AEC: C9610068  lfd f11, 0x68(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82DF8AF0: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82DF8AF4: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82DF8AF8: ED6B682A  fadds f11, f11, f13
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[13].f64) as f32) as f64;
	// 82DF8AFC: ED6B603A  fmadds f11, f11, f0, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 82DF8B00: D16A0000  stfs f11, 0(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8B04: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DF8B08: 409AFFD0  bne cr6, 0x82df8ad8
	if !ctx.cr[6].eq {
	pc = 0x82DF8AD8; continue 'dispatch;
	}
	// 82DF8B0C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DF8B10: 4BEB0944  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8B18 size=960
    let mut pc: u32 = 0x82DF8B18;
    'dispatch: loop {
        match pc {
            0x82DF8B18 => {
    //   block [0x82DF8B18..0x82DF8ED8)
	// 82DF8B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF8B1C: 4BEB08D9  bl 0x82ca93f4
	ctx.lr = 0x82DF8B20;
	sub_82CA93D0(ctx, base);
	// 82DF8B20: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8B24: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DF8B28: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8B2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF8B30: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF8B34: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DF8B38: 7D273050  subf r9, r7, r6
	ctx.r[9].s64 = ctx.r[6].s64 - ctx.r[7].s64;
	// 82DF8B3C: 0CC80000  twi 6, r8, 0
	// 82DF8B40: 55291838  slwi r9, r9, 3
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF8B44: 7F094396  divwu r24, r9, r8
	ctx.r[24].u32 = ctx.r[9].u32 / ctx.r[8].u32;
	// 82DF8B48: 419A0024  beq cr6, 0x82df8b6c
	if ctx.cr[6].eq {
	pc = 0x82DF8B6C; continue 'dispatch;
	}
	// 82DF8B4C: 7D251850  subf r9, r5, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[5].s64;
	// 82DF8B50: 7C095C2E  lfsx f0, r9, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8B54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF8B58: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8B5C: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8B60: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF8B64: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DF8B68: 4198FFE8  blt cr6, 0x82df8b50
	if ctx.cr[6].lt {
	pc = 0x82DF8B50; continue 'dispatch;
	}
	// 82DF8B6C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8B70: 39380007  addi r9, r24, 7
	ctx.r[9].s64 = ctx.r[24].s64 + 7;
	// 82DF8B74: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8B78: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DF8B7C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8B80: 553FE8FE  srwi r31, r9, 3
	ctx.r[31].u32 = ctx.r[9].u32.wrapping_shr(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82DF8B84: 7CEA1A14  add r7, r10, r3
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82DF8B88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DF8B8C: 7CAA2A14  add r5, r10, r5
	ctx.r[5].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 82DF8B90: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DF8B94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF8B98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF8B9C: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 82DF8BA0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82DF8BA4: 2B080008  cmplwi cr6, r8, 8
	ctx.cr[6].compare_u32(ctx.r[8].u32, 8 as u32, &mut ctx.xer);
	// 82DF8BA8: 7C634030  slw r3, r3, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[3].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8BAC: 3D430001  addis r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 65536;
	// 82DF8BB0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF8BB4: 555C043E  clrlwi r28, r10, 0x10
	ctx.r[28].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82DF8BB8: 7D5F3A14  add r10, r31, r7
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[7].u64;
	// 82DF8BBC: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8BC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF8BC4: 3BEA0001  addi r31, r10, 1
	ctx.r[31].s64 = ctx.r[10].s64 + 1;
	// 82DF8BC8: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8BCC: 546A403E  rotlwi r10, r3, 8
	ctx.r[10].u64 = ((ctx.r[3].u32).rotate_left(8)) as u64;
	// 82DF8BD0: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82DF8BD4: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82DF8BD8: 419A0230  beq cr6, 0x82df8e08
	if ctx.cr[6].eq {
	pc = 0x82DF8E08; continue 'dispatch;
	}
	// 82DF8BDC: 2B080010  cmplwi cr6, r8, 0x10
	ctx.cr[6].compare_u32(ctx.r[8].u32, 16 as u32, &mut ctx.xer);
	// 82DF8BE0: 419A0160  beq cr6, 0x82df8d40
	if ctx.cr[6].eq {
	pc = 0x82DF8D40; continue 'dispatch;
	}
	// 82DF8BE4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82DF8BE8: 419A0134  beq cr6, 0x82df8d1c
	if ctx.cr[6].eq {
	pc = 0x82DF8D1C; continue 'dispatch;
	}
	// 82DF8BEC: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82DF8BF0: 89190000  lbz r8, 0(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8BF4: 54DD063E  clrlwi r29, r6, 0x18
	ctx.r[29].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8BF8: 7D08E838  and r8, r8, r29
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[29].u64;
	// 82DF8BFC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DF8C00: 419A0054  beq cr6, 0x82df8c54
	if ctx.cr[6].eq {
	pc = 0x82DF8C54; continue 'dispatch;
	}
	// 82DF8C04: 80E40004  lwz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8C08: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82DF8C0C: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8C10: 7CC94214  add r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DF8C14: 7CE84830  slw r8, r7, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[7].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8C18: 54C9063E  clrlwi r9, r6, 0x18
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8C1C: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82DF8C20: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82DF8C24: 2B08000F  cmplwi cr6, r8, 0xf
	ctx.cr[6].compare_u32(ctx.r[8].u32, 15 as u32, &mut ctx.xer);
	// 82DF8C28: 409900D0  ble cr6, 0x82df8cf8
	if !ctx.cr[6].gt {
	pc = 0x82DF8CF8; continue 'dispatch;
	}
	// 82DF8C2C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82DF8C30: 38C800F0  addi r6, r8, 0xf0
	ctx.r[6].s64 = ctx.r[8].s64 + 240;
	// 82DF8C34: 5567C23E  srwi r7, r11, 8
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF8C38: 39050001  addi r8, r5, 1
	ctx.r[8].s64 = ctx.r[5].s64 + 1;
	// 82DF8C3C: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8C40: 99250000  stb r9, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF8C44: 54C9063E  clrlwi r9, r6, 0x18
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8C48: 38A80001  addi r5, r8, 1
	ctx.r[5].s64 = ctx.r[8].s64 + 1;
	// 82DF8C4C: 98E80000  stb r7, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82DF8C50: 480000A8  b 0x82df8cf8
	pc = 0x82DF8CF8; continue 'dispatch;
	// 82DF8C54: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8C58: 57C8063E  clrlwi r8, r30, 0x18
	ctx.r[8].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DF8C5C: 7F081840  cmplw cr6, r8, r3
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82DF8C60: 40980030  bge cr6, 0x82df8c90
	if !ctx.cr[6].lt {
	pc = 0x82DF8C90; continue 'dispatch;
	}
	// 82DF8C64: 38FF0001  addi r7, r31, 1
	ctx.r[7].s64 = ctx.r[31].s64 + 1;
	// 82DF8C68: 88DF0000  lbz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8C6C: 3BE80010  addi r31, r8, 0x10
	ctx.r[31].s64 = ctx.r[8].s64 + 16;
	// 82DF8C70: 57FE063E  clrlwi r30, r31, 0x18
	ctx.r[30].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82DF8C74: 3BE70001  addi r31, r7, 1
	ctx.r[31].s64 = ctx.r[7].s64 + 1;
	// 82DF8C78: 8AE70000  lbz r23, 0(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8C7C: 56E7403E  rotlwi r7, r23, 8
	ctx.r[7].u64 = ((ctx.r[23].u32).rotate_left(8)) as u64;
	// 82DF8C80: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82DF8C84: 54E7043E  clrlwi r7, r7, 0x10
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000FFFFu64;
	// 82DF8C88: 7CE84030  slw r8, r7, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[7].u32) << ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8C8C: 7D0A5378  or r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ctx.r[10].u64;
	// 82DF8C90: 5787043E  clrlwi r7, r28, 0x10
	ctx.r[7].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82DF8C94: 5548043E  clrlwi r8, r10, 0x10
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82DF8C98: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82DF8C9C: 7D083838  and r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[7].u64;
	// 82DF8CA0: 7CE91A14  add r7, r9, r3
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 82DF8CA4: 7D084830  slw r8, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8CA8: 54E9063E  clrlwi r9, r7, 0x18
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82DF8CAC: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82DF8CB0: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82DF8CB4: 2B08000F  cmplwi cr6, r8, 0xf
	ctx.cr[6].compare_u32(ctx.r[8].u32, 15 as u32, &mut ctx.xer);
	// 82DF8CB8: 40990028  ble cr6, 0x82df8ce0
	if !ctx.cr[6].gt {
	pc = 0x82DF8CE0; continue 'dispatch;
	}
	// 82DF8CBC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82DF8CC0: 38C800F0  addi r6, r8, 0xf0
	ctx.r[6].s64 = ctx.r[8].s64 + 240;
	// 82DF8CC4: 5567C23E  srwi r7, r11, 8
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DF8CC8: 39050001  addi r8, r5, 1
	ctx.r[8].s64 = ctx.r[5].s64 + 1;
	// 82DF8CCC: 556B843E  srwi r11, r11, 0x10
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8CD0: 99250000  stb r9, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF8CD4: 54C9063E  clrlwi r9, r6, 0x18
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8CD8: 38A80001  addi r5, r8, 1
	ctx.r[5].s64 = ctx.r[8].s64 + 1;
	// 82DF8CDC: 98E80000  stb r7, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82DF8CE0: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8CE4: 57C7063E  clrlwi r7, r30, 0x18
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DF8CE8: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DF8CEC: 7CE83850  subf r7, r8, r7
	ctx.r[7].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 82DF8CF0: 54FE063E  clrlwi r30, r7, 0x18
	ctx.r[30].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82DF8CF4: 7D4A4430  srw r10, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8CF8: 57A8083C  slwi r8, r29, 1
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF8CFC: 5506063E  clrlwi r6, r8, 0x18
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82DF8D00: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DF8D04: 409A000C  bne cr6, 0x82df8d10
	if !ctx.cr[6].eq {
	pc = 0x82DF8D10; continue 'dispatch;
	}
	// 82DF8D08: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DF8D0C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82DF8D10: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82DF8D14: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DF8D18: 409AFED8  bne cr6, 0x82df8bf0
	if !ctx.cr[6].eq {
	pc = 0x82DF8BF0; continue 'dispatch;
	}
	// 82DF8D1C: 552A063E  clrlwi r10, r9, 0x18
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82DF8D20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF8D24: 419A0190  beq cr6, 0x82df8eb4
	if ctx.cr[6].eq {
	pc = 0x82DF8EB4; continue 'dispatch;
	}
	// 82DF8D28: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 82DF8D2C: 99650000  stb r11, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DF8D30: 40990184  ble cr6, 0x82df8eb4
	if !ctx.cr[6].gt {
	pc = 0x82DF8EB4; continue 'dispatch;
	}
	// 82DF8D34: 556BC23E  srwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8D38: 99650001  stb r11, 1(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82DF8D3C: 48000178  b 0x82df8eb4
	pc = 0x82DF8EB4; continue 'dispatch;
	// 82DF8D40: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8D44: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82DF8D48: 88640006  lbz r3, 6(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82DF8D4C: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF8D50: 419A0164  beq cr6, 0x82df8eb4
	if ctx.cr[6].eq {
	pc = 0x82DF8EB4; continue 'dispatch;
	}
	// 82DF8D54: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82DF8D58: 89790000  lbz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8D5C: 54C7063E  clrlwi r7, r6, 0x18
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8D60: 7D6B3838  and r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	// 82DF8D64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8D68: 419A0014  beq cr6, 0x82df8d7c
	if ctx.cr[6].eq {
	pc = 0x82DF8D7C; continue 'dispatch;
	}
	// 82DF8D6C: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 82DF8D70: 9BA50000  stb r29, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82DF8D74: 986B0000  stb r3, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 82DF8D78: 48000064  b 0x82df8ddc
	pc = 0x82DF8DDC; continue 'dispatch;
	// 82DF8D7C: 57C9063E  clrlwi r9, r30, 0x18
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DF8D80: 2B090010  cmplwi cr6, r9, 0x10
	ctx.cr[6].compare_u32(ctx.r[9].u32, 16 as u32, &mut ctx.xer);
	// 82DF8D84: 40980030  bge cr6, 0x82df8db4
	if !ctx.cr[6].lt {
	pc = 0x82DF8DB4; continue 'dispatch;
	}
	// 82DF8D88: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82DF8D8C: 891F0000  lbz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8D90: 38C90010  addi r6, r9, 0x10
	ctx.r[6].s64 = ctx.r[9].s64 + 16;
	// 82DF8D94: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82DF8D98: 54DE063E  clrlwi r30, r6, 0x18
	ctx.r[30].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8D9C: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8DA0: 54CB403E  rotlwi r11, r6, 8
	ctx.r[11].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 82DF8DA4: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 82DF8DA8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DF8DAC: 7D6B4830  slw r11, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8DB0: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DF8DB4: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82DF8DB8: 57C9063E  clrlwi r9, r30, 0x18
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DF8DBC: 5568C63E  rlwinm r8, r11, 0x18, 0x18, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF8DC0: 392900F0  addi r9, r9, 0xf0
	ctx.r[9].s64 = ctx.r[9].s64 + 240;
	// 82DF8DC4: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8DC8: 99650000  stb r11, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DF8DCC: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 82DF8DD0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DF8DD4: 553E063E  clrlwi r30, r9, 0x18
	ctx.r[30].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82DF8DD8: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82DF8DDC: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82DF8DE0: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8DE4: 5566063E  clrlwi r6, r11, 0x18
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF8DE8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DF8DEC: 409A000C  bne cr6, 0x82df8df8
	if !ctx.cr[6].eq {
	pc = 0x82DF8DF8; continue 'dispatch;
	}
	// 82DF8DF0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DF8DF4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82DF8DF8: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DF8DFC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DF8E00: 409AFF58  bne cr6, 0x82df8d58
	if !ctx.cr[6].eq {
	pc = 0x82DF8D58; continue 'dispatch;
	}
	// 82DF8E04: 480000B0  b 0x82df8eb4
	pc = 0x82DF8EB4; continue 'dispatch;
	// 82DF8E08: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82DF8E0C: 419A00A8  beq cr6, 0x82df8eb4
	if ctx.cr[6].eq {
	pc = 0x82DF8EB4; continue 'dispatch;
	}
	// 82DF8E10: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DF8E14: 89790000  lbz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8E18: 54C7063E  clrlwi r7, r6, 0x18
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8E1C: 7D6B3838  and r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	// 82DF8E20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8E24: 419A0010  beq cr6, 0x82df8e34
	if ctx.cr[6].eq {
	pc = 0x82DF8E34; continue 'dispatch;
	}
	// 82DF8E28: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF8E2C: 99650000  stb r11, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DF8E30: 4800005C  b 0x82df8e8c
	pc = 0x82DF8E8C; continue 'dispatch;
	// 82DF8E34: 57C9063E  clrlwi r9, r30, 0x18
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DF8E38: 2B090008  cmplwi cr6, r9, 8
	ctx.cr[6].compare_u32(ctx.r[9].u32, 8 as u32, &mut ctx.xer);
	// 82DF8E3C: 40980030  bge cr6, 0x82df8e6c
	if !ctx.cr[6].lt {
	pc = 0x82DF8E6C; continue 'dispatch;
	}
	// 82DF8E40: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82DF8E44: 891F0000  lbz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8E48: 38C90010  addi r6, r9, 0x10
	ctx.r[6].s64 = ctx.r[9].s64 + 16;
	// 82DF8E4C: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82DF8E50: 54DE063E  clrlwi r30, r6, 0x18
	ctx.r[30].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82DF8E54: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8E58: 54CB403E  rotlwi r11, r6, 8
	ctx.r[11].u64 = ((ctx.r[6].u32).rotate_left(8)) as u64;
	// 82DF8E5C: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 82DF8E60: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82DF8E64: 7D6B4830  slw r11, r11, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[11].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF8E68: 7D6A5378  or r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82DF8E6C: 5789043E  clrlwi r9, r28, 0x10
	ctx.r[9].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82DF8E70: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DF8E74: 7D295038  and r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 82DF8E78: 396B00F8  addi r11, r11, 0xf8
	ctx.r[11].s64 = ctx.r[11].s64 + 248;
	// 82DF8E7C: 554AC23E  srwi r10, r10, 8
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8E80: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF8E84: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82DF8E88: 99250000  stb r9, 0(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF8E8C: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8E90: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 82DF8E94: 5566063E  clrlwi r6, r11, 0x18
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82DF8E98: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82DF8E9C: 409A000C  bne cr6, 0x82df8ea8
	if !ctx.cr[6].eq {
	pc = 0x82DF8EA8; continue 'dispatch;
	}
	// 82DF8EA0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82DF8EA4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82DF8EA8: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 82DF8EAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DF8EB0: 409AFF64  bne cr6, 0x82df8e14
	if !ctx.cr[6].eq {
	pc = 0x82DF8E14; continue 'dispatch;
	}
	// 82DF8EB4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8EB8: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DF8EBC: 7D6BD1D6  mullw r11, r11, r26
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[26].s32 as i64);
	// 82DF8EC0: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF8EC4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DF8EC8: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82DF8ECC: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 82DF8ED0: 5563E8FE  srwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DF8ED4: 4BEB0570  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8ED8 size=40
    let mut pc: u32 = 0x82DF8ED8;
    'dispatch: loop {
        match pc {
            0x82DF8ED8 => {
    //   block [0x82DF8ED8..0x82DF8F00)
	// 82DF8ED8: 89640001  lbz r11, 1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF8EDC: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF8EE0: 7D0B1850  subf r8, r11, r3
	ctx.r[8].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82DF8EE4: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DF8EE8: 7D6849D6  mullw r11, r8, r9
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82DF8EEC: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 82DF8EF0: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82DF8EF4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82DF8EF8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DF8EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8F00 size=28
    let mut pc: u32 = 0x82DF8F00;
    'dispatch: loop {
        match pc {
            0x82DF8F00 => {
    //   block [0x82DF8F00..0x82DF8F1C)
	// 82DF8F00: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82DF8F04: 41990018  bgt cr6, 0x82df8f1c
	if ctx.cr[6].gt {
		sub_82DF8F1C(ctx, base);
		return;
	}
	// 82DF8F08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DF8F0C: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8F10: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F14: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8F1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8F1C size=72
    let mut pc: u32 = 0x82DF8F1C;
    'dispatch: loop {
        match pc {
            0x82DF8F1C => {
    //   block [0x82DF8F1C..0x82DF8F64)
	// 82DF8F1C: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8F20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DF8F24: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F28: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82DF8F2C: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8F30: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F34: 419800C8  blt cr6, 0x82df8ffc
	if ctx.cr[6].lt {
		sub_82DF8FDC(ctx, base);
		return;
	}
	// 82DF8F38: 3964FFFC  addi r11, r4, -4
	ctx.r[11].s64 = ctx.r[4].s64 + -4;
	// 82DF8F3C: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 82DF8F40: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF8F44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF8F48: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF8F4C: C00AFFF8  lfs f0, -8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8F50: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8F54: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8F58: 4098000C  bge cr6, 0x82df8f64
	if !ctx.cr[6].lt {
		sub_82DF8F64(ctx, base);
		return;
	}
	// 82DF8F5C: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F60: 48000014  b 0x82df8f74
	sub_82DF8F64(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8F64 size=40
    let mut pc: u32 = 0x82DF8F64;
    'dispatch: loop {
        match pc {
            0x82DF8F64 => {
    //   block [0x82DF8F64..0x82DF8F8C)
	// 82DF8F64: C1A60000  lfs f13, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8F68: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8F6C: 40990008  ble cr6, 0x82df8f74
	if !ctx.cr[6].gt {
	pc = 0x82DF8F74; continue 'dispatch;
	}
	// 82DF8F70: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F74: C00AFFFC  lfs f0, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8F78: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8F7C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8F80: 4098000C  bge cr6, 0x82df8f8c
	if !ctx.cr[6].lt {
		sub_82DF8F8C(ctx, base);
		return;
	}
	// 82DF8F84: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F88: 48000014  b 0x82df8f9c
	sub_82DF8F8C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8F8C size=40
    let mut pc: u32 = 0x82DF8F8C;
    'dispatch: loop {
        match pc {
            0x82DF8F8C => {
    //   block [0x82DF8F8C..0x82DF8FB4)
	// 82DF8F8C: C1A60000  lfs f13, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8F90: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8F94: 40990008  ble cr6, 0x82df8f9c
	if !ctx.cr[6].gt {
	pc = 0x82DF8F9C; continue 'dispatch;
	}
	// 82DF8F98: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8F9C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8FA0: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8FA4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8FA8: 4098000C  bge cr6, 0x82df8fb4
	if !ctx.cr[6].lt {
		sub_82DF8FB4(ctx, base);
		return;
	}
	// 82DF8FAC: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8FB0: 48000014  b 0x82df8fc4
	sub_82DF8FB4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8FB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8FB4 size=40
    let mut pc: u32 = 0x82DF8FB4;
    'dispatch: loop {
        match pc {
            0x82DF8FB4 => {
    //   block [0x82DF8FB4..0x82DF8FDC)
	// 82DF8FB4: C1A60000  lfs f13, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8FB8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8FBC: 40990008  ble cr6, 0x82df8fc4
	if !ctx.cr[6].gt {
	pc = 0x82DF8FC4; continue 'dispatch;
	}
	// 82DF8FC0: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8FC4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF8FC8: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8FCC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8FD0: 4098000C  bge cr6, 0x82df8fdc
	if !ctx.cr[6].lt {
		sub_82DF8FDC(ctx, base);
		return;
	}
	// 82DF8FD4: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8FD8: 48000014  b 0x82df8fec
	sub_82DF8FDC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF8FDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF8FDC size=40
    let mut pc: u32 = 0x82DF8FDC;
    'dispatch: loop {
        match pc {
            0x82DF8FDC => {
    //   block [0x82DF8FDC..0x82DF9004)
	// 82DF8FDC: C1A60000  lfs f13, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF8FE0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF8FE4: 40990008  ble cr6, 0x82df8fec
	if !ctx.cr[6].gt {
	pc = 0x82DF8FEC; continue 'dispatch;
	}
	// 82DF8FE8: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF8FEC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DF8FF0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DF8FF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF8FF8: 409AFF54  bne cr6, 0x82df8f4c
	if !ctx.cr[6].eq {
		sub_82DF8F1C(ctx, base);
		return;
	}
	// 82DF8FFC: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82DF9000: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF9004 size=36
    let mut pc: u32 = 0x82DF9004;
    'dispatch: loop {
        match pc {
            0x82DF9004 => {
    //   block [0x82DF9004..0x82DF9028)
	// 82DF9004: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF9008: 7D492050  subf r10, r9, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82DF900C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DF9010: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF9014: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF9018: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF901C: 4098000C  bge cr6, 0x82df9028
	if !ctx.cr[6].lt {
		sub_82DF9028(ctx, base);
		return;
	}
	// 82DF9020: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF9024: 48000014  b 0x82df9038
	sub_82DF9028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF9028 size=36
    let mut pc: u32 = 0x82DF9028;
    'dispatch: loop {
        match pc {
            0x82DF9028 => {
    //   block [0x82DF9028..0x82DF904C)
	// 82DF9028: C1A60000  lfs f13, 0(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF902C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DF9030: 40990008  ble cr6, 0x82df9038
	if !ctx.cr[6].gt {
	pc = 0x82DF9038; continue 'dispatch;
	}
	// 82DF9034: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF9038: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF903C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DF9040: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF9044: 409AFFCC  bne cr6, 0x82df9010
	if !ctx.cr[6].eq {
		sub_82DF9004(ctx, base);
		return;
	}
	// 82DF9048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF9050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DF9050 size=136
    let mut pc: u32 = 0x82DF9050;
    'dispatch: loop {
        match pc {
            0x82DF9050 => {
    //   block [0x82DF9050..0x82DF90D8)
	// 82DF9050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF9054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DF9058: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DF905C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DF9060: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82DF9064: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF9068: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DF906C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82DF9070: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DF9074: 7C872050  subf r4, r7, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[7].s64;
	// 82DF9078: 98E80001  stb r7, 1(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 82DF907C: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DF9080: 4BFFFE81  bl 0x82df8f00
	ctx.lr = 0x82DF9084;
	sub_82DF8F00(ctx, base);
	// 82DF9084: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF9088: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF908C: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82DF9090: D0080008  stfs f0, 8(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82DF9094: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DF9098: D0080004  stfs f0, 4(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82DF909C: 2B0B000F  cmplwi cr6, r11, 0xf
	ctx.cr[6].compare_u32(ctx.r[11].u32, 15 as u32, &mut ctx.xer);
	// 82DF90A0: 4199001C  bgt cr6, 0x82df90bc
	if ctx.cr[6].gt {
	pc = 0x82DF90BC; continue 'dispatch;
	}
	// 82DF90A4: 9BE80000  stb r31, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 82DF90A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF90AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF90B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF90B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF90B8: 4E800020  blr
	return;
	// 82DF90BC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DF90C0: 99680000  stb r11, 0(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DF90C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DF90C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DF90CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DF90D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DF90D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF90D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF90D8 size=984
    let mut pc: u32 = 0x82DF90D8;
    'dispatch: loop {
        match pc {
            0x82DF90D8 => {
    //   block [0x82DF90D8..0x82DF94B0)
	// 82DF90D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DF90DC: 4BEB032D  bl 0x82ca9408
	ctx.lr = 0x82DF90E0;
	sub_82CA93D0(ctx, base);
	// 82DF90E0: 89450001  lbz r10, 1(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF90E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF90E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF90EC: 419A0054  beq cr6, 0x82df9140
	if ctx.cr[6].eq {
	pc = 0x82DF9140; continue 'dispatch;
	}
	// 82DF90F0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DF90F4: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF90F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DF90FC: D001FFC8  stfs f0, -0x38(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 82DF9100: 8921FFCB  lbz r9, -0x35(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-53 as u32) ) } as u64;
	// 82DF9104: 8901FFCA  lbz r8, -0x36(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-54 as u32) ) } as u64;
	// 82DF9108: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DF910C: 88E1FFC9  lbz r7, -0x37(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-55 as u32) ) } as u64;
	// 82DF9110: 8BE1FFC8  lbz r31, -0x38(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 82DF9114: 99260000  stb r9, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF9118: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 82DF911C: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82DF9120: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF9124: 98E90000  stb r7, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82DF9128: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF912C: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 82DF9130: 9BE90000  stb r31, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 82DF9134: 89250001  lbz r9, 1(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF9138: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DF913C: 4198FFB8  blt cr6, 0x82df90f4
	if ctx.cr[6].lt {
	pc = 0x82DF90F4; continue 'dispatch;
	}
	// 82DF9140: 88E50000  lbz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82DF9148: 89250001  lbz r9, 1(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DF914C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DF9150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DF9154: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82DF9158: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF915C: 7D1D3830  slw r29, r8, r7
	if (ctx.r[7].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[8].u32) << ((ctx.r[7].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF9160: 7D092050  subf r8, r9, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82DF9164: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DF9168: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82DF916C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DF9170: C1690C38  lfs f11, 0xc38(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3128 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF9174: C1480C18  lfs f10, 0xc18(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DF9178: 41980254  blt cr6, 0x82df93cc
	if ctx.cr[6].lt {
	pc = 0x82DF93CC; continue 'dispatch;
	}
	// 82DF917C: 7BA70020  clrldi r7, r29, 0x20
	ctx.r[7].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 82DF9180: 7D3E2050  subf r9, r30, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[30].s64;
	// 82DF9184: 391E0002  addi r8, r30, 2
	ctx.r[8].s64 = ctx.r[30].s64 + 2;
	// 82DF9188: 3929FFFC  addi r9, r9, -4
	ctx.r[9].s64 = ctx.r[9].s64 + -4;
	// 82DF918C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DF9190: F8E1FFC8  std r7, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[7].u64 ) };
	// 82DF9194: C801FFC8  lfd f0, -0x38(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DF9198: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DF919C: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF91A0: 38FDFFFF  addi r7, r29, -1
	ctx.r[7].s64 = ctx.r[29].s64 + -1;
	// 82DF91A4: 3B890001  addi r28, r9, 1
	ctx.r[28].s64 = ctx.r[9].s64 + 1;
	// 82DF91A8: 7FE81A14  add r31, r8, r3
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[3].u64;
	// 82DF91AC: 5789103A  slwi r9, r28, 2
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF91B0: 7FC9F214  add r30, r9, r30
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82DF91B4: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DF91B8: C1BFFFF8  lfs f13, -8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF91BC: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DF91C0: EDAD4828  fsubs f13, f13, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 82DF91C4: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF91C8: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DF91CC: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DF91D0: 40980008  bge cr6, 0x82df91d8
	if !ctx.cr[6].lt {
	pc = 0x82DF91D8; continue 'dispatch;
	}
	// 82DF91D4: FDA05090  fmr f13, f10
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82DF91D8: EC0C0372  fmuls f0, f12, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DF91DC: 3921FFC8  addi r9, r1, -0x38
	ctx.r[9].s64 = ctx.r[1].s64 + -56;
	// 82DF91E0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DF91E4: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82DF91E8: 8121FFC8  lwz r9, -0x38(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 82DF91EC: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DF91F0: 40990008  ble cr6, 0x82df91f8
	if !ctx.cr[6].gt {
	pc = 0x82DF91F8; continue 'dispatch;
	}
	// 82DF91F4: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82DF91F8: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF91FC: 89050000  lbz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9200: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82DF9204: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DF9208: 9121FFC8  stw r9, -0x38(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[9].u32 ) };
	// 82DF920C: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82DF9210: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF9214: 40990024  ble cr6, 0x82df9238
	if !ctx.cr[6].gt {
	pc = 0x82DF9238; continue 'dispatch;
	}
	// 82DF9218: 8921FFC3  lbz r9, -0x3d(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-61 as u32) ) } as u64;
	// 82DF921C: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF9220: 8901FFC2  lbz r8, -0x3e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-62 as u32) ) } as u64;
	// 82DF9224: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DF9228: 99260000  stb r9, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF922C: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 82DF9230: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 82DF9234: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82DF9238: C1BFFFFC  lfs f13, -4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF923C: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DF9240: EDAD4828  fsubs f13, f13, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 82DF9244: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF9248: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DF924C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DF9250: 40980008  bge cr6, 0x82df9258
	if !ctx.cr[6].lt {
	pc = 0x82DF9258; continue 'dispatch;
	}
	// 82DF9254: FDA05090  fmr f13, f10
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82DF9258: EC0C0372  fmuls f0, f12, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DF925C: 3921FFC8  addi r9, r1, -0x38
	ctx.r[9].s64 = ctx.r[1].s64 + -56;
	// 82DF9260: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DF9264: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82DF9268: 8121FFC8  lwz r9, -0x38(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 82DF926C: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DF9270: 40990008  ble cr6, 0x82df9278
	if !ctx.cr[6].gt {
	pc = 0x82DF9278; continue 'dispatch;
	}
	// 82DF9274: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82DF9278: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF927C: 89050000  lbz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9280: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82DF9284: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DF9288: 9121FFC8  stw r9, -0x38(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[9].u32 ) };
	// 82DF928C: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82DF9290: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF9294: 40990024  ble cr6, 0x82df92b8
	if !ctx.cr[6].gt {
	pc = 0x82DF92B8; continue 'dispatch;
	}
	// 82DF9298: 8921FFC3  lbz r9, -0x3d(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-61 as u32) ) } as u64;
	// 82DF929C: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF92A0: 8901FFC2  lbz r8, -0x3e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-62 as u32) ) } as u64;
	// 82DF92A4: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DF92A8: 99260000  stb r9, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF92AC: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 82DF92B0: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 82DF92B4: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82DF92B8: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF92BC: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DF92C0: EDAD4828  fsubs f13, f13, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 82DF92C4: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF92C8: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DF92CC: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DF92D0: 40980008  bge cr6, 0x82df92d8
	if !ctx.cr[6].lt {
	pc = 0x82DF92D8; continue 'dispatch;
	}
	// 82DF92D4: FDA05090  fmr f13, f10
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82DF92D8: EC0C0372  fmuls f0, f12, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DF92DC: 3921FFC8  addi r9, r1, -0x38
	ctx.r[9].s64 = ctx.r[1].s64 + -56;
	// 82DF92E0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DF92E4: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82DF92E8: 8121FFC8  lwz r9, -0x38(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 82DF92EC: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DF92F0: 40990008  ble cr6, 0x82df92f8
	if !ctx.cr[6].gt {
	pc = 0x82DF92F8; continue 'dispatch;
	}
	// 82DF92F4: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82DF92F8: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF92FC: 89050000  lbz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9300: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82DF9304: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DF9308: 9121FFC8  stw r9, -0x38(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[9].u32 ) };
	// 82DF930C: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82DF9310: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF9314: 40990024  ble cr6, 0x82df9338
	if !ctx.cr[6].gt {
	pc = 0x82DF9338; continue 'dispatch;
	}
	// 82DF9318: 8921FFC3  lbz r9, -0x3d(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-61 as u32) ) } as u64;
	// 82DF931C: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF9320: 8901FFC2  lbz r8, -0x3e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-62 as u32) ) } as u64;
	// 82DF9324: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DF9328: 99260000  stb r9, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF932C: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 82DF9330: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 82DF9334: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82DF9338: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF933C: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DF9340: EDAD4828  fsubs f13, f13, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 82DF9344: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF9348: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DF934C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DF9350: 40980008  bge cr6, 0x82df9358
	if !ctx.cr[6].lt {
	pc = 0x82DF9358; continue 'dispatch;
	}
	// 82DF9354: FDA05090  fmr f13, f10
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82DF9358: EC0C0372  fmuls f0, f12, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DF935C: 3921FFC8  addi r9, r1, -0x38
	ctx.r[9].s64 = ctx.r[1].s64 + -56;
	// 82DF9360: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DF9364: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82DF9368: 8121FFC8  lwz r9, -0x38(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 82DF936C: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DF9370: 40990008  ble cr6, 0x82df9378
	if !ctx.cr[6].gt {
	pc = 0x82DF9378; continue 'dispatch;
	}
	// 82DF9374: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82DF9378: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF937C: 89050000  lbz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9380: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82DF9384: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DF9388: 9121FFC8  stw r9, -0x38(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[9].u32 ) };
	// 82DF938C: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82DF9390: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF9394: 40990028  ble cr6, 0x82df93bc
	if !ctx.cr[6].gt {
	pc = 0x82DF93BC; continue 'dispatch;
	}
	// 82DF9398: 8921FFC3  lbz r9, -0x3d(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-61 as u32) ) } as u64;
	// 82DF939C: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF93A0: 8901FFC2  lbz r8, -0x3e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-62 as u32) ) } as u64;
	// 82DF93A4: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DF93A8: 99260000  stb r9, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF93AC: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 82DF93B0: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF93B4: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 82DF93B8: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82DF93BC: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82DF93C0: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82DF93C4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82DF93C8: 409AFDF0  bne cr6, 0x82df91b8
	if !ctx.cr[6].eq {
	pc = 0x82DF91B8; continue 'dispatch;
	}
	// 82DF93CC: 7F1E2040  cmplw cr6, r30, r4
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DF93D0: 409800BC  bge cr6, 0x82df948c
	if !ctx.cr[6].lt {
	pc = 0x82DF948C; continue 'dispatch;
	}
	// 82DF93D4: 7BA80020  clrldi r8, r29, 0x20
	ctx.r[8].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 82DF93D8: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF93DC: 3BFDFFFF  addi r31, r29, -1
	ctx.r[31].s64 = ctx.r[29].s64 + -1;
	// 82DF93E0: 7CE91A14  add r7, r9, r3
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 82DF93E4: 7C9E2050  subf r4, r30, r4
	ctx.r[4].s64 = ctx.r[4].s64 - ctx.r[30].s64;
	// 82DF93E8: F901FFC0  std r8, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[8].u64 ) };
	// 82DF93EC: C801FFC0  lfd f0, -0x40(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DF93F0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DF93F4: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DF93F8: C1A70000  lfs f13, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF93FC: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DF9400: EDAD4828  fsubs f13, f13, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 82DF9404: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF9408: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82DF940C: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82DF9410: 40980008  bge cr6, 0x82df9418
	if !ctx.cr[6].lt {
	pc = 0x82DF9418; continue 'dispatch;
	}
	// 82DF9414: FDA05090  fmr f13, f10
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82DF9418: EC0C0372  fmuls f0, f12, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82DF941C: 3921FFC8  addi r9, r1, -0x38
	ctx.r[9].s64 = ctx.r[1].s64 + -56;
	// 82DF9420: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DF9424: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82DF9428: 8121FFC8  lwz r9, -0x38(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) } as u64;
	// 82DF942C: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DF9430: 40990008  ble cr6, 0x82df9438
	if !ctx.cr[6].gt {
	pc = 0x82DF9438; continue 'dispatch;
	}
	// 82DF9434: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82DF9438: 7D295830  slw r9, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82DF943C: 89050000  lbz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF9440: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82DF9444: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82DF9448: 9121FFC8  stw r9, -0x38(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[9].u32 ) };
	// 82DF944C: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82DF9450: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF9454: 40990028  ble cr6, 0x82df947c
	if !ctx.cr[6].gt {
	pc = 0x82DF947C; continue 'dispatch;
	}
	// 82DF9458: 8921FFC3  lbz r9, -0x3d(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-61 as u32) ) } as u64;
	// 82DF945C: 554A843E  srwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF9460: 8901FFC2  lbz r8, -0x3e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-62 as u32) ) } as u64;
	// 82DF9464: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DF9468: 99260000  stb r9, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82DF946C: 39260001  addi r9, r6, 1
	ctx.r[9].s64 = ctx.r[6].s64 + 1;
	// 82DF9470: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82DF9474: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 82DF9478: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82DF947C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 82DF9480: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 82DF9484: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DF9488: 409AFF70  bne cr6, 0x82df93f8
	if !ctx.cr[6].eq {
	pc = 0x82DF93F8; continue 'dispatch;
	}
	// 82DF948C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DF9490: 4099001C  ble cr6, 0x82df94ac
	if !ctx.cr[6].gt {
	pc = 0x82DF94AC; continue 'dispatch;
	}
	// 82DF9494: 8941FFC3  lbz r10, -0x3d(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-61 as u32) ) } as u64;
	// 82DF9498: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82DF949C: 99460000  stb r10, 0(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82DF94A0: 4099000C  ble cr6, 0x82df94ac
	if !ctx.cr[6].gt {
	pc = 0x82DF94AC; continue 'dispatch;
	}
	// 82DF94A4: 8961FFC2  lbz r11, -0x3e(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-62 as u32) ) } as u64;
	// 82DF94A8: 99660001  stb r11, 1(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82DF94AC: 4BEAFFAC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF94B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF94B0 size=124
    let mut pc: u32 = 0x82DF94B0;
    'dispatch: loop {
        match pc {
            0x82DF94B0 => {
    //   block [0x82DF94B0..0x82DF952C)
	// 82DF94B0: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DF94B4: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82DF94B8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82DF94BC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DF94C0: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82DF94C4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82DF94C8: 4198005C  blt cr6, 0x82df9524
	if ctx.cr[6].lt {
	pc = 0x82DF9524; continue 'dispatch;
	}
	// 82DF94CC: 3944FFFB  addi r10, r4, -5
	ctx.r[10].s64 = ctx.r[4].s64 + -5;
	// 82DF94D0: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DF94D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DF94D8: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DF94DC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82DF94E0: C00BFFFC  lfs f0, -4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF94E4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF94E8: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DF94EC: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF94F0: ED6B0028  fsubs f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82DF94F4: C18BFFF4  lfs f12, -0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DF94F8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DF94FC: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82DF9500: EC0D6028  fsubs f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DF9504: C14BFFF0  lfs f10, -0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DF9508: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF950C: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 82DF9510: EC0C5028  fsubs f0, f12, f10
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[10].f64) as f32) as f64);
	// 82DF9514: D16B0000  stfs f11, 0(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF9518: D00BFFF4  stfs f0, -0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82DF951C: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82DF9520: 409AFFC0  bne cr6, 0x82df94e0
	if !ctx.cr[6].eq {
	pc = 0x82DF94E0; continue 'dispatch;
	}
	// 82DF9524: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DF9528: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DF952C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF952C size=40
    let mut pc: u32 = 0x82DF952C;
    'dispatch: loop {
        match pc {
            0x82DF952C => {
    //   block [0x82DF952C..0x82DF9554)
	// 82DF952C: 7D492050  subf r10, r9, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82DF9530: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DF9534: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DF9538: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DF953C: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82DF9540: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DF9544: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82DF9548: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DF954C: 409AFFE4  bne cr6, 0x82df9530
	if !ctx.cr[6].eq {
	pc = 0x82DF9530; continue 'dispatch;
	}
	// 82DF9550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


